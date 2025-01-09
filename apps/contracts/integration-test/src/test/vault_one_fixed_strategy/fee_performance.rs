use crate::{
    setup::create_vault_one_asset_fixed_strategy,
    test::{EnvTestUtils, IntegrationTest, ONE_YEAR_IN_SECONDS},
    vault::{
        defindex_vault_contract::Instruction,
        },
};
use soroban_sdk::{
    testutils::{MockAuth, MockAuthInvoke},
    vec as svec, IntoVal, Vec,
};

#[test]
fn fee_performance() {
    let enviroment = create_vault_one_asset_fixed_strategy();
    let setup = enviroment.setup;
    let token_address = enviroment.token.address.clone();
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

    let deposit_amount = 100_0_000_000i128;
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

    let vault_balance_in_strategy = enviroment
        .strategy_contract
        .balance(&enviroment.vault_contract.address);

    std::println!("Shares: {:?}", vault_balance_in_strategy);

    // Jump one year
    setup.env.jump_time(ONE_YEAR_IN_SECONDS);

    // Harvest
    enviroment
        .strategy_contract
        .harvest(&enviroment.vault_contract.address);

    let vault_balance_in_strategy = enviroment
        .strategy_contract
        .balance(&enviroment.vault_contract.address);
    std::println!("Shares after one year: {:?}", vault_balance_in_strategy);

    // Report
    let _report = enviroment.vault_contract.mock_all_auths().report();

    let expected_balance = deposit_amount * 11 / 10; // 10% fixed APR
    assert_eq!(vault_balance_in_strategy, expected_balance);

    // Lock fees
    let lock_fees_bps = 2000u32;
    let lock_fees_result = enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "lock_fees",
                args: svec![&setup.env, lock_fees_bps].into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .lock_fees(&Some(lock_fees_bps));

    let locked_fee = lock_fees_result.get(0).unwrap().locked_fee;

    let total_funds_after_lock = enviroment
        .vault_contract
        .fetch_total_managed_funds()
        .get(token_address.clone())
        .unwrap()
        .total_amount;
    assert_eq!(total_funds_after_lock, (expected_balance - locked_fee));

    // Release fees
    let release_fees_amount = 2_0_000_000i128; // release all locked fees (10_0_000_000 * 0.2 = 2_0_000_000)
    let _release_fees_result = enviroment
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
        .release_fees(
            &enviroment.strategy_contract.address.clone(),
            &release_fees_amount,
        );

    let total_funds_after_release = enviroment
        .vault_contract
        .fetch_total_managed_funds()
        .get(token_address.clone())
        .unwrap()
        .total_amount;
    assert_eq!(
        total_funds_after_release,
        (total_funds_after_lock + release_fees_amount)
    );

    // Lock fees
    let lock_fees_bps = 2000u32;
    let _lock_fees_result = enviroment
        .vault_contract
        .mock_auths(&[MockAuth {
            address: &enviroment.manager.clone(),
            invoke: &MockAuthInvoke {
                contract: &enviroment.vault_contract.address.clone(),
                fn_name: "lock_fees",
                args: svec![&setup.env, lock_fees_bps].into_val(&setup.env),
                sub_invokes: &[],
            },
        }])
        .lock_fees(&Some(lock_fees_bps));

    let total_funds_after_lock = enviroment
        .vault_contract
        .fetch_total_managed_funds()
        .get(token_address.clone())
        .unwrap()
        .total_amount;

    // Distribute fees
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

    let total_funds_after_distribute = enviroment
        .vault_contract
        .fetch_total_managed_funds()
        .get(token_address.clone())
        .unwrap()
        .total_amount;
    assert_eq!(total_funds_after_distribute, total_funds_after_lock);
}
