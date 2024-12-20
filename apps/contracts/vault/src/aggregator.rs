use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec, auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}};
use soroswap_library::{get_reserves_with_pair, get_amount_in};

use crate::{
    // models::DexDistribution,
    storage::{get_assets, get_factory, get_soroswap_router},
    ContractError,
};


fn is_supported_asset(e: &Env, token: &Address) -> bool {
    let assets = get_assets(e);
    assets.iter().any(|asset| &asset.address == token)
}

pub fn internal_swap_exact_tokens_for_tokens(
    e: &Env,
    token_in: &Address,
    token_out: &Address,
    amount_in: &i128,
    amount_out_min: &i128,
    deadline: &u64,
) -> Result<(), ContractError> {

    // Check if both tokens are supported by the vault
    if !is_supported_asset(e, token_in) || !is_supported_asset(e, token_out) {
        return Err(ContractError::UnsupportedAsset);
    }
    let swap_args: Vec<Val> = vec![
        e,
        amount_in.into_val(e),
        amount_out_min.into_val(e),
        vec![e, token_in.to_val(), token_out.to_val()].into_val(e), // path
        e.current_contract_address().to_val(),
        deadline.into_val(e),
    ];

    let soroswap_router = get_soroswap_router(e);

    let pair_address: Address = e.invoke_contract(
        &soroswap_router,
        &Symbol::new(&e, "router_pair_for"),
        vec![e, token_in.to_val(), token_out.to_val()],
    );

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_in.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    pair_address.clone(),
                    amount_in.clone()).into_val(e),

            },
            sub_invocations: vec![&e],
        }),
    ]);

    let result: Vec<i128> = e.invoke_contract(
        &get_soroswap_router(e),
        &Symbol::new(&e, "swap_exact_tokens_for_tokens"),
        swap_args,
    );
    Ok(())
    // TODO: Do something with the result
}

pub fn internal_swap_tokens_for_exact_tokens(
    e: &Env,
    token_in: &Address,
    token_out: &Address,
    amount_out: &i128,
    amount_in_max: &i128,
    deadline: &u64,
) -> Result<(), ContractError> {

    // Check if both tokens are supported by the vault
    if !is_supported_asset(e, token_in) || !is_supported_asset(e, token_out) {
        return Err(ContractError::UnsupportedAsset);
    }
    let soroswap_router = get_soroswap_router(e);
    let pair_address: Address = e.invoke_contract(
        &soroswap_router,
        &Symbol::new(&e, "router_pair_for"),
        vec![e, token_in.to_val(), token_out.to_val()],
    );
    let (reserve_in, reserve_out) = get_reserves_with_pair(e.clone(), pair_address.clone(), token_in.clone(), token_out.clone())?;
    let amount_in = get_amount_in(amount_out.clone(), reserve_in, reserve_out);

    let swap_args: Vec<Val> = vec![
        e,
        amount_out.into_val(e),
        amount_in_max.into_val(e),
        vec![e, token_in.to_val(), token_out.to_val()].into_val(e), // path
        e.current_contract_address().to_val(),
        deadline.into_val(e),
    ];

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_in.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    pair_address.clone(),
                    amount_in.clone()).into_val(e),

            },
            sub_invocations: vec![&e],
        }),
    ]);

    let result: Vec<i128> = e.invoke_contract(
        &get_soroswap_router(e),
        &Symbol::new(&e, "swap_tokens_for_exact_tokens"),
        swap_args,
    );
    Ok(())
}
