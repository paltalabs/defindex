use soroban_sdk::{vec as sorobanvec, String, Vec};

use crate::test::{
    create_strategy_params_token0, create_strategy_params_token1,
    defindex_vault::{AssetStrategySet, ContractError},
    DeFindexVaultTest,
};


#[test]
fn test_initialize_and_get_roles() {
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

    test.defindex_contract.initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.vault_fee_receiver,
        &2000u32,
        &test.defindex_protocol_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );

    let manager_role = test.defindex_contract.get_manager();
    let fee_receiver_role = test.defindex_contract.get_fee_receiver();
    let emergency_manager_role = test.defindex_contract.get_emergency_manager();

    assert_eq!(manager_role, test.manager);
    assert_eq!(fee_receiver_role, test.vault_fee_receiver);
    assert_eq!(emergency_manager_role, test.emergency_manager);
}


// Test that if strategy does support other asset we get an error when initializing
#[test]
fn test_initialize_with_unsupported_strategy() {
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

    let result = test.defindex_contract.try_initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.vault_fee_receiver,
        &2000u32,
        &test.defindex_protocol_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );

    assert_eq!(
        result,
        Err(Ok(ContractError::StrategyDoesNotSupportAsset))
    );
}

// test that if we try to initialize with an empty asset allocation fails
#[test]
fn test_initialize_with_empty_asset_allocation() {
    let test = DeFindexVaultTest::setup();
    // let strategy_params_token0 = create_strategy_params_token0(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![&test.env];

    let result = test.defindex_contract.try_initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.vault_fee_receiver,
        &2000u32,
        &test.defindex_protocol_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );

    assert_eq!(result, Err(Ok(ContractError::NoAssetAllocation)));
}

#[test]
fn test_get_roles_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let manager_role = test.defindex_contract.try_get_manager();
    let fee_receiver_role = test.defindex_contract.try_get_manager();
    let emergency_manager_role = test.defindex_contract.try_get_manager();

    assert_eq!(manager_role, Err(Ok(ContractError::RoleNotFound)));
    assert_eq!(fee_receiver_role, Err(Ok(ContractError::RoleNotFound)));
    assert_eq!(emergency_manager_role, Err(Ok(ContractError::RoleNotFound)));
}

#[test]
fn test_initialize_twice() {
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

    test.defindex_contract.initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.vault_fee_receiver,
        &2000u32,
        &test.defindex_protocol_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );

    let result_second_init = test.defindex_contract.try_initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.vault_fee_receiver,
        &2000u32,
        &test.defindex_protocol_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );

    assert_eq!(
        result_second_init,
        Err(Ok(ContractError::AlreadyInitialized))
    );
}


#[test]
fn test_emergency_withdraw_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let strategy_params_token1 = create_strategy_params_token1(&test);

    let result = test
        .defindex_contract
        .try_emergency_withdraw(&strategy_params_token1.first().unwrap().address, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}
