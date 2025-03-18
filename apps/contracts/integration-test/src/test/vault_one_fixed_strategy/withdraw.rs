use crate::{
    setup::create_vault_one_asset_fixed_strategy,
    test::{
        vault_one_fixed_strategy::calculate_yield, IntegrationTest,
        ONE_YEAR_IN_SECONDS,
    },
    vault::{
        defindex_vault_contract::Instruction,
        MINIMUM_LIQUIDITY,
    },
};
use soroban_sdk::{
    testutils::{Ledger, MockAuth, MockAuthInvoke},
    vec as svec, IntoVal, Vec,
};

#[test]
fn fixed_apr_no_invest_withdraw_success() {
    let enviroment = create_vault_one_asset_fixed_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 10_000_0_000_000i128;
    let deposit_amount = 10_000_0_000_000i128;

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

    setup
        .env
        .ledger()
        .set_timestamp(setup.env.ledger().timestamp() + ONE_YEAR_IN_SECONDS);

    // // TODO: The vault should call harvest method on the strategy contract
    // enviroment.strategy_contract.mock_all_auths().harvest(&enviroment.vault_contract.address);

    let df_balance_before_withdraw = enviroment.vault_contract.balance(&user);
    assert_eq!(
        df_balance_before_withdraw,
        deposit_amount - MINIMUM_LIQUIDITY
    );
    let min_amount_out = df_balance_before_withdraw.clone() * (10_000 - 2_000) / 10_000; // amount * (BPS - slippage) / BPS = 20% of tolerance over the amount
    let withdraw_min_amounts_out: Vec<i128> = svec![&setup.env, min_amount_out];
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance_before_withdraw.clone(), withdraw_min_amounts_out.clone(), user.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance_before_withdraw, &withdraw_min_amounts_out, &user);

    let expected_amount_user = deposit_amount - MINIMUM_LIQUIDITY;

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, expected_amount_user);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, MINIMUM_LIQUIDITY);

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

    // TODO: The vault should call harvest method on the strategy contract
    enviroment
        .strategy_contract
        .mock_all_auths()
        .harvest(&enviroment.vault_contract.address);

    let df_balance_before_withdraw = enviroment.vault_contract.balance(&user);
    assert_eq!(
        df_balance_before_withdraw,
        deposit_amount - MINIMUM_LIQUIDITY
    );
    
    let min_amount_out = df_balance_before_withdraw.clone() * (10_000 - 2_000) / 10_000; // amount * (BPS - slippage) / BPS = 20% of tolerance over the amount
    let withdraw_min_amounts_out: Vec<i128> = svec![&setup.env, min_amount_out];
    enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &user.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "withdraw",
                args: (df_balance_before_withdraw.clone(), withdraw_min_amounts_out.clone(), user.clone()).into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .withdraw(&df_balance_before_withdraw, &withdraw_min_amounts_out, &user);

    let report_after_withdraw = enviroment.vault_contract.report();
    let locked_fee = report_after_withdraw.get(0).unwrap().locked_fee;

    let apr_bps = 1000u32;
    let user_expected_reward =
        calculate_yield(deposit_amount.clone(), apr_bps, ONE_YEAR_IN_SECONDS);
    let total_yield = user_expected_reward - locked_fee;

    let minimum_liquidity_reward = calculate_yield(MINIMUM_LIQUIDITY, apr_bps, ONE_YEAR_IN_SECONDS);
    let underlying_minimun_liquidity = enviroment.vault_contract.get_asset_amounts_per_shares(&MINIMUM_LIQUIDITY).get(0).unwrap();

    assert_eq!(minimum_liquidity_reward, 100);
    let expected_amount_user = 
        deposit_amount + total_yield - underlying_minimun_liquidity;

    let user_balance_after_withdraw = enviroment.token.balance(user);
    assert_eq!(user_balance_after_withdraw, expected_amount_user);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, 0);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, 0);
}
