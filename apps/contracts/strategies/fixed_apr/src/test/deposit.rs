use crate::test::FixAprStrategyTest;
use crate::test::StrategyError;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::vec;
use soroban_sdk::{IntoVal, Vec, Val};

// test deposit with negative amount
#[test]
fn deposit_with_negative_amount() {
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

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = -123456;

    let result = test.strategy.try_deposit(&amount, &users[0]);
    assert_eq!(result, Err(Ok(StrategyError::NegativeNotAllowed)));
}

// test deposit with zero amount
#[test]
fn deposit_with_zero_amount() {
    let test = FixAprStrategyTest::setup();
    // MINT 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&test.strategy_admin, &starting_amount);

    let init_fn_args: Vec<Val> = vec![&test.env,
        1000u32.into_val(&test.env),
        test.strategy_admin.into_val(&test.env),
        starting_amount.into_val(&test.env),
    ];

    test.strategy.initialize(&test.token.address, &init_fn_args);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 0;

    test.strategy.deposit(&amount, &users[0]);
}

// test deposit with positive amount
#[test]
fn deposit() {
    let test = FixAprStrategyTest::setup();
    // MINT 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&test.strategy_admin, &starting_amount);

    let init_fn_args: Vec<Val> = vec![&test.env,
        1000u32.into_val(&test.env),
        test.strategy_admin.into_val(&test.env),
        starting_amount.into_val(&test.env),
    ];

    test.strategy.initialize(&test.token.address, &init_fn_args);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &amount);

    test.strategy.deposit(&amount, &users[0]);
    let user_balance = test.token.balance(&users[0]);
    assert_eq!(user_balance, 0);
}

// test deposit with amount exceeding balance
#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")] // Unauthorized
fn deposit_with_exceeding_balance() {
    let test = FixAprStrategyTest::setup();
    // MINT 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&test.strategy_admin, &starting_amount);

    let init_fn_args: Vec<Val> = vec![&test.env,
        1000u32.into_val(&test.env),
        test.strategy_admin.into_val(&test.env),
        starting_amount.into_val(&test.env),
    ];

    test.strategy.initialize(&test.token.address, &init_fn_args);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &(&amount - 100_0_000_000));

    test.strategy.deposit(&amount, &users[0]);
}
