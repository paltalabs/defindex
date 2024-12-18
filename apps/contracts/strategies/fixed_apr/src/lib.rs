#![no_std]
use constants::{MAX_BPS, SECONDS_PER_YEAR};
use soroban_sdk::{
    contract, contractimpl, token::Client as TokenClient, Address, Env, IntoVal, String, Val, Vec
};

mod balance;
mod constants;
mod storage;
mod yield_balance;

use balance::{read_balance, receive_balance, spend_balance};
use storage::{
    extend_instance_ttl, get_underlying_asset, is_initialized, set_initialized, 
    set_underlying_asset, set_apr, get_apr, set_last_harvest_time, get_last_harvest_time,
};

pub use defindex_strategy_core::{DeFindexStrategyTrait, StrategyError, event};
use yield_balance::{read_yield, receive_yield, spend_yield};

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

const STRATEGY_NAME: &str = "FixAprStrategy";

#[contract]
struct FixAprStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for FixAprStrategy {
    fn __constructor(
        e: Env,
        asset: Address,
        init_args: Vec<Val>,
    ) {
        // Extract APR from `init_args`, assumed to be the first argument
        let apr_bps: u32 = init_args.get(0).ok_or(StrategyError::InvalidArgument).unwrap().into_val(&e);
        
        set_initialized(&e);
        set_underlying_asset(&e, &asset);
        set_apr(&e, apr_bps);
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
    ) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        update_yield_balance(&e, &from);

        let contract_address = e.current_contract_address();
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&from, &contract_address, &amount);

        receive_balance(&e, from.clone(), amount);

        set_last_harvest_time(&e, e.ledger().timestamp(), from.clone());
        event::emit_deposit(&e, String::from_str(&e, STRATEGY_NAME), amount, from.clone());

        Ok(read_balance(&e, from))
    }

    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
    
        let yield_balance = update_yield_balance(&e, &from);

        if yield_balance == 0 {
            return Ok(());
        }
    
        // Transfer the reward tokens to the user's balance
        spend_yield(&e, from.clone(), yield_balance)?;
        receive_balance(&e, from.clone(), yield_balance);

        event::emit_harvest(&e, String::from_str(&e, STRATEGY_NAME), yield_balance, from);
    
        Ok(())
    }

    fn withdraw(
        e: Env,
        amount: i128,
        from: Address,
        to: Address,
    ) -> Result<i128, StrategyError> {
        from.require_auth();
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);

        spend_balance(&e, from.clone(), amount)?;
        
        let contract_address = e.current_contract_address();
        let underlying_asset = get_underlying_asset(&e);
        TokenClient::new(&e, &underlying_asset).transfer(&contract_address, &to, &amount);
        event::emit_withdraw(&e, String::from_str(&e, STRATEGY_NAME), amount, from.clone());

        Ok(read_balance(&e, from))
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(read_balance(&e, from))
    }
}


fn calculate_yield(user_balance: i128, apr: u32, time_elapsed: u64) -> i128 {
    // Calculate yield based on the APR, time elapsed, and user's balance
    let seconds_per_year = SECONDS_PER_YEAR;
    let apr_bps = apr as i128;
    let time_elapsed_i128 = time_elapsed as i128;

    (user_balance * apr_bps * time_elapsed_i128) / (seconds_per_year * MAX_BPS)
}

fn update_yield_balance(e: &Env, from: &Address) -> i128 {
    let apr = get_apr(e);
    let last_harvest = get_last_harvest_time(e, from.clone());
    let time_elapsed = e.ledger().timestamp().saturating_sub(last_harvest);

    if time_elapsed == 0 {
        return 0;
    }

    let user_balance = read_balance(e, from.clone());
    let reward_amount = calculate_yield(user_balance, apr, time_elapsed);

    if reward_amount == 0 {
        return 0;
    }

    receive_yield(e, from.clone(), reward_amount);
    set_last_harvest_time(e, e.ledger().timestamp(), from.clone());
    read_yield(e, from.clone())
}

mod test;