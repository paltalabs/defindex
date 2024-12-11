use crate::test::create_fixapr_strategy;
use crate::test::FixAprStrategyTest;
use crate::test::StrategyError;
use soroban_sdk::token::StellarAssetClient;

// test deposit with negative amount
#[test]
fn deposit_with_negative_amount() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = -123456;

    let result = strategy.try_deposit(&amount, &users[0]);
    assert_eq!(result, Err(Ok(StrategyError::NegativeNotAllowed)));
}

// test deposit with zero amount
#[test]
fn deposit_with_zero_amount() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 0;

    strategy.deposit(&amount, &users[0]);
}

// test deposit with positive amount
#[test]
fn deposit() {
    let test = FixAprStrategyTest::setup();
    
    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &amount);

    strategy.deposit(&amount, &users[0]);
    let user_balance = test.token.balance(&users[0]);
    assert_eq!(user_balance, 0);
}

// test deposit with amount exceeding balance
#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")] // Unauthorized
fn deposit_with_exceeding_balance() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &(&amount - 100_0_000_000));

    strategy.deposit(&amount, &users[0]);
}
