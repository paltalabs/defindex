use soroban_sdk::{vec, Address, Vec};

use crate::error::ContractError;
use crate::test::{create_strategy_params, DeFindexVaultTest};

#[test]
fn test_initialize_and_get_roles() {
    let test = DeFindexVaultTest::setup();
    let strategy_params = create_strategy_params(&test);
    let tokens: Vec<Address> = vec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
    let ratios: Vec<u32> = vec![&test.env, 1, 1];

    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &tokens, &ratios, &strategy_params);

    let manager_role = test.defindex_contract.get_manager();
    let fee_receiver_role = test.defindex_contract.get_fee_receiver();
    let emergency_manager_role = test.defindex_contract.get_emergency_manager();
  
    assert_eq!(manager_role, test.manager);
    assert_eq!(fee_receiver_role, test.fee_receiver);
    assert_eq!(emergency_manager_role, test.emergency_manager);
}

#[test]
fn test_get_roles_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let manager_role = test.defindex_contract.try_get_manager();
    let fee_receiver_role = test.defindex_contract.try_get_manager();
    let emergency_manager_role = test.defindex_contract.try_get_manager();

    assert_eq!(manager_role, Err(Ok(ContractError::RoleNotFound)));
    assert_eq!(fee_receiver_role, Err(Ok(ContractError::RoleNotFound)));
    assert_eq!(emergency_manager_role, Err(Ok(ContractError::RoleNotFound)));
}

#[test]
fn test_initialize_twice() {
    let test = DeFindexVaultTest::setup();
    let strategy_params = create_strategy_params(&test);
    let tokens: Vec<Address> = vec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
    let ratios: Vec<u32> = vec![&test.env, 1, 1];

    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &tokens, &ratios, &strategy_params);

    let result_second_init = test.defindex_contract.try_initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &tokens, &ratios, &strategy_params);
    assert_eq!(
        result_second_init,
        Err(Ok(ContractError::AlreadyInitialized))
    );
}

#[test]
fn test_deposit_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_deposit(&100i128, &users[0]);

    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}

#[test]
fn test_withdraw_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&users[0]);
    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}

#[test]
fn test_emergency_withdraw_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_emergency_withdraw(&users[0]);
    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}