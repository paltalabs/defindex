// DeFindex Hodl Strategy Contract
mod hodl_strategy {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm");
    pub type HodlStrategyClient<'a> = Client<'a>;
}

pub use hodl_strategy::HodlStrategyClient;
use soroban_sdk::{Address, Env, Val, Vec};

pub fn create_hodl_strategy_contract<'a>(e: &Env, asset: &Address, init_args: &Vec<Val>) -> HodlStrategyClient<'a> {
    let address = &e.register_contract_wasm(None, hodl_strategy::WASM);
    let strategy = HodlStrategyClient::new(e, address); 
    strategy.initialize(asset, init_args);
    strategy
}