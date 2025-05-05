use soroban_sdk::{testutils::Ledger, token::StellarAssetClient, Bytes};

use crate::{
    calculate_yield,
    test::{create_fixapr_strategy, FixAprStrategyTest},
};

#[test]
fn test_harvest_yields_multiple_users() {
    let test = FixAprStrategyTest::setup();

    let apr = 1000u32;
    let strategy =
        create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    let users = FixAprStrategyTest::generate_random_users(&test.env, 4);

    // Mint tokens to users
    let user1_amount = 1_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[0], &user1_amount);
    let user2_amount = 2_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[1], &user2_amount);
    let user3_amount = 500_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[2], &user3_amount);
    let user4_amount = 10_000_0_000_000i128;
    StellarAssetClient::new(&test.env, &test.token.address).mint(&users[3], &user4_amount);

    // Deposit tokens for each user
    strategy.deposit(&user1_amount, &users[0]);
    let user1_balance = test.token.balance(&users[0]);
    assert_eq!(user1_balance, 0);

    strategy.deposit(&user2_amount, &users[1]);
    let user2_balance = test.token.balance(&users[1]);
    assert_eq!(user2_balance, 0);

    strategy.deposit(&user3_amount, &users[2]);
    let user3_balance = test.token.balance(&users[2]);
    assert_eq!(user3_balance, 0);

    strategy.deposit(&user4_amount, &users[3]);
    let user4_balance = test.token.balance(&users[3]);
    assert_eq!(user4_balance, 0);

    // Simulate one year passing
    let one_year_in_seconds = 31_536_000u64;
    test.env
        .ledger()
        .set_timestamp(test.env.ledger().timestamp() + one_year_in_seconds);

    // Harvest for each user
    strategy.harvest(&users[0], &None::<Bytes>);
    strategy.harvest(&users[1], &None::<Bytes>);
    strategy.harvest(&users[2], &None::<Bytes>);
    strategy.harvest(&users[3], &None::<Bytes>);

    // Check the harvested rewards for each user are correct
    let user1_expected_reward = calculate_yield(user1_amount, apr, one_year_in_seconds);
    let user2_expected_reward = calculate_yield(user2_amount, apr, one_year_in_seconds);
    let user3_expected_reward = calculate_yield(user3_amount, apr, one_year_in_seconds);
    let user4_expected_reward = calculate_yield(user4_amount, apr, one_year_in_seconds);

    assert_eq!(
        strategy.balance(&users[0]),
        user1_amount + user1_expected_reward
    );
    assert_eq!(
        strategy.balance(&users[1]),
        user2_amount + user2_expected_reward
    );
    assert_eq!(
        strategy.balance(&users[2]),
        user3_amount + user3_expected_reward
    );
    assert_eq!(
        strategy.balance(&users[3]),
        user4_amount + user4_expected_reward
    );
}
