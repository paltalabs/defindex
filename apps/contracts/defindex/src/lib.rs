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

pub use error::ContractError;

use storage::{
    get_idle_funds, get_strategy, get_strategy_name, get_total_strategies, set_defindex_receiver, set_ratio, set_strategy, set_strategy_name, set_token, set_total_strategies, set_total_tokens, spend_idle_funds, StrategyParams
};
use funds::{get_current_idle_funds, get_current_invested_funds, get_total_managed_funds};

use defindex_strategy_core::DeFindexStrategyClient;
use token::{internal_burn, internal_mint, write_metadata, VaultToken};

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
        tokens: Vec<Address>,
        ratios: Vec<u32>,
        strategies: Vec<StrategyParams>
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
        let total_tokens = tokens.len();
        set_total_tokens(&e, total_tokens as u32);
        for (i, token) in tokens.iter().enumerate() {
            set_token(&e, i as u32, &token);
            set_ratio(&e, i as u32, ratios.get(i as u32).unwrap());
        }

        // Store strategies
        let total_strategies = strategies.len();
        set_total_strategies(&e, total_strategies as u32);
        for (i, strategy) in strategies.iter().enumerate() {
            set_strategy(&e, i as u32, &strategy.address);
            set_strategy_name(&e, i as u32, &strategy.name);
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

    fn deposit(e: Env, amount: i128, from: Address) -> Result<(), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        from.require_auth();

        let total_strategies = get_total_strategies(&e);
        let total_amount_used: i128 = 0;

        // 1dfToken = [token:ratio]
        internal_mint(e, from, amount);

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
        internal_burn(e.clone(), from.clone(), df_amount);
    
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
            let strategy_address = get_strategy(&e, i);
            let strategy_client = DeFindexStrategyClient::new(&e, &strategy_address);

            strategy_client.withdraw(&amount, &from);
        }

        Ok(())
    }

    fn get_strategies(e: Env) -> Vec<StrategyParams> {
        let total_strategies = get_total_strategies(&e);
        let mut strategies = Vec::new(&e);

        for i in 0..total_strategies {
            let strategy_address = get_strategy(&e, i);
            let strategy_name = get_strategy_name(&e, i);

            strategies.push_back(StrategyParams {
                name: strategy_name,
                address: strategy_address,
            });
        }

        strategies
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

    fn balance(e: Env, from: Address) -> i128 {
        VaultToken::balance(e, from)
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