// Cannot Initialize twice
extern crate std;
use soroban_sdk::{IntoVal, Vec, Val};
use crate::test::HodlStrategyTest;
use crate::test::StrategyError;

#[test]
fn cannot_initialize_twice() {
    let test = HodlStrategyTest::setup();

    let init_fn_args: Vec<Val> = (0,).into_val(&test.env);

    test.strategy.initialize(&test.token.address, &init_fn_args);
    let result = test.strategy.try_initialize(&test.token.address , &init_fn_args);
    assert_eq!(result, Err(Ok(StrategyError::AlreadyInitialized)));
    
}