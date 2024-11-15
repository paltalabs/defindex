#![no_std]
use blend_pool::RequestType;
use soroban_sdk::{
    contract, contractimpl, Address, Env, IntoVal, String, Val, Vec};

mod blend_pool;
mod storage;

use storage::{
    extend_instance_ttl, get_underlying_asset, is_initialized, set_blend_pool, set_initialized, set_underlying_asset
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

fn check_initialized(e: &Env) -> Result<(), StrategyError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(StrategyError::NotInitialized)
    }
}

const STARETEGY_NAME: &str = "BlendStrategy";

#[contract]
struct BlendStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for BlendStrategy {
    fn initialize(
        e: Env,
        asset: Address,
        init_args: Vec<Val>,
    ) -> Result<(), StrategyError> {
        if is_initialized(&e) {
            return Err(StrategyError::AlreadyInitialized);
        }

        let blend_pool_address = init_args.get(0).ok_or(StrategyError::InvalidArgument)?.into_val(&e);

        set_initialized(&e);
        set_blend_pool(&e, blend_pool_address);
        set_underlying_asset(&e, &asset);

        event::emit_initialize(&e, String::from_str(&e, STARETEGY_NAME), asset);
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

        blend_pool::submit(&e, &from, amount, RequestType::SupplyCollateral);

        event::emit_deposit(&e, String::from_str(&e, STARETEGY_NAME), amount, from);
        Ok(())
    }

    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        blend_pool::claim(&e, &from);

        event::emit_harvest(&e, String::from_str(&e, STARETEGY_NAME), 0i128, from);
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
        
        blend_pool::submit(&e, &from, amount, RequestType::WithdrawCollateral);

        event::emit_withdraw(&e, String::from_str(&e, STARETEGY_NAME), amount, from);

        Ok(amount)
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        let positions = blend_pool::get_positions(&e, &from);

        let collateral = positions.collateral.get(1u32).unwrap_or(0i128);
        Ok(collateral)
    }
}

mod test;