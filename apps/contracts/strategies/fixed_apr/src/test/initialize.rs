// Cannot Initialize twice
extern crate std;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::vec;
use soroban_sdk::{IntoVal, Vec, Val};
use crate::test::FixAprStrategyTest;
use crate::test::StrategyError;

#[test]
fn initialize() {
    let test = FixAprStrategyTest::setup();

    //MINT 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&test.strategy_admin, &starting_amount);

    let init_fn_args: Vec<Val> = vec![&test.env,
        1000u32.into_val(&test.env),
        test.strategy_admin.into_val(&test.env),
        starting_amount.into_val(&test.env),
    ];

    test.strategy.initialize(&test.token.address, &init_fn_args);

    // get asset should return underlying asset
    let underlying_asset = test.strategy.asset();
    assert_eq!(underlying_asset, test.token.address);

    // get contract token amount
    let contract_token_amount = test.token.balance(&test.strategy.address);
    assert_eq!(contract_token_amount, starting_amount);
}

#[test]
fn cannot_initialize_twice() {
    let test = FixAprStrategyTest::setup();

    //MINT 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&test.strategy_admin, &starting_amount);

    let init_fn_args: Vec<Val> = vec![&test.env,
        1000u32.into_val(&test.env),
        test.strategy_admin.into_val(&test.env),
        starting_amount.into_val(&test.env),
    ];

    test.strategy.initialize(&test.token.address, &init_fn_args);
    let result = test.strategy.try_initialize(&test.token.address , &init_fn_args);
    assert_eq!(result, Err(Ok(StrategyError::AlreadyInitialized)));

    // get asset should return underlying asset
    let underlying_asset = test.strategy.asset();
    assert_eq!(underlying_asset, test.token.address);
}