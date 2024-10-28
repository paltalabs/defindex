#![no_std]
use fee::collect_fees;
use investment::{execute_investment, prepare_investment};
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token::{TokenClient, TokenInterface}, Address, Env, Map, String, Vec
};
use soroban_token_sdk::metadata::TokenMetadata;

mod access;
mod constants;
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
use funds::{fetch_current_idle_funds, fetch_current_invested_funds, fetch_invested_funds_for_strategy, fetch_total_managed_funds};
use interface::{AdminInterfaceTrait, VaultTrait, VaultManagementTrait};
use models::{AssetAllocation, Investment};
use storage::{
    get_assets, set_asset, set_defindex_receiver, set_factory, set_last_fee_assesment, set_total_assets, set_vault_share
};
use strategies::{get_asset_allocation_from_address, get_strategy_asset, get_strategy_client, get_strategy_struct, pause_strategy, unpause_strategy};
use token::{internal_mint, internal_burn, write_metadata, VaultToken};
use utils::{
    calculate_asset_amounts_for_dftokens, calculate_deposit_amounts_and_shares_to_mint, calculate_withdrawal_amounts, check_initialized, check_nonnegative_amount
};

pub use error::ContractError;

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {
    /// Initializes the DeFindex Vault contract with the required parameters.
    ///
    /// This function sets the roles for emergency manager, fee receiver, and manager.
    /// It also stores the list of assets to be managed by the vault, including strategies for each asset.
    /// 
    /// # Arguments:
    /// * `e` - The environment.
    /// * `assets` - A vector of `AssetAllocation` structs representing the assets and their associated strategies.
    /// * `manager` - The address responsible for managing the vault.
    /// * `emergency_manager` - The address with emergency control over the vault.
    /// * `fee_receiver` - The address that will receive fees from the vault.
    /// * `vault_share` - The percentage of the vault's fees that will be sent to the DeFindex receiver. in BPS.
    /// * `defindex_receiver` - The address that will receive fees for DeFindex from the vault.
    /// * `factory` - The address of the factory that deployed the vault.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn initialize(
        e: Env,
        assets: Vec<AssetAllocation>,
        manager: Address,
        emergency_manager: Address,
        fee_receiver: Address,
        vault_share: u32,
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

        // Set Vault Share (in basis points)
        set_vault_share(&e, &vault_share);

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

    /// Handles deposits into the DeFindex Vault.
    ///
    /// This function transfers the desired amounts of each asset into the vault, distributes the assets
    /// across the strategies according to the vault's ratios, and mints dfTokens representing the user's
    /// share in the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `amounts_desired` - A vector of the amounts the user wishes to deposit for each asset.
    /// * `amounts_min` - A vector of minimum amounts required for the deposit to proceed.
    /// * `from` - The address of the user making the deposit.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
    ) -> Result<(), ContractError> {
        check_initialized(&e)?;
        from.require_auth();

        // Set LastFeeAssessment if it is the first deposit
        if VaultToken::total_supply(e.clone())==0{
            set_last_fee_assesment(&e, &e.ledger().timestamp());
        }

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

        // fees assesment
        collect_fees(&e)?;
        // TODO return amounts and shares to mint
        Ok(())
    }

    /// Withdraws assets from the DeFindex Vault by burning dfTokens.
    ///
    /// This function calculates the amount of assets to withdraw based on the number of dfTokens being burned,
    /// then transfers the appropriate assets back to the user, pulling from both idle funds and strategies
    /// as needed.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `df_amount` - The amount of dfTokens to burn for the withdrawal.
    /// * `from` - The address of the user requesting the withdrawal.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<Vec<i128>, ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(df_amount)?;
        from.require_auth();
    
        // Check if the user has enough dfTokens
        let df_user_balance = VaultToken::balance(e.clone(), from.clone());
        if df_user_balance < df_amount {
            return Err(ContractError::InsufficientBalance);
        }
    
        // Calculate the withdrawal amounts for each asset based on the dfToken amount
        let asset_amounts = calculate_asset_amounts_for_dftokens(&e, df_amount);

        // Burn the dfTokens after calculating the withdrawal amounts (so total supply is correct)
        internal_burn(e.clone(), from.clone(), df_amount);
    
        // Create a map to store the total amounts to transfer for each asset address
        let mut total_amounts_to_transfer: Map<Address, i128> = Map::new(&e);
    
        // Get idle funds for each asset (Map<Address, i128>)
        let idle_funds = fetch_current_idle_funds(&e);
    
        // Loop through each asset and handle the withdrawal
        for (asset_address, required_amount) in asset_amounts.iter() {
            // Check idle funds for this asset
            let idle_balance = idle_funds.get(asset_address.clone()).unwrap_or(0);

            let mut remaining_amount = required_amount;
    
            // Withdraw as much as possible from idle funds first
            if idle_balance > 0 {
                if idle_balance >= required_amount {
                    // Idle funds cover the full amount
                    total_amounts_to_transfer.set(asset_address.clone(), required_amount);
                    continue;  // No need to withdraw from the strategy
                } else {
                    // Partial withdrawal from idle funds
                    total_amounts_to_transfer.set(asset_address.clone(), idle_balance);
                    remaining_amount = required_amount - idle_balance;  // Update remaining amount
                }
            }
    
            // Find the corresponding asset address for this strategy
            let asset_allocation = get_asset_allocation_from_address(&e, asset_address.clone())?;
            let withdrawal_amounts = calculate_withdrawal_amounts(&e, remaining_amount, asset_allocation);

            for (strategy_address, amount) in withdrawal_amounts.iter() {
                let strategy_client = get_strategy_client(&e, strategy_address.clone());
                // TODO: What if the withdraw method exceeds the instructions limit? since im trying to ithdraw from all strategies of all assets...
                strategy_client.withdraw(&amount, &e.current_contract_address());
    
                // Update the total amounts to transfer map
                let current_amount = total_amounts_to_transfer.get(strategy_address.clone()).unwrap_or(0);
                total_amounts_to_transfer.set(asset_address.clone(), current_amount + amount);
            }
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

        events::emit_withdraw_event(&e, from, df_amount, amounts_withdrawn.clone());

        // fees assesment
        collect_fees(&e)?;
    
        Ok(amounts_withdrawn)
    }

    /// Executes an emergency withdrawal from a specific strategy.
    ///
    /// This function allows the emergency manager or manager to withdraw all assets from a particular strategy
    /// and store them as idle funds within the vault. It also pauses the strategy to prevent further use until
    /// unpaused.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `strategy_address` - The address of the strategy to withdraw from.
    /// * `caller` - The address initiating the emergency withdrawal (must be the manager or emergency manager).
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
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

    /// Pauses a strategy to prevent it from being used in the vault.
    ///
    /// This function pauses a strategy by setting its `paused` field to `true`. Only the manager or emergency
    /// manager can pause a strategy.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `strategy_address` - The address of the strategy to pause.
    /// * `caller` - The address initiating the pause (must be the manager or emergency manager).
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn pause_strategy(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError> {
        // Ensure the caller is the Manager or Emergency Manager
        // TODO: Should check if the strategy has any amount invested on it, and return an error if it has, should we let the manager to pause a strategy with funds invested?
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(&[RolesDataKey::EmergencyManager, RolesDataKey::Manager], &caller);

        pause_strategy(&e, strategy_address.clone())?;
        events::emit_strategy_paused_event(&e, strategy_address, caller);
        Ok(())
    }
    
    /// Unpauses a previously paused strategy.
    ///
    /// This function unpauses a strategy by setting its `paused` field to `false`, allowing it to be used
    /// again in the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `strategy_address` - The address of the strategy to unpause.
    /// * `caller` - The address initiating the unpause (must be the manager or emergency manager).
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn unpause_strategy(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError> {
        // Ensure the caller is the Manager or Emergency Manager
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(&[RolesDataKey::EmergencyManager, RolesDataKey::Manager], &caller);

        unpause_strategy(&e, strategy_address.clone())?;
        events::emit_strategy_unpaused_event(&e, strategy_address, caller);
        Ok(())
    }

    /// Retrieves the list of assets managed by the DeFindex Vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Vec<AssetAllocation>` - A vector of `AssetAllocation` structs representing the assets managed by the vault.
    fn get_assets(e: Env) -> Vec<AssetAllocation> {
        get_assets(&e)
    }

    /// Returns the total managed funds of the vault, including both invested and idle funds.
    ///
    /// This function provides a map where the key is the asset address and the value is the total amount
    /// of that asset being managed by the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Map<Address, i128>` - A map of asset addresses to their total managed amounts.
    fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128> {
        fetch_total_managed_funds(e)
    }

    /// Returns the current invested funds, representing the total assets allocated to strategies.
    ///
    /// This function provides a map where the key is the asset address and the value is the total amount
    /// of that asset currently invested in various strategies.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Map<Address, i128>` - A map of asset addresses to their total invested amounts.
    fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128> {
        fetch_current_invested_funds(e)
    }

    /// Returns the current idle funds, representing the total assets held directly by the vault (not invested).
    ///
    /// This function provides a map where the key is the asset address and the value is the total amount
    /// of that asset held as idle funds within the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Map<Address, i128>` - A map of asset addresses to their total idle amounts.
    fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128> {
        fetch_current_idle_funds(e)
    }

    // TODO: DELETE THIS, USED FOR TESTING
    /// Temporary method for testing purposes.
    fn get_asset_amounts_for_dftokens(e: Env, df_tokens: i128) -> Map<Address, i128> {
        calculate_asset_amounts_for_dftokens(&e, df_tokens)
    }
}

#[contractimpl]
impl AdminInterfaceTrait for DeFindexVault {
    /// Sets the fee receiver for the vault.
    ///
    /// This function allows the manager or emergency manager to set a new fee receiver address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `caller` - The address initiating the change (must be the manager or emergency manager).
    /// * `fee_receiver` - The new fee receiver address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_fee_receiver(e: Env, caller: Address, new_fee_receiver: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_fee_receiver(&caller, &new_fee_receiver);

        events::emit_fee_receiver_changed_event(&e, new_fee_receiver, caller);
    }

    /// Retrieves the current fee receiver address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The fee receiver address if successful, otherwise returns a ContractError.
    fn get_fee_receiver(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_fee_receiver()
    }

    /// Sets the manager for the vault.
    ///
    /// This function allows the current manager or emergency manager to set a new manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `manager` - The new manager address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_manager(e: Env, manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_manager(&manager);

        events::emit_manager_changed_event(&e, manager);
    }

    /// Retrieves the current manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The manager address if successful, otherwise returns a ContractError.
    fn get_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_manager()
    }

    /// Sets the emergency manager for the vault.
    ///
    /// This function allows the current manager or emergency manager to set a new emergency manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `emergency_manager` - The new emergency manager address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_emergency_manager(e: Env, emergency_manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_emergency_manager(&emergency_manager);

        events::emit_emergency_manager_changed_event(&e, emergency_manager);
    }

    /// Retrieves the current emergency manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The emergency manager address if successful, otherwise returns a ContractError.
    fn get_emergency_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_emergency_manager()
    }
}

#[contractimpl]
impl VaultManagementTrait for DeFindexVault {
    /// Invests the vault's idle funds into the specified strategies.
    /// 
    /// # Arguments:
    /// * `e` - The environment.
    /// * `investment` - A vector of `Investment` structs representing the amount to invest in each strategy.
    /// * `caller` - The address of the caller.
    /// 
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
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

    /// Rebalances the vault’s investments to match the target allocations specified.
    /// 
    /// # Arguments:
    /// * `e` - The environment.
    /// * `allocations` - A vector of `Investment` structs, each representing a target allocation amount for a specific strategy.
    /// 
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    ///
    /// # Notes:
    /// This function adjusts current holdings by withdrawing from over-allocated strategies and
    /// investing in under-allocated ones to achieve the target allocation.
    fn rebalance(e: Env, allocations: Vec<Investment>) -> Result<(), ContractError> {
        check_initialized(&e)?;
    
        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);
        // e.current_contract_address().require_auth();
    
        // Calculate necessary withdrawals and investments
        let mut withdrawals = Vec::new(&e); // Vector of (strategy, amount to withdraw)
        let mut investments: Vec<Investment> = Vec::new(&e); // Vector of (strategy, amount to invest)
    
        for allocation in allocations.iter() {
            let strategy = allocation.strategy;
            let target_amount = allocation.amount;
            let current_amount = fetch_invested_funds_for_strategy(&e, &strategy);
    
            if current_amount > target_amount {
                // Calculate amount to withdraw
                let withdraw_amount = current_amount - target_amount;
                withdrawals.push_back((strategy.clone(), withdraw_amount));
            } else if current_amount < target_amount {
                // Calculate amount to invest
                let invest_amount = target_amount - current_amount;
                investments.push_back(
                    Investment{
                        strategy: strategy.clone(), 
                        amount: invest_amount
                    }
                );
            }
        }
    
        // Execute withdrawals
        for (strategy, amount) in withdrawals.iter() {
            let strategy_client = get_strategy_client(&e, strategy.clone());
            strategy_client.withdraw(&amount, &e.current_contract_address());
        }
    
        // Execute investments
        let idle_funds = fetch_current_idle_funds(&e);
        prepare_investment(&e, investments.clone(), idle_funds)?;
        execute_investment(&e, investments)?;
    
        Ok(())
    }
}