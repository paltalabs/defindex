// DeFindex Hodl Strategy Contract
mod fixed_strategy {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/fixed_apr_strategy.optimized.wasm");
    pub type FixedStrategyClient<'a> = Client<'a>;
}

pub use fixed_strategy::FixedStrategyClient;
use soroban_sdk::{Address, Env, Val, Vec};

pub fn create_fixed_strategy_contract<'a>(e: &Env, asset: &Address, init_args: &Vec<Val>) -> FixedStrategyClient<'a> {
    let address = &e.register_contract_wasm(None, fixed_strategy::WASM);
    let strategy = FixedStrategyClient::new(e, address); 

    strategy.mock_all_auths().initialize(asset, init_args);
    strategy
}


