use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec};

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

    e.invoke_contract(
        &get_soroswap_router(e),
        &Symbol::new(&e, "swap_exact_tokens_for_tokens"),
        swap_args,
    )
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

    let swap_args: Vec<Val> = vec![
        e,
        amount_out.into_val(e),
        amount_in_max.into_val(e),
        vec![e, token_in.to_val(), token_out.to_val()].into_val(e), // path
        e.current_contract_address().to_val(),
        deadline.into_val(e),
    ];

    e.invoke_contract(
        &get_soroswap_router(e),
        &Symbol::new(&e, "swap_tokens_for_exact_tokens"),
        swap_args,
    )
}
