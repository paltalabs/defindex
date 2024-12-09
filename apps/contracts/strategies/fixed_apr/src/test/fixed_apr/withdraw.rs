use crate::test::create_fixapr_strategy;
use crate::{calculate_yield, test::FixAprStrategyTest};
use defindex_strategy_core::StrategyError;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::token::StellarAssetClient;

#[test]
fn withdraw() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &amount);

    strategy.deposit(&amount, &users[0]);
    let user_balance = test.token.balance(&users[0]);
    assert_eq!(user_balance, 0);

    strategy.withdraw(&amount, &users[0]);
    let user_balance_after_withdraw = test.token.balance(&users[0]);
    assert_eq!(user_balance_after_withdraw, amount);
}

#[test]
fn withdraw_with_harvest() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &amount);

    strategy.deposit(&amount, &users[0]);
    let user_balance = test.token.balance(&users[0]);
    assert_eq!(user_balance, 0);

    // Simulate one year passing
    let one_year_in_seconds = 31_536_000u64;
    test.env.ledger().set_timestamp(test.env.ledger().timestamp() + one_year_in_seconds);

    strategy.harvest(&users[0]);

    let expected_reward = calculate_yield(amount, 1000u32, one_year_in_seconds);
    let user_balance_after_harvest = strategy.balance(&users[0]);
    assert_eq!(user_balance_after_harvest, amount + expected_reward);

    strategy.withdraw(&amount, &users[0]);
    let user_balance_after_withdraw = test.token.balance(&users[0]);
    assert_eq!(user_balance_after_withdraw, amount);
}

#[test]
fn withdraw_then_harvest_then_withdraw_again() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_000_000;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &amount);

    strategy.deposit(&amount, &users[0]);
    let user_balance = test.token.balance(&users[0]);
    assert_eq!(user_balance, 0);
    
    // Simulate one year passing
    let one_year_in_seconds = 31_536_000u64;
    test.env.ledger().set_timestamp(test.env.ledger().timestamp() + one_year_in_seconds);
    
    let user_balance_before_harvest = strategy.balance(&users[0]);
    assert_eq!(user_balance_before_harvest, amount);

    strategy.withdraw(&amount, &users[0]);
    let user_balance_after_withdraw = test.token.balance(&users[0]);
    assert_eq!(user_balance_after_withdraw, amount);

    // strategy.harvest(&users[0]);

    // let expected_reward = calculate_yield(amount, 1000u32, one_year_in_seconds);
    // let user_balance_after_harvest = strategy.balance(&users[0]);
    // assert_eq!(user_balance_after_harvest, expected_reward);

    // strategy.withdraw(&expected_reward, &users[0]);
    // let user_balance_after_second_withdraw = test.token.balance(&users[0]);
    // assert_eq!(user_balance_after_second_withdraw, amount + expected_reward);
}

#[test]
fn withdraw_with_no_balance() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 1);

    let amount = 1_000_0_00_000;

    let result = strategy.try_withdraw(&amount, &users[0]);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
}