use soroban_sdk::{vec as sorobanvec, String, Vec};

use crate::test::{
    create_defindex_vault, create_strategy_params_token0, create_strategy_params_token1, defindex_vault::AssetStrategySet, DeFindexVaultTest
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
        2500u32,
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
        2500u32,
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
        2500u32,
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
}

// test initialzie with one asset and several strategies for the same asset
#[test]
fn with_one_asset_and_several_strategies() {
    todo!();
}