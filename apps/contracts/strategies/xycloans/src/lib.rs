#![no_std]

mod soroswap_router;
mod storage;
mod xycloans_pool;

use defindex_strategy_core::{DeFindexStrategyTrait, StrategyError};
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, IntoVal, Val, Vec};
use soroswap_router::{get_amount_out, get_reserves, swap};
use storage::{
    extend_instance_ttl, get_pool_token, get_soroswap_factory_address, get_token_in,
    get_xycloans_pool_address, set_pool_token, set_soroswap_factory_address,
    set_soroswap_router_address, set_token_in, set_xycloans_pool_address,
};
use xycloans_pool::XycloansPoolClient;

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        Err(StrategyError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

// TODO: Needs check initialized

#[contract]
struct XycloansAdapter;

#[contractimpl]
impl DeFindexStrategyTrait for XycloansAdapter {
    fn __constructor(e: Env, _asset: Address, init_args: Vec<Val>) {
        let soroswap_router_address = init_args
            .get(0)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let soroswap_factory_address = init_args
            .get(1)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let xycloans_pool_address = init_args
            .get(2)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let pool_token = init_args
            .get(3)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);
        let token_in = init_args
            .get(4)
            .ok_or(StrategyError::InvalidArgument)
            .unwrap()
            .into_val(&e);

        set_soroswap_router_address(&e, soroswap_router_address);
        set_soroswap_factory_address(&e, soroswap_factory_address);
        set_xycloans_pool_address(&e, xycloans_pool_address);
        set_pool_token(&e, pool_token);
        set_token_in(&e, token_in);
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        extend_instance_ttl(&e);

        Ok(get_token_in(&e))
    }

    fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        let pool_token = get_pool_token(&e);
        let token_in = get_token_in(&e);

        let total_swapped_amount = swap(&e, &from, &token_in, &pool_token, &amount);

        // Xycloans Deposit XLM (WORKING)
        let xycloans_address = get_xycloans_pool_address(&e);
        let xycloans_pool_client = XycloansPoolClient::new(&e, &xycloans_address);
        xycloans_pool_client.deposit(&from, &total_swapped_amount);

        Ok(0)
    }

    fn harvest(e: Env, _from: Address, _data: Option<Bytes>) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);

        Ok(())
    }

    fn withdraw(e: Env, _amount: i128, from: Address, _to: Address) -> Result<i128, StrategyError> {
        from.require_auth();
        extend_instance_ttl(&e);

        let xycloans_address = get_xycloans_pool_address(&e);
        let xycloans_pool_client = XycloansPoolClient::new(&e, &xycloans_address);

        let shares: i128 = xycloans_pool_client.shares(&from);
        xycloans_pool_client.withdraw(&from, &shares);

        xycloans_pool_client.update_fee_rewards(&from);
        let matured: i128 = xycloans_pool_client.matured(&from);
        xycloans_pool_client.withdraw_matured(&from);

        // SWAP XLM TOTAL TO USDC and return USDC
        let total: i128 = shares.checked_add(matured).unwrap();

        let pool_token = get_pool_token(&e);
        let token_in = get_token_in(&e);

        let result = swap(&e, &from, &pool_token, &token_in, &total);

        Ok(result)
    }

    fn balance(e: Env, from: Address) -> Result<i128, StrategyError> {
        let xycloans_address = get_xycloans_pool_address(&e);
        let xycloans_pool_client = XycloansPoolClient::new(&e, &xycloans_address);

        let shares: i128 = xycloans_pool_client.shares(&from);
        let matured: i128 = xycloans_pool_client.matured(&from);

        let total: i128 = shares.checked_add(matured).unwrap();

        // If total is zero, return it
        if total == 0 {
            return Ok(total);
        }

        // XLM TO USDC QUOTE from SOROSWAP
        let soroswap_factory = get_soroswap_factory_address(&e);
        let pool_token = get_pool_token(&e);
        let token_in = get_token_in(&e);

        // Setting up Soroswap router client
        let (reserve_0, reserve_1) = get_reserves(
            e.clone(),
            soroswap_factory.clone(),
            pool_token.clone(),
            token_in.clone(),
        )
        .map_err(|_| StrategyError::ProtocolAddressNotFound)?;

        let amount_out = get_amount_out(total, reserve_0, reserve_1)
            .map_err(|_| StrategyError::ExternalError)?;

        Ok(amount_out)
    }
}
