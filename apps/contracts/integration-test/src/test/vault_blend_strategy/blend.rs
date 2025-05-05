#[allow(unused_imports)]
use crate::{
    setup::{
        blend_setup::Request,
        create_vault_one_blend_strategy,
    },
    test::{
        DAY_IN_LEDGERS,
        EnvTestUtils,
        IntegrationTest,
    },
    vault::defindex_vault_contract::{
        AssetInvestmentAllocation,
        Instruction,
        StrategyAllocation,
    },
};
#[allow(unused_imports)]
use soroban_sdk::{
    testutils::{
        Address as _,
        AuthorizedFunction,
        AuthorizedInvocation,
        MockAuth,
        MockAuthInvoke,
    },
    vec as svec,
    Address,
    IntoVal,
    Symbol,
    Vec,
    Bytes,
};
#[test]
fn success() {
    let enviroment = create_vault_one_blend_strategy();
    let setup = enviroment.setup;

    let usdc_client = enviroment.usdc_client;
    let usdc = enviroment.usdc;

    let vault_contract = enviroment.vault_contract;

    let users = IntegrationTest::generate_random_users(&setup.env, 3);

    // Setup pool util rate
    // admins deposits 200k tokens and borrows 100k tokens for a 50% util rate
    let requests = svec![&setup.env,
        Request {
            address: usdc.address.clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: usdc.address.clone(),
            amount: 100_000_0000000,
            request_type: 4,
        }
    ];
    enviroment.blend_pool_client
        .mock_all_auths()
        .submit(&enviroment.admin, &enviroment.admin, &enviroment.admin, &requests);

    /*
     * Deposit into pool
     * -> deposit 100 into blend strategy for each users[0] and users[1]
     * -> deposit 200 into pool for users[2]
     * -> admin borrow from pool to return to 50% util rate
     * -> verify a deposit into an uninitialized vault fails
     */
    let pool_usdc_balace_start = usdc.balance(&enviroment.blend_pool_client.address);
    println!("Pool USDC Balance: {}", pool_usdc_balace_start);
    let starting_balance = 100_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    let user_0_balance = usdc.balance(&users[0]);
    assert_eq!(user_0_balance, starting_balance);

    let user_1_balance = usdc.balance(&users[1]);
    assert_eq!(user_1_balance, starting_balance);

    println!("USDC Vault Balance: {}", usdc.balance(&vault_contract.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));

    println!("--- Depositing user0 ---");
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

    println!("USDC Vault Balance: {}", usdc.balance(&vault_contract.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));

    println!("--- Depositing user1 ---");
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

    println!("Vault USDC Balance: {}", usdc.balance(&vault_contract.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));

    println!("--report before investing--");
    let report = vault_contract.report();
    println!("{:?}", report);

    println!("--- Investing ---");

    let invest_instructions = svec![
        &setup.env,
        Instruction::Invest(
            enviroment.strategy_contract.address.clone(),
            starting_balance * 2,
        ),
    ];

    vault_contract
    .mock_auths(&[MockAuth {
        address: &enviroment.manager.clone(),   
        invoke: &MockAuthInvoke {
            contract: &vault_contract.address.clone(),
            fn_name: "rebalance",
            args: (enviroment.manager.clone(), invest_instructions.clone()).into_val(&setup.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&enviroment.manager, &invest_instructions);

    println!("--report after investing 0 0 2 --");
    let report = vault_contract.report();
    println!("{:?}", report);

    println!("Vault USDC Balance: {}", usdc.balance(&vault_contract.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));

    assert_eq!(
        usdc.balance(&enviroment.blend_pool_client.address),
        pool_usdc_balace_start + starting_balance * 2
    );
    let vault_positions = enviroment.blend_pool_client.get_positions(&enviroment.strategy_contract.address);
    assert_eq!(vault_positions.supply.get(0).unwrap(), starting_balance * 2);

    // user_2 deposit directly into pool
    let user_2_starting_balance = 200_0000000;
    usdc_client.mint(&users[2], &user_2_starting_balance);
    enviroment.blend_pool_client.submit(
        &users[2],
        &users[2],
        &users[2],
        &svec![
            &setup.env,
            Request {
                request_type: 0,
                address: usdc.address.clone(),
                amount: user_2_starting_balance,
            },
        ],
    );

    println!("external User 2 deposits into blend");
    println!("Vault USDC Balance: {}", usdc.balance(&vault_contract.address));

    // admin borrow back to 50% util rate
    let borrow_amount = (user_2_starting_balance + starting_balance * 2) / 2;
    println!("user 2, borrows from blend pool");
    usdc_client.mint(&enviroment.admin, &borrow_amount);
    enviroment.blend_pool_client.submit(
        &enviroment.admin,
        &enviroment.admin,
        &enviroment.admin,
        &svec![
            &setup.env,
            Request {
                request_type: 4,
                address: usdc.address.clone(),
                amount: borrow_amount,
            },
        ],
    );

    println!("report before any time passes");
    let report = vault_contract.report();
    println!("report = {:?}", report);
    /*
     * Allow 1 week to pass
     */
    setup.env.jump(DAY_IN_LEDGERS * 7);

    /*
     * Withdraw from pool
     * -> withdraw all funds from pool for users[2]
     * -> withdraw (excluding dust) from blend strategy for users[0] and users[1]
     * -> verify a withdraw from an uninitialized vault fails
     * -> verify a withdraw from an empty vault fails
     * -> verify an over withdraw fails
     */

    // withdraw all funds from pool for users[2]
    enviroment.blend_pool_client.submit(
        &users[2],
        &users[2],
        &users[2],
        &svec![
            &setup.env,
            Request {
                request_type: 1,
                address: usdc.address.clone(),
                amount: user_2_starting_balance * 2,
            },
        ],
    );
    let user_2_final_balance = usdc.balance(&users[2]);
    let user_2_profit = user_2_final_balance - user_2_starting_balance;

    // withdraw from blend strategy for users[0] and users[1]
    // they are expected to receive half of the profit of users[2] minus the vault fees
    let expected_user_2_profit = user_2_profit / 2;
    let withdraw_amount = starting_balance + expected_user_2_profit;
    // withdraw_amount = 100_0958904
    std::println!("withdraw_amount = {}", withdraw_amount);

    println!("users[0] vault balance before report= {}", vault_contract.balance(&users[0]));
    println!("users[1] vault balance before report= {}", vault_contract.balance(&users[1]));

    std::println!("-- Harvesting --");
    enviroment.strategy_contract.harvest(&enviroment.keeper, &None::<Bytes>);

    let report = vault_contract.report();
    println!("report = {:?}", report);

    let lock_fees = vault_contract.lock_fees(&None);
    println!("locked_fees = {:?}", lock_fees);


    println!("Pool USDC Balance: {}", usdc.balance(&enviroment.blend_pool_client.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));

    println!("-- Distributing Fees --");
    vault_contract.distribute_fees(&enviroment.manager);

    println!("Pool USDC Balance: {}", usdc.balance(&enviroment.blend_pool_client.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));

    let report = vault_contract.report();
    println!("report = {:?}", report);

    let new_user_random = Address::generate(&setup.env);
    usdc_client.mint(&new_user_random, &100_000_000_0);

    vault_contract.deposit(
        &svec!(&setup.env, 100_000_000_0),
        &svec!(&setup.env, 100_000_000_0),
        &new_user_random, 
        &true
    );

    let report = vault_contract.report();
    println!("Pool USDC Balance: {}", usdc.balance(&enviroment.blend_pool_client.address));
    println!("Vault Balance on Strategy: {}", enviroment.strategy_contract.balance(&vault_contract.address));
    println!("report NEW = {:?}", report);

    // let lock_fees = vault_contract.lock_fees(&None);
    // println!("locked_fees = {:?}", lock_fees);

    // -> verify over withdraw fails
    // let result =
    //     strategy_client.try_withdraw(&(withdraw_amount + 100_000_000_0000000), &users[1], &users[1]);
    // assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    // strategy_client.withdraw(&withdraw_amount, &users[0], &users[0]);
    // // -> verify withdraw auth
    // assert_eq!(
    //     e.auths()[0],
    //     (
    //         users[0].clone(),
    //         AuthorizedInvocation {
    //             function: AuthorizedFunction::Contract((
    //                 strategy.clone(),
    //                 Symbol::new(&e, "withdraw"),
    //                 vec![
    //                     &e,
    //                     withdraw_amount.into_val(&e),
    //                     users[0].to_val(),
    //                     users[0].to_val(),
    //                 ]
    //             )),
    //             sub_invocations: std::vec![]
    //         }
    //     )
    // );

    // // -> verify withdraw
    // assert_eq!(usdc_client.balance(&users[0]), withdraw_amount);
    // assert_eq!(strategy_client.balance(&users[0]), 0);

    // // -> verify withdraw from empty vault fails
    // let result = strategy_client.try_withdraw(&MIN_DUST, &users[0], &users[0]);
    // assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    // // TODO: Finish harvest testings, pending soroswap router setup with a blend token pair with the underlying asset
    // /*
    //  * Harvest
    //  * -> claim emissions for the strategy
    //  * -> Swaps them into the underlying asset
    //  * -> Re invest this claimed usdc into the blend pool
    //  */

    // // harvest
    // let blnd_strategy_balance = blnd_client.balance(&strategy);
    // assert_eq!(blnd_strategy_balance, 0);

    // strategy_client.harvest(&users[1]);

    // let blnd_strategy_balance = blnd_client.balance(&strategy);
    // assert_eq!(blnd_strategy_balance, 0);

    // let usdc_strategy_balance = usdc_client.balance(&strategy);
    // assert_eq!(usdc_strategy_balance, 0);

    // let user_3_strategy_balance = strategy_client.balance(&users[1]);
    // assert_eq!(user_3_strategy_balance, 1226627059);

}
