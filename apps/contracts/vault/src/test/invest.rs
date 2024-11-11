use soroban_sdk::{vec as sorobanvec, String, Vec, Map, vec};

use crate::test::defindex_vault::{
    AssetStrategySet, 
    AssetInvestmentAllocation, 
    StrategyInvestment, 
    Strategy,
    ContractError};
use crate::test::{
    create_strategy_params_token0, create_strategy_params_token1, DeFindexVaultTest,
};

// check that invest can only be called after initialized
#[test]
fn test_invest_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();

    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: 100,
        })],
    })];

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}


// try to invest with a wrong AssetInvestmentAllocation length
#[test]
fn test_invest_wrong_asset_investment_length() {
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

    // now will try to invest with less lengh (only one instead of 2)
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: 100,
            })],
        }) // First Asset
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongInvestmentLength)));

    // now will try to invest with more length (3 instead of 2)
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: 100,
            })
        ],
    }),
    None,
    None];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongInvestmentLength)));
}

// check that fails if strategy length is wrong
#[test]
fn test_invest_wrong_strategy_length() {
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
            strategies: sorobanvec![
                &test.env,
                Strategy {
                    name: String::from_str(&test.env, "Strategy 1"),
                    address: test.strategy_client_token1.address.clone(),
                    paused: false,
                },
                Strategy {
                    name: String::from_str(&test.env, "Strategy 2"),
                    address: test.strategy_client_token1.address.clone(),
                    paused: false,
                }
            ]
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

    // now will try to invest with more strategy length for asset 0
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: 100,
            }),
            Some(StrategyInvestment {
            strategy: test.strategy_client_token1.address.clone(),
            amount: 100,
            }),
        ],
    }),
    None];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongStrategiesLength)));

    // now will try to invest with less strategy length for asset 0
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env, // 0 instead of 1
            ],
        }),
        Some(AssetInvestmentAllocation {
            asset: test.token1.address.clone(),
            strategy_investments: vec![
                &test.env,
                Some(StrategyInvestment {
                strategy: test.strategy_client_token1.address.clone(),
                amount: 100,
                }),
                None
            ]
        })
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongStrategiesLength)));


}

// check that fails if asset address is wrong
#[test]
fn test_invest_wrong_asset_address() {
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

    // now will try to invest with wrong asset address
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token1.address.clone(), // wrong address, should be asset 0
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: 100,
            }),
        ],
    }),
    None // for asset
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongAssetAddress)));
}

// check that we cannot invest with negative amounts
#[test]
fn test_invest_negative_amount() {
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

    // now will try to invest with negative amount
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: -100, // negative amount
            }),
        ],
    }),
    None // for asset 1
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::NegativeNotAllowed)));
}

// check that we cannot invest in paused strategy. Will initialize with paused strategy and then try to invest
#[test]
fn test_invest_paused_strategy() {
    let test = DeFindexVaultTest::setup();


    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: sorobanvec![
                &test.env,
                Strategy {
                    name: String::from_str(&test.env, "Strategy 0"),
                    address: test.strategy_client_token0.address.clone(),
                    paused: true,
                }            ]
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

    // now will try to invest with some amount 
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token0.address.clone(),
        strategy_investments: vec![
            &test.env,
            Some(StrategyInvestment {
            strategy: test.strategy_client_token0.address.clone(),
            amount: 100,
            }),
        ],
    })];
    
    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = test.defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::StrategyPaused)));
}


// invest in strategy should work
#[test]
fn test_invest_in_strategy() {
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

    todo!();
   
    // let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    // let amount_0 = 987654321i128;
    // let amount_1 = 123456789i128;
    
    // test.env.mock_all_auths(); 

    // // mint amount to user 0
    // test.token0_admin_client.mint(&users[0], &amount);

    // // check user amount
    // let user_amount = test.token0.balance(&users[0]);

    // // // deposit
    // test.defindex_contract.deposit(
    //     &sorobanvec![&test.env, amount], // asset 0
    //     &sorobanvec![&test.env, amount], // asset 1 
    //     &users[0],
    // );



    // // now will try to invest with negative amount
    // let asset_investments = vec![
    //     &test.env,
    //     Some(AssetInvestmentAllocation {
    //     asset: test.token0.address.clone(),
    //     strategy_investments: vec![
    //         &test.env,
    //         Some(StrategyInvestment {
    //         strategy: test.strategy_client_token0.address.clone(),
    //         amount: amount,
    //         }),
    //     ],
    // }),
    // None // for asset 1
    // ];


    // // check balances after deposit
    // let df_balance = test.defindex_contract.balance(&users[0]);
    // assert_eq!(df_balance, amount - 1000);
    
    // let user_balance = test.token0.balance(&users[0]);
    // assert_eq!(user_balance, 0i128);

    // //map shuould be map
    // let mut expected_map = Map::new(&test.env);
    // expected_map.set(test.token0.address.clone(), amount);

    // // check that all the assets are in the vault
    // let vault_balance = test.token0.balance(&test.defindex_contract.address);
    // assert_eq!(vault_balance, amount);

    // // check that fetch_total_managed_funds returns correct amount
    // let total_managed_funds = test.defindex_contract.fetch_total_managed_funds();
    // assert_eq!(total_managed_funds, expected_map);

    // // check current idle funds, 
    // let current_idle_funds = test.defindex_contract.fetch_current_idle_funds();
    // assert_eq!(current_idle_funds, expected_map);

    // //map shuould be map
    // let mut expected_map = Map::new(&test.env);
    // expected_map.set(test.token0.address.clone(), 0i128);

    // // check that current invested funds is now 0, funds still in idle funds
    // let current_invested_funds = test.defindex_contract.fetch_current_invested_funds();
    // assert_eq!(current_invested_funds, expected_map);


    

    // let result = test.defindex_contract.invest(
    //     &asset_investments,
    // );

    
}

// check that try to invest without idle funds return error


// check if initialized vault, can only be called by manaer (todo)
#[test]
fn test_invest_initialized_only_by_manager() {
    todo!();
   
}
