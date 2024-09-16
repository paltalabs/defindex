use defindex_strategy_core::StrategyError;
use soroban_sdk::{Val, Vec, IntoVal};
use crate::test::XycloansAdapterTest;

#[test]
fn test_balance_not_initialized() {
    let test = XycloansAdapterTest::setup();

    let result = test.adapter_contract.try_balance(&test.user);
    assert_eq!(result, Err(Ok(StrategyError::NotInitialized)));
}

#[test]
fn test_balance_not_deposited() {
    let test = XycloansAdapterTest::setup();

    let init_fn_args: Vec<Val> = (
        &test.soroswap_router_contract.address, 
        &test.soroswap_factory_contract.address, 
        &test.xycloans_pool.address, 
        &test.token_0.address, 
        &test.token_1.address
    ).into_val(&test.env);

    // Initialize Adapter
    test.adapter_contract.initialize(&test.token_0.address, &init_fn_args);

    let balance = test.adapter_contract.balance(&test.user);
    assert_eq!(balance, 0);
}

#[test]
fn test_balance_deposited() {
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

    test.adapter_contract.deposit(&1_000_000_000_000_000_000, &test.user);
    assert_eq!(test.token_1.balance(&test.user), 19_999_999_999_999_000_000);
    
    // 1_000_000_000_000_000_000
    // 0_995_201_450_377_842_726
    // TODO: Calculate expected output. considering swap and fees for the swap on soroswap
    let balance = test.adapter_contract.balance(&test.user);
    assert_eq!(balance, 0);
}