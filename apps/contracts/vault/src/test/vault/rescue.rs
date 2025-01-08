use soroban_sdk::{vec as sorobanvec, Address, Map, String, Vec};

use crate::test::{
    create_defindex_vault, create_strategy_params_token_0,
    defindex_vault::{AssetInvestmentAllocation, AssetStrategySet, ContractError, RolesDataKey, StrategyAllocation},
    DeFindexVaultTest,
};

extern crate std;

#[test]
fn rescue_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
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
        true,
    );

    let amount = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    // Balance of the token_0 on the vault should be `amount` since it is deposited into the vault first
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    // Balance of the token_0 on the vault should be 0
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, 0);

    // Balance of the vault on the strategy contract should be `amount`
    let strategy_balance = test
        .strategy_client_token_0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, amount);

    defindex_contract.rescue(
        &strategy_params_token_0.first().unwrap().address,
        &test.emergency_manager,
    );

    // Balance of the vault on the strategy should be 0
    let strategy_balance = test
        .strategy_client_token_0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, 0);

    // Balance of the token_0 on the vault should be `amount`
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);

    // check if strategy is paused
    let asset = defindex_contract.get_assets().first().unwrap();
    assert_eq!(asset.strategies.first().unwrap().paused, true);
}

#[test]
fn rescue_errors() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
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

    let amount = 987654321i128;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);

    // Deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    // rescue with wrong strategy address
    let result = defindex_contract.try_rescue(
        &users[0],
        &test.emergency_manager,
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result, Err(Ok(ContractError::StrategyNotFound)));

    // rescue with wrong caller
    let result = defindex_contract.try_rescue(
        &strategy_params_token_0.first().unwrap().address,
        &users[0],
    );

    assert_eq!(result.is_err(), true);
    assert_eq!(result, Err(Ok(ContractError::Unauthorized)));
}

#[test]
fn pause_then_rescue() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
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

    let amount = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    // Balance of the token_0 on the vault should be `amount` since it is deposited into the vault first
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    // Balance of the token_0 on the vault should be 0
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, 0);

    // Balance of the vault on the strategy contract should be `amount`
    let strategy_balance = test
        .strategy_client_token_0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, amount);

    // pause strategy
    defindex_contract.pause_strategy(&strategy_params_token_0.first().unwrap().address, &test.emergency_manager);

    defindex_contract.rescue(
        &strategy_params_token_0.first().unwrap().address,
        &test.emergency_manager,
    );

    // Balance of the vault on the strategy should be 0
    let strategy_balance = test
        .strategy_client_token_0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, 0);

    // Balance of the token_0 on the vault should be `amount`
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);

    // check if strategy is paused
    let asset = defindex_contract.get_assets().first().unwrap();
    assert_eq!(asset.strategies.first().unwrap().paused, true);
}

#[test]
fn distribute_fees_on_rescue() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
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
    );

    let amount = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    // Balance of the token_0 on the vault should be `amount` since it is deposited into the vault first
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    // Balance of the token_0 on the vault should be 0
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, 0);

    // Balance of the vault on the strategy contract should be `amount`
    let strategy_balance = test
        .strategy_client_token_0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, amount);

    // Simulate earning on the strategy
    test.token_0_admin_client.mint(&defindex_contract.address, &10_0_000_000i128);
    test.strategy_client_token_0.deposit(&10_0_000_000i128, &defindex_contract.address);
    defindex_contract.report();

    defindex_contract.lock_fees(&None);

    let balance_defindex_protocol = test.token_0.balance(&test.defindex_protocol_receiver);
    assert_eq!(balance_defindex_protocol, 0i128);

    let balance_vault_fee_receiver = test.token_0.balance(&test.vault_fee_receiver);
    assert_eq!(balance_vault_fee_receiver, 0i128);

    defindex_contract.rescue(
        &strategy_params_token_0.first().unwrap().address,
        &test.emergency_manager,
    );

    let balance_defindex_protocol = test.token_0.balance(&test.defindex_protocol_receiver);
    assert_eq!(balance_defindex_protocol, 5_000_000i128);

    let balance_vault_fee_receiver = test.token_0.balance(&test.vault_fee_receiver);
    assert_eq!(balance_vault_fee_receiver, 1_5_000_000i128);

    // Balance of the vault on the strategy should be 0
    let strategy_balance = test
        .strategy_client_token_0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, 0);

    // Balance of the token_0 on the vault should be `amount`
    let vault_balance_of_token = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount + 8_0_000_000);

    // check if strategy is paused
    let asset = defindex_contract.get_assets().first().unwrap();
    assert_eq!(asset.strategies.first().unwrap().paused, true);
}