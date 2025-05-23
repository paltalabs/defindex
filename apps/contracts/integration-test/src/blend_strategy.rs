// DeFindex Blend Strategy Contract
mod blend_strategy {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/blend_strategy.optimized.wasm");
    pub type BlendStrategyClient<'a> = Client<'a>;
}

pub use blend_strategy::BlendStrategyClient;
use soroban_sdk::{vec, Address, Env, IntoVal, Val, Vec};

pub(crate) fn create_blend_strategy_contract(
    e: &Env,
    asset: &Address,
    blend_pool: &Address,
    blend_token: &Address,
    soroswap_router: &Address,
    reward_threshold: i128,
    keeper: &Address,
) -> Address {
    let init_args: Vec<Val> = vec![
        e,
        blend_pool.into_val(e),
        blend_token.into_val(e),
        soroswap_router.into_val(e),
        reward_threshold.into_val(e),
        keeper.into_val(e),
    ];

    let args = (asset, init_args);
    e.register(blend_strategy::WASM, args)
}