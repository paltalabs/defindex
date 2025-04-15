use soroban_sdk::{vec as sorobanvec, Address, Map, String, Vec};

use crate::test::{
    create_defindex_vault, create_hodl_strategy, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{AssetStrategySet, CurrentAssetInvestmentAllocation, RolesDataKey, Strategy, StrategyAllocation}, DeFindexVaultTest,
};
fn _create_expected_current_invested_funds(test: &DeFindexVaultTest) -> Map<Address, i128> {
    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);
    expected_current_invested_funds.set(test.token_1.address.clone(), 0i128);
    expected_current_invested_funds
}

fn _create_expected_current_idle_funds(test: &DeFindexVaultTest) -> Map<Address, i128> {
    let mut expected_current_idle_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_idle_funds.set(test.token_0.address.clone(), 0i128);
    expected_current_idle_funds.set(test.token_1.address.clone(), 0i128);
    expected_current_idle_funds
}

#[test]
fn get_roles() {
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
        },
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
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );

    let manager_role = defindex_contract.get_manager();
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    let emergency_manager_role = defindex_contract.get_emergency_manager();

    let vault_assets = defindex_contract.get_assets();
    let asset_0 = vault_assets.get(0).unwrap();
    let asset_1 = vault_assets.get(1).unwrap();
    
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    let current_invested_funds_0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let current_invested_funds_1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;

    let mut expected_total_managed_funds: Vec<CurrentAssetInvestmentAllocation> = Vec::new(&test.env);
    
    // Add entry for token_0
    expected_total_managed_funds.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 0i128,
            idle_amount: 0i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: 0i128,
                    paused: false,
                },
            ],
        },
    );

    // Add entry for token_1
    expected_total_managed_funds.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: 0i128,
            idle_amount: 0i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: test.strategy_client_token_1.address.clone(),
                    amount: 0i128,
                    paused: false,
                },
            ],
        },
    );

    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);
    expected_current_invested_funds.set(test.token_1.address.clone(), 0i128);

    assert_eq!(asset_0.address, test.token_0.address);
    assert_eq!(asset_1.address, test.token_1.address);
    assert_eq!(vault_assets.len(), 2);

    assert_eq!(total_managed_funds, expected_total_managed_funds);
    assert_eq!(current_invested_funds_0, expected_current_invested_funds.get(test.token_0.address.clone()).unwrap());
    assert_eq!(current_invested_funds_1, expected_current_invested_funds.get(test.token_1.address.clone()).unwrap());

    //Checking idle balance
    let current_idle_funds_token_0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_0, 0i128);
    let current_idle_funds_token_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_1, 0i128);

    assert_eq!(manager_role, test.manager);
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);
    assert_eq!(emergency_manager_role, test.emergency_manager);
}

// Test that if strategy does support other asset we get an error when initializing
#[test]
#[should_panic(expected = "HostError: Error(Context, InvalidAction)")]
fn deploy_unsupported_strategy() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_0.clone() // Here Strategy 0 supports token_0
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

    create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
}

// test that if we try to initialize with an empty asset allocation fails
#[test]
#[should_panic(expected = "HostError: Error(Context, InvalidAction)")]
fn initialize_with_empty_asset_allocation() {
    let test = DeFindexVaultTest::setup();
    // let strategy_params_token_0 = create_strategy_params_token_0(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![&test.env];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
}

// test initialzie with one asset and several strategies for the same asset
#[test]
fn with_one_asset_and_several_strategies() {
    let test = DeFindexVaultTest::setup();

    let strategy_0 = create_hodl_strategy(&test.env, &test.token_0.address.clone());
    let strategy_1 = create_hodl_strategy(&test.env, &test.token_0.address.clone());
    let strategy_2 = create_hodl_strategy(&test.env, &test.token_0.address.clone());

    let strategy_params = sorobanvec![
        &test.env,
        Strategy {
            name: String::from_str(&test.env, "Strategy 1"),
            address: strategy_0.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "Strategy 2"),
            address: strategy_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "Strategy 3"),
            address: strategy_2.address.clone(),
            paused: false,
        },
    ];

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params.clone()
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
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );

    let manager_role = defindex_contract.get_manager();
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    let emergency_manager_role = defindex_contract.get_emergency_manager();
    
    let vault_assets = defindex_contract.get_assets();
    let asset = vault_assets.get(0).unwrap();
    let vault_strategies = asset.strategies;
    
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;

    let mut expected_total_managed_funds: Vec<CurrentAssetInvestmentAllocation> = Vec::new(&test.env);
    expected_total_managed_funds.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 0i128,
            idle_amount: 0i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: strategy_0.address.clone(),
                    amount: 0i128,
                    paused: false,
                },
                StrategyAllocation {
                    strategy_address: strategy_1.address.clone(),
                    amount: 0i128,
                    paused: false,
                },
                StrategyAllocation {
                    strategy_address: strategy_2.address.clone(),
                    amount: 0i128,
                    paused: false,
                },
            ],
        },
    );

    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);

    let current_idle_funds = test.token_0.balance(&defindex_contract.address);

    assert_eq!(manager_role, test.manager);
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);
    assert_eq!(emergency_manager_role, test.emergency_manager);

    assert_eq!(asset.address, test.token_0.address);
    assert_eq!(vault_assets.len(), 1);
    assert_eq!(vault_strategies.len(), strategy_params.len());

    assert_eq!(total_managed_funds, expected_total_managed_funds);
    assert_eq!(current_invested_funds, expected_current_invested_funds.get(test.token_0.address.clone()).unwrap());
    assert_eq!(current_idle_funds, 0i128);
 
}

#[test]
fn with_one_asset_no_strategies(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params: Vec<Strategy> = sorobanvec![&test.env];
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params.clone()
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
        1u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
    
    let vault_assets = defindex_contract.get_assets();
    let asset = vault_assets.get(0).unwrap();
    let vault_strategies = asset.strategies;

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);

    let mut expected_total_managed_funds: Vec<CurrentAssetInvestmentAllocation> = Vec::new(&test.env);
    expected_total_managed_funds.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 0i128,
            idle_amount: 0i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![&test.env]
        },
    );
    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);

    assert_eq!(vault_assets.len(), 1);
    assert_eq!(vault_strategies.len(), strategy_params.len());

    assert_eq!(total_managed_funds, expected_total_managed_funds);
    assert_eq!(current_invested_funds, expected_current_invested_funds.get(test.token_0.address.clone()).unwrap());
    assert_eq!(current_idle_funds, 0i128);
    
    //Deposit
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    let amount0 = 5_000_000i128;
    test.token_0_admin_client.mint(&users[0], &amount0);
    let _deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;

    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);

    assert_eq!(current_idle_funds, amount0);
    assert_eq!(current_invested_funds, expected_current_invested_funds.get(test.token_0.address.clone()).unwrap());

    let vault_shares = defindex_contract.balance(&users[0]);
    let withdraw_amount = defindex_contract.try_get_asset_amounts_per_shares(&vault_shares).unwrap().unwrap().get(0).unwrap();

    let min_amounts_out = sorobanvec![&test.env, withdraw_amount];
    
    let _withdraw_result = defindex_contract.withdraw(
        &withdraw_amount,
        &min_amounts_out,
        &users[0].clone(),
    ); 

    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, 1000i128);
}

// Test that if we try to initialize with a fee greater than 9000 basis points it fails
#[test]
#[should_panic(expected = "Error(Contract, #106)")]
fn initialize_with_excessive_vault_fee() {
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

    create_defindex_vault(
        &test.env,
        assets,
        roles,
        9500u32, // Fee greater than 9000 basis points (90%)
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #106)")]
fn initialize_with_excessive_protocol_fee() {
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

    create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        9500u32, // Protocol fee greater than 9000 basis points (90%)
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #107)")]
fn initialize_duplicated_asset_address(){
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
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_1.clone()
        },
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
}
#[test]
#[should_panic(expected = "Error(Contract, #108)")]
fn initialize_duplicated_strategy_address(){
    let test = DeFindexVaultTest::setup();
    let strategy_0 = Strategy {
        name: String::from_str(&test.env, "Strategy 0"),
        address: test.strategy_client_token_0.address.clone(),
        paused: false,
    };

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: sorobanvec![&test.env, strategy_0.clone(), strategy_0.clone()]
        },
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    create_defindex_vault(
        &test.env,
        assets,
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );
}