use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke, Events},
    Address, BytesN, IntoVal, Symbol, FromVal, Vec, Val, String, symbol_short,
    vec as sorobanvec,
};
extern crate alloc;
extern crate std;

use alloc::vec;

use crate::test::DeFindexFactoryTest;
use crate::events::NewVaultWasmHashEvent;

#[test]
fn set_new_admin_by_admin() {
    let test = DeFindexFactoryTest::setup();

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.factory_contract.address.clone(),
                fn_name: "set_new_admin",
                args: (&users[0],).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_new_admin(&users[0]);

    let expected_auth = AuthorizedInvocation {
        function: AuthorizedFunction::Contract((
            test.factory_contract.address.clone(),
            Symbol::new(&test.env, "set_new_admin"),
            (users[0].clone(),).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.admin, expected_auth)]);

    let new_admin: Address = test.factory_contract.admin();
    assert_eq!(new_admin, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn set_new_admin_by_unauthorized() {
    let test = DeFindexFactoryTest::setup();

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
        .mock_auths(&[MockAuth {
            address: &users[0],
            invoke: &MockAuthInvoke {
                contract: &test.factory_contract.address.clone(),
                fn_name: "set_new_admin",
                args: (&users[0],).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_new_admin(&users[0]);
}

#[test]
fn set_defindex_receiver_by_admin() {
    let test = DeFindexFactoryTest::setup();

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.factory_contract.address.clone(),
                fn_name: "set_defindex_receiver",
                args: (&users[0],).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_defindex_receiver(&users[0]);

    let expected_auth = AuthorizedInvocation {
        function: AuthorizedFunction::Contract((
            test.factory_contract.address.clone(),
            Symbol::new(&test.env, "set_defindex_receiver"),
            (users[0].clone(),).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.admin, expected_auth)]);

    let new_fee_receiver: Address = test.factory_contract.defindex_receiver();
    assert_eq!(new_fee_receiver, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn set_fee_receiver_by_unauthorized() {
    let test = DeFindexFactoryTest::setup();

    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    test.factory_contract
        .mock_auths(&[MockAuth {
            address: &users[0],
            invoke: &MockAuthInvoke {
                contract: &test.factory_contract.address.clone(),
                fn_name: "set_defindex_receiver",
                args: (&users[0],).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_defindex_receiver(&users[0]);
}

#[test]
fn set_vault_wasm_hash_by_admin() {
    let test = DeFindexFactoryTest::setup();
    let new_wasm_hash = BytesN::from_array(&test.env, &[1; 32]);

    test.factory_contract
        .mock_auths(&[MockAuth {
            address: &test.admin,
            invoke: &MockAuthInvoke {
                contract: &test.factory_contract.address.clone(),
                fn_name: "set_vault_wasm_hash",
                args: (&new_wasm_hash,).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_vault_wasm_hash(&new_wasm_hash);

    let expected_auth = AuthorizedInvocation {
        function: AuthorizedFunction::Contract((
            test.factory_contract.address.clone(),
            Symbol::new(&test.env, "set_vault_wasm_hash"),
            (new_wasm_hash.clone(),).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.admin, expected_auth)]);

    // Verify the event was emitted
    let events = test.env.events().all();
    let wasm_hash_events: std::vec::Vec<(Address, Vec<Val>, Val)> = events
        .iter()
        .filter(|event| {
            event.1 == sorobanvec![
                &test.env,
                String::from_str(&test.env, "DeFindexFactory").into_val(&test.env),
                symbol_short!("n_wasm").into_val(&test.env)
            ]
        })
        .collect();

    assert_eq!(wasm_hash_events.len(), 1);
    let event = wasm_hash_events.last().unwrap();
    let new_wasm_hash_event: NewVaultWasmHashEvent = FromVal::from_val(&test.env, &event.2);
    assert_eq!(new_wasm_hash_event.new_vault_wasm_hash, new_wasm_hash);

    // Verify the storage was updated
    let stored_hash = test.factory_contract.vault_wasm_hash();
    assert_eq!(stored_hash, new_wasm_hash);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn set_vault_wasm_hash_by_unauthorized() {
    let test = DeFindexFactoryTest::setup();
    let new_wasm_hash = BytesN::from_array(&test.env, &[1; 32]);
    let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);

    test.factory_contract
        .mock_auths(&[MockAuth {
            address: &users[0],
            invoke: &MockAuthInvoke {
                contract: &test.factory_contract.address.clone(),
                fn_name: "set_vault_wasm_hash",
                args: (&new_wasm_hash,).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_vault_wasm_hash(&new_wasm_hash);
}
