use soroban_sdk::{
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke}, vec as sorobanvec, Address, IntoVal, Map, String, Symbol, Vec
};

use crate::{constants::ONE_DAY_IN_SECONDS, test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{AssetStrategySet, ContractError, RolesDataKey}, DeFindexVaultTest, EnvTestUtils,
}};

extern crate alloc;
use alloc::vec;

#[test]
fn set_new_fee_receiver_by_fee_receiver() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        name_symbol,
        true
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
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);
    // let tokens: Vec<Address> = sorobanvec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()];
    // let ratios: Vec<u32> = sorobanvec![&test.env, 1, 1];

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        name_symbol,
        true
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
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);
    // let tokens: Vec<Address> = sorobanvec![&test.env, test.token_0.address.clone(), test.token_1.address.clone()];
    // let ratios: Vec<u32> = sorobanvec![&test.env, 1, 1];

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Now Emergency Manager is setting the new fee receiver
    defindex_contract.set_fee_receiver(&test.emergency_manager, &users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #130)")] // Unauthorized
fn set_new_fee_receiver_invalid_sender() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Trying to set the new fee receiver with an invalid sender
    defindex_contract.set_fee_receiver(&users[0], &users[0]);
}

#[test]
fn set_new_manager_by_manager() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
    let manager_role = defindex_contract.get_manager();
    assert_eq!(manager_role, test.manager);

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);
    // Manager is setting the new manager
    let response = defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_set_manager();

    assert_eq!(response, Err(Ok(ContractError::QueueEmpty)));
    
    let result = defindex_contract.mock_auths(
        &[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "queue_manager",
                args: (&users[0].clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }]
    ).try_queue_manager(&users[0]);
    assert_eq!(result, Ok(Ok(users[0].clone())));

    let queued_manager = defindex_contract.get_queued_manager();
    assert_eq!(queued_manager, users[0]);

    let response = defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_set_manager();

    assert_eq!(response, Err(Ok(ContractError::SetManagerBeforeTime)));

    test.env.jump_time(ONE_DAY_IN_SECONDS * 3);

    let response = defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_set_manager();
    assert_eq!(response, Err(Ok(ContractError::SetManagerBeforeTime)));

    test.env.jump_time(ONE_DAY_IN_SECONDS * 3);

    let response = defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_set_manager();
    assert_eq!(response, Err(Ok(ContractError::SetManagerBeforeTime)));

    test.env.jump_time(ONE_DAY_IN_SECONDS * 1);

    defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_manager();

    let expected_auth = AuthorizedInvocation {
        // Top-level authorized function is `deploy` with all the arguments.
        function: AuthorizedFunction::Contract((
            defindex_contract.address.clone(),
            Symbol::new(&test.env, "set_manager"),
            ().into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.manager, expected_auth)]);

    let new_manager_role = defindex_contract.get_manager();
    assert_eq!(new_manager_role, users[0]);
}

#[test]
fn queue_manager(){
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // Manager is queuing the new manager

    // Check initial empty queue
    let queued_manager = defindex_contract.try_get_queued_manager();
    assert_eq!(queued_manager, Err(Ok(ContractError::QueueEmpty)));

    // Try unauthorized queue (should fail)
    let unauthorized_queue = defindex_contract.try_queue_manager(&users[0]);
    assert_eq!(unauthorized_queue.is_err(), true);


    // Add MockAuth for queue_manager
    let mut manager_data: Map<u64, Address> = Map::new(&test.env);
    let current_timestamp = test.env.ledger().timestamp();
    manager_data.set(current_timestamp, users[0].clone());

    defindex_contract.mock_auths(
        &[MockAuth {
            address: &test.manager,
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address,
                fn_name: "queue_manager",
                args: (users[0].clone(),).into_val(&test.env),
                sub_invokes: &[],
            },
        }]
    ).queue_manager(&users[0]);

    // Add MockAuth for get_queued_manager
    let queued_manager = defindex_contract.mock_auths(
        &[MockAuth {
            address: &test.manager,
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address,
                fn_name: "get_queued_manager",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }]
    ).get_queued_manager();
    assert_eq!(queued_manager, users[0]);
    

    // Verify queue is empty
    defindex_contract.mock_auths(
        &[MockAuth {
            address: &test.manager,
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address,
                fn_name: "clear_queue",
                args: ().into_val(&test.env),
                sub_invokes: &[],
            },
        }]
    ).clear_queue();
    let queued_manager = defindex_contract.try_get_queued_manager();
    assert_eq!(queued_manager, Err(Ok(ContractError::QueueEmpty))) 
}