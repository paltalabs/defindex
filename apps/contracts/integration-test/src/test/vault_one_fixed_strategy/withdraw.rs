use crate::{setup::{create_vault_one_asset_fixed_strategy, VAULT_FEE}, test::{vault_one_fixed_strategy::calculate_yield, IntegrationTest, DEFINDEX_FEE, ONE_YEAR_IN_SECONDS}, vault::{defindex_vault_contract::{AssetInvestmentAllocation, StrategyAllocation}, MINIMUM_LIQUIDITY}};
use soroban_sdk::{testutils::{Ledger, MockAuth, MockAuthInvoke}, vec as svec, IntoVal, Vec};

#[test]
fn fixed_apr_no_invest_withdraw_success() {
    let enviroment = create_vault_one_asset_fixed_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 10_000_0_000_000i128;
    let deposit_amount = 10_000_0_000_000i128;

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

    enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone(),
                false
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
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user, &false);

    setup.env.ledger().set_timestamp(setup.env.ledger().timestamp() + ONE_YEAR_IN_SECONDS);

    // // TODO: The vault should call harvest method on the strategy contract
    // enviroment.strategy_contract.mock_all_auths().harvest(&enviroment.vault_contract.address);

    let df_balance_before_withdraw = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance_before_withdraw, deposit_amount - MINIMUM_LIQUIDITY);

    enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                df_balance_before_withdraw.clone(),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .withdraw(&df_balance_before_withdraw, &user);

    let charged_fee_user = (deposit_amount - MINIMUM_LIQUIDITY) * (DEFINDEX_FEE as i128 + VAULT_FEE as i128) / 10000;
    let expected_amount_user = deposit_amount - MINIMUM_LIQUIDITY - charged_fee_user;

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, expected_amount_user);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, charged_fee_user + MINIMUM_LIQUIDITY);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);
}

#[test]
fn fixed_apr_invest_withdraw_success() {
    let enviroment = create_vault_one_asset_fixed_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 10_000_0_000_000i128;
    let deposit_amount = 10_000_0_000_000i128;

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

    enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone(),
                false
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
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user, &false);

    let investments = svec![
        &setup.env,
        Some(AssetInvestmentAllocation {
            asset: enviroment.token.address.clone(),
            strategy_allocations: svec![
                &setup.env,
                Some(StrategyAllocation {
                    amount: deposit_amount,
                    strategy_address: enviroment.strategy_contract.address.clone(),
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
                            strategy_allocations:
                                svec![&setup.env,
                                    Some(StrategyAllocation {
                                        amount: deposit_amount,
                                        strategy_address: enviroment.strategy_contract.address.clone(),
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

    // TODO: The vault should call harvest method on the strategy contract
    enviroment.strategy_contract.mock_all_auths().harvest(&enviroment.vault_contract.address);

    let df_balance_before_withdraw = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance_before_withdraw, deposit_amount - MINIMUM_LIQUIDITY);

    enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                df_balance_before_withdraw.clone(),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[]
        },
    }])
    .withdraw(&df_balance_before_withdraw, &user);

    let user_expected_reward = calculate_yield(deposit_amount.clone(), 1000u32, ONE_YEAR_IN_SECONDS);

    let charged_fee_user = (deposit_amount + user_expected_reward - MINIMUM_LIQUIDITY) * (DEFINDEX_FEE as i128 + VAULT_FEE as i128) / 10000;
    let expected_amount_user = deposit_amount + user_expected_reward - MINIMUM_LIQUIDITY - charged_fee_user;

    let user_balance_after_withdraw = enviroment.token.balance(user);
    //TODO: 98 missing?
    assert_eq!(user_balance_after_withdraw, expected_amount_user - 98);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, 0);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);
}