#![no_std]
use constants::MIN_DUST;
use soroban_sdk::{
    contract, contractimpl, panic_with_error, token::TokenClient, Address, Env, IntoVal, String, Val, Vec};

mod blend_pool;
mod constants;
mod positions;
mod reserves;
mod storage;

use storage::{extend_instance_ttl, is_initialized, set_initialized, Config};

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

        let blend_pool_address: Address = init_args.get(0).ok_or(StrategyError::InvalidArgument)?.into_val(&e);
        let reserve_id: u32 = init_args.get(1).ok_or(StrategyError::InvalidArgument)?.into_val(&e);

        set_initialized(&e);

        let config = Config {
            asset: asset.clone(),
            pool: blend_pool_address.clone(),
            reserve_id: reserve_id.clone(),
        };
        
        storage::set_config(&e, config);
        
        event::emit_initialize(&e, String::from_str(&e, STARETEGY_NAME), asset);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        Ok(storage::get_config(&e).asset)
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

        // protect against rouding of reserve_vault::update_rate, as small amounts
        // can cause incorrect b_rate calculations due to the pool rounding
        if amount < MIN_DUST {
            return Err(StrategyError::InvalidArgument); //TODO: create a new error type for this
        }

        let mut reserves = storage::get_strategy_reserves(&e);

        let config = storage::get_config(&e);
        // transfer tokens from the vault to the strategy contract
        TokenClient::new(&e, &config.asset).transfer(&from, &e.current_contract_address(), &amount);

        let b_tokens_minted = blend_pool::supply(&e, &from, &amount, &config);

        // Keeping track of the total deposited amount and the total bTokens owned by the strategy depositors
        reserves.add(amount, b_tokens_minted);

        // Keeping track of the total amount deposited by the user and the total amount of bTokens owned by the user
        let mut vault_position = storage::get_vault_position(&e, &from);
        vault_position.add(amount, b_tokens_minted);

        storage::set_strategy_reserves(&e, reserves);
        storage::set_vault_position(&e, &from, vault_position);

        event::emit_deposit(&e, String::from_str(&e, STARETEGY_NAME), amount, from);
        Ok(())
    }

    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        from.require_auth();        

        let config = storage::get_config(&e);
        blend_pool::claim(&e, &from, &config);

        event::emit_harvest(&e, String::from_str(&e, STARETEGY_NAME), 0i128, from);
        Ok(())
    }

    fn withdraw(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        // protect against rouding of reserve_vault::update_rate, as small amounts
        // can cause incorrect b_rate calculations due to the pool rounding
        if amount < MIN_DUST {
            return Err(StrategyError::InvalidArgument) //TODO: create a new error type for this
        }

        let mut reserves = storage::get_strategy_reserves(&e);

        let config = storage::get_config(&e);


        let (tokens_withdrawn, b_tokens_burnt) = blend_pool::withdraw(&e, &from, &amount, &config);


        if tokens_withdrawn <= 0 {
            panic_with_error!(e, StrategyError::InvalidArgument);
        }
        if b_tokens_burnt <= 0 {
            panic_with_error!(e, StrategyError::InvalidArgument);
        }
        
        reserves.remove(tokens_withdrawn, b_tokens_burnt);
    
        let mut vault_position = storage::get_vault_position(&e, &from);
        vault_position.remove(amount, b_tokens_burnt);
        
        storage::set_strategy_reserves(&e, reserves);
        storage::set_vault_position(&e, &from, vault_position);

        event::emit_withdraw(&e, String::from_str(&e, STARETEGY_NAME), amount, from);

        Ok(tokens_withdrawn)
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        let vault_position = storage::get_vault_position(&e, &from);

        Ok(vault_position.b_tokens)
    }
}

// mod test;