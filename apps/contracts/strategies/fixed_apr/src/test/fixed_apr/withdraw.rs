use crate::{calculate_yield, test::FixAprStrategyTest};
use defindex_strategy_core::StrategyError;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::token::StellarAssetClient;
use soroban_sdk::vec;
use soroban_sdk::{IntoVal, Vec, Val};

#[test]
fn withdraw() {
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

    test.strategy.withdraw(&amount, &users[0]);
    let user_balance_after_withdraw = test.token.balance(&users[0]);
    assert_eq!(user_balance_after_withdraw, amount);
}

#[test]
fn withdraw_with_harvest() {
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

    // Simulate one year passing
    let one_year_in_seconds = 31_536_000u64;
    test.env.ledger().set_timestamp(test.env.ledger().timestamp() + one_year_in_seconds);

    test.strategy.harvest(&users[0]);

    let expected_reward = calculate_yield(amount, 1000u32, one_year_in_seconds);
    let user_balance_after_harvest = test.strategy.balance(&users[0]);
    assert_eq!(user_balance_after_harvest, amount + expected_reward);

    test.strategy.withdraw(&amount, &users[0]);
    let user_balance_after_withdraw = test.token.balance(&users[0]);
    assert_eq!(user_balance_after_withdraw, amount);
}

#[test]
fn withdraw_then_harvest_then_withdraw_again() {
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

    let amount = 1_000_0_000_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &amount);

    test.strategy.deposit(&amount, &users[0]);
    let user_balance = test.token.balance(&users[0]);
    assert_eq!(user_balance, 0);
    
    // Simulate one year passing
    let one_year_in_seconds = 31_536_000u64;
    test.env.ledger().set_timestamp(test.env.ledger().timestamp() + one_year_in_seconds);
    
    let user_balance_before_harvest = test.strategy.balance(&users[0]);
    assert_eq!(user_balance_before_harvest, amount);

    test.strategy.withdraw(&amount, &users[0]);
    let user_balance_after_withdraw = test.token.balance(&users[0]);
    assert_eq!(user_balance_after_withdraw, amount);

    // test.strategy.harvest(&users[0]);

    // let expected_reward = calculate_yield(amount, 1000u32, one_year_in_seconds);
    // let user_balance_after_harvest = test.strategy.balance(&users[0]);
    // assert_eq!(user_balance_after_harvest, expected_reward);

    // test.strategy.withdraw(&expected_reward, &users[0]);
    // let user_balance_after_second_withdraw = test.token.balance(&users[0]);
    // assert_eq!(user_balance_after_second_withdraw, amount + expected_reward);
}

#[test]
fn withdraw_with_no_balance() {
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

    let result = test.strategy.try_withdraw(&amount, &users[0]);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
}