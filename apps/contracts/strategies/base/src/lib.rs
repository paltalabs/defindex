#![no_std]
use balance::{read_balance, receive_balance, spend_balance};
use soroban_sdk::{
    contract, contractimpl, Address, Env, Val, Vec};
use soroban_sdk::token::Client as TokenClient;

mod balance;
mod event;
mod storage;

use storage::{
    extend_instance_ttl, get_underlying_asset, is_initialized, set_initialized, set_underlying_asset
};
use defindex_strategy_interface::{DeFindexStrategyTrait, StrategyError};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        Err(StrategyError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), StrategyError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(StrategyError::NotInitialized)
    }
}

#[contract]
struct BaseStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for BaseStrategy {
    fn initialize(
        e: Env,
        asset: Address,
        _init_args: Vec<Val>,
    ) -> Result<(), StrategyError> {
        if is_initialized(&e) {
            return Err(StrategyError::AlreadyInitialized);
        }

        set_initialized(&e);
        set_underlying_asset(&e, asset);

        event::initialized(&e, true);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        Ok(get_underlying_asset(&e))
    }

    fn deposit(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        let contract_address = e.current_contract_address();
        
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&from, &contract_address, &amount);

        receive_balance(&e, from, amount);

        Ok(())
    }

    fn harvest(e: Env) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        Ok(())
    }

    fn withdraw(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<i128, StrategyError> {
        from.require_auth();
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);

        let contract_address = e.current_contract_address();
        
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&contract_address, &from, &amount);

        spend_balance(&e, from, amount);

        Ok(amount)
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, StrategyError> {
        from.require_auth();
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        Ok(read_balance(&e, from))
    }
}
