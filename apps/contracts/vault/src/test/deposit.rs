use soroban_sdk::{vec as sorobanvec, String, Vec, Map};

use crate::test::defindex_vault::{AssetAllocation, ContractError};
use crate::test::{
    create_strategy_params_token0, create_strategy_params_token1, DeFindexVaultTest,
};

// Test deposit not yet initialized
#[test]
fn test_deposit_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, 100i128],
        &sorobanvec![&test.env, 100i128],
        &users[0],
    );

    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}


#[test]
fn deposit_amounts_desired_less_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetAllocation {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount], // wrong amount desired
        &sorobanvec![&test.env, amount, amount],
        &users[0],
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test deposit amount desired more length
#[test]
fn deposit_amounts_desired_more_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount], // wrong amount desired
        &sorobanvec![&test.env, amount],
        &users[0],
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test deposit amount min less length
#[test]
fn deposit_amounts_min_less_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetAllocation {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount],
        &sorobanvec![&test.env, amount], // wrong amount min
        &users[0],
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}


// test deposit amount min more length
#[test]
fn deposit_amounts_min_more_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetAllocation {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount],
        &sorobanvec![&test.env, amount, amount, amount], // wrong amount min
        &users[0],
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test amount desired negative
#[test]
fn deposit_amounts_desired_negative() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetAllocation {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, -amount, amount],
        &sorobanvec![&test.env, amount, amount],
        &users[0],
    );

    assert_eq!(response, Err(Ok(ContractError::NegativeNotAllowed)));
}

// test deposit one asset success 
#[test]
fn deposit_one_asset_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);

    // initialize with 1 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
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
    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Balances before deposit
    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
    );

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount);
    
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), amount);

    // check that all the assets are in the vault
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount);

    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, expected_map);

    // check current idle funds, 
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_map);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_map);

    // Now user deposits for the second time
    let amount2 = 987654321i128;
    test.token0_admin_client.mint(&users[0], &amount2);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount2);

    // deposit
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount2],
        &sorobanvec![&test.env, amount2],
        &users[0],
    );

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), amount + amount2);

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount + amount2);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);
    
    // check that all the assets are in the vault
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount + amount2);
    
    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, expected_map);
    
    // check current idle funds
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_map);
    

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), 0i128);
    
    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_map);



}

// test deposit one asset with minimum more than desired
#[test]
fn deposit_one_asset_min_more_than_desired() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);

    // initialize with 1 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
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
    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Balances before deposit
    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let result=test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount + 1],
        &users[0],
    );
    // this should fail
    assert_eq!(result, Err(Ok(ContractError::AmountLessThanMinimum)));
    
}


#[test]
fn deposit_several_assets_success() {
    todo!();
}
