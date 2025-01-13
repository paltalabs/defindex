// DeFindex Hodl Strategy Contract
mod fixed_strategy {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/fixed_apr_strategy.optimized.wasm"
    );
    pub type FixedStrategyClient<'a> = Client<'a>;
}

pub use fixed_strategy::FixedStrategyClient;
use soroban_sdk::{token::StellarAssetClient, vec, Address, Env, IntoVal, Val, Vec};

pub fn create_fixed_strategy_contract<'a>(
    e: &Env,
    asset: &Address,
    apr_bps: u32,
    token_admin_client: &StellarAssetClient,
) -> FixedStrategyClient<'a> {
    let init_args: Vec<Val> = vec![e, apr_bps.into_val(e)];

    let args = (asset, init_args);

    let client = FixedStrategyClient::new(e, &e.register(fixed_strategy::WASM, args));

    // Mint 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    token_admin_client
        .mock_all_auths()
        .mint(&client.address, &starting_amount);

    client
}
