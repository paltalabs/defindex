use defindex_strategy_core::StrategyError;
use soroban_sdk::{IntoVal, Val, Vec};
use crate::test::XycloansAdapterTest;

#[test]
fn test_initialize() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    let init_fn_args: Vec<Val> = (
        &test.soroswap_router_contract.address, 
        &test.soroswap_factory_contract.address, 
        &test.xycloans_pool.address, 
        &test.token_0.address, 
        &test.token_1.address
    ).into_val(&test.env);

    // Initialize Adapter
    test.adapter_contract.initialize(&test.token_0.address, &init_fn_args);
}

#[test]
fn test_initialize_twice() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    let init_fn_args: Vec<Val> = (
        &test.soroswap_router_contract.address, 
        &test.soroswap_factory_contract.address, 
        &test.xycloans_pool.address, 
        &test.token_0.address, 
        &test.token_1.address
    ).into_val(&test.env);

    // Initialize Adapter
    test.adapter_contract.initialize(&test.token_0.address, &init_fn_args);

    let result_second_init = test.adapter_contract.try_initialize(&test.token_0.address,&init_fn_args);

    assert_eq!(
        result_second_init,
        (Err(Ok(StrategyError::AlreadyInitialized)))
    );
}

#[test]
fn test_deposit_not_yet_initialized() {
    let test = XycloansAdapterTest::setup();
    let result = test.adapter_contract.try_deposit(&1_000_000_000, &test.user);

    assert_eq!(result, Err(Ok(StrategyError::NotInitialized)));
}

#[test]
fn test_withdraw_not_yet_initialized() {
    let test = XycloansAdapterTest::setup();
    let result = test.adapter_contract.try_withdraw(&100i128, &test.user);

    assert_eq!(result, Err(Ok(StrategyError::NotInitialized)));
}