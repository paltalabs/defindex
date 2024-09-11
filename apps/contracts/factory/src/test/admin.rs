use soroban_sdk::{testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke}, Address, IntoVal, Symbol};
extern crate alloc;
use alloc::vec;

use crate::test::DeFindexFactoryTest;

#[test]
fn test_set_new_admin_by_admin() {

    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin,
            invoke: 
                &MockAuthInvoke {
                    contract: &test.factory_contract.address.clone(),
                    fn_name: "set_new_admin",
                    args: (&users[0],).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_new_admin(&users[0]);

    let expected_auth = AuthorizedInvocation {
        function: AuthorizedFunction::Contract((
            test.factory_contract.address.clone(),
            Symbol::new(&test.env, "set_new_admin"),
            (
                users[0].clone(),
            )
                .into_val(&test.env),
        )),
        sub_invocations: vec![], 
    };
    assert_eq!(test.env.auths(), vec![(test.admin, expected_auth)]);

    let new_admin: Address = test.factory_contract.admin();
    assert_eq!(new_admin, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn test_set_new_admin_by_unauthorized() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
    .mock_auths(&[
        MockAuth {
            address: &users[0],
            invoke: 
                &MockAuthInvoke {
                    contract: &test.factory_contract.address.clone(),
                    fn_name: "set_new_admin",
                    args: (&users[0],).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_new_admin(&users[0]);
}

#[test]
fn test_set_defindex_receiver_by_admin() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
    .mock_auths(&[
        MockAuth {
            address: &test.admin,
            invoke: 
                &MockAuthInvoke {
                    contract: &test.factory_contract.address.clone(),
                    fn_name: "set_defindex_receiver",
                    args: (&users[0],).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_defindex_receiver(&users[0]);

    let expected_auth = AuthorizedInvocation {
        function: AuthorizedFunction::Contract((
            test.factory_contract.address.clone(),
            Symbol::new(&test.env, "set_defindex_receiver"),
            (
                users[0].clone(),
            )
                .into_val(&test.env),
        )),
        sub_invocations: vec![], 
    };
    assert_eq!(test.env.auths(), vec![(test.admin, expected_auth)]);

    let new_fee_receiver: Address = test.factory_contract.defindex_receiver();
    assert_eq!(new_fee_receiver, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn test_set_fee_receiver_by_unauthorized() {
    let test = DeFindexFactoryTest::setup();

    test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
    .mock_auths(&[
        MockAuth {
            address: &users[0],
            invoke: 
                &MockAuthInvoke {
                    contract: &test.factory_contract.address.clone(),
                    fn_name: "set_defindex_receiver",
                    args: (&users[0],).into_val(&test.env),
                    sub_invokes: &[],
                },
        }
    ])
    .set_defindex_receiver(&users[0]);
}