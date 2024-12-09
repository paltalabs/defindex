#![no_std]
use soroban_sdk::{
    contract, 
    contractimpl, 
    Address, 
    Env, 
    String,
    token::Client as TokenClient, 
    Val, 
    Vec};

mod balance;
mod storage;

use balance::{
    read_balance, 
    receive_balance, 
    spend_balance};

use storage::{
    extend_instance_ttl, 
    get_underlying_asset, 
    set_underlying_asset
};

pub use defindex_strategy_core::{
    DeFindexStrategyTrait, 
    StrategyError, 
    event};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        Err(StrategyError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

const STARETEGY_NAME: &str = "HodlStrategy";

#[contract]
struct HodlStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for HodlStrategy {
    fn __constructor(
        e: Env,
        asset: Address,
        _init_args: Vec<Val>,
    ) {
        set_underlying_asset(&e, &asset);
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        extend_instance_ttl(&e);

        Ok(get_underlying_asset(&e))
    }

    fn deposit(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<(), StrategyError> {
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        let contract_address = e.current_contract_address();
        
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&from, &contract_address, &amount);

        receive_balance(&e, from.clone(), amount);
        event::emit_deposit(&e, String::from_str(&e, STARETEGY_NAME), amount, from);

        Ok(())
    }

    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);

        event::emit_harvest(&e, String::from_str(&e, STARETEGY_NAME), 0i128, from);
        Ok(())
    }

    fn withdraw(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<i128, StrategyError> {
        from.require_auth();
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);

        spend_balance(&e, from.clone(), amount)?;
        
        let contract_address = e.current_contract_address();
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&contract_address, &from, &amount);
        event::emit_withdraw(&e, String::from_str(&e, STARETEGY_NAME), amount, from);

        Ok(amount)
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, StrategyError> {
        extend_instance_ttl(&e);

        Ok(read_balance(&e, from))
    }
}

mod test;