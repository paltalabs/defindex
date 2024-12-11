use crate::test::create_hodl_strategy;
use crate::test::HodlStrategyTest;
use crate::test::StrategyError;

// test deposit with negative amount
#[test]
fn deposit_with_negative_amount() {
    let test = HodlStrategyTest::setup();
    let strategy = create_hodl_strategy(&test.env, &test.token.address);

    let amount = -123456;

    let result = strategy.try_deposit(&amount, &test.user);
    assert_eq!(result, Err(Ok(StrategyError::NegativeNotAllowed)));
}

// check auth
#[test]
fn deposit_mock_auths() {
    let test = HodlStrategyTest::setup();
    let _strategy = create_hodl_strategy(&test.env, &test.token.address);

    test.env.mock_all_auths();
}

#[test]
fn deposit_and_withdrawal_flow() {
    let test = HodlStrategyTest::setup();

    // initialize
    let strategy = create_hodl_strategy(&test.env, &test.token.address);

    // Initial user token balance
    let balance = test.token.balance(&test.user);

    let amount = 123456;

    // Deposit amount of token from the user to the strategy
    strategy.deposit(&amount, &test.user);

    let balance_after_deposit = test.token.balance(&test.user);
    assert_eq!(balance_after_deposit, balance - amount);

    // Reading strategy balance
    let strategy_balance_after_deposit = test.token.balance(&strategy.address);
    assert_eq!(strategy_balance_after_deposit, amount);

    // Reading user balance on strategy contract
    let user_balance_on_strategy = strategy.balance(&test.user);
    assert_eq!(user_balance_on_strategy, amount);


    let amount_to_withdraw = 100_000;
    // Withdrawing token from the strategy to user
    strategy.withdraw(&amount_to_withdraw, &test.user);

    // Reading user balance in token
    let balance = test.token.balance(&test.user);
    assert_eq!(balance, balance_after_deposit + amount_to_withdraw);

    // Reading strategy balance in token
    let balance = test.token.balance(&strategy.address);
    assert_eq!(balance, amount - amount_to_withdraw);

    // Reading user balance on strategy contract
    let user_balance = strategy.balance(&test.user);
    assert_eq!(user_balance, amount - amount_to_withdraw);

    // now we will want to withdraw more of the remaining balance
    let amount_to_withdraw = 200_000;
    let result = strategy.try_withdraw(&amount_to_withdraw, &test.user);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

}

#[test]
fn deposit_from_a_withdrawal_from_b() {
    let test = HodlStrategyTest::setup();

    let result = test.strategy.try_deposit(&10_000_000, &test.user);
    assert_eq!(result, Err(Ok(StrategyError::NotInitialized)));

    // initialize
    let init_fn_args: Vec<Val> = (0,).into_val(&test.env);
    test.strategy.initialize(&test.token.address, &init_fn_args);   

    // Initial user token balance
    let balance = test.token.balance(&test.user);

    let amount = 123456;

    // Deposit amount of token from the user to the strategy
    test.strategy.deposit(&amount, &test.user);

    let balance_after_deposit = test.token.balance(&test.user);
    assert_eq!(balance_after_deposit, balance - amount);

    // Reading strategy balance
    let strategy_balance_after_deposit = test.token.balance(&test.strategy.address);
    assert_eq!(strategy_balance_after_deposit, amount);

    // Reading user balance on strategy contract
    let user_balance_on_strategy = test.strategy.balance(&test.user);
    assert_eq!(user_balance_on_strategy, amount);


    let amount_to_withdraw = 100_000;
    // Withdrawing token from the strategy to user
    let result = test.strategy.try_withdraw(&amount_to_withdraw, &test.user1);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
}