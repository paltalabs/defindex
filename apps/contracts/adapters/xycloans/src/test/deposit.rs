use defindex_adapter_interface::AdapterError;
use crate::test::XycloansAdapterTest;

#[test]
fn test_deposit() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    test.adapter_contract.initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);

    test.adapter_contract.deposit(&1_000_000_000_000_000_000, &test.user);
    assert_eq!(test.token_1.balance(&test.user), 19_000_000_000_000_000_000);
}

#[test]
fn deposit_amount_in_negative() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    test.adapter_contract.initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);

    let result = test.adapter_contract.try_deposit(&-1_000_000_000_000_000_000, &test.user);
    assert_eq!(
        result,
        Err(Ok(AdapterError::NegativeNotAllowed))
    );
}

#[test]
#[should_panic]
fn deposit_amount_insufficient_input() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    test.adapter_contract.initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);

    test.adapter_contract.deposit(&100_000_000_000_000_000_000, &test.user);
}

#[test]
fn test_deposit_not_initialized() {
    let test = XycloansAdapterTest::setup();

    let result = test.adapter_contract.try_deposit(&1_000_000_000_000_000_000, &test.user);

    assert_eq!(result, Err(Ok(AdapterError::NotInitialized)));
}