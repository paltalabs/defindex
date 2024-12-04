use soroban_sdk::{vec as sorobanvec, InvokeError, Map, String, Vec};

use crate::test::defindex_vault::{AssetStrategySet, ContractError, CurrentAssetInvestmentAllocation, StrategyAllocation};
use crate::test::{
    create_strategy_params_token0, create_strategy_params_token1, DeFindexVaultTest,
};

// Test deposit not yet initialized
#[test]
fn not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, 100i128],
        &sorobanvec![&test.env, 100i128],
        &users[0],
        &false,
    );

    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}


#[test]
fn amounts_desired_less_length() {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
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
    let strategy_params_token0 = create_strategy_params_token0(&test);
    // let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, -amount, amount],
        &sorobanvec![&test.env, amount, amount],
        &users[0],
        &false,
    );

    assert_eq!(response, Err(Ok(ContractError::NegativeNotAllowed)));
}

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
        &false, 
    );

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);

    // check that all the assets are in the vault
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount);
    
    // check total manage funds

    // fn fetch_total_managed_funds(e: &Env) -> Map<Address, CurrentAssetInvestmentAllocation> {
    //     extend_instance_ttl(&e);
    //     fetch_total_managed_funds(e)
    // }
    // pub struct CurrentAssetInvestmentAllocation {
    //     pub asset: Address,
    //     pub total_amount: i128,
    //     pub idle_amount: i128,
    //     pub invested_amount: i128,
    //     pub strategy_allocations: Vec<StrategyAllocation>,
    // }
    // pub struct StrategyAllocation {
    //     pub strategy: Address,
    //     pub amount: i128,
    // }

    // construct expected total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, //funds has not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: amount,
            idle_amount: amount,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        }
    );

   

    // check that fetch_total_managed_funds returns correct amount
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds, 
    let mut expected_idle_funds_map = Map::new(&test.env);
    expected_idle_funds_map.set(test.token0.address.clone(), amount);

    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_idle_funds_map);

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
        &false,
    );

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), amount + amount2);

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount + amount2 - 1000);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, 0i128);
    
    // check that all the assets are in the vault
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount + amount2);
    
    // check that fetch_total_managed_funds returns correct amount
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: amount + amount2,
            idle_amount: amount + amount2,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        }
    );
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
    
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
fn one_asset_min_more_than_desired() {
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
    let result=test.defindex_contract.try_deposit(
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
    let deposit_result=test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
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
    assert_eq!(vault_balance0, amount0);
    // check vault balance of asset 1
    let vault_balance1 = test.token1.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance1, amount1);

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token1.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        }
    );
    total_managed_funds_expected.set(test.token1.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token1.address.clone(),
            total_amount: amount1,
            idle_amount: amount1,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        }
    );
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

     //map shuould be map
     let mut expected_map = Map::new(&test.env);
     expected_map.set(test.token0.address.clone(), amount0);
     expected_map.set(test.token1.address.clone(), amount1);
 

    // check current idle funds
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_map);
    
    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), 0i128);
    expected_map.set(test.token1.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_map);


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
    let deposit_result=test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
        &false,
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
    assert_eq!(vault_balance0, 3*amount0);
    // check vault balance of asset 1
    let vault_balance1 = test.token1.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance1, 3*amount1);

    
    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token1.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: 3*amount0,
            idle_amount: 3*amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        }
    );
    total_managed_funds_expected.set(test.token1.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token1.address.clone(),
            total_amount: 3*amount1,
            idle_amount: 3*amount1,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        }
    );
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), 3*amount0);
    expected_map.set(test.token1.address.clone(), 3*amount1);
    // check current idle funds
    let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_map);

    //map shuould be map
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token0.address.clone(), 0i128);
    expected_map.set(test.token1.address.clone(), 0i128);

    // check that current invested funds is now 0, funds still in idle funds
    let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_map);

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
    let deposit_result=test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, 0i128, 0i128],
        &users[1],
        &false,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // Vec<i128>, i128
    assert_eq!(deposit_result, (sorobanvec![&test.env, amount0*2, amount1*2], amount0*2 + amount1*2));

}

// test deposit of several asset, imposing a minimum amount greater than optimal for asset 0
#[test]
fn several_assets_min_greater_than_optimal() {
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
    let deposit_result=test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0 + 1, amount1],
        &users[0],
        &false,
    );

    // this should fail
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));
    
    // now we manage to deposit
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // check deposit result
    
    // and now will try again with minimum more than optimal

    // new user wants to do a deposit with more assets 0 than the proportion, but with minium amount 0
    // multiply amount0 by 2
    let amount0_new =  amount0*2 +100 ;
    let amount1_new = amount1*2;

    // mint this to user 
    test.token0_admin_client.mint(&users[0], &amount0_new);
    test.token1_admin_client.mint(&users[0], &amount1_new);

    let deposit_result=test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0_new, amount1_new],
        &sorobanvec![&test.env, amount0*2+1, amount1*2],
        &users[0],
        &false,
    );

    // this should fail

    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));

}

//test deposit amounts_min greater than amounts_desired
#[test]
fn amounts_min_greater_than_amounts_desired(){
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
    let deposit_result=test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0 + 1, amount1 + 1],
        &users[0],
        &false
    );

    // this should fail
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));
}

//Test token transfer from user to vault on deposit
#[test]
fn transfers_tokens_from_user_to_vault(){
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
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false
    );

    // check balances after deposit
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount0 + amount1 - 1000);

    let user_balance0 = test.token0.balance(&users[0]);
    assert_eq!(user_balance0, 0i128);
}

#[test]
fn arithmetic_error() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);

    // Initialize with 1 asset
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

    // Mock the environment to provoke a division by zero
    let mut mock_map = Map::new(&test.env);
    mock_map.set(test.token0.address.clone(), 0i128); // Total funds for token0 is 0

    let amount = 123456789i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Mint tokens to user
    test.token0_admin_client.mint(&users[0], &amount);

    let large_amount = i128::MAX / 2;
    test.token0_admin_client.mint(&users[0], &large_amount);

    //first deposit to overflow the balance
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, large_amount],
        &sorobanvec![&test.env, large_amount],
        &users[0],
        &false
    );
    
    // Try to deposit a large amount
    let result = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, large_amount],
        &sorobanvec![&test.env, large_amount],
        &users[0],
        &false
    );

    // Verify that the returned error is ContractError::ArithmeticError
    assert_eq!(result, Err(Ok(ContractError::ArithmeticError)));
}

//all amounts are cero
#[test]
fn amounts_desired_zero() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);

    // Initialize with 1 asset
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

    // Mint tokens to user
    test.token0_admin_client.mint(&users[0], &amount);

    // Balances before deposit
    let user_balance_before = test.token0.balance(&users[0]);
    assert_eq!(user_balance_before, amount);

    let vault_balance_before = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance_before, 0i128);

    let df_balance_before = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance_before, 0i128);

    // Attempt to deposit with amounts_desired all set to 0
    let deposit_result = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, 0i128],
        &sorobanvec![&test.env, 0i128],
        &users[0],
        &false
    );

    // Verify that the returned error is ContractError::InsufficientAmount
    assert_eq!(deposit_result, Err(Ok(ContractError::InsufficientAmount)));
}


    // Deposit with insufficient funds and check for specific error message
#[test]
fn insufficient_funds_with_error_message() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // Initialize with 2 assets
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    // Mint tokens to user
    test.token0_admin_client.mint(&users[0], &amount0);
    test.token1_admin_client.mint(&users[0], &amount1);

    // Balances before deposit
    let user_balance0 = test.token0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Attempt to deposit more than available balance
    let deposit_result = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount0 + 1, amount1 + 1],
        &sorobanvec![&test.env, amount0 + 1, amount1 + 1],
        &users[0],
        &false
    );

    if deposit_result == Err(Err(InvokeError::Contract(10))) {
        return;
    } else {
        panic!("Expected error not returned");
    }

}