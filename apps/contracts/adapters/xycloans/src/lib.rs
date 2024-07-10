#![no_std]
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, contract, contractimpl, vec, Address, Env, IntoVal, String, Symbol, Val, Vec};

mod event;
mod storage;
mod soroswap_router;
mod xycloans_pool;

use storage::{
    extend_instance_ttl, get_soroswap_router_address, get_token_0_address, get_token_1_address, get_xycloans_pool_address, is_initialized, set_initialized, set_soroswap_router_address, set_token_0_address, set_token_1_address, set_xycloans_pool_address
};
use soroswap_router::SoroswapRouterClient;
use xycloans_pool::XycloansPoolClient;
use defindex_adapter_interface::{DeFindexAdapterTrait, AdapterError};


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
        xycloans_pool_address: Address,
        token_0_address: Address,
        token_1_address: Address
    ) -> Result<(), AdapterError>;
}

#[contractimpl]
impl InitializeTrait for XycloansAdapter {
    fn initialize(
        e: Env,
        soroswap_router_address: Address,
        xycloans_pool_address: Address,
        token_0_address: Address,
        token_1_address: Address
    ) -> Result<(), AdapterError> {

        if is_initialized(&e) {
            return Err(AdapterError::AlreadyInitialized);
        }
    
        set_initialized(&e);
        set_soroswap_router_address(&e, soroswap_router_address);
        set_xycloans_pool_address(&e, xycloans_pool_address);
        set_token_0_address(&e, token_0_address);
        set_token_1_address(&e, token_1_address);

        event::initialized(&e, true);
        extend_instance_ttl(&e);
        Ok(())
    }
}

#[contractimpl]
impl DeFindexAdapterTrait for XycloansAdapter {
    fn deposit(
        e: Env,
        usdc_amount: i128,
        from: Address,
    ) -> Result<(), AdapterError> {
        check_initialized(&e)?;
        check_nonnegative_amount(usdc_amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        let token_0_address = get_token_0_address(&e);
        let token_1_address = get_token_1_address(&e);

        // Setting up Soroswap router client
        let soroswap_router_address = get_soroswap_router_address(&e);
        let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);

        // This could be hardcoded to perform less instructions
        let pair_address = soroswap_router_client.router_pair_for(&token_0_address, &token_1_address);

        let mut path: Vec<Address> = Vec::new(&e);
        path.push_back(token_0_address.clone());
        path.push_back(token_1_address.clone());

        let mut args: Vec<Val> = vec![&e];
        args.push_back(from.into_val(&e));
        args.push_back(pair_address.into_val(&e));
        args.push_back(usdc_amount.into_val(&e));

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: token_0_address.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: args.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);

        // let swap_amount = usdc_amount/2;
        // e.current_contract_address().require_auth();
        let res = soroswap_router_client.swap_exact_tokens_for_tokens(
            &usdc_amount,
            &0,
            &path,
            &from,
            &u64::MAX,
        );

        let total_swapped_amount = res.last().unwrap();

        // Xycloans Deposit XLM (WORKING)
        let xycloans_address = get_xycloans_pool_address(&e);
        let xycloans_pool_client = XycloansPoolClient::new(&e, &xycloans_address);
        xycloans_pool_client.deposit(&from, &total_swapped_amount);

        Ok(())

    }

    fn withdraw(
        e: Env,
        from: Address,
    ) -> Result<(), AdapterError> {
        from.require_auth();
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        let xycloans_address = get_xycloans_pool_address(&e);
        let xycloans_pool_client = XycloansPoolClient::new(&e, &xycloans_address);
        
        let shares: i128 = xycloans_pool_client.shares(&from);
        xycloans_pool_client.withdraw(&from, &shares);

        xycloans_pool_client.update_fee_rewards(&from);
        xycloans_pool_client.withdraw_matured(&from);

        Ok(())
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, AdapterError> {
        // Constants
        let xycloans_address = get_xycloans_pool_address(&e);
        let xycloans_pool_client = XycloansPoolClient::new(&e, &xycloans_address);
        
        let shares: i128 = xycloans_pool_client.shares(&from);
        let matured: i128 = xycloans_pool_client.matured(&from);
        
        let total: i128 = shares.checked_add(matured).unwrap();
    
        Ok(total)
    }
}
