use soroban_sdk::{vec as sorobanvec, String, Vec};

use crate::test::{
    create_defindex_vault, create_hodl_strategy, create_strategy_params_token0, create_strategy_params_token1, defindex_vault::{AssetStrategySet, Strategy}, DeFindexVaultTest
};


#[test]
fn get_roles() {
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
    let fee_receiver_role = defindex_contract.get_fee_receiver();
    let emergency_manager_role = defindex_contract.get_emergency_manager();

    assert_eq!(manager_role, test.manager);
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);
    assert_eq!(emergency_manager_role, test.emergency_manager);
}


// Test that if strategy does support other asset we get an error when initializing
#[test]
#[should_panic(expected = "HostError: Error(Context, InvalidAction)")]
fn deploy_unsupported_strategy() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token0 = create_strategy_params_token0(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token0.clone() // Here Strategy 0 supports token0
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
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
}

// test that if we try to initialize with an empty asset allocation fails
#[test]
#[should_panic(expected = "HostError: Error(Context, InvalidAction)")]
fn initialize_with_empty_asset_allocation() {
    let test = DeFindexVaultTest::setup();
    // let strategy_params_token0 = create_strategy_params_token0(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![&test.env];

    create_defindex_vault(
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
}

// test initialzie with one asset and several strategies for the same asset
#[test]
fn with_one_asset_and_several_strategies() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_client_1 = create_hodl_strategy(&test.env, &test.token0.address.clone());
    let strategy_client_2 = create_hodl_strategy(&test.env, &test.token0.address.clone());
    let strategy_client_3 = create_hodl_strategy(&test.env, &test.token0.address.clone());
    let strategy_client_4 = create_hodl_strategy(&test.env, &test.token0.address.clone());
    
    let strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "strategy1"),
            address: test.strategy_client_token0.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy2"),
            address: strategy_client_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy3"),
            address: strategy_client_2.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy4"),
            address: strategy_client_3.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy4"),
            address: strategy_client_4.address.clone(),
            paused: false,
        },
    ];

    // initialize with 1 asset, 3 strategies
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params.clone(),
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
    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 1);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.strategies.len(), strategy_params.len());
}