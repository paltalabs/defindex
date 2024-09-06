use soroban_sdk::{symbol_short, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, Address, IntoVal, Symbol};

use crate::error::ContractError;
use crate::test::{create_adapter_params, DeFindexVaultTest};

extern crate alloc;
use alloc::vec;

#[test]
fn test_set_new_fee_receiver_by_fee_receiver() {
    let test = DeFindexVaultTest::setup();
    let adapter_params = create_adapter_params(&test);
    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &adapter_params);

    let fee_receiver_role = test.defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Fee Receiver is setting the new fee receiver
    test.defindex_contract.set_fee_receiver(&test.fee_receiver, &users[0]);

    let new_fee_receiver_role = test.defindex_contract.get_fee_receiver();
    assert_eq!(new_fee_receiver_role, users[0]);
}

#[test]
fn test_set_new_fee_receiver_by_manager() {
    let test = DeFindexVaultTest::setup();
    let adapter_params = create_adapter_params(&test);
    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &adapter_params);

    let fee_receiver_role = test.defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Now Manager is setting the new fee receiver
    test.defindex_contract.set_fee_receiver(&test.manager, &users[0]);

    let new_fee_receiver_role = test.defindex_contract.get_fee_receiver();
    assert_eq!(new_fee_receiver_role, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #400)")] // Unauthorized
fn test_set_new_fee_receiver_by_emergency_manager() {
    let test = DeFindexVaultTest::setup();
    let adapter_params = create_adapter_params(&test);
    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &adapter_params);

    let fee_receiver_role = test.defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Now Emergency Manager is setting the new fee receiver
    test.defindex_contract.set_fee_receiver(&test.emergency_manager, &users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #400)")] // Unauthorized
fn test_set_new_fee_receiver_invalid_sender() {
  let test = DeFindexVaultTest::setup();
  let adapter_params = create_adapter_params(&test);
  test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &adapter_params);

  let fee_receiver_role = test.defindex_contract.get_fee_receiver();
  assert_eq!(fee_receiver_role, test.fee_receiver);

  let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
  // Trying to set the new fee receiver with an invalid sender
  test.defindex_contract.set_fee_receiver(&users[0], &users[0]);
}

#[test]
fn test_set_new_manager_by_manager() {
    let test = DeFindexVaultTest::setup();
    let adapter_params = create_adapter_params(&test);
    test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &adapter_params);

    let manager_role = test.defindex_contract.get_manager();
    assert_eq!(manager_role, test.manager);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Manager is setting the new manager
    test.defindex_contract.set_manager(&users[0]);

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            test.defindex_contract.address.clone(),
            Symbol::new(&test.env, "set_manager"),
            (
                users[0].clone(),
            )
                .into_val(&test.env),
        )),
        sub_invocations: vec![], 
    };
    assert_eq!(test.env.auths(), vec![(test.manager, expected_auth)]);

    let new_manager_role = test.defindex_contract.get_manager();
    assert_eq!(new_manager_role, users[0]);
}