#![no_std]

mod event;
mod storage;
mod soroswap_router;
mod xycloans_pool;

use soroban_sdk::{auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, contract, contractimpl, vec, Address, Env, IntoVal, Symbol, Val, Vec};
use storage::{
    extend_instance_ttl, get_soroswap_router_address, get_pool_token, get_token_in, get_xycloans_pool_address, is_initialized, set_initialized, set_soroswap_router_address, set_pool_token, set_token_in, set_xycloans_pool_address, set_soroswap_factory_address, get_soroswap_factory_address
};
use soroswap_router::{get_amount_out, get_reserves, pair_for, swap, SoroswapRouterClient};
use xycloans_pool::XycloansPoolClient;
use defindex_strategy_interface::{AdapterError, DeFindexAdapterTrait};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), AdapterError> {
    if amount < 0 {
        Err(AdapterError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), AdapterError> {
    if is_initialized(e) {
        Ok(())
    } else {
        Err(AdapterError::NotInitialized)
    }
}

#[contract]
struct XycloansAdapter;

pub trait InitializeTrait {
    /// token_0 is the token that the user is sending and token_1 is the one is being deposit into the pool, if token_0 and token_1 are the same token it should skip the swap in the router?
    fn initialize(
        e: Env, 
        soroswap_router_address: Address, 
        soroswap_factory_address: Address, 
        xycloans_pool_address: Address,
        pool_token: Address,
        token_in: Address
    ) -> Result<(), AdapterError>;
}

#[contractimpl]
impl InitializeTrait for XycloansAdapter {
    fn initialize(
        e: Env,
        soroswap_router_address: Address,
        soroswap_factory_address: Address,
        xycloans_pool_address: Address,
        pool_token: Address,
        token_in: Address
    ) -> Result<(), AdapterError> {

        if is_initialized(&e) {
            return Err(AdapterError::AlreadyInitialized);
        }
    
        set_initialized(&e);
        set_soroswap_router_address(&e, soroswap_router_address);
        set_soroswap_factory_address(&e, soroswap_factory_address);
        set_xycloans_pool_address(&e, xycloans_pool_address);
        set_pool_token(&e, pool_token);
        set_token_in(&e, token_in);

        event::initialized(&e, true);
        extend_instance_ttl(&e);
        Ok(())
    }
}

#[contractimpl]
impl DeFindexAdapterTrait for XycloansAdapter {
    fn deposit(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<(), AdapterError> {
        check_initialized(&e)?;
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

        Ok(())
    }

    fn withdraw(
        e: Env,
        from: Address,
    ) -> Result<i128, AdapterError> {
        from.require_auth();
        check_initialized(&e)?;
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

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, AdapterError> {
        check_initialized(&e)?;

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
        ).map_err(|_| AdapterError::ProtocolAddressNotFound)?;
        
        let amount_out = get_amount_out(total, reserve_0, reserve_1).map_err(|_| AdapterError::ExternalError)?;
    
        Ok(amount_out)
    }
}

mod test;