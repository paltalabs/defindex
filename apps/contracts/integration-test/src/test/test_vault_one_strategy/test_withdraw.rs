use crate::{setup::{create_vault_one_asset_hodl_strategy, VAULT_FEE}, test::{IntegrationTest, DEFINDEX_FEE, ONE_YEAR_IN_SECONDS}, vault::{defindex_vault_contract::{AssetInvestmentAllocation, StrategyInvestment}, VaultContractError, MINIMUM_LIQUIDITY}};
use soroban_sdk::{testutils::{Ledger, MockAuth, MockAuthInvoke}, vec as svec, IntoVal, Vec};

#[test]
fn test_withdraw_no_invest_success() {
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

    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                df_balance.clone(),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .withdraw(&df_balance, &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, MINIMUM_LIQUIDITY);

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, user_starting_balance - MINIMUM_LIQUIDITY);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);

    let total_supply = enviroment.vault_contract.total_supply();
    assert_eq!(total_supply, MINIMUM_LIQUIDITY);
}

#[test]
fn test_withdraw_partial_success() {
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

    let withdraw_amount = df_balance / 2;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                withdraw_amount.clone(),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .withdraw(&withdraw_amount, &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, withdraw_amount);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, deposit_amount - withdraw_amount);

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, user_starting_balance - (deposit_amount - withdraw_amount));
}

#[test]
fn test_withdraw_insufficient_balance() {
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

    let withdraw_amount = df_balance + 1; // Attempt to withdraw more than the balance
    let result = enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                withdraw_amount.clone(),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .try_withdraw(&withdraw_amount, &user);
    assert_eq!(result, Err(Ok(VaultContractError::InsufficientBalance)));

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, deposit_amount);

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, user_starting_balance - deposit_amount);
}

#[test]
fn test_withdraw_after_invest() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 10_000_0_000_000i128;

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

    let user_balance_after_deposit = enviroment.token.balance(user);
    assert_eq!(user_balance_after_deposit, 0);

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
    
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                df_balance.clone(),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .withdraw(&df_balance, &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0); 

    let token_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_withdraw, 0);

    let charged_fee = (deposit_amount - MINIMUM_LIQUIDITY) * (DEFINDEX_FEE as i128 + VAULT_FEE as i128) / 10000;
    let expected_amount = deposit_amount - MINIMUM_LIQUIDITY - charged_fee;

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, expected_amount);

    let strategy_balance = enviroment.strategy_contract.balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, charged_fee + MINIMUM_LIQUIDITY);   

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, 0);
}