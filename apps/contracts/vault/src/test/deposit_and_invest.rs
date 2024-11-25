use soroban_sdk::{vec as sorobanvec, String, Vec, Map};

use crate::test::defindex_vault::{AssetStrategySet};
use crate::test::{
    create_strategy_params_token0, create_strategy_params_token1, DeFindexVaultTest,
};

// test deposit one asset success 
#[test]
fn deposit_and_invest_one_asset_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);

    // initialize with 1 assets
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
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
    test.defindex_contract.deposit_and_invest(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
    );

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);
    
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    //map shuould be map
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), amount);

    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);

    // check that all the assets are invested
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, 0);

    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, expected_invested_map);

    // check current idle funds, 
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_invested_map);

    // Now user deposits for the second time
    let amount2 = 987654321i128;
    test.token0_admin_client.mint(&users[0], &amount2);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount2);

    // deposit
    test.defindex_contract.deposit_and_invest(
        &sorobanvec![&test.env, amount2],
        &sorobanvec![&test.env, amount2],
        &users[0],
    );

    //map shuould be map
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), amount + amount2);

    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount + amount2 - 1000);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);
    
    // check that the assets are not in the vault
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, 0);
    
    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, expected_invested_map);
    
    // check current idle funds
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);
    
    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_invested_map);
}

// test deposit of several asset, considering different proportion of assets
#[test]
fn deposit_and_invest_several_assets_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
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
    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token0_admin_client.mint(&users[0], &amount0);
    test.token1_admin_client.mint(&users[0], &amount1);
    let user_balance0 = test.token0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let deposit_result=test.defindex_contract.deposit_and_invest(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
    );

    // check deposit result
    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0, amount1], amount0 + amount1));

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    // For first deposit, a minimum amount LIQUIDITY OF 1000 is being locked in the contract
    assert_eq!(df_balance, amount0 + amount1 - 1000);

    // check that the vault holds 1000 shares
    let vault_df_shares = test.defindex_contract.balance(&test.defindex_contract.address);
    assert_eq!(vault_df_shares, 1000i128);
    
    let user_balance0 = test.token0.balance(&users[0]);
    assert_eq!(user_balance0,0i128);
    let user_balance1 = test.token1.balance(&users[0]);
    assert_eq!(user_balance1,0i128);

    // check vault balance of asset 0
    let vault_balance0 = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance0, 0);
    // check vault balance of asset 1
    let vault_balance1 = test.token1.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance1, 0);

    //map shuould be map
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), amount0);
    expected_invested_map.set(test.token1.address.clone(), amount1);

    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);
    expected_idle_map.set(test.token1.address.clone(), 0);

    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, expected_invested_map);

    // check current idle funds
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);
    
    // check that current invested funds is now correct, funds should be invested
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_invested_map);

    // new user wants to do a deposit with more assets 0 than the proportion, but with minium amount 0
    // multiply amount0 by 2
    let amount0_new =  amount0*2 +100 ;
    let amount1_new = amount1*2;

    // mint this to user 1
    test.token0_admin_client.mint(&users[1], &amount0_new);
    test.token1_admin_client.mint(&users[1], &amount1_new);

    // check user balances
    let user_balance0 = test.token0.balance(&users[1]);
    assert_eq!(user_balance0, amount0_new);
    let user_balance1 = test.token1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new);


    // user 1 deposits
    let deposit_result=test.defindex_contract.deposit_and_invest(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128

    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0*2, amount1*2], amount0*2 + amount1*2));


    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[1]);
    assert_eq!(df_balance, 2*(amount0 + amount1));

    let user_balance0 = test.token0.balance(&users[1]);
    assert_eq!(user_balance0, amount0_new - 2*amount0);

    let user_balance1 = test.token1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new - 2*amount1);

    // check vault balance of asset 0
    let vault_balance0 = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance0, 0);
    // check vault balance of asset 1
    let vault_balance1 = test.token1.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance1, 0);

    //map shuould be map
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), 3*amount0);
    expected_invested_map.set(test.token1.address.clone(), 3*amount1);

    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);
    expected_idle_map.set(test.token1.address.clone(), 0);

    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, expected_invested_map);

    // check current idle funds
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_invested_map);

    // we will repeat one more time, now enforcing the first asset
    let amount0_new =  amount0*2;
    let amount1_new = amount1*2+100;

    // mint this to user 1
    test.token0_admin_client.mint(&users[1], &amount0_new);
    test.token1_admin_client.mint(&users[1], &amount1_new);
    
    // check user balances
    let user_balance0 = test.token0.balance(&users[1]);
    assert_eq!(user_balance0, 100 + amount0_new); // we still have 100 from before
    let user_balance1 = test.token1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new);

    // user 1 deposits
    let deposit_result=test.defindex_contract.deposit_and_invest(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128
    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0*2, amount1*2], amount0*2 + amount1*2));

}