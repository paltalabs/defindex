// Cannot Initialize twice
extern crate std;
use soroban_sdk::token::StellarAssetClient;
use crate::test::{create_fixapr_strategy, FixAprStrategyTest};

#[test]
fn check_storage() {
    let test = FixAprStrategyTest::setup();

    //MINT 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&test.strategy_admin, &starting_amount);

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.strategy_admin, starting_amount);

    // get asset should return underlying asset
    let underlying_asset = strategy.asset();
    assert_eq!(underlying_asset, test.token.address);

    // get contract token amount
    let contract_token_amount = test.token.balance(&strategy.address);
    assert_eq!(contract_token_amount, starting_amount);
}