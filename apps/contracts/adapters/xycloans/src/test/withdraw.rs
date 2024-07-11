use defindex_adapter_interface::AdapterError;
use crate::test::XycloansAdapterTest;

#[test]
fn test_initialize() {
    let test = XycloansAdapterTest::setup();

    // Initialize Adapter
    test.adapter_contract.initialize(&test.router_contract.address, &test.xycloans_pool.address, &test.token_0.address, &test.token_1.address);
}