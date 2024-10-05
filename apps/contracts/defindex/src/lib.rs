#![no_std]
use investment::{execute_investment, prepare_investment};
use soroban_sdk::{
    contract, contractimpl, panic_with_error,
    token::{TokenClient, TokenInterface},
    Address, Env, Map, String, Vec,
};
use soroban_token_sdk::metadata::TokenMetadata;

mod access;
mod error;
mod events;
mod fee;
mod funds;
mod interface;
mod investment;
mod models;
mod storage;
mod strategies;
mod test;
mod token;
mod utils;

use access::{AccessControl, AccessControlTrait, RolesDataKey};
use funds::{fetch_current_idle_funds, fetch_current_invested_funds, fetch_total_managed_funds};
use interface::{AdminInterfaceTrait, VaultTrait, VaultManagementTrait};
use models::{AssetAllocation, Investment};
use storage::{
    get_assets, set_asset, set_defindex_receiver, set_factory, set_total_assets
};
use strategies::{get_strategy_asset, get_strategy_client, get_strategy_struct, pause_strategy, unpause_strategy};
use token::{internal_mint, internal_burn, write_metadata, VaultToken};
use utils::{
    calculate_deposit_amounts_and_shares_to_mint, calculate_withdrawal_amounts, check_initialized,
    check_nonnegative_amount,
};

pub use error::ContractError;

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {
    fn initialize(
        e: Env,
        assets: Vec<AssetAllocation>,
        manager: Address,
        emergency_manager: Address,
        fee_receiver: Address,
        defindex_receiver: Address,
        factory: Address,
    ) -> Result<(), ContractError> {
        let access_control = AccessControl::new(&e);
        if access_control.has_role(&RolesDataKey::Manager) {
            panic_with_error!(&e, ContractError::AlreadyInitialized);
        }

        access_control.set_role(&RolesDataKey::EmergencyManager, &emergency_manager);
        access_control.set_role(&RolesDataKey::FeeReceiver, &fee_receiver);
        access_control.set_role(&RolesDataKey::Manager, &manager);

        // Set Paltalabs Fee Receiver
        set_defindex_receiver(&e, &defindex_receiver);

        // Set the factory address
        set_factory(&e, &factory);

        // Store Assets Objects
        let total_assets = assets.len();
        set_total_assets(&e, total_assets as u32);
        for (i, asset) in assets.iter().enumerate() {
            // for every asset, we need to check that the list of strategyes indeed support this asset
            
            // TODO Fix, currently failing
            // for strategy in asset.strategies.iter() {
            //     let strategy_client = DeFindexStrategyClient::new(&e, &strategy.address);
            //     if strategy_client.asset() != asset.address {
            //         panic_with_error!(&e, ContractError::StrategyDoesNotSupportAsset);
            //     }
            // }
            set_asset(&e, i as u32, &asset);
        }

        // Metadata for the contract's token (unchanged)
        // TODO: Name should be concatenated with some other name giving when initializing. Check how soroswap pairs  token are called.
        let decimal: u32 = 7;
        let name: String = String::from_str(&e, "dfToken");
        let symbol: String = String::from_str(&e, "DFT");

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        );

        events::emit_initialized_vault(&e, emergency_manager, fee_receiver, manager, defindex_receiver, assets);

        Ok(())
    }

    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
    ) -> Result<(), ContractError> {
        check_initialized(&e)?;
        from.require_auth();

        // get assets
        let assets = get_assets(&e);
        // assets lenght should be equal to amounts_desired and amounts_min length
        let assets_length = assets.len();
        if assets_length != amounts_desired.len() || assets_length != amounts_min.len() {
            panic_with_error!(&e, ContractError::WrongAmuntsLength);
        }

        // for every amount desired, check non negative
        for amount in amounts_desired.iter() {
            check_nonnegative_amount(amount)?;
        }
        // for amount min is not necesary to check if it is negative

        let (amounts, shares_to_mint) = if assets_length == 1 {
        // If Total Assets == 1
            let shares = if VaultToken::total_supply(e.clone())==0{
                // TODO In this case we might also want to mint a MINIMUM LIQUIDITY to be locked forever in the contract
                // this might be for security and practical reasons as well
                // shares will be equal to the amount desired to deposit, just for simplicity
                amounts_desired.get(0).unwrap() // here we have already check same lenght
            } else{
                // in this case we will mint a share proportional to the total managed funds
                let total_managed_funds = fetch_total_managed_funds(&e);
                VaultToken::total_supply(e.clone()) * amounts_desired.get(0).unwrap() / total_managed_funds.get(assets.get(0).unwrap().address.clone()).unwrap()
            };
            (amounts_desired, shares)
        } else {
        // If Total Assets > 1
            calculate_deposit_amounts_and_shares_to_mint(
                &e,
                &assets,
                &amounts_desired,
                &amounts_min,
            )
        };

        // for every asset
        for (i, amount) in amounts.iter().enumerate() {
            if amount > 0 {
                let asset = assets.get(i as u32).unwrap();
                let asset_client = TokenClient::new(&e, &asset.address);
                // send the current amount to this contract
                asset_client.transfer(&from, &e.current_contract_address(), &amount);
            }
        }

        // now we mint the corresponding dfTOkenb
        internal_mint(e.clone(), from.clone(), shares_to_mint);

        events::emit_deposit_event(&e, from, amounts, shares_to_mint);
        // TODO return amounts and shares to mint
        Ok(())
    }

    fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<(), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(df_amount)?;
        from.require_auth();
    
        // Check if the user has enough dfTokens
        let df_user_balance = VaultToken::balance(e.clone(), from.clone());
        if df_user_balance < df_amount {
            return Err(ContractError::InsufficientBalance);
        }
    
        // Burn the dfTokens
        internal_burn(e.clone(), from.clone(), df_amount);
    
        // Calculate the withdrawal amounts for each strategy based on the dfToken amount
        let withdrawal_amounts = calculate_withdrawal_amounts(&e, df_amount)?;
    
        // Create a map to store the total amounts to transfer for each asset address
        let mut total_amounts_to_transfer: Map<Address, i128> = Map::new(&e);
    
        // Get idle funds for each asset (Map<Address, i128>)
        let idle_funds = fetch_current_idle_funds(&e);
    
        // Loop through each strategy and handle the withdrawal
        for (strategy_address, required_amount) in withdrawal_amounts.iter() {
            // Find the corresponding asset address for this strategy
            let asset_address = get_strategy_asset(&e, &strategy_address)?.address;
    
            // Check idle funds for this asset
            let idle_balance = idle_funds.get(asset_address.clone()).unwrap_or(0);
    
            let mut remaining_amount = required_amount;
    
            // Withdraw as much as possible from idle funds first
            if idle_balance > 0 {
                if idle_balance >= required_amount {
                    // Idle funds cover the full amount
                    let current_amount = total_amounts_to_transfer.get(asset_address.clone()).unwrap_or(0);
                    total_amounts_to_transfer.set(asset_address.clone(), current_amount + required_amount);
                    continue;  // No need to withdraw from the strategy
                } else {
                    // Partial withdrawal from idle funds
                    let current_amount = total_amounts_to_transfer.get(asset_address.clone()).unwrap_or(0);
                    total_amounts_to_transfer.set(asset_address.clone(), current_amount + idle_balance);
                    remaining_amount = required_amount - idle_balance;  // Update remaining amount
                }
            }
    
            // Withdraw the remaining amount from the strategy
            let strategy_client = get_strategy_client(&e, strategy_address.clone());
            strategy_client.withdraw(&remaining_amount, &from);
    
            // Update the total amounts to transfer map
            let current_amount = total_amounts_to_transfer.get(asset_address.clone()).unwrap_or(0);
            total_amounts_to_transfer.set(asset_address.clone(), current_amount + remaining_amount);
        }
    
        // Perform the transfers for the total amounts
        let mut amounts_withdrawn: Vec<i128> = Vec::new(&e);
        for (asset_address, total_amount) in total_amounts_to_transfer.iter() {
            TokenClient::new(&e, &asset_address).transfer(
                &e.current_contract_address(),
                &from,
                &total_amount,
            );
            amounts_withdrawn.push_back(total_amount);
        }

        events::emit_withdraw_event(&e, from, df_amount, amounts_withdrawn);
    
        Ok(())
    }

    fn emergency_withdraw(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError> {
        check_initialized(&e)?;
    
        // Ensure the caller is the Manager or Emergency Manager
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(&[RolesDataKey::EmergencyManager, RolesDataKey::Manager], &caller);
    
        // Find the strategy and its associated asset
        let asset = get_strategy_asset(&e, &strategy_address)?;
        // This ensures that the vault has this strategy in its list of assets
        let strategy = get_strategy_struct(&strategy_address, &asset)?;
        
        // Withdraw all assets from the strategy
        let strategy_client = get_strategy_client(&e, strategy.address.clone());
        let strategy_balance = strategy_client.balance(&e.current_contract_address());
    
        if strategy_balance > 0 {
            strategy_client.withdraw(&strategy_balance, &e.current_contract_address());

            //TODO: Should we check if the idle funds are corresponding to the strategy balance withdrawed?
        }
        
        // Pause the strategy
        pause_strategy(&e, strategy_address.clone())?;
    
        events::emit_emergency_withdraw_event(&e, caller, strategy_address, strategy_balance);
        Ok(())
    }

    fn pause_strategy(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError> {
        // Ensure the caller is the Manager or Emergency Manager
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(&[RolesDataKey::EmergencyManager, RolesDataKey::Manager], &caller);

        pause_strategy(&e, strategy_address.clone())?;
        events::emit_strategy_paused_event(&e, strategy_address, caller);
        Ok(())
    }
    
    fn unpause_strategy(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError> {
        // Ensure the caller is the Manager or Emergency Manager
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(&[RolesDataKey::EmergencyManager, RolesDataKey::Manager], &caller);

        unpause_strategy(&e, strategy_address.clone())?;
        events::emit_strategy_unpaused_event(&e, strategy_address, caller);
        Ok(())
    }

    fn get_assets(e: Env) -> Vec<AssetAllocation> {
        get_assets(&e)
    }

    fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128> {
        fetch_total_managed_funds(e)
    }

    fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128> {
        fetch_current_invested_funds(e)
    }

    fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128> {
        fetch_current_idle_funds(e)
    }

    fn user_balance(e: Env, from: Address) -> i128 {
        VaultToken::balance(e, from)
    }
}

#[contractimpl]
impl AdminInterfaceTrait for DeFindexVault {
    fn set_fee_receiver(e: Env, caller: Address, new_fee_receiver: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_fee_receiver(&caller, &new_fee_receiver);

        events::emit_fee_receiver_changed_event(&e, new_fee_receiver, caller);
    }

    fn get_fee_receiver(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_fee_receiver()
    }

    fn set_manager(e: Env, manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_manager(&manager);

        events::emit_manager_changed_event(&e, manager);
    }

    fn get_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_manager()
    }

    fn set_emergency_manager(e: Env, emergency_manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_emergency_manager(&emergency_manager);

        events::emit_emergency_manager_changed_event(&e, emergency_manager);
    }

    fn get_emergency_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_emergency_manager()
    }
}

#[contractimpl]
impl VaultManagementTrait for DeFindexVault {
    fn invest(e: Env, investments: Vec<Investment>) -> Result<(), ContractError> {
        check_initialized(&e)?;
    
        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);
        e.current_contract_address().require_auth();
    
        // Get the current idle funds for all assets
        let idle_funds = fetch_current_idle_funds(&e);
    
        // Prepare investments based on current idle funds
        // This checks if the total investment exceeds the idle funds
        prepare_investment(&e, investments.clone(), idle_funds)?;
    
        // Now proceed with the actual investments if all checks passed
        execute_investment(&e, investments)?;

        // auto invest mockup
        // if auto_invest {
        //     let idle_funds = fetch_current_idle_funds(&e);
            
        //     // Prepare investments based on current ratios of invested funds
        //     let investments = calculate_investments_based_on_ratios(&e);
        //     prepare_investment(&e, investments.clone(), idle_funds)?;
        //     execute_investment(&e, investments)?;
        // }
        Ok(())
    }
}