use soroban_sdk::{
    panic_with_error, vec, Address, Env, IntoVal, InvokeError, Symbol, Val, Vec,
    auth::{
        ContractContext, 
        InvokerContractAuthEntry, 
        SubContractInvocation}, 
};
use soroswap_library::{
    get_amount_in, 
    get_reserves_with_pair
};
use crate::{
    ContractError,
    storage::{get_assets, get_soroswap_router}
};

fn is_supported_asset(e: &Env, token: &Address) -> Result<bool, ContractError> {
    let assets = get_assets(e)?;
    Ok(assets.iter().any(|asset| &asset.address == token))
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
    if !is_supported_asset(e, token_in)? || !is_supported_asset(e, token_out)? {
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

    let pair_address: Address = e.try_invoke_contract::<Address, InvokeError>(
        &soroswap_router,
        &Symbol::new(&e, "router_pair_for"),
        vec![e, token_in.to_val(), token_out.to_val()],
    ).unwrap_or_else(|_| panic_with_error!(e, ContractError::SoroswapRouterError)).unwrap();

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_in.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    pair_address.clone(),
                    amount_in.clone(),
                )
                    .into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);

    let _result = e.try_invoke_contract::<Vec<i128>, InvokeError>(
        &get_soroswap_router(e),
        &Symbol::new(&e, "swap_exact_tokens_for_tokens"),
        swap_args.clone(),
    ).unwrap_or_else(|_| {
        panic_with_error!(e, ContractError::SwapExactInError);
    });
    Ok(())
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
    if !is_supported_asset(e, token_in)? || !is_supported_asset(e, token_out)? {
        return Err(ContractError::UnsupportedAsset);
    }
    let soroswap_router = get_soroswap_router(e);

    let pair_address: Address = e.try_invoke_contract::<Address, InvokeError>(
        &soroswap_router,
        &Symbol::new(&e, "router_pair_for"),
        vec![e, token_in.to_val(), token_out.to_val()],
    ).unwrap_or_else(|_| panic_with_error!(e, ContractError::SoroswapRouterError)).unwrap();

    let (reserve_in, reserve_out) = get_reserves_with_pair(
        e.clone(),
        pair_address.clone(),
        token_in.clone(),
        token_out.clone(),
    )?;
    let amount_in = get_amount_in(amount_out.clone(), reserve_in, reserve_out)
        .map_err(|_| ContractError::SwapExactOutError)?;

    if amount_in > *amount_in_max {
        return Err(ContractError::ExcessiveInputAmount);
    }

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
                    amount_in.clone(),
                )
                    .into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);

    let _result = e.try_invoke_contract::<Vec<i128>, InvokeError>(
        &get_soroswap_router(e),
        &Symbol::new(&e, "swap_tokens_for_exact_tokens"),
        swap_args.clone(),
    ).unwrap_or_else(|_| {
        panic_with_error!(e, ContractError::SwapExactOutError);
    }).unwrap();
    Ok(())
}
