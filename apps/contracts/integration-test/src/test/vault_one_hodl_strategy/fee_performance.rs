use crate::{
    setup::create_vault_one_asset_hodl_strategy,
    test::{EnvTestUtils, IntegrationTest, ONE_YEAR_IN_SECONDS},
    vault::{defindex_vault_contract::Instruction, VaultContractError},
};
use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    vec as svec, IntoVal, Vec,
};

extern crate std;
#[test]
fn fee_performance() {
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

    // Split deposit amount in two
    let half_deposit = deposit_amount / 2;

    // let investments = svec![
    //     &setup.env,
    //     Some(AssetInvestmentAllocation {
    //         asset: enviroment.token.address.clone(),
    //         strategy_allocations: svec![
    //             &setup.env,
    //             Some(StrategyAllocation {
    //                 amount: half_deposit,
    //                 strategy_address: enviroment.strategy_contract.address.clone(),
    //             }),
    //         ],
    //     }),
    // ];

    let invest_instructions = svec![
        &setup.env,
        Instruction::Invest(
            enviroment.strategy_contract.address.clone(),
            half_deposit,
        ),
    ];
        // // First investment
        // enviroment
        // .vault_contract
        // .mock_auths(&[MockAuth {
        //     address: &enviroment.manager.clone(),
        //     invoke: &MockAuthInvoke {
        //         contract: &enviroment.vault_contract.address.clone(),
        //         fn_name: "invest",
        //         args: (Vec::from_array(
        //             &setup.env,
        //             [Some(AssetInvestmentAllocation {
        //                 asset: enviroment.token.address.clone(),
        //                 strategy_allocations: svec![
        //                     &setup.env,
        //                     Some(StrategyAllocation {
        //                         amount: half_deposit,
        //                         strategy_address: enviroment.strategy_contract.address.clone(),
        //                     })
        //                 ],
        //             })],
        //         ),)
        //             .into_val(&setup.env),
        //         sub_invokes: &[],
        //     },
        // }])
        // .invest(&investments);

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

    let report_result_after_1_invest = enviroment.vault_contract.report();
    println!(
        "report_result_after_1_invest: {:?}",
        report_result_after_1_invest
    );

    // Second investment
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


    let report_result_after_2_invests = enviroment.vault_contract.report();
    println!(
        "report_result_after_2_invests: {:?}",
        report_result_after_2_invests
    );

    setup.env.jump_time(ONE_YEAR_IN_SECONDS);
    enviroment
        .strategy_contract
        .harvest(&enviroment.vault_contract.address);

    enviroment.vault_contract.mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "report",
            args: ().into_val(&setup.env),
            sub_invokes: &[],
        },
    }]);

    let _lock_fees_result = enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "lock_fees",
                args: svec![&setup.env, 2000u32].into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .lock_fees(&Some(2000u32));
    let total_funds_after_lock = enviroment
        .vault_contract
        .fetch_total_managed_funds()
        .get(enviroment.token.address.clone())
        .unwrap()
        .total_amount;
    assert_eq!(total_funds_after_lock, deposit_amount);

    let release_fees_amount = 1_0_000_000i128;
    let release_fees_result = enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "release_fees",
                args: (
                    &enviroment.strategy_contract.address.clone(),
                    release_fees_amount,
                )
                    .into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .try_release_fees(
            &enviroment.strategy_contract.address.clone(),
            &release_fees_amount,
        );

    assert_eq!(
        release_fees_result,
        Err(Ok(VaultContractError::InsufficientManagedFunds))
    );

    let _distribute_fees_result = enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "distribute_fees",
                args: ().into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .distribute_fees();

    let _report_result = enviroment.vault_contract.report();

    let total_funds_after_distribute = enviroment
        .vault_contract
        .fetch_total_managed_funds()
        .get(enviroment.token.address.clone())
        .unwrap()
        .total_amount;
    assert_eq!(total_funds_after_distribute, deposit_amount);
}
