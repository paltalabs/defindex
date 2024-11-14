use crate::{setup::create_vault_one_asset_hodl_strategy, test::{IntegrationTest, ONE_YEAR_IN_SECONDS}, vault::{defindex_vault_contract::{AssetInvestmentAllocation, StrategyInvestment}, VaultContractError, MINIMUM_LIQUIDITY}};
use soroban_sdk::{testutils::{Ledger, MockAuth, MockAuthInvoke, Address as _}, vec as svec, Address, IntoVal, Vec};

#[test]
fn test_invest_success() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    // Create investment strategies for the deposited tokens
    let investments = svec![
        &setup.env,
        Some(AssetInvestmentAllocation {
            asset: enviroment.token.address.clone(),
            strategy_investments: svec![
                &setup.env,
                Some(StrategyInvestment {
                    strategy: enviroment.strategy_contract.address.clone(),
                    amount: deposit_amount,
                }),
            ],
        }),
    ];

    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "invest",
            args: (
                Vec::from_array(&setup.env,[
                    Some(
                        AssetInvestmentAllocation {
                            asset: enviroment.token.address.clone(),
                            strategy_investments:
                                svec![&setup.env,
                                    Some(StrategyInvestment {
                                        strategy: enviroment.strategy_contract.address.clone(),
                                        amount: deposit_amount,
                                    })
                                ]
                        }
                    )
                ]),
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .invest(&investments);

    setup.env.ledger().set_timestamp(setup.env.ledger().timestamp() + ONE_YEAR_IN_SECONDS);

    let token_balance_after_invest = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_invest, 0);

    let strategy_balance = enviroment.strategy_contract.balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, deposit_amount);
}

#[test]
fn test_invest_exceeding_investing_lenght() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let asset_address_2 = Address::generate(&setup.env);
    let strategy_address_2 = Address::generate(&setup.env);
    // Create investment strategies exceeding the allowed number
    let investments = svec![
        &setup.env,
        Some(AssetInvestmentAllocation {
            asset: enviroment.token.address.clone(),
            strategy_investments: svec![
                &setup.env,
                Some(StrategyInvestment {
                    strategy: enviroment.strategy_contract.address.clone(),
                    amount: deposit_amount,
                }),
            ],
        }),
        Some(AssetInvestmentAllocation {
            asset: asset_address_2.clone(),
            strategy_investments: svec![
                &setup.env,
                Some(StrategyInvestment {
                    strategy: strategy_address_2.clone(),
                    amount: deposit_amount,
                }),
            ],
        }),
    ];

    let result = enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "invest",
            args: (
                Vec::from_array(&setup.env,[
                    Some(
                        AssetInvestmentAllocation {
                            asset: enviroment.token.address.clone(),
                            strategy_investments:
                                svec![&setup.env,
                                    Some(StrategyInvestment {
                                        strategy: enviroment.strategy_contract.address.clone(),
                                        amount: deposit_amount,
                                    })
                                ]
                        }
                    ),
                    Some(
                        AssetInvestmentAllocation {
                            asset: asset_address_2.clone(),
                            strategy_investments:
                                svec![&setup.env,
                                    Some(StrategyInvestment {
                                        strategy: strategy_address_2.clone(),
                                        amount: deposit_amount,
                                    })
                                ]
                        }
                    )
                ]),
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .try_invest(&investments);

    assert_eq!(result, Err(Ok(VaultContractError::WrongInvestmentLength)));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")]
fn test_invest_insufficient_balance() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 5_000_0_000_000i128; // Less than deposit amount

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user);
}

#[test]
fn test_invest_multiple_users() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 2);
    let user1 = &users[0];
    let user2 = &users[1];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user1, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user1, &user_starting_balance);
    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user2, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user2, &user_starting_balance);

    let user1_balance = enviroment.token.balance(user1);
    let user2_balance = enviroment.token.balance(user2);
    assert_eq!(user1_balance, user_starting_balance);
    assert_eq!(user2_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user1.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user1.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user1.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user1);

    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user2.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user2.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user2.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user2);

    let df_balance_user1 = enviroment.vault_contract.balance(&user1);
    let df_balance_user2 = enviroment.vault_contract.balance(&user2);
    assert_eq!(df_balance_user1, deposit_amount - MINIMUM_LIQUIDITY);
    assert_eq!(df_balance_user2, deposit_amount);

    // Create investment strategies for the deposited tokens
    let investments = svec![
        &setup.env,
        Some(AssetInvestmentAllocation {
            asset: enviroment.token.address.clone(),
            strategy_investments: svec![
                &setup.env,
                Some(StrategyInvestment {
                    strategy: enviroment.strategy_contract.address.clone(),
                    amount: deposit_amount*2,
                }),
            ],
        }),
    ];

    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "invest",
            args: (
                Vec::from_array(&setup.env,[
                    Some(
                        AssetInvestmentAllocation {
                            asset: enviroment.token.address.clone(),
                            strategy_investments:
                                svec![&setup.env,
                                    Some(StrategyInvestment {
                                        strategy: enviroment.strategy_contract.address.clone(),
                                        amount: deposit_amount*2,
                                    })
                                ]
                        }
                    )
                ]),
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .invest(&investments);

    setup.env.ledger().set_timestamp(setup.env.ledger().timestamp() + ONE_YEAR_IN_SECONDS);

    let token_balance_after_invest = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_invest, 0);

    let strategy_balance = enviroment.strategy_contract.balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, deposit_amount * 2);
}