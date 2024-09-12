use defindex_strategy_interface::StrategyError;
use soroban_sdk::{IntoVal, Val, Vec};
use crate::test::XycloansAdapterTest;

#[test]
fn test_deposit() {
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
    test.adapter_contract.initialize(&init_fn_args);

    test.adapter_contract.deposit(&1_000_000_000_000_000_000, &test.user);
    assert_eq!(test.token_1.balance(&test.user), 19_000_000_000_000_000_000);
}

#[test]
fn deposit_amount_in_negative() {
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
    test.adapter_contract.initialize(&init_fn_args);

    let result = test.adapter_contract.try_deposit(&-1_000_000_000_000_000_000, &test.user);
    assert_eq!(
        result,
        Err(Ok(StrategyError::NegativeNotAllowed))
    );
}

#[test]
#[should_panic]
fn deposit_amount_insufficient_input() {
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
    test.adapter_contract.initialize(&init_fn_args);

    test.adapter_contract.deposit(&100_000_000_000_000_000_000, &test.user);
}

#[test]
fn test_deposit_not_initialized() {
    let test = XycloansAdapterTest::setup();

    let result = test.adapter_contract.try_deposit(&1_000_000_000_000_000_000, &test.user);

    assert_eq!(result, Err(Ok(StrategyError::NotInitialized)));
}