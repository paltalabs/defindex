// DeFindex Hodl Strategy Contract
mod hodl_strategy {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm"
    );
    pub type HodlStrategyClient<'a> = Client<'a>;
}

pub use hodl_strategy::HodlStrategyClient;
use soroban_sdk::{vec, Address, Env, Val, Vec};

pub fn create_hodl_strategy_contract<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val> = vec![e];
    let args = (asset.clone(), init_args);

    HodlStrategyClient::new(e, &e.register(hodl_strategy::WASM, args))
}
