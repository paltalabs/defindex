use defindex_strategy_core::StrategyError;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, Env, IntoVal, Symbol, Vec,
};

use crate::storage::Config;

/// Performs a token swap using the Soroswap router.
///
/// This function swaps the specified amount of input tokens for a minimum amount of output tokens
/// along a given path, sending the output tokens to the specified recipient address before the
/// given deadline. It also handles the necessary authorization and contract invocations.
///
/// # Arguments
///
/// * `e` - The environment context.
/// * `amount_in` - The amount of the input token to be swapped.
/// * `amount_out_min` - The minimum amount of the output token expected from the swap.
/// * `path` - The swap path, specifying the sequence of tokens to be swapped.
/// * `to` - The recipient address for the output token.
/// * `deadline` - The deadline timestamp by which the swap must be completed.
/// * `config` - The configuration containing the router address.
///
/// # Returns
///
/// A result containing a vector of output amounts or a `StrategyError` if the swap fails.
pub fn internal_swap_exact_tokens_for_tokens(
    e: &Env,
    amount_in: &i128,
    amount_out_min: &i128,
    path: Vec<Address>,
    to: &Address,
    deadline: &u64,
    config: &Config,
) -> Result<Vec<i128>, StrategyError> {
    let swap_args = vec!(
        e,
        amount_in.into_val(e),
        amount_out_min.into_val(e),
        path.into_val(e),
        to.to_val(),
        deadline.into_val(e)
    );

    // Maybe instead of using the router directly, we should use the pair for swaps
    let pair_address: Address = e.invoke_contract(
        &config.router,
        &Symbol::new(&e, "router_pair_for"),
        path.into_val(e),
    );

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: match path.get(0) {
                    Some(address) => address.clone(),
                    None => {
                        panic!("Path must have at least one element")
                    }
                },
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
        swap_args.into_val(e),
    )
}
