use soroban_sdk::{vec as sorobanvec, String, Vec, Map};

use crate::test::defindex_vault::{AssetStrategySet, StrategyAllocation, CurrentAssetInvestmentAllocation};
use crate::test::{
    create_defindex_vault, create_strategy_params_token0, create_strategy_params_token1, DeFindexVaultTest
};

// test deposit one asset success 
#[test]
fn one_asset_success() {
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
    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Balances before deposit
    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
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

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);
    
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    let vault_balance = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: amount, // everything has been invested
    }];
    
    total_managed_funds_expected.set(test.token0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        total_amount: amount,
        idle_amount: 0,
        invested_amount: amount,
        strategy_allocations: strategy_investments_expected_token_0,
    });

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    
    
    
    // check current idle funds, 
    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);
    
    // check that current invested funds is now 0, funds still in idle funds
    //map shuould be map
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), amount);
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_invested_map);

    // Now user deposits for the second time
    let amount2 = 987654321i128;
    test.token0_admin_client.mint(&users[0], &amount2);
    let user_balance = test.token0.balance(&users[0]);
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
    
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);
    
    // check that the assets are not in the vault
    let vault_balance = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);
    
    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: amount + amount2, // everything has been invested
    }];

    total_managed_funds_expected.set(test.token0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        total_amount: amount + amount2,
        idle_amount: 0,
        invested_amount: amount + amount2,
        strategy_allocations: strategy_investments_expected_token_0,
    });

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
    
    // check current idle funds
    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);
    
    // check that current invested funds is now 0, funds still in idle funds
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), amount + amount2);
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_invested_map);
}

// test deposit of several asset, considering different proportion of assets
#[test]
fn several_assets_success() {
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

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    let deposit_result=defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &true,
    );

    // check deposit result
    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0, amount1], amount0 + amount1));

    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    // For first deposit, a minimum amount LIQUIDITY OF 1000 is being locked in the contract
    assert_eq!(df_balance, amount0 + amount1 - 1000);

    // check that the vault holds 1000 shares
    let vault_df_shares = defindex_contract.balance(&defindex_contract.address);
    assert_eq!(vault_df_shares, 1000i128);
    
    let user_balance0 = test.token0.balance(&users[0]);
    assert_eq!(user_balance0,0i128);
    let user_balance1 = test.token1.balance(&users[0]);
    assert_eq!(user_balance1,0i128);

    // check vault balance of asset 0
    let vault_balance0 = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance0, 0);
    // check vault balance of asset 1
    let vault_balance1 = test.token1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, 0);

    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: amount0, // everything has been invested
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token1.address.clone(),
        amount: amount1, // everything has been invested
    }];
    total_managed_funds_expected.set(test.token0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        total_amount: amount0,
        idle_amount: 0,
        invested_amount: amount0,
        strategy_allocations: strategy_investments_expected_token_0,
    });
    total_managed_funds_expected.set(test.token1.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token1.address.clone(),
        total_amount: amount1,
        idle_amount: 0,
        invested_amount: amount1,
        strategy_allocations: strategy_investments_expected_token_1,
    });

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
    
    // check current idle funds
    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);
    expected_idle_map.set(test.token1.address.clone(), 0);
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);
    
    // check that current invested funds is now correct, funds should be invested
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), amount0);
    expected_invested_map.set(test.token1.address.clone(), amount1);
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
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
    let deposit_result=defindex_contract.deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
        &true,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128

    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0*2, amount1*2], amount0*2 + amount1*2));


    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[1]);
    assert_eq!(df_balance, 2*(amount0 + amount1));

    let user_balance0 = test.token0.balance(&users[1]);
    assert_eq!(user_balance0, amount0_new - 2*amount0);

    let user_balance1 = test.token1.balance(&users[1]);
    assert_eq!(user_balance1, amount1_new - 2*amount1);

    // check vault balance of asset 0
    let vault_balance0 = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance0, 0);
    // check vault balance of asset 1
    let vault_balance1 = test.token1.balance(&defindex_contract.address);
    assert_eq!(vault_balance1, 0);

    
    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: amount0*3, // everything has been invested
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token1.address.clone(),
        amount: amount1*3, // everything has been invested
    }];
    total_managed_funds_expected.set(test.token0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        total_amount: amount0*3,
        idle_amount: 0,
        invested_amount: amount0*3,
        strategy_allocations: strategy_investments_expected_token_0,
    });
    total_managed_funds_expected.set(test.token1.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token1.address.clone(),
        total_amount: amount1*3,
        idle_amount: 0,
        invested_amount: amount1*3,
        strategy_allocations: strategy_investments_expected_token_1,
    });
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
    
    // check current idle funds
    let mut expected_idle_map = Map::new(&test.env);
    expected_idle_map.set(test.token0.address.clone(), 0);
    expected_idle_map.set(test.token1.address.clone(), 0);
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_map);
    
    // check that current invested funds is now 0, funds still in idle funds
    let mut expected_invested_map = Map::new(&test.env);
    expected_invested_map.set(test.token0.address.clone(), 3*amount0);
    expected_invested_map.set(test.token1.address.clone(), 3*amount1);
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
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
    let deposit_result=defindex_contract.deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
        &true,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128
    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0*2, amount1*2], amount0*2 + amount1*2));

}

#[test] 
fn one_asset_several_strategies() {
    /*
        What happens when no previous investment has been done?
    
    */
    todo!();
}



#[test] 
fn deposit_simple_then_deposit_and_invest() {
    /*
        Here we will check that everything works ok if the user first do a simple deposit without invest, and then does the deposit and invest
        and if then does the deposit again without invest?
    
    */
    todo!();
}
