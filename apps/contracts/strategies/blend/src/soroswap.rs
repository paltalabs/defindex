use defindex_strategy_core::StrategyError;
use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec};

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

    e.invoke_contract(
        &config.router,
        &Symbol::new(&e, "swap_exact_tokens_for_tokens"),
        swap_args,
    )
}