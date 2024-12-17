use soroban_sdk::{vec as sorobanvec, String, Vec, Map, vec};
use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    IntoVal,
};

use crate::test::defindex_vault::{
    CurrentAssetInvestmentAllocation,
    AssetStrategySet, 
    AssetInvestmentAllocation,  
    StrategyAllocation, 
    Strategy,
    ContractError};
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1, DeFindexVaultTest
};


// try to invest with a wrong AssetInvestmentAllocation length
#[test]
fn wrong_asset_investment_length() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    // now will try to invest with less lengh (only one instead of 2)
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 100,
            })],
        }) // First Asset
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongInvestmentLength)));

    // now will try to invest with more length (3 instead of 2)
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 100,
            })
        ],
    }),
    None,
    None];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongInvestmentLength)));
}

// check that fails if strategy length is wrong
#[test]
fn wrong_strategy_length() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    // let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: sorobanvec![
                &test.env,
                Strategy {
                    name: String::from_str(&test.env, "Strategy 1"),
                    address: test.strategy_client_token_1.address.clone(),
                    paused: false,
                },
                Strategy {
                    name: String::from_str(&test.env, "Strategy 2"),
                    address: test.strategy_client_token_1.address.clone(),
                    paused: false,
                }
            ]
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

    // now will try to invest with more strategy length for asset 0
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 100,
            }),
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 100,
            }),
        ],
    }),
    None];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongStrategiesLength)));

    // now will try to invest with less strategy length for asset 0
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env, // 0 instead of 1
            ],
        }),
        Some(AssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            strategy_allocations: vec![
                &test.env,
                Some(StrategyAllocation {
                strategy_address: test.strategy_client_token_1.address.clone(),
                amount: 100,
                }),
                None
            ]
        })
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongStrategiesLength)));


}

// check that fails if asset address is wrong
#[test]
fn wrong_asset_address() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    // now will try to invest with wrong asset address
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_1.address.clone(), // wrong address, should be asset 0
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 100,
            }),
        ],
    }),
    None // for asset
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::WrongAssetAddress)));
}

// check that we cannot invest with negative amounts
#[test]
fn negative_amount() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    // now will try to invest with negative amount
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: -100, // negative amount
            }),
        ],
    }),
    None // for asset 1
    ];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::NegativeNotAllowed)));
}

// check that we cannot invest in paused strategy. Will initialize with paused strategy and then try to invest
#[test]
fn paused_strategy() {
    let test = DeFindexVaultTest::setup();


    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: sorobanvec![
                &test.env,
                Strategy {
                    name: String::from_str(&test.env, "Strategy 0"),
                    address: test.strategy_client_token_0.address.clone(),
                    paused: true,
                }            ]
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

    // now will try to invest with some amount 
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 100,
            }),
        ],
    })];
    
    test.env.mock_all_auths(); // TODO, Mock only Manager

    let result = defindex_contract.try_invest(
        &asset_investments,
    );

    assert_eq!(result, Err(Ok(ContractError::StrategyPaused)));
}


// invest in strategy should work
#[test]
fn in_strategy() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    let amount_0 = 987654321i128;
    let amount_1 = 123456789i128;
    
    test.env.mock_all_auths(); 

    // mint
    test.token_0_admin_client.mint(&users[0], &amount_0);
    test.token_1_admin_client.mint(&users[0], &amount_1);

    // check user amount
    let user_amount_0 = test.token_0.balance(&users[0]);
    let user_amount_1 = test.token_1.balance(&users[0]);
    
    assert_eq!(user_amount_0, amount_0);
    assert_eq!(user_amount_1, amount_1);
    

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_0, amount_1], // asset 0
        &sorobanvec![&test.env, amount_0, amount_1], // asset 1 
        &users[0],
        &false,
    );


    // check balances after deposit
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_0 + amount_1 - 1000);

    // check that all the assets are in the vault
    let vault_balance_0 = test.token_0.balance(&defindex_contract.address);
    let vault_balance_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance_0, amount_0);
    assert_eq!(vault_balance_1, amount_1);
    
    
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_0.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_1.address.clone(),
        amount: 0, // funds have not been invested yet!
    }];
    total_managed_funds_expected.set(test.token_0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        total_amount: amount_0,
        idle_amount: amount_0,
        invested_amount: 0i128,
        strategy_allocations: strategy_investments_expected_token_0,
    });
    total_managed_funds_expected.set(test.token_1.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        total_amount: amount_1,
        idle_amount: amount_1,
        invested_amount: 0i128,
        strategy_allocations: strategy_investments_expected_token_1,
    });
    
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    
    
    // check current idle funds,
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), amount_0);
    expected_map.set(test.token_1.address.clone(), amount_1);
    let current_idle_funds = defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_map);

    // check that current invested funds is now 0, funds still in idle funds
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 0i128);
    expected_map.set(test.token_1.address.clone(), 0i128);
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_map);


    // Prepare investments object
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 100,
            }),
        ],
    }),
    Some(AssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 200,
            }),
        ],
    })];

    defindex_contract.invest(
        &asset_investments,
    );

    // now only amunt_0 - 100 should be in the vault as idle funds
    let vault_balance_0 = test.token_0.balance(&defindex_contract.address);
    let vault_balance_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance_0, amount_0 - 100);
    assert_eq!(vault_balance_1, amount_1 - 200);

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_0.address.clone(),
        amount: 100,
    }];
    let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
        strategy_address: test.strategy_client_token_1.address.clone(),
        amount: 200,
    }];
    total_managed_funds_expected.set(test.token_0.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        total_amount: amount_0,
        idle_amount: amount_0 - 100,
        invested_amount: 100i128,
        strategy_allocations: strategy_investments_expected_token_0,
    });
    total_managed_funds_expected.set(test.token_1.address.clone(),
    CurrentAssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        total_amount: amount_1,
        idle_amount: amount_1 - 200,
        invested_amount: 200i128,
        strategy_allocations: strategy_investments_expected_token_1,
    });

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // check current idle funds, for token_0 should be amount 0 - 100
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), amount_0 - 100);
    expected_map.set(test.token_1.address.clone(), amount_1 - 200);

    let current_idle_funds = defindex_contract.fetch_current_idle_funds();
    assert_eq!(current_idle_funds, expected_map);

    // check that current invested funds is now 100 and 200
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 100i128);
    expected_map.set(test.token_1.address.clone(), 200i128);

    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    assert_eq!(current_invested_funds, expected_map);

    // check that 100 and 200 are invested in the strategies
    let strategy_0_balance = test.strategy_client_token_0.balance(&defindex_contract.address);
    let strategy_1_balance = test.strategy_client_token_1.balance(&defindex_contract.address);
    assert_eq!(strategy_0_balance, 100);
    assert_eq!(strategy_1_balance, 200);

    // if we ask strategy.balance(vault) they should be 100 and 200
    let strategy_0_balance = test.strategy_client_token_0.balance(&defindex_contract.address);
    let strategy_1_balance = test.strategy_client_token_1.balance(&defindex_contract.address);
    assert_eq!(strategy_0_balance, 100);
    assert_eq!(strategy_1_balance, 200);


    
}

// check that try to invest more than idle funds return balance error
#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")] // balance is not sufficient to spend
// we get the error from the token contract
fn more_than_idle_funds() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    let amount_0 = 987654321i128;
    let amount_1 = 123456789i128;
    
    test.env.mock_all_auths(); 

    // mint
    test.token_0_admin_client.mint(&users[0], &amount_0);
    test.token_1_admin_client.mint(&users[0], &amount_1);

    // check user amount
    let user_amount_0 = test.token_0.balance(&users[0]);
    let user_amount_1 = test.token_1.balance(&users[0]);
    
    assert_eq!(user_amount_0, amount_0);
    assert_eq!(user_amount_1, amount_1);
    

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_0, amount_1], // asset 0
        &sorobanvec![&test.env, amount_0, amount_1], // asset 1 
        &users[0],
        &false,
    );

    // check vault balances
    let vault_balance_0 = test.token_0.balance(&defindex_contract.address);
    let vault_balance_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance_0, amount_0);

    // try to invest vault_balance_0 + 1 for asset 0
    let asset_investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: vault_balance_0 + 1,
            }),
        ],
    }),
    Some(AssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        strategy_allocations: vec![
            &test.env,
            Some(StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: vault_balance_1 + 1,
            }),
        ],
    })];

    test.env.mock_all_auths(); // TODO, Mock only Manager

    defindex_contract.invest(
        &asset_investments,
    );
}

// invest without mock aut, mocking only specific auths
#[test]
fn without_mock_all_auths() {
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    let amount_0 = 987654321i128;
    let amount_1 = 123456789i128;
    

    // mock admin auth and mint
    test.token_0_admin_client.mock_auths(&[MockAuth {
        address: &test.token_0_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &test.token_0.address.clone(),
            fn_name: "mint",
            args: (users[0].clone(), amount_0).into_val(&test.env),
            sub_invokes: &[],
        },
    }]).mint(&users[0], &amount_0);


    test.token_1_admin_client.mock_auths(&[MockAuth {
        address: &test.token_1_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &test.token_1.address.clone(),
            fn_name: "mint",
            args: (users[0].clone(), amount_1).into_val(&test.env),
            sub_invokes: &[],
        },
    }]).mint(&users[0], &amount_1);

    // check user amount
    let user_amount_0 = test.token_0.balance(&users[0]);
    let user_amount_1 = test.token_1.balance(&users[0]);
    
    assert_eq!(user_amount_0, amount_0);
    assert_eq!(user_amount_1, amount_1);
    

    // mock deposit auth from user and deposit

    defindex_contract.mock_auths(&[MockAuth {
        address: &users[0].clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&test.env,[amount_0, amount_1]),
                Vec::from_array(&test.env,[amount_0, amount_1]),
                users[0].clone(),
                false
            ).into_val(&test.env),
            // mock toke 0 and token 1 subtransfer
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &test.token_0.address.clone(),
                    fn_name: "transfer",
                    args: (
                        users[0].clone(), 
                        defindex_contract.address.clone(),
                        amount_0
                     ).into_val(&test.env),
                    sub_invokes: &[]
                },
                MockAuthInvoke {
                    contract: &test.token_1.address.clone(),
                    fn_name: "transfer",
                    args: (
                        users[0].clone(), 
                        defindex_contract.address.clone(),
                        amount_1
                     ).into_val(&test.env),
                    sub_invokes: &[]
                }]
        },
    }]).deposit(
        &sorobanvec![&test.env, amount_0, amount_1], // asset 0
        &sorobanvec![&test.env, amount_0, amount_1], // asset 1 
        &users[0],
        &false,
    );

    // TODO check that the blockchain saw this authorizations

    // check vault balances
    let vault_balance_0 = test.token_0.balance(&defindex_contract.address);
    let vault_balance_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance_0, amount_0);
    assert_eq!(vault_balance_1, amount_1);

    // mock auth from manager to invest and invest
    defindex_contract.mock_auths(&[MockAuth {
        address: &test.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "invest",
            args: (
                Vec::from_array(&test.env,[
                    Some(
                        AssetInvestmentAllocation {
                            asset: test.token_0.address.clone(),
                            strategy_allocations:
                                sorobanvec![&test.env,
                                    Some(StrategyAllocation {
                                        strategy_address: test.strategy_client_token_0.address.clone(),
                                        amount: 100,
                                    })]
                        }
                    ),
                    Some(
                        AssetInvestmentAllocation {
                            asset: test.token_1.address.clone(),
                            strategy_allocations: sorobanvec![&test.env,
                                Some(StrategyAllocation {
                                    strategy_address: test.strategy_client_token_1.address.clone(),
                                    amount: 200,
                                })]
                        }
                    )]),
            ).into_val(&test.env),
            sub_invokes: &[]
        },
    }]).invest(
        &sorobanvec![
            &test.env, 
            Some(
                AssetInvestmentAllocation {
                    asset: test.token_0.address.clone(),
                    strategy_allocations: sorobanvec![&test.env,
                        Some(StrategyAllocation {
                            strategy_address: test.strategy_client_token_0.address.clone(),
                            amount: 100,
                        })]
                }
            ), 
            Some(
                AssetInvestmentAllocation {
                    asset: test.token_1.address.clone(),
                    strategy_allocations: sorobanvec![&test.env,
                        Some(StrategyAllocation {
                            strategy_address: test.strategy_client_token_1.address.clone(),
                            amount: 200,
                        })]
                }
            )]
    );

    // check that now vault has amount0 -100 in token 0, and amount1 -200 in token 1
    let vault_balance_0 = test.token_0.balance(&defindex_contract.address);
    let vault_balance_1 = test.token_1.balance(&defindex_contract.address);
    assert_eq!(vault_balance_0, amount_0 - 100);
    assert_eq!(vault_balance_1, amount_1 - 200);

    // check invested funds
    let current_invested_funds = defindex_contract.fetch_current_invested_funds();
    let mut expected_map = Map::new(&test.env);
    expected_map.set(test.token_0.address.clone(), 100i128);
    expected_map.set(test.token_1.address.clone(), 200i128);
    assert_eq!(current_invested_funds, expected_map);
    

}

#[test]
fn one_asset_several_strategies() { 

}

