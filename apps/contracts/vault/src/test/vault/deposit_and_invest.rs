use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{vec as sorobanvec, vec, Address, IntoVal, Map, String, Vec};

use crate::test::defindex_vault::{
    AssetInvestmentAllocation, AssetStrategySet, CurrentAssetInvestmentAllocation, RolesDataKey, Strategy, StrategyAllocation,Instruction
};
use crate::test::{
    create_defindex_vault, create_hodl_strategy, create_strategy_params_token_0, create_strategy_params_token_1, create_token_contract, get_token_admin_client, DeFindexVaultTest, EnvTestUtils
};

// with no previous investment, there should not be any investment
#[test]
fn one_asset_no_previous_investment() {
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

    // deposit AND INVEST
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &true,
    );

    // however because there was no previous investment, all the amount should be in idle funds

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    // all in idle funds
    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount);

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // all in idle funds
            paused: false,
        }
    ];

    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount,
            idle_amount: amount,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds,
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, amount);

    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token_0.address.clone(), 0);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, 0i128);

    // Now user deposits for the second time
    let amount2 = 987654321i128;
    test.token_0_admin_client.mint(&users[0], &amount2);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount2);

    // deposit AND INVEST
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount2],
        &sorobanvec![&test.env, amount2],
        &users[0],
        &true,
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount + amount2 - 1000);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount + amount2);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0,
            paused: false,
        }
    ];

    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount + amount2,
            idle_amount: amount + amount2,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, amount + amount2);

    // check that current invested funds is now 0, funds still in idle funds
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token_0.address.clone(), 0);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, expected_invested_map.get(test.token_0.address.clone()).unwrap());
}

#[test]
fn one_asset_previous_investment_success() {
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

    // all in idle funds
    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount);

    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // everything has been invested
            paused: false,
        }
    ];

    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount,
            idle_amount: amount,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    let amount_to_invest = 100000000i128;

    // GENERATE INVESTMENT
    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest
        ),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);

    // Now we should have:
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: amount_to_invest, // everything has been invested
            paused: false,
        }
    ];

    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount,
            idle_amount: amount - amount_to_invest,
            invested_amount: amount_to_invest,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds,
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, amount - amount_to_invest);

    // check that current invested funds is now 0, funds still in idle funds
    //map shuould be map
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token_0.address.clone(), amount_to_invest);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, expected_invested_map.get(test.token_0.address.clone()).unwrap());

    // DEPOSIT AND INVEST
    let amount2 = 987654321i128;
    test.token_0_admin_client.mint(&users[0], &amount2);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount2);

    // deposit AND INVEST
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount2],
        &sorobanvec![&test.env, amount2],
        &users[0],
        &true,
    );

    // because there was already some strategy allocation, all of the amount2 should be invested

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount + amount2 - 1000);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    // check that the assets are not in the vault
    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount - amount_to_invest);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: amount_to_invest + amount2, // everything has been invested
            paused: false,
        }
    ];

    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount + amount2,
            idle_amount: amount - amount_to_invest,
            invested_amount: amount_to_invest + amount2,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds, amount - amount_to_invest);

    // check that current invested funds
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token_0.address.clone(), amount_to_invest + amount2);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, expected_invested_map.get(test.token_0.address.clone()).unwrap());
}

#[test]
fn several_assets_no_previous_investment() {
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

    // deposit // however wih no previous investment yet
    let deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &true,
    );

    // check deposit result
    assert_eq!(
        deposit_result,
        (
            sorobanvec![&test.env, amount0, amount1], 
            amount0 + amount1, 
            Some(sorobanvec![&test.env, None, None])
        )
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

    // all in idle funds
    let vault_balance0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance0, amount0);
    // check vault balance of asset 1
    let vault_balance1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, amount1);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0,
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0,
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1,
            idle_amount: amount1,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds
    let current_idle_funds_token_0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_0, amount0);
    let current_idle_funds_token_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(current_idle_funds_token_1, amount1);

    // check that current invested funds is now correct,
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token_0.address.clone(), 0);
    expected_invested_map.set(test.token_1.address.clone(), 0);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(current_invested_funds, expected_invested_map.get(test.token_0.address.clone()).unwrap());

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
        &true,
    );

    assert_eq!(
        deposit_result,
        (
            sorobanvec![&test.env, amount0 * 2, amount1 * 2],
            amount0 * 2 + amount1 * 2,
            Some(sorobanvec![&test.env, None, None])
        )
    );

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[1]);
    assert_eq!(df_balance, 2 * (amount0 + amount1));

    let user_balance0 = test.token_0.balance(&users[1]);
    assert_eq!(user_balance0, amount0_new - 2 * amount0);

    let user_balance1 = test.token_1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new - 2 * amount1);

    // check vault balance of asset 0, all in idle funds
    let vault_balance0 = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance0, amount0 * 3);
    // check vault balance of asset 1
    let vault_balance1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, amount1 * 3);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // everything has been invested
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // everything has been invested
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0 * 3,
            idle_amount: amount0 * 3,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1 * 3,
            idle_amount: amount1 * 3,
            invested_amount: 0,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
}

#[test]
fn several_assets_wih_previous_investment_success() {
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

    // deposit with no previous investment
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &true,
    );

    // GENERATE INVESTMENT
    let amount_to_invest_0 = 100000000i128;
    let amount_to_invest_1 = 200000000i128;

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount_to_invest_0),
        Instruction::Invest(test.strategy_client_token_1.address.clone(), amount_to_invest_1),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);

    // total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: amount_to_invest_0, // everything has been invested
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: amount_to_invest_1, // everything has been invested
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0 - amount_to_invest_0,
            invested_amount: amount_to_invest_0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1,
            idle_amount: amount1 - amount_to_invest_1,
            invested_amount: amount_to_invest_1,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // Now that we have previous invesment, we will do deposit and invest and this deposit should be invested directly

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
        &true,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128

    assert_eq!(
        deposit_result,
        (
            sorobanvec![&test.env, amount0 * 2, amount1 * 2],
            amount0 * 2 + amount1 * 2,
            Some(sorobanvec![
                &test.env,
                Some(AssetInvestmentAllocation{
                    asset: test.token_0.address.clone(),
                    strategy_allocations: vec![
                        &test.env,
                        Some(StrategyAllocation {
                            strategy_address: test.strategy_client_token_0.address.clone(),
                            amount: (amount0_new-100),
                            paused: false,
                        }),
                    ],
                }),
                Some(AssetInvestmentAllocation{
                    asset: test.token_1.address.clone(),
                    strategy_allocations: vec![
                        &test.env,
                        Some(StrategyAllocation {
                            strategy_address: test.strategy_client_token_1.address.clone(),
                            amount: amount1_new,
                            paused: false,
                        }),
                    ],
                }),
            ])
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
    assert_eq!(vault_balance0, amount0 - amount_to_invest_0);
    // check vault balance of asset 1
    let vault_balance1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, amount1 - amount_to_invest_1);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: amount0 * 2 + amount_to_invest_0, // only new deposit and invest
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: amount1 * 2 + amount_to_invest_1, // only new deposit and invest
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0 * 3,
            idle_amount: amount0 - amount_to_invest_0,
            invested_amount: amount0 * 2 + amount_to_invest_0,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1 * 3,
            idle_amount: amount1 - amount_to_invest_1,
            invested_amount: amount1 * 2 + amount_to_invest_1,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // invested_amount: 1780246914, strategy_allocations: Vec(Ok(StrategyAllocation { amount: 1780246914, strategy_address: Contract(CDDD62URLXHZ2SEMZ3ZWWKRA2DCK75PELPRVLAW4PPO5PRL2HJW25HLF) })), total_amount: 2567901235 })), Ok((Contract(CDS3FDGQ4JA2V3F26Y4BMWWJEC5TT26RJBN7KIQKUMVO2MAOCMDTSZ7A), CurrentAssetInvestmentAllocation { asset: Contract(CDS3FDGQ4JA2V3F26Y4BMWWJEC5TT26RJBN7KIQKUMVO2MAOCMDTSZ7A), idle_amount: 23456789, invested_amount: 297530863, strategy_allocations: Vec(Ok(StrategyAllocation { amount: 297530863, strategy_address: Contract(CB457TMKS3NBPJJRHNCRJMSAWP2YMCNIORWHHF6MNZJQQGZQRPSANQSE) })), total_amount: 320987652 })))
    // invested_amount: 2175308642, strategy_allocations: Vec(Ok(StrategyAllocation { amount: 2175308642, strategy_address: Contract(CDDD62URLXHZ2SEMZ3ZWWKRA2DCK75PELPRVLAW4PPO5PRL2HJW25HLF) })), total_amount: 2962962963 })), Ok((Contract(CDS3FDGQ4JA2V3F26Y4BMWWJEC5TT26RJBN7KIQKUMVO2MAOCMDTSZ7A), CurrentAssetInvestmentAllocation { asset: Contract(CDS3FDGQ4JA2V3F26Y4BMWWJEC5TT26RJBN7KIQKUMVO2MAOCMDTSZ7A), idle_amount: 23456789, invested_amount: 346913578, strategy_allocations: Vec(Ok(StrategyAllocation { amount: 346913578, strategy_address: Contract(CB457TMKS3NBPJJRHNCRJMSAWP2YMCNIORWHHF6MNZJQQGZQRPSANQSE) })), total_amount: 370370367 })))
    // // check current idle funds
    // let mut expected_idle_map = Map::new(&test.env);
    // expected_idle_map.set(test.token_0.address.clone(), 0);
    // expected_idle_map.set(test.token_1.address.clone(), 0);
    // let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    // assert_eq!(current_idle_funds, expected_idle_map);

    // // check that current invested funds is now 0, funds still in idle funds
    // let mut expected_invested_map = Map::new(&test.env);
    // expected_invested_map.set(test.token_0.address.clone(), 3*amount0);
    // expected_invested_map.set(test.token_1.address.clone(), 3*amount1);
    // let current_invested_funds = test.defindex_contract.fetch_total_managed_funds();
    // //🟢assert_eq!(current_invested_funds, expected_invested_map);

    // // we will repeat one more time, now enforcing the first asset
    // let amount0_new =  amount0*2;
    // let amount1_new = amount1*2+100;

    // // mint this to user 1
    // test.token_0_admin_client.mint(&users[1], &amount0_new);
    // test.token_1_admin_client.mint(&users[1], &amount1_new);

    // // check user balances
    // let user_balance0 = test.token_0.balance(&users[1]);
    // assert_eq!(user_balance0, 100 + amount0_new); // we still have 100 from before
    // let user_balance1 = test.token_1.balance(&users[1]);
    // assert_eq!(user_balance1, amount1_new);

    // // user 1 deposits
    // let deposit_result=test.defindex_contract.deposit(
    //     &sorobanvec![&test.env, amount0_new, amount1_new],
    //     &sorobanvec![&test.env, 0i128, 0i128],
    //     &users[1],
    //     &true,
    // );

    // // check deposit result. Ok((amounts, shares_to_mint))
    // // Vec<i128>, i128
    // assert_eq!(deposit_result, (sorobanvec![&test.env, amount0*2, amount1*2], amount0*2 + amount1*2));
}

#[test]
fn one_asset_several_strategies() {
    /*
        What happens when no previous investment has been done?

    */
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_client_1 = create_hodl_strategy(&test.env, &test.token_0.address.clone());
    let strategy_client_2 = create_hodl_strategy(&test.env, &test.token_0.address.clone());
    let strategy_client_3 = create_hodl_strategy(&test.env, &test.token_0.address.clone());
    
    let strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "strategy1"),
            address: test.strategy_client_token_0.address.clone(),
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
    ];

    // initialize with 1 asset, 3 strategies
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params.clone(),
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
    
    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 1);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.strategies.len(), strategy_params.len());

    let amount0 = 10_0_000_000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
   
    let deposit_amount = 5_0_000_000i128;
    // deposit with no previous investment
    let _deposit0 = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount],
        &sorobanvec![&test.env, deposit_amount],
        &users[0],
        &true,
    );
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds, 0i128);
    assert_eq!(idle_funds, deposit_amount);

  
    // Invest
    let amount_to_invest = 1_0_000_000i128;
    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount_to_invest),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount_to_invest),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount_to_invest),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount_to_invest),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);
    assert_eq!(invested_funds, (amount_to_invest * 4));
    assert_eq!(idle_funds, deposit_amount - (amount_to_invest * 4));

    // deposit with invest
    let deposit_amount_2 = 1_000_000i128;
    let _deposit1 = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount_2],
        &sorobanvec![&test.env, deposit_amount_2],
        &users[0],
        &true,
    );
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds, (amount_to_invest * 4) + deposit_amount_2);
    assert_eq!(idle_funds, (deposit_amount + deposit_amount_2) - (amount_to_invest * 4) - deposit_amount_2);

}

#[test]
fn deposit_simple_then_deposit_and_invest() {
    /*
        Here we will check that everything works ok if the user first do a simple deposit without invest, and then does the deposit and invest
        and if then does the deposit again without invest?
    */
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params = create_strategy_params_token_0(&test);

    // initialize with 1 asset, 3 strategies
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params.clone(),
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

    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 1);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.strategies.len(), 1);

    let amount0 = 12_3_456_789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
   
    let deposit_amount = 6_0_000_000i128;
    // deposit with no previous investment

    let mut total_deposit = deposit_amount;
    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount],
        &sorobanvec![&test.env, deposit_amount],
        &users[0],
        &false,
    );
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds, 0i128);
    assert_eq!(idle_funds, total_deposit);

    let deposit_and_invest_amount = 2_0_000_000i128;
    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_and_invest_amount],
        &sorobanvec![&test.env, deposit_and_invest_amount],
        &users[0],
        &true,
    );

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    total_deposit += deposit_and_invest_amount;
    assert_eq!(invested_funds, 0i128);
    assert_eq!(idle_funds, total_deposit);

    let deposit_amount_1 = 2_0_000_000i128;
    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount_1],
        &sorobanvec![&test.env, deposit_amount_1],
        &users[0],
        &false,
    );

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    total_deposit += deposit_amount_1;
    assert_eq!(invested_funds, 0i128);
    assert_eq!(idle_funds, total_deposit);

    let amount_to_invest = 4_0_000_000i128;
    let mut total_invested = amount_to_invest;

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds, total_invested);
    assert_eq!(idle_funds, total_deposit - total_invested);

    let deposit_amount_2 = 1_0_000_000i128;

    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount_2],
        &sorobanvec![&test.env, deposit_amount_2],
        &users[0],
        &true,
    );
    total_deposit += deposit_amount_2;
    total_invested += deposit_amount_2;

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds, total_invested);
    assert_eq!(idle_funds, total_deposit - total_invested);
    let deposit_amount_3 = 1_000_000i128;

    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount_3],
        &sorobanvec![&test.env, deposit_amount_3],
        &users[0],
        &false,
    );
    let expected_idle_funds = idle_funds + deposit_amount_3;
    
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds = test.token_0.balance(&defindex_contract.address);
    
    assert_eq!(invested_funds, total_invested);
    assert_eq!(idle_funds, expected_idle_funds);

}


#[test]
fn several_assets_several_strategies() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let asset_0_strategy_1 = create_hodl_strategy(&test.env, &test.token_0.address.clone());
    let asset_0_strategy_2 = create_hodl_strategy(&test.env, &test.token_0.address.clone());

    let asset_1_strategy_1 = create_hodl_strategy(&test.env, &test.token_1.address.clone());
    let asset_1_strategy_2 = create_hodl_strategy(&test.env, &test.token_1.address.clone());

    let new_token_admin = Address::generate(&test.env);
    let token_3 = create_token_contract(&test.env, &new_token_admin);
    let token_3_admin_client = get_token_admin_client(&test.env, &token_3.address.clone());

    let asset_2_strategy_1 = create_hodl_strategy(&test.env, &token_3.address.clone());
    let asset_2_strategy_2 = create_hodl_strategy(&test.env, &token_3.address.clone());
    
    let asset_0_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "strategy0"),
            address: test.strategy_client_token_0.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy1"),
            address: asset_0_strategy_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy2"),
            address: asset_0_strategy_2.address.clone(),
            paused: false,
        },
    ];

    let asset_1_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "strategy0"),
            address: test.strategy_client_token_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy1"),
            address: asset_1_strategy_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy2"),
            address: asset_1_strategy_2.address.clone(),
            paused: false,
        },
    ];

    let asset_2_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "strategy0"),
            address: asset_2_strategy_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy1"),
            address: asset_2_strategy_2.address.clone(),
            paused: false,
        },
    ];

    // initialize with 2 asset, 2 strategies each
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: asset_0_strategy_params.clone(),
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: asset_1_strategy_params.clone(),
        },
        AssetStrategySet {
            address: token_3.address.clone(),
            strategies: asset_2_strategy_params.clone(),
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
    
    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 3);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.strategies.len(), asset_0_strategy_params.len());
    let asset_2 = assets.get(1).unwrap();
    assert_eq!(asset_2.strategies.len(), asset_1_strategy_params.len());
    let asset_3 = assets.get(2).unwrap();
    assert_eq!(asset_3.strategies.len(), asset_2_strategy_params.len());


    let amount0 = 100_0_000_000i128;
    let amount1 = 2000_0_000_000i128;
    let amount2 = 200_0_000_000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    token_3_admin_client.mint(&users[0], &amount2);
   
    let deposit_amount_0 = 50_0_000_000i128;
    let deposit_amount_1 = 1000_0_000_000i128;
    let deposit_amount_2 = 100_0_000_000i128;
    
    // first deposit with no previous investment will no be invested
    defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount_0, deposit_amount_1, deposit_amount_2],
        &sorobanvec![&test.env, deposit_amount_0, deposit_amount_1, deposit_amount_2],
        &users[0],
        &true, // Even if it is true
    );
    
    assert_eq!(test.token_0.balance(&test.strategy_client_token_0.address), 0);
    assert_eq!(test.token_0.balance(&asset_0_strategy_1.address), 0); 
    assert_eq!(test.token_0.balance(&asset_0_strategy_2.address), 0); 
    assert_eq!(test.token_1.balance(&test.strategy_client_token_1.address), 0);
    assert_eq!(test.token_1.balance(&asset_1_strategy_1.address), 0);
    assert_eq!(test.token_1.balance(&asset_1_strategy_2.address), 0);
    assert_eq!(token_3.balance(&asset_2_strategy_1.address), 0);
    assert_eq!(token_3.balance(&asset_2_strategy_2.address), 0);

    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, 0i128);
    assert_eq!(idle_funds_a0, deposit_amount_0);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, 0i128);
    assert_eq!(idle_funds_a1, deposit_amount_1);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = token_3.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a2, 0i128);
    assert_eq!(idle_funds_a2, deposit_amount_2);

  
    // First needs to have some funds invested so the deposit_and_invest works
    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), deposit_amount_0 / 4 * 2),
        Instruction::Invest(asset_0_strategy_1.address.clone(), deposit_amount_0 / 4),
        Instruction::Invest(asset_0_strategy_2.address.clone(), deposit_amount_0 / 4),
        Instruction::Invest(test.strategy_client_token_1.address.clone(), deposit_amount_1 / 4 * 2),
        Instruction::Invest(asset_1_strategy_1.address.clone(), deposit_amount_1 / 4),
        Instruction::Invest(asset_1_strategy_2.address.clone(), deposit_amount_1 / 4),
        Instruction::Invest(asset_2_strategy_1.address.clone(), deposit_amount_2 / 2),
        Instruction::Invest(asset_2_strategy_2.address.clone(), deposit_amount_2 / 2),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);

    assert_eq!(test.token_0.balance(&test.strategy_client_token_0.address), deposit_amount_0 / 4 * 2);
    assert_eq!(test.token_0.balance(&asset_0_strategy_1.address), deposit_amount_0 / 4); 
    assert_eq!(test.token_0.balance(&asset_0_strategy_2.address), deposit_amount_0 / 4); 
    assert_eq!(test.token_1.balance(&test.strategy_client_token_1.address), deposit_amount_1 / 4 * 2);
    assert_eq!(test.token_1.balance(&asset_1_strategy_1.address), deposit_amount_1 / 4);
    assert_eq!(test.token_1.balance(&asset_1_strategy_2.address), deposit_amount_1 / 4);
    assert_eq!(token_3.balance(&asset_2_strategy_1.address), deposit_amount_2 / 2);
    assert_eq!(token_3.balance(&asset_2_strategy_2.address), deposit_amount_2 / 2);
    
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, deposit_amount_0);
    assert_eq!(idle_funds_a0, 0i128);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, deposit_amount_1);
    assert_eq!(idle_funds_a1, 0i128);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = token_3.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a2, deposit_amount_2);
    assert_eq!(idle_funds_a2, 0i128);

    // deposit and invest
    defindex_contract.deposit(
        &sorobanvec![&test.env, deposit_amount_0, deposit_amount_1, deposit_amount_2],
        &sorobanvec![&test.env, deposit_amount_0, deposit_amount_1, deposit_amount_2],
        &users[0],
        &true,
    );

    assert_eq!(test.token_0.balance(&test.strategy_client_token_0.address), amount0 / 4 * 2);
    assert_eq!(test.token_0.balance(&asset_0_strategy_1.address), amount0 / 4); 
    assert_eq!(test.token_0.balance(&asset_0_strategy_2.address), amount0 / 4); 
    assert_eq!(test.token_1.balance(&test.strategy_client_token_1.address), amount1 / 4 * 2);
    assert_eq!(test.token_1.balance(&asset_1_strategy_1.address), amount1 / 4);
    assert_eq!(test.token_1.balance(&asset_1_strategy_2.address), amount1 / 4);
    assert_eq!(token_3.balance(&asset_2_strategy_1.address), amount2 / 2);
    assert_eq!(token_3.balance(&asset_2_strategy_2.address), amount2 / 2);

    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, amount0);
    assert_eq!(idle_funds_a0, 0i128);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, amount1);
    assert_eq!(idle_funds_a1, 0i128);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = token_3.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a2, amount2);
    assert_eq!(idle_funds_a2, 0i128);

    test.env.jump_time(518400);

    // Depositing from other users
    test.token_0_admin_client.mint(&users[1], &amount0);
    test.token_1_admin_client.mint(&users[1], &amount1);
    token_3_admin_client.mint(&users[1], &amount2);
    
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1, amount2],
        &sorobanvec![&test.env, amount0, amount1, amount2],
        &users[1],
        &true,
    );

    test.token_0_admin_client.mint(&users[2], &amount0);
    test.token_1_admin_client.mint(&users[2], &amount1);
    token_3_admin_client.mint(&users[2], &amount2);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1, amount2],
        &sorobanvec![&test.env, amount0, amount1, amount2],
        &users[2],
        &true,
    );

    assert_eq!(test.token_0.balance(&test.strategy_client_token_0.address), (amount0 * 3) / 4 * 2);
    assert_eq!(test.token_0.balance(&asset_0_strategy_1.address), (amount0 * 3) / 4); 
    assert_eq!(test.token_0.balance(&asset_0_strategy_2.address), (amount0 * 3) / 4); 
    assert_eq!(test.token_1.balance(&test.strategy_client_token_1.address), (amount1 * 3) / 4 * 2);
    assert_eq!(test.token_1.balance(&asset_1_strategy_1.address), (amount1 * 3) / 4);
    assert_eq!(test.token_1.balance(&asset_1_strategy_2.address), (amount1 * 3) / 4);
    assert_eq!(token_3.balance(&asset_2_strategy_1.address), (amount2 * 3) / 2);
    assert_eq!(token_3.balance(&asset_2_strategy_2.address), (amount2 * 3) / 2);
    
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, amount0 * 3);
    assert_eq!(idle_funds_a0, 0i128);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, amount1 * 3);
    assert_eq!(idle_funds_a1, 0i128);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = token_3.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a2, amount2 * 3);
    assert_eq!(idle_funds_a2, 0i128);

}

#[test]
fn several_assets_one_strategy_paused() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    
    let new_token_admin = Address::generate(&test.env);
    let token_2 = create_token_contract(&test.env, &new_token_admin);
    let token_2_admin_client = get_token_admin_client(&test.env, &token_2.address.clone());

    let asset_2_strategy = create_hodl_strategy(&test.env, &token_2.address.clone());

    let asset_0_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "token 0 strategy0"),
            address: test.strategy_client_token_0.address.clone(),
            paused: false,
        },
    ];

    let asset_1_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "token 1 strategy0"),
            address: test.strategy_client_token_1.address.clone(),
            paused: false,
        },
    ];

    let asset_2_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "token 2 strategy0"),
            address: asset_2_strategy.address.clone(),
            paused: false,
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
        sorobanvec![
            &test.env,
            AssetStrategySet {
                address: test.token_0.address.clone(),
                strategies: asset_0_strategy_params.clone(),
            },
            AssetStrategySet {
                address: test.token_1.address.clone(),
                strategies: asset_1_strategy_params.clone(),
            },
            AssetStrategySet {
                address: token_2.address.clone(),
                strategies: asset_2_strategy_params.clone(),
            }
        ],
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );

    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 3);
    assert_eq!(assets.get(0).unwrap().strategies.len(), 1);
    assert_eq!(assets.get(1).unwrap().strategies.len(), 1);
    assert_eq!(assets.get(2).unwrap().strategies.len(), 1);

    let mint_amount = 10_0_000_000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    test.token_0_admin_client.mint(&users[0], &mint_amount);
    test.token_1_admin_client.mint(&users[0], &mint_amount);
    token_2_admin_client.mint(&users[0], &mint_amount);

    let user_0_token_0_balance = test.token_0.balance(&users[0]);
    let user_0_token_1_balance = test.token_1.balance(&users[0]);
    let user_0_token_2_balance = token_2.balance(&users[0]);

    assert_eq!(user_0_token_0_balance, mint_amount);
    assert_eq!(user_0_token_1_balance, mint_amount);
    assert_eq!(user_0_token_2_balance, mint_amount);

    defindex_contract.deposit(
        &sorobanvec![&test.env, mint_amount/2, mint_amount/2, mint_amount/2],
        &sorobanvec![&test.env, mint_amount/2, mint_amount/2, mint_amount/2],
        &users[0].clone(),
        &false
    );

    // Check that the funds are idle
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, 0);
    assert_eq!(idle_funds_a0, mint_amount/2);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, 0);
    assert_eq!(idle_funds_a1, mint_amount/2);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = token_2.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a2, 0);
    assert_eq!(idle_funds_a2, mint_amount/2);

    // Rebalance to invest the funds
    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), mint_amount/2),
        Instruction::Invest(test.strategy_client_token_1.address.clone(), mint_amount/2),
        Instruction::Invest(asset_2_strategy.address.clone(), mint_amount/2),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);

    // Check that the funds are invested
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, mint_amount/2);
    assert_eq!(idle_funds_a0, 0);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, mint_amount/2);
    assert_eq!(idle_funds_a1, 0);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = test.token_2.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a2, mint_amount/2);
    assert_eq!(idle_funds_a2, 0);

    test.env.mock_auths(&[MockAuth {
        address: &test.manager,
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address,
            fn_name: "pause_strategy",
            args: (asset_2_strategy.address.clone(), test.manager.clone()).into_val(&test.env),
            sub_invokes: &[],
        },
    }]);
    // Pause the strategy
    defindex_contract.pause_strategy(&asset_2_strategy.address.clone(), &test.manager.clone());
    
    // Check the strategy is paused
    let strategy = defindex_contract.get_assets().get(2).unwrap().strategies.get(0).unwrap();
    assert_eq!(strategy.paused, true);

    let amount_0 =  2_0_000_000i128;
    let amount_1 =  2_0_000_000i128;
    let amount_2 =  2_0_000_000i128;
    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount_0, amount_1, amount_2], 
        &sorobanvec![&test.env, amount_0, amount_1, amount_2], 
        &users[0].clone(), 
        &true
    );

    // Check that the funds are in the right place

    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    let invested_funds_a2 = defindex_contract.fetch_total_managed_funds().get(2).unwrap().invested_amount;
    let idle_funds_a2 = token_2.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, mint_amount/2 + amount_0);
    assert_eq!(idle_funds_a0, 0);

    assert_eq!(invested_funds_a1, mint_amount/2 + amount_1);
    assert_eq!(idle_funds_a1, 0);

    assert_eq!(invested_funds_a2, mint_amount/2);
    assert_eq!(idle_funds_a2, amount_2);
}

/* 
//Testear con varios assets:
    - [x] Uno de los assets tiene varias estrategias y sólo una de estas está pausada con fondos en ella
    - [x] Entonces, la cantidad correspondiente a esa estrategia debería mantenerse en fondos inactivos. 
    - [x] Luego, rescatar los fondos y depositar de nuevo
 */
#[test]
fn several_assets_one_strategy_paused_and_rescue() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_1 = create_hodl_strategy(&test.env, &test.token_1.address.clone());
    let asset_0_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "token 0 strategy0"),
            address: test.strategy_client_token_0.address.clone(),
            paused: false,
        }
    ];

    let asset_1_strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "token 1 strategy0"),
            address: test.strategy_client_token_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "token 1 strategy0"),
            address: strategy_1.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "token 1 strategy1"),
            address: test.fixed_strategy_client_token_1.address.clone(),
            paused: false,
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
        sorobanvec![
            &test.env,
            AssetStrategySet {
                address: test.token_0.address.clone(),
                strategies: asset_0_strategy_params.clone(),
            },
            AssetStrategySet {
                address: test.token_1.address.clone(),
                strategies: asset_1_strategy_params.clone(),
            }
        ],
        roles,
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.soroswap_router.address.clone(),
        name_symbol,
        true
    );

    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 2);
    assert_eq!(assets.get(0).unwrap().strategies.len(), 1);
    assert_eq!(assets.get(1).unwrap().strategies.len(), 3);

    let ten_lumens = 10_0_000_000i128;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &ten_lumens);
    test.token_1_admin_client.mint(&users[0], &(ten_lumens*3));

    let user_0_token_0_balance = test.token_0.balance(&users[0]);
    let user_0_token_1_balance = test.token_1.balance(&users[0]);

    assert_eq!(user_0_token_0_balance, ten_lumens);
    assert_eq!(user_0_token_1_balance, ten_lumens*3);

    defindex_contract.deposit(
        &sorobanvec![&test.env, ten_lumens, ten_lumens*3],
        &sorobanvec![&test.env, ten_lumens, ten_lumens*3],
        &users[0].clone(),
        &true
    );

    // Check that the funds are idle
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    let invested_full_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, 0);
    assert_eq!(idle_funds_a0, ten_lumens);

    assert_eq!(invested_full_funds_a1, 0);
    assert_eq!(idle_funds_a1, ten_lumens*3);

    // Rebalance to invest the funds

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), ten_lumens),
        Instruction::Invest(test.strategy_client_token_1.address.clone(), ten_lumens),
        Instruction::Invest(strategy_1.address.clone(), ten_lumens),
        Instruction::Invest(test.fixed_strategy_client_token_1.address.clone(), ten_lumens),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);

    // Check that the funds are invested
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    let invested_full_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, ten_lumens);
    assert_eq!(idle_funds_a0, 0);

    assert_eq!(invested_full_funds_a1, ten_lumens*3);
    assert_eq!(idle_funds_a1, 0);

    // Pause the strategy
    defindex_contract.pause_strategy(&strategy_1.address.clone(), &test.manager.clone());

    // Check only the strategy_1 is paused
    let strategy_a0 = defindex_contract.get_assets().get(0).unwrap().strategies.get(0).unwrap();
    assert_eq!(strategy_a0.paused, false);

    let a1_strategy0 = defindex_contract.get_assets().get(1).unwrap().strategies.get(0).unwrap();
    let a1_strategy1 = defindex_contract.get_assets().get(1).unwrap().strategies.get(1).unwrap();
    let a1_strategy2 = defindex_contract.get_assets().get(1).unwrap().strategies.get(2).unwrap();

    assert_eq!(a1_strategy0.paused, false);
    assert_eq!(a1_strategy1.paused, true);
    assert_eq!(a1_strategy2.paused, false);


    // Once paused, we will deposit again respecting the 1:3 ratio
    let a0_amount =  2_0_000_000i128;
    let a1_amount =  6_0_000_000i128;

    test.token_0_admin_client.mint(&users[0], &a0_amount);
    test.token_1_admin_client.mint(&users[0], &a1_amount);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, a0_amount, a1_amount], 
        &sorobanvec![&test.env, a0_amount, a1_amount], 
        &users[0].clone(), 
        &true
    );

    // Check that the funds are in the right place

    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    let invested_full_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);


    assert_eq!(invested_funds_a0, ten_lumens + a0_amount);
    assert_eq!(idle_funds_a0, 0);

    assert_eq!(invested_full_funds_a1, ten_lumens*3 + 4_0_000_000i128); //Representing the first deposit + the 2/3 part of the 6 lumens deposited in the paused strategy.
    assert_eq!(idle_funds_a1, 2_0_000_000i128); //Representing the part of the 6 lumens deposited in the paused strategy.

    defindex_contract.rescue(&strategy_1.address, &test.emergency_manager);

    //check if strategy_1 funds are in idle funds
    let invested_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a1, (ten_lumens*2 + (a1_amount/3)*2)); //(a1_amount/3)*2 is the amount of the not paused strategies (2/3) of the desposit
    assert_eq!(idle_funds_a1, (ten_lumens + (a1_amount/3))); //a1_amount/3 is the amount of the paused strategy

    //Deposit again (the strategy_1 funds should be in idle funds) respecting the 1:3 ratio
    test.token_0_admin_client.mint(&users[0], &2_0_000_000i128);
    test.token_1_admin_client.mint(&users[0], &6_0_000_000i128);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, a0_amount, a1_amount], 
        &sorobanvec![&test.env, 0i128, 0i128], 
        &users[0].clone(), 
        &true
    );

    // Check that the funds are in the right place
    let invested_funds_a0 = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let idle_funds_a0 = test.token_0.balance(&defindex_contract.address);

    let invested_full_funds_a1 = defindex_contract.fetch_total_managed_funds().get(1).unwrap().invested_amount;
    let idle_funds_a1 = test.token_1.balance(&defindex_contract.address);

    assert_eq!(invested_funds_a0, 14_0_000_000i128); // 10_000_000i128 + 2_000_000i128 + 2_000_000i128
    assert_eq!(idle_funds_a0, 0i128); // All invested

    assert_eq!(invested_full_funds_a1, 30_0_000_000i128); // 20_000_000i128 + 4_000_000i128 + 6_000_000i128 (the last deposit splits into the active strategies)
    assert_eq!(idle_funds_a1, 12_0_000_000i128); // 10_000_000i128 + 2_000_000i128 + 0i128 (the last deposit goes full to the active strategies)

}
