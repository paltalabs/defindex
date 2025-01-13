use crate::{
    setup::create_vault_one_asset_hodl_strategy,
    test::{IntegrationTest, ONE_YEAR_IN_SECONDS},
    vault::{defindex_vault_contract::Instruction, VaultContractError, MINIMUM_LIQUIDITY},
};
use soroban_sdk::{
    testutils::{Ledger, MockAuth, MockAuthInvoke},
    vec as svec, IntoVal, Vec,
};

#[test]
fn test_withdraw_no_invest_success() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user,
            &false,
        );

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance.clone(), user.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance, &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, MINIMUM_LIQUIDITY);

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(
        user_balance_after_withdraw,
        user_starting_balance - MINIMUM_LIQUIDITY
    );

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

    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user,
            &false,
        );

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let withdraw_amount = df_balance / 2;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (withdraw_amount.clone(), user.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&withdraw_amount, &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, withdraw_amount);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(
        vault_balance_after_withdraw,
        deposit_amount - withdraw_amount
    );

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(
        user_balance_after_withdraw,
        user_starting_balance - (deposit_amount - withdraw_amount)
    );
}

#[test]
fn test_withdraw_insufficient_balance() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user,
            &false,
        );

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let withdraw_amount = df_balance + 1; // Attempt to withdraw more than the balance
    let result = enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (withdraw_amount.clone(), user.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .try_withdraw(&withdraw_amount, &user);
    assert_eq!(result, Err(Ok(VaultContractError::InsufficientBalance)));

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, deposit_amount);

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(
        user_balance_after_withdraw,
        user_starting_balance - deposit_amount
    );
}

#[test]
fn test_withdraw_after_invest() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 10_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user,
            &false,
        );

    let user_balance_after_deposit = enviroment.token.balance(user);
    assert_eq!(user_balance_after_deposit, 0);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);


    let invest_instructions = svec![
        &setup.env,
        Instruction::Invest(
            enviroment.strategy_contract.address.clone(),
            deposit_amount,
        ),
    ];

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "rebalance",
                args: (enviroment.manager.clone(), invest_instructions.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .rebalance(&enviroment.manager, &invest_instructions);

    setup
        .env
        .ledger()
        .set_timestamp(setup.env.ledger().timestamp() + ONE_YEAR_IN_SECONDS);

    let token_balance_after_invest = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_invest, 0);

    let strategy_balance = enviroment
        .strategy_contract
        .balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, deposit_amount);

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance.clone(), user.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance, &user);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);

    let token_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_withdraw, 0);

    let expected_amount = deposit_amount - MINIMUM_LIQUIDITY;

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, expected_amount);

    let strategy_balance = enviroment
        .strategy_contract
        .balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, MINIMUM_LIQUIDITY);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, 0);
}

#[test]
fn test_withdraw_multiple_users() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 2);
    let user1 = &users[0];
    let user2 = &users[1];

    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user1, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user1, &user_starting_balance);
    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user2, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user2, &user_starting_balance);

    let user1_balance = enviroment.token.balance(user1);
    let user2_balance = enviroment.token.balance(user2);
    assert_eq!(user1_balance, user_starting_balance);
    assert_eq!(user2_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user1.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user1.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user1.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user1,
            &false,
        );

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user2.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user2.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user2.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user2,
            &false,
        );

    let df_balance_user1 = enviroment.vault_contract.balance(&user1);
    let df_balance_user2 = enviroment.vault_contract.balance(&user2);
    assert_eq!(df_balance_user1, deposit_amount - MINIMUM_LIQUIDITY);
    assert_eq!(df_balance_user2, deposit_amount);

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user1.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance_user1.clone(), user1.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance_user1, &user1);

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user2.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance_user2.clone(), user2.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance_user2, &user2);

    let df_balance_user1 = enviroment.vault_contract.balance(&user1);
    let df_balance_user2 = enviroment.vault_contract.balance(&user2);
    assert_eq!(df_balance_user1, 0);
    assert_eq!(df_balance_user2, 0);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, MINIMUM_LIQUIDITY);

    let user1_balance_after_withdraw = enviroment.token.balance(user1);
    let user2_balance_after_withdraw = enviroment.token.balance(user2);
    assert_eq!(
        user1_balance_after_withdraw,
        user_starting_balance - MINIMUM_LIQUIDITY
    );
    assert_eq!(user2_balance_after_withdraw, user_starting_balance);

    let total_supply = enviroment.vault_contract.total_supply();
    assert_eq!(total_supply, MINIMUM_LIQUIDITY);
}

#[test]
fn test_withdraw_after_invest_multiple_users() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 10_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 2);
    let user1 = &users[0];
    let user2 = &users[1];

    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user1, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user1, &user_starting_balance);
    enviroment
        .token_admin_client
        .mock_auths(&[MockAuth {
            address: &enviroment.token_admin.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.token.address.clone(),
                fn_name: "mint",
                args: (user2, user_starting_balance).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .mint(user2, &user_starting_balance);

    let user1_balance = enviroment.token.balance(user1);
    let user2_balance = enviroment.token.balance(user2);
    assert_eq!(user1_balance, user_starting_balance);
    assert_eq!(user2_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user1.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user1.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user1.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user1,
            &false,
        );

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user2.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "deposit",
                args: (
                    Vec::from_array(&setup.env, [deposit_amount]),
                    Vec::from_array(&setup.env, [deposit_amount]),
                    user2.clone(),
                    false,
                )
                    .into_val(&setup.env),
                sub_invokes: &[MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user2.clone(),
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount,
                    )
                        .into_val(&setup.env),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(
            &svec![&setup.env, deposit_amount],
            &svec![&setup.env, deposit_amount],
            &user2,
            &false,
        );

    let df_balance_user1 = enviroment.vault_contract.balance(&user1);
    let df_balance_user2 = enviroment.vault_contract.balance(&user2);
    assert_eq!(df_balance_user1, deposit_amount - MINIMUM_LIQUIDITY);
    assert_eq!(df_balance_user2, deposit_amount);

    let invest_instructions = svec![
        &setup.env,
        Instruction::Invest(
            enviroment.strategy_contract.address.clone(),
            deposit_amount * 2,
        ),
    ];

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "rebalance",
                args: (enviroment.manager.clone(), invest_instructions.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .rebalance(&enviroment.manager, &invest_instructions);

    setup
        .env
        .ledger()
        .set_timestamp(setup.env.ledger().timestamp() + ONE_YEAR_IN_SECONDS);

    let token_balance_after_invest = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_invest, 0);

    let strategy_balance = enviroment
        .strategy_contract
        .balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, deposit_amount * 2);

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user1.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance_user1.clone(), user1.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance_user1, &user1);

    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user2.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance_user2.clone(), user2.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance_user2, &user2);

    let df_balance_user1 = enviroment.vault_contract.balance(&user1);
    let df_balance_user2 = enviroment.vault_contract.balance(&user2);
    assert_eq!(df_balance_user1, 0);
    assert_eq!(df_balance_user2, 0);

    let token_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(token_balance_after_withdraw, 0);

    let expected_amount_user1 = deposit_amount - MINIMUM_LIQUIDITY;

    let expected_amount_user2 = deposit_amount;

    let user1_balance_after_withdraw = enviroment.token.balance(user1);
    let user2_balance_after_withdraw = enviroment.token.balance(user2);
    assert_eq!(user1_balance_after_withdraw, expected_amount_user1);
    assert_eq!(user2_balance_after_withdraw, expected_amount_user2);

    let strategy_balance = enviroment
        .strategy_contract
        .balance(&enviroment.vault_contract.address);
    assert_eq!(strategy_balance, MINIMUM_LIQUIDITY);

    let vault_balance_after_withdraw = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance_after_withdraw, 0);
}
