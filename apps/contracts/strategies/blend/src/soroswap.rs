use defindex_strategy_core::StrategyError;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

use crate::storage::Config;

pub fn internal_swap_exact_tokens_for_tokens(
    e: &Env,
    amount_in: &i128,
    amount_out_min: &i128,
    path: Vec<Address>,
    to: &Address,
    deadline: &u64,
    config: &Config,
) -> Result<Vec<i128>, StrategyError> {
    let mut swap_args: Vec<Val> = vec![&e];
    swap_args.push_back(amount_in.into_val(e));
    swap_args.push_back(amount_out_min.into_val(e));
    swap_args.push_back(path.into_val(e));
    swap_args.push_back(to.to_val());
    swap_args.push_back(deadline.into_val(e));

    // Maybe instead of using the router directly, we should use the pair for swaps
    let pair_address: Address = e.invoke_contract(
        &config.router,
        &Symbol::new(&e, "router_pair_for"),
        vec![
            &e,
            path.get(0).unwrap().into_val(e),
            path.get(1).unwrap().into_val(e),
        ],
    );

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: path.get(0).unwrap().clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    pair_address,
                    amount_in.clone(),
                )
                    .into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);

    e.invoke_contract(
        &config.router,
        &Symbol::new(&e, "swap_exact_tokens_for_tokens"),
        swap_args,
    )
}
