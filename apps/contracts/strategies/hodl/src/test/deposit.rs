use crate::test::HodlStrategyTest;
use crate::test::StrategyError;

#[test]
fn test_deposit_and_withdrawal_flow() {
    let test = HodlStrategyTest::setup();
    let users = HodlStrategyTest::generate_random_users(&test.env, 1);

    // try deposit, harves, withdraw and balance function should return NotInitialized error before being initialize

    let result = test.strategy.try_deposit(&10_000_000, &users[0]);
    assert_eq!(result, Err(Ok(StrategyError::NotInitialized)));


    let amount: i128 = 1_000_000;
   

    // Reading user 0 balance
    let balance = test.token.balance(&users[0]);
    assert_eq!(balance, amount);

    // Depositing token 0 to the strategy from user
    test.strategy.deposit(&amount, &users[0]);

    // Reading user 0 balance
    let balance = test.token.balance(&users[0]);
    assert_eq!(balance, 0);

    // Reading strategy balance
    let balance = test.token.balance(&test.strategy.address);
    assert_eq!(balance, amount);

    // Reading user balance on strategy contract
    let user_balance = test.strategy.balance(&users[0]);
    assert_eq!(user_balance, amount);

    // Withdrawing token 0 from the strategy to user
    test.strategy.withdraw(&amount, &users[0]);

    // Reading user 0 balance
    let balance = test.token.balance(&users[0]);
    assert_eq!(balance, amount);

    // Reading strategy balance
    let balance = test.token.balance(&test.strategy.address);
    assert_eq!(balance, 0);

    // Reading user balance on strategy contract
    let user_balance = test.strategy.balance(&users[0]);
    assert_eq!(user_balance, 0);
}