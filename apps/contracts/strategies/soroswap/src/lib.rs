#![no_std]
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, contract, contractimpl, vec, Address, Env, IntoVal, String, Symbol, Val, Vec};

mod storage;
mod soroswap_router;
mod soroswap_pair;

use storage::{
    extend_instance_ttl, get_soroswap_router_address, has_soroswap_router_address, set_soroswap_router_address
};
use soroswap_router::SoroswapRouterClient;
use soroswap_pair::SoroswapPairClient;
use defindex_strategy_core::{DeFindexStrategyTrait, StrategyError};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        Err(StrategyError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

fn check_initialized(e: &Env) -> Result<(), StrategyError> {
    if has_soroswap_router_address(e) {
        Ok(())
    } else {
        Err(StrategyError::NotInitialized)
    }
}

#[contract]
struct SoroswapAdapter;

#[contractimpl]
impl DeFindexStrategyTrait for SoroswapAdapter {
    fn __constructor(
        e: Env,
        _asset: Address,
        init_args: Vec<Val>,
    ) {
        let protocol_address = init_args.get(0).ok_or(StrategyError::InvalidArgument).unwrap().into_val(&e);
    
        set_soroswap_router_address(&e, protocol_address);
    }

    fn asset(e: Env) -> Result<Address, StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        let protocol_address = get_soroswap_router_address(&e);
        Ok(protocol_address)
    }

    fn deposit(
        e: Env,
        amount: i128,
        from: Address,
    ) -> Result<i128, StrategyError> {
        from.require_auth();
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);

        // let usdc_address = Address::from_string(&String::from_str(&e, "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75"));
        // let xlm_address = Address::from_string(&String::from_str(&e, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"));
        let usdc_address = Address::from_string(&String::from_str(&e, "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA"));
        let xlm_address = Address::from_string(&String::from_str(&e, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"));
        
        // Setting up Soroswap router client
        let soroswap_router_address = get_soroswap_router_address(&e);
        let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);
        
        // let pair_address = Address::from_string(&String::from_str(&e, "CAM7DY53G63XA4AJRS24Z6VFYAFSSF76C3RZ45BE5YU3FQS5255OOABP"));
        let pair_address = Address::from_string(&String::from_str(&e, "CAAXGP7LTPV4A57LSKDWTSPPJUGFGNU34KQ3FYIPYUUP2SLFGVMTYKYU"));
        // let pair_address = soroswap_router_client.router_pair_for(&usdc_address, &xlm_address);

        let swap_amount = amount.checked_div(2).unwrap();

        let mut path: Vec<Address> = Vec::new(&e);
        path.push_back(usdc_address.clone());
        path.push_back(xlm_address.clone());

        let mut swap_args: Vec<Val> = vec![&e];
        swap_args.push_back(from.into_val(&e));
        swap_args.push_back(pair_address.into_val(&e));
        swap_args.push_back(swap_amount.into_val(&e));

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: usdc_address.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: swap_args.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);

        let swap_result = soroswap_router_client.swap_exact_tokens_for_tokens(
            &swap_amount,
            &0,
            &path,
            &from,
            &u64::MAX,
        );

        let total_swapped_amount = swap_result.last().unwrap();

        // Add liquidity
        let _result = soroswap_router_client.add_liquidity(
            &usdc_address,
            &xlm_address,
            &swap_amount,
            &total_swapped_amount,
            &0,
            &0,
            &from,
            &u64::MAX,
        );

        Ok(0)
    }

    fn harvest(e: Env, _from: Address) -> Result<(), StrategyError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        Ok(())
    }

    fn withdraw(
        e: Env,
        _amount: i128,
        from: Address,
    ) -> Result<i128, StrategyError> {
        from.require_auth();
        check_initialized(&e)?;
        extend_instance_ttl(&e);

        // let usdc_address = Address::from_string(&String::from_str(&e, "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75"));
        // let xlm_address = Address::from_string(&String::from_str(&e, "CAS3J7GYLGXMF6TDJBBYYSE3HQ6BBSMLNUQ34T6TZMYMW2EVH34XOWMA"));
        let usdc_address = Address::from_string(&String::from_str(&e, "CCKW6SMINDG6TUWJROIZ535EW2ZUJQEDGSKNIK3FBK26PAMBZDVK2BZA"));
        let xlm_address = Address::from_string(&String::from_str(&e, "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC"));
        
        // Setting up Soroswap router client
        let soroswap_router_address = get_soroswap_router_address(&e);
        let soroswap_router_client = SoroswapRouterClient::new(&e, &soroswap_router_address);
        
        // let pair_address = Address::from_string(&String::from_str(&e, "CAM7DY53G63XA4AJRS24Z6VFYAFSSF76C3RZ45BE5YU3FQS5255OOABP"));
        let pair_address = Address::from_string(&String::from_str(&e, "CAAXGP7LTPV4A57LSKDWTSPPJUGFGNU34KQ3FYIPYUUP2SLFGVMTYKYU"));
    
        let soroswap_pair_client = SoroswapPairClient::new(&e, &pair_address);
        let lp_balance = soroswap_pair_client.balance(&from);

        let mut swap_args: Vec<Val> = vec![&e];
        swap_args.push_back(from.into_val(&e));
        swap_args.push_back(pair_address.into_val(&e));
        swap_args.push_back(lp_balance.into_val(&e));

        e.authorize_as_current_contract(vec![
            &e,
            InvokerContractAuthEntry::Contract( SubContractInvocation {
                context: ContractContext {
                    contract: pair_address.clone(),
                    fn_name: Symbol::new(&e, "transfer"),
                    args: swap_args.clone(),
                },
                sub_invocations: vec![&e]
            })
        ]);

        // Remove liquidity
        let (_usdc_amount, xlm_amount) = soroswap_router_client.remove_liquidity(
            &usdc_address,
            &xlm_address,
            &lp_balance,
            &0,
            &0,
            &from,
            &u64::MAX,
        );

        let mut path: Vec<Address> = Vec::new(&e);
        path.push_back(xlm_address.clone());
        path.push_back(usdc_address.clone());

        let swap_result = soroswap_router_client.swap_exact_tokens_for_tokens(
            &xlm_amount,
            &0,
            &path,
            &from,
            &u64::MAX,
        );

        let total_swapped_amount = swap_result.last().unwrap();

        Ok(total_swapped_amount)
    }

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, StrategyError> {
        // Constants
        const SCALE: i128 = 10_000_000; // A scaling factor to maintain precision within 7 decimals
    
        // Should get pair reserves
        // let pair_address = Address::from_string(&String::from_str(&e, "CAM7DY53G63XA4AJRS24Z6VFYAFSSF76C3RZ45BE5YU3FQS5255OOABP"));
        let pair_address = Address::from_string(&String::from_str(&e, "CAAXGP7LTPV4A57LSKDWTSPPJUGFGNU34KQ3FYIPYUUP2SLFGVMTYKYU"));
        let soroswap_pair_client = SoroswapPairClient::new(&e, &pair_address);
    
        // Get the reserves from the pair
        let (reserve_usdc, reserve_xlm) = soroswap_pair_client.get_reserves();
    
        // Get the total supply of LP tokens and the user's LP token balance
        let total_lp_tokens = soroswap_pair_client.total_supply();
        let user_lp_tokens = soroswap_pair_client.balance(&from);
    
        // Ensure no division by zero
        if total_lp_tokens == 0 {
            return Err(StrategyError::NegativeNotAllowed);
        }
    
        // Calculate the user's share of the pool as a scaled integer
        let user_share = (user_lp_tokens as i128 * SCALE) / total_lp_tokens as i128;
    
        // Calculate the user's share of each reserve
        let user_usdc_share = (reserve_usdc as i128 * user_share) / SCALE;
        let user_xlm_share = (reserve_xlm as i128 * user_share) / SCALE;
    
        // Ensure no division by zero in price calculation
        if reserve_xlm == 0 {
            return Err(StrategyError::NegativeNotAllowed);
        }
    
        // Calculate the price of XLM in USDC as a scaled integer
        let xlm_price_in_usdc = (reserve_usdc as i128 * SCALE) / reserve_xlm as i128;
    
        // Convert the user's XLM share to USDC
        let user_xlm_in_usdc = (user_xlm_share * xlm_price_in_usdc) / SCALE;
    
        // Calculate the total USDC value
        let total_usdc_value = user_usdc_share + user_xlm_in_usdc;
    
        Ok(total_usdc_value)
    }
}
