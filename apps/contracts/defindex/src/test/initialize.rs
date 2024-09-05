use soroban_sdk::{Address, testutils::Address as _};

use crate::error::ContractError;
use crate::test::{create_adapter_params, DeFindexVaultTest};

#[test]
fn test_initializ_and_get_roles() {
    let test = DeFindexVaultTest::setup();
    let adapter_params = create_adapter_params(&test);
    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &adapter_params);

    let manager_role = test.defindex_contract.get_manager();
    assert_eq!(manager_role, test.manager)
    //TODO:; Check more roles
}

// #[test]
// fn test_get_factory_not_yet_initialized() {
//     let test = SoroswapRouterTest::setup();
//     let result = test.contract.try_get_factory();

//     assert_eq!(result, Err(Ok(CombinedRouterError::RouterNotInitialized)));
// }

// #[test]
// fn test_initialize_twice() {
//     let test = SoroswapRouterTest::setup();
//     test.contract.initialize(&test.factory.address);

//     let factory_another = Address::generate(&test.env);
//     let result_second_init = test.contract.try_initialize(&factory_another);
//     assert_eq!(
//         result_second_init,
//         Err(Ok(CombinedRouterError::RouterInitializeAlreadyInitialized))
//     );
// }