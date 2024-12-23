#![no_std]
use blend_pool::perform_reinvest;
use constants::{MIN_DUST, SCALAR_9};
use reserves::StrategyReserves;
use soroban_sdk::{
    contract, contractimpl, token::TokenClient, Address, Env, IntoVal, String, Val, Vec,
};

mod blend_pool;
mod constants;
mod reserves;
mod soroswap;
mod storage;

use storage::{extend_instance_ttl, has_config, Config};

pub use defindex_strategy_core::{event, DeFindexStrategyTrait, StrategyError};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        Err(StrategyError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), StrategyError> {
    if has_config(e) {
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
    fn __constructor(e: Env, asset: Address, init_args: Vec<Val>) {
        let blend_pool_address: Address = init_args
            .get(0)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let reserve_id: u32 = init_args
            .get(1)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let blend_token: Address = init_args
            .get(2)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let soroswap_router: Address = init_args
            .get(3)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);

        let config = Config {
            asset: asset.clone(),
            pool: blend_pool_address,
            reserve_id,
            blend_token,
            router: soroswap_router,
        };

        storage::set_config(&e, config);
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        Ok(storage::get_config(&e).asset)
    }

    fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        // protect against rouding of reserve_vault::update_rate, as small amounts
        // can cause incorrect b_rate calculations due to the pool rounding
        if amount < MIN_DUST {
            return Err(StrategyError::InvalidArgument); //TODO: create a new error type for this
        }

        let config = storage::get_config(&e);
        blend_pool::claim(&e, &e.current_contract_address(), &config);
        perform_reinvest(&e, &config)?;

        let reserves = storage::get_strategy_reserves(&e);

        // transfer tokens from the vault to the strategy contract
        TokenClient::new(&e, &config.asset).transfer(&from, &e.current_contract_address(), &amount);

        let b_tokens_minted = blend_pool::supply(&e, &from, &amount, &config);

        // Keeping track of the total deposited amount and the total bTokens owned by the strategy depositors
        let vault_shares = reserves::deposit(&e, reserves.clone(), &from, amount, b_tokens_minted);

        let underlying_balance = shares_to_underlying(vault_shares, reserves);

        event::emit_deposit(&e, String::from_str(&e, STARETEGY_NAME), amount, from);
        Ok(underlying_balance)
    }

    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        let config = storage::get_config(&e);
        let harvested_blend = blend_pool::claim(&e, &e.current_contract_address(), &config);

        perform_reinvest(&e, &config)?;

        event::emit_harvest(
            &e,
            String::from_str(&e, STARETEGY_NAME),
            harvested_blend,
            from,
        );
        Ok(())
    }

    fn withdraw(e: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        // protect against rouding of reserve_vault::update_rate, as small amounts
        // can cause incorrect b_rate calculations due to the pool rounding
        if amount < MIN_DUST {
            return Err(StrategyError::InvalidArgument); //TODO: create a new error type for this
        }

        let reserves = storage::get_strategy_reserves(&e);

        let config = storage::get_config(&e);

        let (tokens_withdrawn, b_tokens_burnt) = blend_pool::withdraw(&e, &to, &amount, &config);

        let vault_shares = reserves::withdraw(
            &e,
            reserves.clone(),
            &from,
            tokens_withdrawn,
            b_tokens_burnt,
        );
        let underlying_balance = shares_to_underlying(vault_shares, reserves);

        event::emit_withdraw(&e, String::from_str(&e, STARETEGY_NAME), amount, from);

        Ok(underlying_balance)
    }

    fn balance(e: Env, from: Address) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        // Get the vault's shares
        let vault_shares = storage::get_vault_shares(&e, &from);

        // Get the strategy's total shares and bTokens
        let reserves = storage::get_strategy_reserves(&e);

        let underlying_balance = shares_to_underlying(vault_shares, reserves);

        Ok(underlying_balance)
    }
}

fn shares_to_underlying(shares: i128, reserves: StrategyReserves) -> i128 {
    let total_shares = reserves.total_shares;
    let total_b_tokens = reserves.total_b_tokens;

    if total_shares == 0 || total_b_tokens == 0 {
        // No shares or bTokens in the strategy
        return 0i128;
    }
    // Calculate the bTokens corresponding to the vault's shares
    let vault_b_tokens = (shares * total_b_tokens) / total_shares;

    // Use the b_rate to convert bTokens to underlying assets
    (vault_b_tokens * reserves.b_rate) / SCALAR_9
}
mod test;
