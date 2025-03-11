use soroban_sdk::{vec as sorobanvec, Address, InvokeError, Map, String, Vec};

use crate::test::defindex_vault::{
    AssetStrategySet, ContractError, CurrentAssetInvestmentAllocation, RolesDataKey, StrategyAllocation
};
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1, create_strategy_params,
    DeFindexVaultTest,
};
use crate::MINIMUM_LIQUIDITY;
extern crate std;

#[test]
fn amounts_desired_less_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount], // wrong amount desired
        &sorobanvec![&test.env, amount, amount],
        &users[0],
        &false,
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test deposit amount desired more length
#[test]
fn amounts_desired_more_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    // let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount], // wrong amount desired
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test deposit amount min less length
#[test]
fn amounts_min_less_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount],
        &sorobanvec![&test.env, amount], // wrong amount min
        &users[0],
        &false,
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test deposit amount min more length
#[test]
fn amounts_min_more_length() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount],
        &sorobanvec![&test.env, amount, amount, amount], // wrong amount min
        &users[0],
        &false,
    );

    assert_eq!(response, Err(Ok(ContractError::WrongAmountsLength)));
}

// test amount desired negative
#[test]
fn amounts_desired_negative() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = defindex_contract.try_deposit(
        &sorobanvec![&test.env, -amount, amount],
        &sorobanvec![&test.env, amount, amount],
        &users[0],
        &false,
    );

    assert_eq!(response, Err(Ok(ContractError::AmountNotAllowed)));
}

// test deposit one asset success
#[test]
fn one_asset_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // initialize with 1 assets
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
    
    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    // check that all the assets are in the vault
    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount);

    // check total manage funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, //funds has not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount,
            idle_amount: amount,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        },
    );

    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds,
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, amount);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, expected_map.get(test.token_0.address.clone()).unwrap());

    // Now user deposits for the second time
    let amount2 = 987654321i128;
    test.token_0_admin_client.mint(&users[0], &amount2);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount2);

    // deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount2],
        &sorobanvec![&test.env, amount2],
        &users[0],
        &false,
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount + amount2 - 1000);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    // check that all the assets are in the vault
    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount + amount2);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount + amount2,
            idle_amount: amount + amount2,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap();
    assert_eq!(total_managed_funds, total_managed_funds_expected.get(0).unwrap());

    // check current idle funds
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, amount + amount2);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, expected_map.get(test.token_0.address.clone()).unwrap());
}

// test deposit one asset with minimum more than desired
#[test]
fn one_asset_min_more_than_desired() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // initialize with 1 assets
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
    
    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount + 1],
        &users[0],
        &false,
    );
    // this should fail
    assert_eq!(result, Err(Ok(ContractError::InsufficientAmount)));
}

// test deposit of several asset, considering different proportion of assets
#[test]
fn several_assets_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // check deposit result
    assert_eq!(
        deposit_result,
        (sorobanvec![&test.env, amount0, amount1], amount0 + amount1, None)
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    // For first deposit, a minimum amount LIQUIDITY OF 1000 is being locked in the contract
    assert_eq!(df_balance, amount0 + amount1 - 1000);

    // check that the vault holds 1000 shares
    let vault_df_shares = defindex_contract.balance(&defindex_contract.address);
    assert_eq!(vault_df_shares, 1000i128);

    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, 0i128);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, 0i128);

    // check vault balance of asset 0
    let vault_balance0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance0, amount0);
    // check vault balance of asset 1
    let vault_balance1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, amount1);

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1,
            idle_amount: amount1,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    //Checking idle balance
    let current_idle_funds_token_0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_0, amount0);
    let current_idle_funds_token_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_1, amount1);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 0i128);
    expected_map.set(test.token_1.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds_0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let current_invested_funds_1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    assert_eq!(current_invested_funds_0, expected_map.get(test.token_0.address.clone()).unwrap());
    assert_eq!(current_invested_funds_1, expected_map.get(test.token_1.address.clone()).unwrap());

    // new user wants to do a deposit with more assets 0 than the proportion, but with minium amount 0
    // multiply amount0 by 2
    let amount0_new = amount0 * 2 + 100;
    let amount1_new = amount1 * 2;

    // mint this to user 1
    test.token_0_admin_client.mint(&users[1], &amount0_new);
    test.token_1_admin_client.mint(&users[1], &amount1_new);

    // check user balances
    let user_balance0 = test.token_0.balance(&users[1]);
    assert_eq!(user_balance0, amount0_new);
    let user_balance1 = test.token_1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new);

    // user 1 deposits
    let deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
        &false,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128

    assert_eq!(
        deposit_result,
        (
            sorobanvec![&test.env, amount0 * 2, amount1 * 2],
            amount0 * 2 + amount1 * 2,
            None
        )
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[1]);
    assert_eq!(df_balance, 2 * (amount0 + amount1));

    let user_balance0 = test.token_0.balance(&users[1]);
    assert_eq!(user_balance0, amount0_new - 2 * amount0);

    let user_balance1 = test.token_1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new - 2 * amount1);

    // check vault balance of asset 0
    let vault_balance0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance0, 3 * amount0);
    // check vault balance of asset 1
    let vault_balance1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, 3 * amount1);

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 3 * amount0,
            idle_amount: 3 * amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: 3 * amount1,
            idle_amount: 3 * amount1,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    //Checking idle balance
    let current_idle_funds_token_0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_0, 3 * amount0);
    let current_idle_funds_token_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_1, 3 * amount1);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 0i128);
    expected_map.set(test.token_1.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds_0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let current_invested_funds_1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    assert_eq!(current_invested_funds_0, expected_map.get(test.token_0.address.clone()).unwrap());
    assert_eq!(current_invested_funds_1, expected_map.get(test.token_1.address.clone()).unwrap());

    // we will repeat one more time, now enforcing the first asset
    let amount0_new = amount0 * 2;
    let amount1_new = amount1 * 2 + 100;

    // mint this to user 1
    test.token_0_admin_client.mint(&users[1], &amount0_new);
    test.token_1_admin_client.mint(&users[1], &amount1_new);

    // check user balances
    let user_balance0 = test.token_0.balance(&users[1]);
    assert_eq!(user_balance0, 100 + amount0_new); // we still have 100 from before
    let user_balance1 = test.token_1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new);

    // user 1 deposits
    let deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
        &false,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128
    assert_eq!(
        deposit_result,
        (
            sorobanvec![&test.env, amount0 * 2, amount1 * 2],
            amount0 * 2 + amount1 * 2,
            None
        )
    );
}

// test deposit of several asset, imposing a minimum amount greater than optimal for asset 0
#[test]
fn several_assets_min_greater_than_optimal() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0 + 1, amount1],
        &users[0],
        &false,
    );

    // this should fail
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

    // now we manage to deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // check deposit result

    // and now will try again with minimum more than optimal

    // new user wants to do a deposit with more assets 0 than the proportion, but with minium amount 0
    // multiply amount0 by 2
    let amount0_new = amount0 * 2 + 100;
    let amount1_new = amount1 * 2;

    // mint this to user
    test.token_0_admin_client.mint(&users[0], &amount0_new);
    test.token_1_admin_client.mint(&users[0], &amount1_new);

    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, amount0 * 2 + 1, amount1 * 2],
        &users[0],
        &false,
    );

    // this should fail

    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));
}

//test deposit amounts_min greater than amounts_desired
#[test]
fn amounts_min_greater_than_amounts_desired() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0 + 1, amount1 + 1],
        &users[0],
        &false,
    );

    // this should fail
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));
}

//Test token transfer from user to vault on deposit
#[test]
fn transfers_tokens_from_user_to_vault() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
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
    
    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount0 + amount1 - 1000);

    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, 0i128);
}

#[test]
fn arithmetic_error() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // Initialize with 1 asset
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
    

    // Mock the environment to provoke a division by zero
    let mut mock_map = Map::new(&test.env);
    mock_map.set(test.token_0.address.clone(), 0i128); // Total funds for token_0 is 0

    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Mint tokens to user
    test.token_0_admin_client.mint(&users[0], &amount);

    let large_amount = i128::MAX / 2;
    test.token_0_admin_client.mint(&users[0], &large_amount);

    //first deposit to overflow the balance
    defindex_contract.deposit(
        &sorobanvec![&test.env, large_amount],
        &sorobanvec![&test.env, large_amount],
        &users[0],
        &false,
    );

    // Try to deposit a large amount
    let result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, large_amount],
        &sorobanvec![&test.env, large_amount],
        &users[0],
        &false,
    );

    // Verify that the returned error is ContractError::ArithmeticError
    assert_eq!(result, Err(Ok(ContractError::ArithmeticError)));
}

//all amounts are cero
#[test]
fn amounts_desired_zero() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // Initialize with 1 asset
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
    

    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Mint tokens to user
    test.token_0_admin_client.mint(&users[0], &amount);

    // Balances before deposit
    let user_balance_before = test.token_0.balance(&users[0]);
    assert_eq!(user_balance_before, amount);

    let vault_balance_before = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_before, 0i128);

    let df_balance_before = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance_before, 0i128);

    // Attempt to deposit with amounts_desired all set to 0
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0i128],
        &sorobanvec![&test.env, 0i128],
        &users[0],
        &false,
    );

    // Verify that the returned error is ContractError::InsufficientAmount
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));
}

// Deposit with insufficient funds and check for specific error message
#[test]
fn insufficient_funds_with_error_message() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // Initialize with 2 assets
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
    

    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Mint tokens to user
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);

    // Balances before deposit
    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Attempt to deposit more than available balance
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0 + 1, amount1 + 1],
        &sorobanvec![&test.env, amount0 + 1, amount1 + 1],
        &users[0],
        &false,
    );

    if deposit_result == Err(Err(InvokeError::Contract(10))) {
        return;
    } else {
        panic!("Expected error not returned");
    }
}


#[test]
fn test_deposit_with_zero_amounts() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);
    let strategy_params_token_2 = create_strategy_params(&test, test.strategy_client_token_2.address.clone());

    // initialize with 2 assets
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
        AssetStrategySet {
            address: test.token_2.address.clone(),
            strategies: strategy_params_token_2.clone()
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, test.manager.clone());
    roles.set(
        RolesDataKey::EmergencyManager as u32,
        test.emergency_manager.clone(),
    );
    roles.set(
        RolesDataKey::VaultFeeReceiver as u32,
        test.vault_fee_receiver.clone(),
    );
    roles.set(
        RolesDataKey::RebalanceManager as u32,
        test.rebalance_manager.clone(),
    );

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(
        String::from_str(&test.env, "name"),
        String::from_str(&test.env, "dfToken"),
    );
    name_symbol.set(
        String::from_str(&test.env, "symbol"),
        String::from_str(&test.env, "DFT"),
    );

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

    // Deposit more than min liquidity
    let deposit_amount_token_1 = MINIMUM_LIQUIDITY + 1;
    let deposit_amount_token_2 = 2*deposit_amount_token_1;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    test.token_1_admin_client.mint(&users[0], &deposit_amount_token_1);
    test.token_2_admin_client.mint(&users[0], &deposit_amount_token_2);
    // One should be allowed to deposit 0 amount for the any token
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0, deposit_amount_token_1, deposit_amount_token_2],
        &sorobanvec![&test.env, 0, 0, 0],
        &users[0],
        &false,
    );
    std::println!("deposit_result: {:?}", deposit_result);
    // user[0] should have deposit_amount_token_1 + deposit_amount_token_2 - MINIMUM_LIQUIDITY shares minted
    // Total supply should be deposit_amount_token_1 + deposit_amount_token_2
    assert_eq!(deposit_result.is_err(), false);
    let total_supply = defindex_contract.total_supply();
    let user_balance = defindex_contract.balance(&users[0]);
    assert_eq!(user_balance, deposit_amount_token_1 + deposit_amount_token_2 - MINIMUM_LIQUIDITY);
    assert_eq!(total_supply, deposit_amount_token_1 + deposit_amount_token_2);


    // one should be allowed to deposit 0 amount for the any token and the other token with a positive amount
    test.token_1_admin_client.mint(&users[1], &deposit_amount_token_1);
    test.token_2_admin_client.mint(&users[1], &deposit_amount_token_2);
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0, deposit_amount_token_1, deposit_amount_token_2],
        &sorobanvec![&test.env, 0, 0, 0],
        &users[1],
        &false,
    );
    std::println!("deposit_result second balance: {:?}", deposit_result);
    assert_eq!(deposit_result.is_err(), false);
    // User[0] should have deposit_amount_token_1 + deposit_amount_token_2 shares minted
    // Total supply should be 2*(deposit_amount_token_1 + deposit_amount_token_2)
    let user_balance = defindex_contract.balance(&users[1]);
    let total_supply = defindex_contract.total_supply();
    std::println!("user_balance: {:?}", user_balance);
    std::println!("total_supply: {:?}", total_supply);
    assert_eq!(user_balance, deposit_amount_token_1 + deposit_amount_token_2);
    assert_eq!(total_supply, 2*(deposit_amount_token_1 + deposit_amount_token_2));

    // User 1 attempts to deposit 100_000_000 from each token
    let amount = 100_000_000;
    test.token_0_admin_client.mint(&users[1], &amount);
    test.token_1_admin_client.mint(&users[1], &amount);
    // We don't care about the min amounts for the purpose of the PoC
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, amount, 0],
        &sorobanvec![&test.env, amount*9/10, amount*9/10, 0],
        &users[1],
        &false,
    );
    // this call will fail because the last amount should not be 0
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

    // Test depositing all zeros
    // It should fail with InsufficientAmount because mint_shares are 0
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount, 0, 0],
        &sorobanvec![&test.env, amount*9/10, 0, 0], 
        &users[1],
        &false,
    );

    // Should fail since we can't deposit all zeros
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

    // Test depositing all zeros
    // It should fail with InsufficientAmount because mint_shares are 0
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0, 0, 0],
        &sorobanvec![&test.env, 0, 0, 0],
        &users[1], 
        &false,
    );

    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

    // Test depositing [0, amount, 0]
    // It should fail with InsufficientAmount since last amount is 0
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0, amount, 0],
        &sorobanvec![&test.env, 0, amount*9/10, 0],
        &users[1],
        &false,
    );

    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

    // Test depositing [0, 0, amount]
    // It should fail with InsufficientAmount since first two amounts are 0
    let deposit_result = defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0, 0, amount],
        &sorobanvec![&test.env, 0, 0, amount*9/10],
        &users[1],
        &false,
    );

    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

}