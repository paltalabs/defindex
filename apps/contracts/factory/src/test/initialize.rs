use soroban_sdk::{vec, Address, BytesN, Vec};

use crate::error::FactoryError;
use crate::test::{create_asset_params, DeFindexFactoryTest};

#[test]
fn test_initialize_and_get_storage() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &100u32, &test.defindex_wasm_hash);

    let factory_admin = test.factory_contract.admin();
    let factory_defindex_receiver = test.factory_contract.defindex_receiver();
  
    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);
}

#[test]
fn test_get_storage_not_yet_initialized() {
    let test = DeFindexFactoryTest::setup();
    let factory_admin = test.factory_contract.try_admin();
    let factory_defindex_receiver = test.factory_contract.try_defindex_receiver();

    assert_eq!(factory_admin, Err(Ok(FactoryError::NotInitialized)));
    assert_eq!(factory_defindex_receiver, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn test_initialize_twice() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &100u32, &test.defindex_wasm_hash);

    let result_second_init = test.factory_contract.try_initialize(&test.admin, &test.defindex_receiver, &100u32, &test.defindex_wasm_hash);
    assert_eq!(
        result_second_init,
        Err(Ok(FactoryError::AlreadyInitialized))
    );
}

#[test]
fn test_create_defindex_not_yet_initialized() {
    let test = DeFindexFactoryTest::setup();

    let asset_params = create_asset_params(&test);
    let salt = BytesN::from_array(&test.env, &[0; 32]);
    
    let result = test.factory_contract.try_create_defindex_vault(
        &test.emergency_manager, 
        &test.fee_receiver,
        &test.manager,
        &asset_params,
        &salt
    );

    assert_eq!(result, Err(Ok(FactoryError::NotInitialized)));
}