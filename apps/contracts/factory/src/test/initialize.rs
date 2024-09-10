use soroban_sdk::{vec, Address, BytesN, Vec};

use crate::error::FactoryError;
use crate::test::{create_strategy_params, DeFindexFactoryTest};

#[test]
fn test_initialize_and_get_storage() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

    let factory_admin = test.factory_contract.get_admin();
    let factory_defindex_receiver = test.factory_contract.get_defindex_receiver();
  
    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);
}

#[test]
fn test_get_storage_not_yet_initialized() {
    let test = DeFindexFactoryTest::setup();
    let factory_admin = test.factory_contract.try_get_admin();
    let factory_defindex_receiver = test.factory_contract.try_get_defindex_receiver();

    assert_eq!(factory_admin, Err(Ok(FactoryError::NotInitialized)));
    assert_eq!(factory_defindex_receiver, Err(Ok(FactoryError::NotInitialized)));
}

#[test]
fn test_initialize_twice() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

    let result_second_init = test.factory_contract.try_initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);
    assert_eq!(
        result_second_init,
        Err(Ok(FactoryError::AlreadyInitialized))
    );
}

#[test]
fn test_create_defindex_not_yet_initialized() {
    let test = DeFindexFactoryTest::setup();

    let tokens: Vec<Address> = vec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
    let ratios: Vec<u32> = vec![&test.env, 1, 1];

    let strategy_params = create_strategy_params(&test);
    let salt = BytesN::from_array(&test.env, &[0; 32]);
    
    let result = test.factory_contract.try_create_defindex_vault(
        &test.emergency_manager, 
        &test.fee_receiver,
        &test.manager,
        &tokens,
        &ratios,
        &strategy_params,
        &salt
    );

    assert_eq!(result, Err(Ok(FactoryError::NotInitialized)));
}