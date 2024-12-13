use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec, String, auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}};

use crate::{
    models::DexDistribution,
    storage::{get_assets, get_factory},
    ContractError,
};

// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct Adapter {
//     pub protocol_id: String,
//     pub address: Address,
//     pub paused: bool,
// }

fn fetch_aggregator_address(e: &Env) -> Address {
    let factory_address = get_factory(e);

    e.invoke_contract(
        &factory_address,
        &Symbol::new(&e, "aggregator"),
        Vec::new(&e),
    )
}

fn is_supported_asset(e: &Env, token: &Address) -> bool {
    let assets = get_assets(e);
    assets.iter().any(|asset| &asset.address == token)
}

pub fn internal_swap_exact_tokens_for_tokens(
    e: &Env,
    router_address: &Address,
    pair_address: &Address,
    token_in: &Address,
    token_out: &Address,
    amount_in: &i128,
    amount_out_min: &i128,
    // distribution: &Vec<DexDistribution>,
    deadline: &u64,
) -> Result<(), ContractError> {


    // let aggregator_address = fetch_aggregator_address(e);

    // Ok(soroswap_router_client.swap_exact_tokens_for_tokens(
    //     &amount_in,
    //     &amount_out_min,
    //     &path,
    //     &to,
    //     &deadline
    // ))

    // Check if both tokens are supported by the vault
    if !is_supported_asset(e, token_in) || !is_supported_asset(e, token_out) {
        return Err(ContractError::UnsupportedAsset);
    }
    let path = vec![&e, token_in.clone(), token_out.clone()];

    let mut swap_args: Vec<Val> = vec![&e];
    // swap_args.push_back(token_in.to_val());
    // swap_args.push_back(token_out.to_val());
    swap_args.push_back(amount_in.into_val(e));
    swap_args.push_back(amount_out_min.into_val(e));
    swap_args.push_back(path.into_val(e)); 
    // swap_args.push_back(distribution.into_val(e));
    swap_args.push_back(e.current_contract_address().to_val());
    swap_args.push_back(deadline.into_val(e));

    
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

    e.invoke_contract(
        &router_address,
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
    distribution: &Vec<DexDistribution>,
    deadline: &u64,
) -> Result<(), ContractError> {
    let aggregator_address = fetch_aggregator_address(e);

    // Check if both tokens are supported by the vault
    if !is_supported_asset(e, token_in) || !is_supported_asset(e, token_out) {
        return Err(ContractError::UnsupportedAsset);
    }

    let mut swap_args: Vec<Val> = vec![&e];
    swap_args.push_back(token_in.to_val());
    swap_args.push_back(token_out.to_val());
    swap_args.push_back(amount_out.into_val(e));
    swap_args.push_back(amount_in_max.into_val(e));
    swap_args.push_back(distribution.into_val(e));
    swap_args.push_back(e.current_contract_address().to_val());
    swap_args.push_back(deadline.into_val(e));

    e.invoke_contract(
        &aggregator_address,
        &Symbol::new(&e, "swap_tokens_for_exact_tokens"),
        swap_args,
    )
}
