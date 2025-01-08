use crate::{setup::create_vault_one_blend_strategy, test::IntegrationTest, vault::defindex_vault_contract::{AssetInvestmentAllocation, StrategyAllocation}};
use soroban_sdk::{testutils::{AuthorizedFunction, AuthorizedInvocation}, vec as svec, Symbol, IntoVal, Vec};

#[test]
fn success() {
    let enviroment = create_vault_one_blend_strategy();
    let setup = enviroment.setup;

    let usdc_client = enviroment.usdc_client;
    let usdc = enviroment.usdc;

    let vault_contract = enviroment.vault_contract;

    let users = IntegrationTest::generate_random_users(&setup.env, 2);
    /*
     * Deposit into pool
     * -> deposit 100 into blend strategy for each users[0] and users[1]
     * -> deposit 200 into pool for user_4
     * -> admin borrow from pool to return to 50% util rate
     * -> verify a deposit into an uninitialized vault fails
     */
    let pool_usdc_balace_start = usdc.balance(&enviroment.blend_pool_client.address);
    let starting_balance = 100_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    let user_0_balance = usdc.balance(&users[0]);
    assert_eq!(user_0_balance, starting_balance);

    let user_1_balance = usdc.balance(&users[1]);
    assert_eq!(user_1_balance, starting_balance);

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );

    // -> verify deposit auth
    assert_eq!(
        setup.env.auths()[0],
        (
            users[0].clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault_contract.address.clone(),
                    Symbol::new(&setup.env, "deposit"),
                    svec![
                        &setup.env, 
                        svec!(&setup.env, starting_balance.clone()).into_val(&setup.env),
                        svec!(&setup.env, starting_balance.clone()).into_val(&setup.env),
                        users[0].to_val(), 
                        false.into_val(&setup.env)
                    ]
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        usdc.address.clone(),
                        Symbol::new(&setup.env, "transfer"),
                        svec![
                            &setup.env,
                            users[0].to_val(),
                            vault_contract.address.clone().to_val(),
                            starting_balance.into_val(&setup.env)
                        ]
                    )),
                    sub_invocations: std::vec![]
                }]
            }
        )
    );

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &false
    );

    // verify deposit (pool b_rate still 1 as no time has passed)
    assert_eq!(usdc.balance(&users[0]), 0);
    assert_eq!(usdc.balance(&users[1]), 0);
    assert_eq!(usdc.balance(&vault_contract.address), starting_balance * 2);

    let investments = svec![
        &setup.env,
        Some(AssetInvestmentAllocation {
            asset: usdc.address.clone(),
            strategy_allocations: svec![
                &setup.env,
                Some(StrategyAllocation {
                    amount: starting_balance * 2,
                    strategy_address: enviroment.strategy_contract.address.clone(),
                }),
            ],
        }),
    ];

    vault_contract.invest(&investments);

    // -> verify deposit auth
    assert_eq!(
        setup.env.auths()[0],
        (
            enviroment.manager.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    vault_contract.address.clone(),
                    Symbol::new(&setup.env, "invest"),
                    svec![
                        &setup.env, 
                        Some(AssetInvestmentAllocation {
                            asset: usdc.address.clone(),
                            strategy_allocations: svec![
                                &setup.env,
                                Some(StrategyAllocation {
                                    amount: starting_balance * 2,
                                    strategy_address: enviroment.strategy_contract.address.clone(),
                                })
                            ],
                        }).into_val(&setup.env)
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    assert_eq!(
        usdc.balance(&enviroment.blend_pool_client.address),
        pool_usdc_balace_start + starting_balance * 2
    );
    let vault_positions = enviroment.blend_pool_client.get_positions(&enviroment.strategy_contract.address);
    assert_eq!(vault_positions.supply.get(0).unwrap(), starting_balance * 2);

}

// #[test]
// fn success(e: &Env) {

//     // user_4 deposit directly into pool
//     let user_4_starting_balance = 200_0000000;
//     usdc_client.mint(&user_4, &user_4_starting_balance);
//     pool_client.submit(
//         &user_4,
//         &user_4,
//         &user_4,
//         &vec![
//             &e,
//             Request {
//                 request_type: 0,
//                 address: usdc.address().clone(),
//                 amount: user_4_starting_balance,
//             },
//         ],
//     );

//     // admin borrow back to 50% util rate
//     let borrow_amount = (user_4_starting_balance + starting_balance * 2) / 2;
//     pool_client.submit(
//         &admin,
//         &admin,
//         &admin,
//         &vec![
//             &e,
//             Request {
//                 request_type: 4,
//                 address: usdc.address().clone(),
//                 amount: borrow_amount,
//             },
//         ],
//     );

//     /*
//      * Allow 1 week to pass
//      */
//     e.jump(DAY_IN_LEDGERS * 7);

//     /*
//      * Withdraw from pool
//      * -> withdraw all funds from pool for user_4
//      * -> withdraw (excluding dust) from blend strategy for users[0] and users[1]
//      * -> verify a withdraw from an uninitialized vault fails
//      * -> verify a withdraw from an empty vault fails
//      * -> verify an over withdraw fails
//      */

//     // withdraw all funds from pool for user_4
//     pool_client.submit(
//         &user_4,
//         &user_4,
//         &user_4,
//         &vec![
//             &e,
//             Request {
//                 request_type: 1,
//                 address: usdc.address().clone(),
//                 amount: user_4_starting_balance * 2,
//             },
//         ],
//     );
//     let user_4_final_balance = usdc_client.balance(&user_4);
//     let user_4_profit = user_4_final_balance - user_4_starting_balance;

//     // withdraw from blend strategy for users[0] and users[1]
//     // they are expected to receive half of the profit of user_4
//     let expected_user_4_profit = user_4_profit / 2;
//     let withdraw_amount = starting_balance + expected_user_4_profit;
//     // withdraw_amount = 100_0958904

//     // -> verify over withdraw fails
//     let result =
//         strategy_client.try_withdraw(&(withdraw_amount + 100_000_000_0000000), &users[1], &users[1]);
//     assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

//     strategy_client.withdraw(&withdraw_amount, &users[0], &users[0]);
//     // -> verify withdraw auth
//     assert_eq!(
//         e.auths()[0],
//         (
//             users[0].clone(),
//             AuthorizedInvocation {
//                 function: AuthorizedFunction::Contract((
//                     strategy.clone(),
//                     Symbol::new(&e, "withdraw"),
//                     vec![
//                         &e,
//                         withdraw_amount.into_val(&e),
//                         users[0].to_val(),
//                         users[0].to_val(),
//                     ]
//                 )),
//                 sub_invocations: std::vec![]
//             }
//         )
//     );

//     // -> verify withdraw
//     assert_eq!(usdc_client.balance(&users[0]), withdraw_amount);
//     assert_eq!(strategy_client.balance(&users[0]), 0);

//     // -> verify withdraw from empty vault fails
//     let result = strategy_client.try_withdraw(&MIN_DUST, &users[0], &users[0]);
//     assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

//     // TODO: Finish harvest testings, pending soroswap router setup with a blend token pair with the underlying asset
//     /*
//      * Harvest
//      * -> claim emissions for the strategy
//      * -> Swaps them into the underlying asset
//      * -> Re invest this claimed usdc into the blend pool
//      */

//     // harvest
//     let blnd_strategy_balance = blnd_client.balance(&strategy);
//     assert_eq!(blnd_strategy_balance, 0);

//     strategy_client.harvest(&users[1]);

//     let blnd_strategy_balance = blnd_client.balance(&strategy);
//     assert_eq!(blnd_strategy_balance, 0);

//     let usdc_strategy_balance = usdc_client.balance(&strategy);
//     assert_eq!(usdc_strategy_balance, 0);

//     let user_3_strategy_balance = strategy_client.balance(&users[1]);
//     assert_eq!(user_3_strategy_balance, 1226627059);
// }
