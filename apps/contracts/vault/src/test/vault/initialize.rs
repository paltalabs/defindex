use soroban_sdk::{vec as sorobanvec, String, Vec, Map, Address};

use crate::test::{
    create_defindex_vault, create_hodl_strategy, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{{AssetStrategySet, Strategy}, CurrentAssetInvestmentAllocation, StrategyAllocation}, DeFindexVaultTest,
};

fn create_expected_current_invested_funds(test: &DeFindexVaultTest) -> Map<Address, i128> {
    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);
    expected_current_invested_funds.set(test.token_1.address.clone(), 0i128);
    expected_current_invested_funds
}

fn create_expected_current_idle_funds(test: &DeFindexVaultTest) -> Map<Address, i128> {
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

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        sorobanvec![
            &test.env,
            String::from_str(&test.env, "dfToken"),
            String::from_str(&test.env, "DFT")
        ],
    );

    let manager_role = defindex_contract.get_manager();
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    let emergency_manager_role = defindex_contract.get_emergency_manager();

    let vault_assets = defindex_contract.get_assets();
    let asset_0 = vault_assets.get(0).unwrap();
    let asset_1 = vault_assets.get(1).unwrap();
    
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();

    let mut expected_total_managed_funds: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(&test.env);
    
    // Add entry for token_0
    expected_total_managed_funds.set(
        test.token_0.address.clone(),
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
                },
            ],
        },
    );

    // Add entry for token_1
    expected_total_managed_funds.set(
        test.token_1.address.clone(),
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
                },
            ],
        },
    );

    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);
    expected_current_invested_funds.set(test.token_1.address.clone(), 0i128);

    let mut expected_current_idle_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_idle_funds.set(test.token_0.address.clone(), 0i128);
    expected_current_idle_funds.set(test.token_1.address.clone(), 0i128);

    assert_eq!(asset_0.address, test.token_0.address);
    assert_eq!(asset_1.address, test.token_1.address);
    assert_eq!(vault_assets.len(), 2);

    assert_eq!(total_managed_funds, expected_total_managed_funds);
    assert_eq!(current_invested_funds, expected_current_invested_funds);
    assert_eq!(current_idle_funds, expected_current_idle_funds);

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

    create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        sorobanvec![
            &test.env,
            String::from_str(&test.env, "dfToken"),
            String::from_str(&test.env, "DFT")
        ],
    );
}

// test that if we try to initialize with an empty asset allocation fails
#[test]
#[should_panic(expected = "HostError: Error(Context, InvalidAction)")]
fn initialize_with_empty_asset_allocation() {
    let test = DeFindexVaultTest::setup();
    // let strategy_params_token_0 = create_strategy_params_token_0(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![&test.env];

    create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        sorobanvec![
            &test.env,
            String::from_str(&test.env, "dfToken"),
            String::from_str(&test.env, "DFT")
        ],
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

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        sorobanvec![
            &test.env,
            String::from_str(&test.env, "dfToken"),
            String::from_str(&test.env, "DFT")
        ],
    );

    let manager_role = defindex_contract.get_manager();
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    let emergency_manager_role = defindex_contract.get_emergency_manager();
    
    let vault_assets = defindex_contract.get_assets();
    let asset = vault_assets.get(0).unwrap();
    let vault_strategies = asset.strategies;
    
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();

    let mut expected_total_managed_funds: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(&test.env);
    expected_total_managed_funds.set(
        test.token_0.address.clone(),
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
                },
                StrategyAllocation {
                    strategy_address: strategy_1.address.clone(),
                    amount: 0i128,
                },
                StrategyAllocation {
                    strategy_address: strategy_2.address.clone(),
                    amount: 0i128,
                },
            ],
        },
    );

    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);

    let mut expected_current_idle_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_idle_funds.set(test.token_0.address.clone(), 0i128);


    assert_eq!(manager_role, test.manager);
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);
    assert_eq!(emergency_manager_role, test.emergency_manager);

    assert_eq!(asset.address, test.token_0.address);
    assert_eq!(vault_assets.len(), 1);
    assert_eq!(vault_strategies.len(), strategy_params.len());

    assert_eq!(total_managed_funds, expected_total_managed_funds);
    assert_eq!(current_invested_funds, expected_current_invested_funds);
    assert_eq!(current_idle_funds, expected_current_idle_funds);
 
}