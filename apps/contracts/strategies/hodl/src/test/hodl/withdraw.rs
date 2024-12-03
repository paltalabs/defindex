use crate::test::HodlStrategyTest;
use crate::test::StrategyError;
use soroban_sdk::{IntoVal, Vec, Val};
#[test]
fn withdraw() {
    let test = HodlStrategyTest::setup();

    // initialize
    let init_fn_args: Vec<Val> = (0,).into_val(&test.env);
    test.strategy.initialize(&test.token.address, &init_fn_args);   

    let balance = test.token.balance(&test.user);
    let amount = 123456;

    //Try to withdraw before depositing
    let result = test.strategy.try_withdraw(&amount, &test.user);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    // Deposit amount of token from the user to the strategy
    test.strategy.deposit(&amount, &test.user);

    let user_balance_after_deposit = test.token.balance(&test.user);
    assert_eq!(user_balance_after_deposit, balance - amount);

    // Reading strategy balance
    let strategy_balance_after_deposit = test.token.balance(&test.strategy.address);
    assert_eq!(strategy_balance_after_deposit, amount);

    // Reading user balance on strategy contract
    let user_balance_on_strategy = test.strategy.balance(&test.user);
    assert_eq!(user_balance_on_strategy, amount);


    let amount_to_withdraw = 100_000;
    // Withdrawing token from the strategy to user
    test.strategy.withdraw(&amount_to_withdraw, &test.user);

    // Reading user balance in token
    let balance = test.token.balance(&test.user);
    assert_eq!(balance, user_balance_after_deposit + amount_to_withdraw);

    // Reading strategy balance in token
    let balance = test.token.balance(&test.strategy.address);
    assert_eq!(balance, amount - amount_to_withdraw);

    // Reading user balance on strategy contract
    let user_balance = test.strategy.balance(&test.user);
    assert_eq!(user_balance, amount - amount_to_withdraw);

    //withdraw more than the user has
    let amount_to_withdraw = user_balance + 1;
    let result = test.strategy.try_withdraw(&amount_to_withdraw, &test.user);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
    
}
