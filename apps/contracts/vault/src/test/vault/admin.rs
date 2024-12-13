use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke},
    vec as sorobanvec, IntoVal, String, Symbol, Vec,
};

use crate::test::{
    create_defindex_vault, create_strategy_params_token0, create_strategy_params_token1, defindex_vault::AssetStrategySet, DeFindexVaultTest
};

extern crate alloc;
use alloc::vec;

#[test]
fn set_new_fee_receiver_by_fee_receiver() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token1.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Fee Receiver is setting the new fee receiver
    defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.vault_fee_receiver,
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_fee_receiver",
                args: (&test.vault_fee_receiver, &users[0]).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_receiver(&test.vault_fee_receiver, &users[0]);

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            defindex_contract.address.clone(),
            Symbol::new(&test.env, "set_fee_receiver"),
            (&test.vault_fee_receiver, users[0].clone()).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(
        test.env.auths(),
        vec![(test.vault_fee_receiver, expected_auth)]
    );

    let new_fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(new_fee_receiver_role, users[0]);
}

#[test]
fn set_new_fee_receiver_by_manager() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);
    // let tokens: Vec<Address> = sorobanvec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
    // let ratios: Vec<u32> = sorobanvec![&test.env, 1, 1];

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token1.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Now Manager is setting the new fee receiver
    defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_fee_receiver",
                args: (&test.manager, &users[0]).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_fee_receiver(&test.manager, &users[0]);

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            defindex_contract.address.clone(),
            Symbol::new(&test.env, "set_fee_receiver"),
            (&test.manager, users[0].clone()).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.manager, expected_auth)]);

    let new_fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(new_fee_receiver_role, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #130)")] // Unauthorized
fn set_new_fee_receiver_by_emergency_manager() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);
    // let tokens: Vec<Address> = sorobanvec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
    // let ratios: Vec<u32> = sorobanvec![&test.env, 1, 1];

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token1.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Now Emergency Manager is setting the new fee receiver
    defindex_contract
        .set_fee_receiver(&test.emergency_manager, &users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #130)")] // Unauthorized
fn set_new_fee_receiver_invalid_sender() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token1.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Trying to set the new fee receiver with an invalid sender
    defindex_contract
        .set_fee_receiver(&users[0], &users[0]);
}

#[test]
fn set_new_manager_by_manager() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token1.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let manager_role = defindex_contract.get_manager();
    assert_eq!(manager_role, test.manager);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Manager is setting the new manager
    defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: (&users[0],).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_manager(&users[0]);

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            defindex_contract.address.clone(),
            Symbol::new(&test.env, "set_manager"),
            (users[0].clone(),).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.manager, expected_auth)]);

    let new_manager_role = defindex_contract.get_manager();
    assert_eq!(new_manager_role, users[0]);
}
