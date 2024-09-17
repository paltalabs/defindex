#![no_std]
use access::{AccessControl, AccessControlTrait, RolesDataKey};
use interface::AdminInterfaceTrait;
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token::{TokenClient, TokenInterface}, Address, Env, Map, String, Vec
};
use soroban_token_sdk::metadata::TokenMetadata;
use strategies::get_strategy_client;
use utils::calculate_withdrawal_amounts;
use crate::interface::VaultTrait;

mod access;
mod error;
mod interface;
mod storage;
mod test;
mod token;
mod utils;
mod funds;
mod strategies;
mod models;

pub use error::ContractError;

use storage::{
    set_defindex_receiver, 
    set_asset, set_total_assets, get_assets,
    set_total_strategies,spend_idle_funds,
    get_idle_funds,
    get_strategy, get_strategies, get_total_strategies, set_strategy,
};
use models::{Strategy, Asset};
use funds::{get_current_idle_funds, get_current_invested_funds, get_total_managed_funds};

use defindex_strategy_core::DeFindexStrategyClient;
use token::{write_metadata, VaultToken, internal_mint};

fn check_initialized(e: &Env) -> Result<(), ContractError> {
    //TODO: Should also check if adapters/strategies have been set
    let access_control = AccessControl::new(&e);
    if access_control.has_role(&RolesDataKey::Manager) {
        Ok(())
    } else {
        panic_with_error!(&e, ContractError::NotInitialized);
    }
}

pub fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

pub fn get_optimal_amounts_and_shares_to_mint_enforcing_asset_i(
    e: &Env,
    total_managed_funds: &Map<Address, i128>,
    assets: &Vec<Asset>,
    amounts_desired: &Vec<i128>,
    i: &u32) -> (Vec<i128>, i128) {
    // we have to calculate the optimal amount to deposit for the rest of the assets
    // we need the total amount managed by this vault in order for the deposit to be proportional
    // reserve (total manage funds) of the asset we are enforcing
    let reserve_target = total_managed_funds.get(assets.get(*i).unwrap().address).unwrap(); // i128
    if reserve_target == 0 {
        // return sum of amounts desired as shares
        return (amounts_desired.clone(), amounts_desired.iter().sum()) // first shares will be equal to the first amounts_desired
        // TODO, this amounts desired might be too little?
        // this might be the first deposit... in this case, the ratio will be enforced by the first depositor
        // TODO: might happen that the reserve_target is zero because everything is in one asset!?
        // in this case we ned to check the ratio
    }
    let amount_desired_target = amounts_desired.get(*i).unwrap(); // i128

    let mut optimal_amounts = Vec::new(e);

    for (j, (_asset_address, reserve)) in total_managed_funds.iter().enumerate() {
        if j == (*i as usize) {
            optimal_amounts.push_back(amount_desired_target);
        } else {
            // amount = amount_desired_target * reserve[j] / reserve_target
            // factor is (amount_desired_target  / reserve_target;)
            let amount = reserve * amount_desired_target  / reserve_target;
            optimal_amounts.push_back(amount);
        }
        
    }
    let shares_to_mint = 0; // TODO: calculate the shares to mint = total_supply * amount_desired_target  / reserve_target
    (optimal_amounts, shares_to_mint)
}

pub fn get_deposit_amounts_and_shares_to_mint(
    e: &Env, 
    assets: &Vec<Asset>,
    amounts_desired: &Vec<i128>, 
    amounts_min: &Vec<i128>) -> (Vec<i128>, i128) {
    // here we have already 3 vectors with same length, and all vectors are corrclty organized.
    // meaning that amounts_desired[i] is the amount desired for asset[i] and amounts_min[i] is the minimum amount for asset[i]

    // for each index, we calculate the optimal amount to deposit for the rest of the assets
    // and then we see if it is possible to deposit that amount, considering the amounts_min
    // if it is not possible, we calculate the optimal amount considering the next asset and so on
    // if it is not possible to deposit the optimal amount for the last asset, we throw an error
    
    let total_managed_funds = get_total_managed_funds(e); // Map<Address, i128>// a number for each asset
    for i in 0..assets.len() {
        // TODO dont enforce asset i if ratio of asset i is 0... in this case we need to enforce the next one
        let (optimal_amounts, shares_to_mint) = get_optimal_amounts_and_shares_to_mint_enforcing_asset_i(
                                &e,
                                &total_managed_funds,
                                &assets,
                                &amounts_desired, 
                                &i);
        
        // if optimal _amounts[i]  is less than amounts_desired[i], but greater than amouints_min[i], then we cfalculate with the next one
        
        // Flag to indicate if we should skip the current asset {i} and continue
        let mut should_skip = false;
        
        for j in i+1..assets.len() {
            // if optimal_amounts.get(j)  is less than amounts_desired.get(j), then we check if is at least more than the minimum, if yes, this might work!
            if optimal_amounts.get(j).unwrap() <= amounts_desired.get(j).unwrap() {
                // if not, this will never work, because the optimal amount with that amount_min 
                if optimal_amounts.get(j).unwrap() < amounts_min.get(j).unwrap() {
                    panic!("insufficient amount"); // TODO transform panic in error
                }
                // if not, this is great. we continue, hoping this will be the answer
            }
            else{
               // If the optimal amount is greater to the desired amount, we skip the current asset {i}
               should_skip = true;
               // if we are in the last asset, we should throw an error
                if j == assets.len() - 1 {
                     panic!("didnt find optimal amounts"); // TODO transform panic in error
                }
               break; // Skip further checks for this asset {i}

            }
        }
        if should_skip {
            continue;
        } 
        else {
            return (optimal_amounts, shares_to_mint);
        }

    }
    panic!("didnt finfd");
    
    
}

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {
    fn initialize(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        manager: Address,
        defindex_receiver: Address,
        assets: Vec<Asset>,
        strategies: Vec<Strategy>
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

        // Store tokens and their ratios
        let total_tokens = assets.len();
        set_total_assets(&e, total_tokens as u32);
        for (i, asset) in assets.iter().enumerate() {
            set_asset(&e, i as u32, &asset);
        }

        // Store strategies
        let total_strategies = strategies.len();
        set_total_strategies(&e, total_strategies as u32);
        for (i, strategy) in strategies.iter().enumerate() {
            set_strategy(&e, i as u32, &strategy);
        }

        // Metadata for the contract's token (unchanged)
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

        Ok(())
    }

    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>, 
        amounts_min: Vec<i128>,
        from: Address) -> Result<(), ContractError> {

        check_initialized(&e)?;
        from.require_auth();

        // get assets
        let assets = get_assets(&e);
        // assets lenght should be equal to amounts_desired and amounts_min length
        let assets_length = assets.len();
        if assets_length != amounts_desired.len() || assets_length != amounts_min.len() {
            panic!("Invalid amounts"); // TODO transform panic in error
        }

        // for every amount desired, check non negative
        for amount in amounts_desired.iter() {
            check_nonnegative_amount(amount)?;
        }

        let (amounts, shares_to_mint) = if assets_length == 1 {
            let shares = 0; //TODO
            (amounts_desired, shares)
        }
        else {
            get_deposit_amounts_and_shares_to_mint(&e, &assets, &amounts_desired, &amounts_min)
        };

        // for every asset, 
        for (i, amount) in amounts.iter().enumerate() {
            if amount > 0 {
                let asset = assets.get(i as u32).unwrap();
                let asset_client = TokenClient::new(&e, &asset.address);
                // send the current amount to this contract
                asset_client.transfer(&from, &e.current_contract_address(), &amount);
                // increase the 
            }

        }

        // now we mint the corresponding dfTOkenb
        internal_mint(e, from, shares_to_mint);

        Ok(())
    }

    fn withdraw(
        e: Env,
        df_amount: i128,
        from: Address,
    ) -> Result<(), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(df_amount)?;
        from.require_auth();
    
        // Check if the user has enough dfTokens
        let df_user_balance = VaultToken::balance(e.clone(), from.clone());
        if df_user_balance < df_amount {
            panic_with_error!(&e, ContractError::InsufficientBalance);
        }
    
        // Calculate the withdrawal amounts for each token based on the dfToken amount
        let withdrawal_amounts = calculate_withdrawal_amounts(&e, df_amount)?;
    
        // Get idle funds for each token
        let idle_funds = get_current_idle_funds(&e);
    
        // Loop through each token and handle the withdrawal
        for (token_address, required_amount) in withdrawal_amounts.iter() {
            let mut total_amount_to_transfer = 0;
    
            // Get idle funds for this specific token, if it exists
            let idle_balance = idle_funds.get(token_address.clone()).unwrap_or(0);
    
            // Withdraw as much as possible from idle funds
            if idle_balance > 0 {
                if idle_balance >= required_amount {
                    // If idle funds cover the full amount, no need to check strategies
                    total_amount_to_transfer = required_amount;
                } else {
                    // Partial amount from idle funds
                    total_amount_to_transfer = idle_balance;
                    // If we want to keep a minimum amount of idle funds we should add it here so it weithdraws the required amount for the withdrawal and some more to keep the minimum 
                    let mut remaining_amount = required_amount - idle_balance;
    
                    // Now, withdraw the remaining amount from strategies
                    let total_strategies = get_total_strategies(&e);
                    for i in 0..total_strategies {
                        let strategy_client = get_strategy_client(&e, i);
                        
                        // Check if the strategy supports this token via the asset method
                        let strategy_asset = strategy_client.asset();
                        if strategy_asset == token_address {
                            let strategy_balance = strategy_client.balance(&from);
                            if strategy_balance >= remaining_amount {
                                strategy_client.withdraw(&remaining_amount, &from);
                                total_amount_to_transfer += remaining_amount;
                                break;
                            } else {
                                // Withdraw as much as possible from this strategy
                                strategy_client.withdraw(&strategy_balance, &from);
                                total_amount_to_transfer += strategy_balance;
    
                                // Reduce remaining amount by the amount withdrawn
                                remaining_amount -= strategy_balance;
                            }
                        }
    
                        // If no strategies can fulfill the remaining amount, throw an error
                        if remaining_amount > 0 && i == total_strategies - 1 {
                            panic_with_error!(&e, ContractError::InsufficientBalance);
                        }
                    }
                }
            }
    
            // Perform the transfer once the total amount to transfer has been calculated
            TokenClient::new(&e, &token_address).transfer(&e.current_contract_address(), &from, &total_amount_to_transfer);
        }
    
        // Burn the dfTokens after the successful withdrawal
        VaultToken::burn(e.clone(), from.clone(), df_amount);
    
        Ok(())
    }

    fn emergency_withdraw(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<(), ContractError>{
        check_initialized(&e)?;
        from.require_auth();
        let total_strategies = get_total_strategies(&e);

        for i in 0..total_strategies {
            let strategy = get_strategy(&e, i);
            let strategy_client = DeFindexStrategyClient::new(&e, &strategy.address);

            strategy_client.withdraw(&amount, &from);
        }

        Ok(())
    }

    fn get_strategies(e: Env) -> Vec<Strategy> {
        // TODO: CHECK INITIALIZED
        get_strategies(&e)
    }

    fn get_total_managed_funds(e: &Env) -> Map<Address, i128> {
        get_total_managed_funds(e)
    }

    fn get_current_invested_funds(e: &Env) -> Map<Address, i128> {
        get_current_invested_funds(e)

    }
  
    fn get_current_idle_funds(e: &Env) -> Map<Address, i128> {
        get_current_idle_funds(e)

    }
}

#[contractimpl]
impl AdminInterfaceTrait for DeFindexVault {  
    fn set_fee_receiver(e: Env, caller: Address, fee_receiver: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_fee_receiver(&caller, &fee_receiver)
    }
  
    fn get_fee_receiver(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_fee_receiver()
    }
  
    fn set_manager(e: Env, manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_manager(&manager)
    }
  
    fn get_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_manager()
    }
  
    fn set_emergency_manager(e: Env, emergency_manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_emergency_manager(&emergency_manager)
    }
  
    fn get_emergency_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_emergency_manager()
    }
}