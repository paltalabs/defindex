use defindex_strategy_interface::AdapterError;
use crate::test::XycloansAdapterTest;

#[test]
fn test_initialize() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    test.adapter_contract.initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);
}

#[test]
fn test_initialize_twice() {
    let test = XycloansAdapterTest::setup();

    //Initialize aggregator
    test.adapter_contract.initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);

    let result_second_init = test.adapter_contract.try_initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);
    assert_eq!(
        result_second_init,
        (Err(Ok(AdapterError::AlreadyInitialized)))
    );
}

#[test]
fn test_deposit_not_yet_initialized() {
    let test = XycloansAdapterTest::setup();
    let result = test.adapter_contract.try_deposit(&1_000_000_000, &test.user);

    assert_eq!(result, Err(Ok(AdapterError::NotInitialized)));
}

#[test]
fn test_withdraw_not_yet_initialized() {
    let test = XycloansAdapterTest::setup();
    let result = test.adapter_contract.try_withdraw(&test.user);

    assert_eq!(result, Err(Ok(AdapterError::NotInitialized)));
}