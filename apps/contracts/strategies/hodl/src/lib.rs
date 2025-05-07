#![no_std]
use soroban_sdk::{
    contract, contractimpl, token::Client as TokenClient, Address, Bytes, Env, String, Val, Vec,
};

mod balance;
mod storage;

use balance::{read_balance, receive_balance, spend_balance};

use storage::{extend_instance_ttl, get_underlying_asset, set_underlying_asset};

pub use defindex_strategy_core::{event, DeFindexStrategyTrait, StrategyError};

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
    fn __constructor(e: Env, asset: Address, _init_args: Vec<Val>) {
        set_underlying_asset(&e, &asset);
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        extend_instance_ttl(&e);

        Ok(get_underlying_asset(&e))
    }

    fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        let contract_address = e.current_contract_address();

        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&from, &contract_address, &amount);

        receive_balance(&e, from.clone(), amount);
        event::emit_deposit(
            &e,
            String::from_str(&e, STARETEGY_NAME),
            amount,
            from.clone(),
        );

        Ok(read_balance(&e, from))
    }

    fn harvest(e: Env, from: Address, _data: Option<Bytes>) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);

        event::emit_harvest(&e, String::from_str(&e, STARETEGY_NAME), 0i128, from, 0i128);
        Ok(())
    }

    fn withdraw(e: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError> {
        from.require_auth();
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);

        spend_balance(&e, from.clone(), amount)?;

        let contract_address = e.current_contract_address();
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&contract_address, &to, &amount);
        event::emit_withdraw(
            &e,
            String::from_str(&e, STARETEGY_NAME),
            amount,
            from.clone(),
        );

        Ok(read_balance(&e, from))
    }

    fn balance(e: Env, from: Address) -> Result<i128, StrategyError> {
        extend_instance_ttl(&e);

        Ok(read_balance(&e, from))
    }
}

mod test;
