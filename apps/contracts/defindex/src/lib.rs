#![no_std]
use access::{AccessControl, AccessControlTrait, RolesDataKey};
use interface::AdminInterfaceTrait;
use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, Env, String, Vec
};
use soroban_token_sdk::metadata::TokenMetadata;
use crate::interface::VaultTrait;

mod access;
mod error;
mod interface;
mod storage;
mod test;
mod token;
mod utils;

pub use error::ContractError;

use storage::{
    get_strategy, get_strategy_name, get_total_strategies, set_defindex_receiver, set_ratio, set_strategy, set_strategy_name, set_token, set_total_strategies, set_total_tokens, StrategyParams
};

use defindex_strategy_interface::DeFindexStrategyClient;
use token::write_metadata;

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

        for i in 0..total_strategies {
            let strategy_address = get_strategy(&e, i);
            let strategy_client = DeFindexStrategyClient::new(&e, &strategy_address);

            let adapter_amount = if i == (total_strategies - 1) {
                amount - total_amount_used
            } else {
                amount
            };

            strategy_client.deposit(&adapter_amount, &from);
            //should run deposit functions on adapters
        }

        Ok(())
    }

    fn withdraw(
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

    fn current_invested_funds(e: Env) -> i128 {
        0i128
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