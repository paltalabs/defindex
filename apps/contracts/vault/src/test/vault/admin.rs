use soroban_sdk::{
    symbol_short, FromVal,
    testutils::{AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke, Events},
    vec as sorobanvec, Address, IntoVal, Map, String, Symbol, Vec, Val
};

use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{AssetStrategySet, ContractError, RolesDataKey, ManagerChangedEvent}, DeFindexVaultTest, EnvTestUtils,
};

extern crate std;
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
    test.env.mock_all_auths();
    let deposit_amount = 10_0_000_000i128;
    // Mint before deposit
    test.token_0_admin_client.mint(&users[0], &deposit_amount);
    test.token_1_admin_client.mint(&users[0], &deposit_amount);


// Deposit
defindex_contract.deposit(
    &sorobanvec![&test.env, deposit_amount, deposit_amount],
    &sorobanvec![&test.env, deposit_amount, deposit_amount],
    &users[0],
    &true,
);
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

    let new_manager_role = defindex_contract.get_manager();
    assert_eq!(new_manager_role, users[0]);

    // Verify the event was emitted correctly
    let events = test.env.events().all();
    std::println!("events: {:?}", events);
    // Get the last manager change event
    // let manager_changed_event: ManagerChangedEvent = FromVal::from_val(&test.env, &events.2);

    // Verify the event data
    // assert_eq!(manager_changed_event.new_manager, users[0]);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn set_new_manager_by_unauthorized_user() {
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    defindex_contract.set_manager(&users[0]);

    // Try set_manager from unauthorized user
    defindex_contract
        .mock_auths(&[MockAuth {
            address: &users[0],
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "set_manager",
                args: (&users[0],).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .set_manager(&users[0]);
    
}

#[test]
fn lock_fees_with_new_fee() {
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

    // Try to set an excessive fee (should fail)
    let result = defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager,
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "lock_fees",
                args: (&Some(9500u32),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .try_lock_fees(&Some(9500u32));
    
    assert_eq!(result, Err(Ok(ContractError::MaximumFeeExceeded)));

    // Set a valid fee (should succeed)
    defindex_contract
        .mock_auths(&[MockAuth {
            address: &test.manager,
            invoke: &MockAuthInvoke {
                contract: &defindex_contract.address.clone(),
                fn_name: "lock_fees",
                args: (&Some(2000u32),).into_val(&test.env),
                sub_invokes: &[],
            },
        }])
        .lock_fees(&Some(2000u32));

    let expected_auth = AuthorizedInvocation {
        function: AuthorizedFunction::Contract((
            defindex_contract.address.clone(),
            Symbol::new(&test.env, "lock_fees"),
            (&Some(2000u32),).into_val(&test.env),
        )),
        sub_invocations: vec![],
    };
    assert_eq!(test.env.auths(), vec![(test.manager, expected_auth)]);

    // Verify the new fee was set
    let (vault_fee, _defindex_fee) = defindex_contract.get_fees();
    assert_eq!(vault_fee, 2000u32);
}