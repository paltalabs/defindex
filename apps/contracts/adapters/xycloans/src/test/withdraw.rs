use defindex_adapter_interface::AdapterError;
use crate::test::XycloansAdapterTest;

#[test]
fn test_withdraw() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    test.adapter_contract.initialize(&test.soroswap_router_contract.address, &test.soroswap_factory_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);

    test.adapter_contract.deposit(&1_000_000_000_000_000_000, &test.user);
    assert_eq!(test.token_1.balance(&test.user), 19_000_000_000_000_000_000);
    
    // 1_000_000_000_000_000_000
    // 0_995_201_450_377_842_726
    let balance = test.adapter_contract.balance(&test.user);
    assert_eq!(balance, 0);
}

#[test]
fn test_withdraw_not_initialized() {
    let test = XycloansAdapterTest::setup();

    let result = test.adapter_contract.try_withdraw(&test.user);

    assert_eq!(result, Err(Ok(AdapterError::NotInitialized)));
}