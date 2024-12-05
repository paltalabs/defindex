use soroban_sdk::{vec as sorobanvec, String, Vec, Map};

// use super::hodl_strategy::StrategyError;
use crate::test::{
    create_strategy_params_token0,
    create_strategy_params_token1,
    defindex_vault::{
        AssetStrategySet,
        AssetInvestmentAllocation,  
        StrategyAllocation,  
        Strategy,
        ContractError, CurrentAssetInvestmentAllocation
    },
    DeFindexVaultTest,
};


#[test]
fn not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}

// check that withdraw with negative amount after initialized returns error
#[test]
fn negative_amount() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&-100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::NegativeNotAllowed)));
}


// check that withdraw without balance after initialized returns error AmountOverTotalSupply
#[test]
fn zero_total_supply() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::AmountOverTotalSupply)));
}

// check that withdraw with not enough balance returns error InsufficientBalance
#[test]
fn not_enough_balance() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
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

    // We need to generate 2 users, to have more total supply than the amount to withdraw
    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    let amount_to_deposit = 567890i128;
    test.token0_admin_client.mint(&users[0], &amount_to_deposit);
    test.token0_admin_client.mint(&users[1], &amount_to_deposit);
    
    assert_eq!(test.token0.balance(&users[0]), amount_to_deposit);
    assert_eq!(test.token0.balance(&users[1]), amount_to_deposit);

    // first the user deposits
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[0],
        &false
    );

    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[1],
        &false
    );

    // check that the every user has vault shares
    assert_eq!(test.defindex_contract.balance(&users[0]), amount_to_deposit - 1000);
    assert_eq!(test.defindex_contract.balance(&users[1]), amount_to_deposit);
    // check that total supply of vault shares is indeed amount_to_deposit*2
    assert_eq!(test.defindex_contract.total_supply(), amount_to_deposit*2);
    
    // now user 0 tries to withdraw amount_to_deposit - 1000 +1 (more that it has)

    let result = test.defindex_contract.try_withdraw(&(amount_to_deposit - 1000 +1), &users[0]);
    assert_eq!(result, Err(Ok(ContractError::InsufficientBalance)));
}

#[test]
fn from_idle_one_strategy_success() { 
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
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
    let amount = 1234567890i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    let amount_to_deposit = 567890i128;
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[0],
        &false
    );

    // Check Balances after deposit

    // Token balance of user
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount - amount_to_deposit);

    // Token balance of vault should be amount_to_deposit
    // Because balances are still in indle, balances are not in strategy, but in idle

    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount_to_deposit);

    // Token balance of hodl strategy should be 0 (all in idle)
    let strategy_balance = test.token0.balance(&test.strategy_client_token0.address);
    assert_eq!(strategy_balance, 0);

    // Df balance of user should be equal to deposited amount - 1000
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit - 1000 ); // 1000  gets locked in the vault forever

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, //funds has not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: amount_to_deposit,
            idle_amount: amount_to_deposit,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        }
    );

    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);


    // user decides to withdraw a portion of deposited amount
    let amount_to_withdraw = 123456i128;
    test.defindex_contract
        .withdraw(&amount_to_withdraw, &users[0]);

    // Check Balances after withdraw

    // Token balance of user should be amount - amount_to_deposit + amount_to_withdraw
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(
        user_balance,
        amount - amount_to_deposit + amount_to_withdraw
    );

    // Token balance of vault should be amount_to_deposit - amount_to_withdraw
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount_to_deposit - amount_to_withdraw);

    // Token balance of hodl strategy should be 0 (all in idle)
    let strategy_balance = test.token0.balance(&test.strategy_client_token0.address);
    assert_eq!(strategy_balance, 0);

    // Df balance of user should be equal to deposited amount - amount_to_withdraw - 1000 
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit - amount_to_withdraw - 1000);

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, //funds has not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: amount_to_deposit - amount_to_withdraw,
            idle_amount: amount_to_deposit - amount_to_withdraw,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        }
    );

    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);


    // user tries to withdraw more than deposited amount
    let amount_to_withdraw_more = amount_to_deposit + 1;
    let result = test
        .defindex_contract
        .try_withdraw(&amount_to_withdraw_more, &users[0]);
    
    assert_eq!(result, 
        Err(Ok(ContractError::AmountOverTotalSupply)));


    // // withdraw remaining balance
   let result= test.defindex_contract
        .withdraw(&(amount_to_deposit - amount_to_withdraw - 1000), &users[0]);

    assert_eq!(result, sorobanvec![&test.env, amount_to_deposit - amount_to_withdraw - 1000]);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token0.address.clone(),
        amount: 0, //funds has not been invested yet!
    }];
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: 1000,
            idle_amount: 1000,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        }
    );

    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

}

#[test]
fn from_idle_two_assets_success() { 
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
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
    let amount = 1234567890i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    test.token1_admin_client.mint(&users[0], &amount);
    assert_eq!(test.token0.balance(&users[0]), amount);
    assert_eq!(test.token0.balance(&users[0]), amount);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    let amount_to_deposit_0 = 567890i128;
    let amount_to_deposit_1 = 987654i128;
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
        &sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
        &users[0],
        &false
    );

    // Check Balances after deposit

    // Token balance of user
    assert_eq!(test.token0.balance(&users[0]), amount - amount_to_deposit_0);
    assert_eq!(test.token1.balance(&users[0]), amount - amount_to_deposit_1);

    // Token balance of vault should be amount_to_deposit
    // Because balances are still in indle, balances are not in strategy, but in idle

    assert_eq!(test.token0.balance(&test.defindex_contract.address), amount_to_deposit_0);
    assert_eq!(test.token1.balance(&test.defindex_contract.address), amount_to_deposit_1);

    // Token balance of hodl strategy should be 0 (all in idle)
    assert_eq!(test.token0.balance(&test.strategy_client_token0.address), 0);
    assert_eq!(test.token1.balance(&test.strategy_client_token1.address), 0);

    // Df balance of user should be equal to amount_to_deposit_0+amount_to_deposit_1 - 1000
    // 567890+987654-1000 = 1554544
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 1554544 );

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: 567890i128,
            idle_amount: 567890i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![&test.env, 
            StrategyAllocation {
                strategy_address: test.strategy_client_token0.address.clone(),
                amount: 0, //funds has not been invested yet!
            }],
        }
    );

    total_managed_funds_expected.set(test.token1.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token1.address.clone(),
            total_amount: 987654i128,
            idle_amount: 987654i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![&test.env, 
            StrategyAllocation {
                strategy_address: test.strategy_client_token1.address.clone(),
                amount: 0, //funds has not been invested yet!
            }],
        }
    );
    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // user decides to withdraw a portion of their vault shares
    // from 1554544 it will withdraw 123456.
    // total shares = 567890+987654 = 1555544
    // asset 0 = withdaw_shares*total_asset_0/total_shares = 123456*567890/1555544 = 45070.681279347 = 45070
    // asset 1 = withdaw_shares*total_asset_1/total_shares = 123456*987654/1555544 = 78385.318720653 = 78385

    let amount_to_withdraw = 123456i128;
    let result = test.defindex_contract
    .withdraw(&amount_to_withdraw, &users[0]);

    // expected asset vec Vec<AssetStrategySet>
    // pub struct AssetStrategySet {
    //     pub address: Address,
    //     pub strategies: Vec<Strategy>,
    // }
    // pub struct Strategy {
    //     pub address: Address,
    //     pub name: String,
    //     pub paused: bool,
    // }
    let expected_asset_vec = sorobanvec![&test.env, AssetStrategySet {
        address: test.token0.address.clone(),
        strategies: sorobanvec![&test.env, Strategy {
            address: test.strategy_client_token0.address.clone(),
            name: String::from_str(&test.env, "Strategy 1"),
            paused: false,
        }],
    }, AssetStrategySet {
        address: test.token1.address.clone(),
        strategies: sorobanvec![&test.env, Strategy {
            address: test.strategy_client_token1.address.clone(),
            name: String::from_str(&test.env, "Strategy 1"),
            paused: false,
        }],
    }];
    assert_eq!(test.defindex_contract.get_assets(), expected_asset_vec);
    let expected_result = sorobanvec![&test.env, 45070, 78385];
    assert_eq!(result, expected_result);

    // Token balance of user
    assert_eq!(test.token0.balance(&users[0]), amount - amount_to_deposit_0 + 45070);
    assert_eq!(test.token1.balance(&users[0]), amount - amount_to_deposit_1 + 78385);

    // Token balance of vault (still idle)

    assert_eq!(test.token0.balance(&test.defindex_contract.address), amount_to_deposit_0 - 45070);
    assert_eq!(test.token1.balance(&test.defindex_contract.address), amount_to_deposit_1 - 78385);

    // Token balance of hodl strategy should be 0 (all in idle)
    assert_eq!(test.token0.balance(&test.strategy_client_token0.address), 0);
    assert_eq!(test.token1.balance(&test.strategy_client_token1.address), 0);

    // Df balance of user should be equal to amount_to_deposit_0+amount_to_deposit_1 - 1000 - 123456
    // 567890+987654-1000 -123456 = 1434088
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 1431088 );

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    total_managed_funds_expected.set(test.token0.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            total_amount: 567890i128 - 45070,
            idle_amount: 567890i128 - 45070,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![&test.env, 
            StrategyAllocation {
                strategy_address: test.strategy_client_token0.address.clone(),
                amount: 0, //funds has not been invested yet!
            }],
        }
    );

    total_managed_funds_expected.set(test.token1.address.clone(), 
        CurrentAssetInvestmentAllocation {
            asset: test.token1.address.clone(),
            total_amount: 987654i128 - 78385,
            idle_amount: 987654i128 - 78385,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![&test.env, 
            StrategyAllocation {
                strategy_address: test.strategy_client_token1.address.clone(),
                amount: 0, //funds has not been invested yet!
            }],
        }
    );

    let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

}

#[test]
fn withdraw_from_strategy_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
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

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];


    test.defindex_contract.invest(&investments);

    let vault_balance = test.token0.balance(&test.defindex_contract.address);
     assert_eq!(vault_balance, 0);

    test.defindex_contract.withdraw(&df_balance, &users[0]);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);
}
 

// test withdraw without mock all auths
#[test]
fn from_strategy_success_no_mock_all_auths() {
    todo!();
}