#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::constants::{SCALAR_12};
use crate::storage::ONE_DAY_LEDGERS;
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::std;
use crate::test::assert_approx_eq_rel;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils};
use crate::BlendStrategyClient;
use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use soroban_sdk::{vec, Address, Env, IntoVal, Symbol};
use crate::test::std::println;

#[test]
fn success() {
    let min_dust: i128 = 0_0010000;

    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);
    let user_3 = Address::generate(&e);
    let user_4 = Address::generate(&e);
    let initial_depositor = Address::generate(&e);
    let keeper = Address::generate(&e);

    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());
    let blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    // Setting up soroswap pool
    let pool_admin = Address::generate(&e);
    let amount_a = 100000000_0_000_000;
    let amount_b = 50000000_0_000_000;
    blnd_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    let soroswap_router = create_soroswap_pool(
        &e,
        &pool_admin,
        &blnd.address(),
        &usdc.address(),
        &amount_a,
        &amount_b,
    );
    // End of setting up soroswap pool 

    let blend_fixture = BlendFixture::deploy(&e, &admin, &blnd.address(), &usdc.address());

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let pool_client = BlendPoolClient::new(&e, &pool);
    
    // Setup pool util rate
    // admins deposits 200k tokens and borrows 100k tokens for a 50% util rate
    let requests = vec![
        &e,
        Request {
            address: usdc.address().clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: usdc.address().clone(),
            amount: 100_000_0000000,
            request_type: 4,
        },
        Request {
            address: xlm.address().clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: xlm.address().clone(),
            amount: 100_000_0000000,
            request_type: 4,
        },
    ];
    pool_client
        .mock_all_auths()
        .submit(&admin, &admin, &admin, &requests);
    // usdc (0) and xlm (1) charge a fixed 10% borrow rate with 0% backstop take rate
    // admin deposits 200k tokens and borrows 100k tokens for a 50% util rate for every token

    let strategy = create_blend_strategy(
        &e, 
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);
    assert_eq!(pool_client.get_reserve(&usdc.address().clone()).config.index, 0);


    // get asset returns correct asset
    assert_eq!(strategy_client.asset(), usdc.address().clone());

    

    /*
     * Deposit into pool
     * -> deposit 100 into blend strategy for each user_2 and user_3 (Total 200 USDC deposited through the Strategy)
     * -> deposit 200 directly into pool for both usdc and xlm for user_4 (Total 200 USDC deposited direclty on the pool)
     * -> admin borrow from pool to return to 50% util rate
     */
    let pool_usdc_balace_start = usdc_client.balance(&pool);
    let starting_balance = 100_0000000;
    usdc_client.mint(&user_2, &starting_balance);
    usdc_client.mint(&user_3, &starting_balance);
    usdc_client.mint(&initial_depositor, &starting_balance);

    assert_eq!(usdc_client.balance(&user_3), starting_balance);

    let deposit_result_initial_depositor = strategy_client.deposit(&starting_balance, &initial_depositor);
    assert_eq!(deposit_result_initial_depositor, starting_balance - 1000);



    let deposit_result_0 = strategy_client.deposit(&starting_balance, &user_2);
    assert_eq!(deposit_result_0, starting_balance);
    // -> verify deposit auth

    // * -> deposit 100 into blend strategy for each user_2 and user_3
    // for user 2 we will also check auths
        
    assert_eq!(
        e.auths()[0],
        (
            user_2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    strategy.clone(),
                    Symbol::new(&e, "deposit"),
                    vec![&e, 
                    starting_balance.into_val(&e), 
                    user_2.to_val(),]
                )),
                sub_invocations: std::vec![AuthorizedInvocation {
                    function: AuthorizedFunction::Contract((
                        usdc.address().clone(),
                        Symbol::new(&e, "transfer"),
                        vec![
                            &e,
                            user_2.to_val(),
                            strategy.to_val(),
                            starting_balance.into_val(&e)
                        ]
                    )),
                    sub_invocations: std::vec![]
                }
            ]
            }
        )
    );
    

    // for user 3 we check the result
    let deposit_result_1 = strategy_client.deposit(&starting_balance, &user_3);
    assert_eq!(deposit_result_1, starting_balance);

    // Check balances are healthy
    assert_eq!(usdc_client.balance(&user_2), 0);
    assert_eq!(usdc_client.balance(&user_3), 0);
    assert_eq!(strategy_client.balance(&user_2), starting_balance);
    assert_eq!(strategy_client.balance(&user_3), starting_balance);
    assert_eq!(
        usdc_client.balance(&pool),
        pool_usdc_balace_start + starting_balance * 3
    );
    let strategy_positions = pool_client.get_positions(&strategy);
    assert_eq!(strategy_positions.supply.get(0).unwrap(), starting_balance * 3);
    // (pool b_rate still 1 as no time has passed)
    assert_eq!(pool_client.get_reserve(&usdc.address().clone()).data.b_rate, 1000000000000);
    
    // user_4 deposit directly into pool. This will help us guess the emissions
    let user_4_starting_balance = starting_balance * 2;
    usdc_client.mint(&user_4, &user_4_starting_balance);
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 0,
                address: usdc.address().clone(),
                amount: user_4_starting_balance,
            },
        ],
    );

    assert_eq!(
        usdc_client.balance(&pool),
        pool_usdc_balace_start + starting_balance * 5
    );

    // admin borrow from pool to return USDC to 50% util rate
    let borrow_amount = (user_4_starting_balance + starting_balance * 2) / 2;
    pool_client.submit(
        &admin,
        &admin,
        &admin,
        &vec![
            &e,
            Request {
                request_type: 4,
                address: usdc.address().clone(),
                amount: borrow_amount,
            },
        ],
    );

    assert_eq!(
        usdc_client.balance(&pool),
        pool_usdc_balace_start + starting_balance * 3
    );

    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    // Check Helthy Strategy USDC Balance in the pool
    assert_eq!((strategy_b_tokens * b_rate) / SCALAR_12, starting_balance * 3);
    

    /*
     * Allow 1 week to pass
     */
    e.jump(ONE_DAY_LEDGERS * 7);

    /*
     * Withdraw from pool
     * -> withdraw all funds from pool for user_4 so we can calculate USDC that the strategy should earn
     * -> claim emissions for user 4 so we can know how much will be reinvested while harvesting on withdraw
     * -> withdraw from blend strategy for user_2
     * -> verify a withdraw from an uninitialized vault fails
     * -> verify a withdraw from an empty vault fails
     * -> verify an over withdraw fails
     */

    // withdraw all funds directly from pool for user_4
    println!("USER 4 Withdraws from Blend Pool: 200_1_917_808");
    println!("User 4 Balance before withdrawal {}", usdc_client.balance(&user_4));
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 1,
                address: usdc.address().clone(),
                amount: user_4_starting_balance * 10, // We just put a big amount to take everything
            },
        ],
    );
    let user_4_final_balance = usdc_client.balance(&user_4);
    println!("User 4 Balance after withdrawal {}", usdc_client.balance(&user_4));
    let user_4_profit = user_4_final_balance - user_4_starting_balance;


    /*
        We are expecting that user_4 profit 
        is equal to the profit of the strategy
    
    */

    let expected_users_profit = user_4_profit / 2;
    let expected_strategy_profit = (user_4_profit * 3) / 2;
    println!("Expected strategy profit {}", expected_strategy_profit);
    println!("Expected users profit {}", expected_users_profit);

    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    // Check Helthy Strategy USDC Balance in the pool

    assert_approx_eq_rel(
        (strategy_b_tokens * b_rate) / SCALAR_12,
        starting_balance * 3 + expected_strategy_profit,
        0_00001000,
    );  

    // Now User 4 will claim so we can calculate the emissions:
    // Claim emissions for user_4 (that deposited directly on the pool). We do this to guess the emissions for the strategy
    let reserve_token_ids = vec![&e, 0, 1, 2, 3];
    let amounts_claimed = pool_client.claim(&user_4, &reserve_token_ids, &user_4);
    println!("Merry amounts_claimed {}", amounts_claimed);
    let merry_emissions = blnd_client.balance(&user_4);
    println!("Merry emissions {}", merry_emissions);
    assert_eq!(amounts_claimed, merry_emissions);
    // if user 4 got merry_emissions, then the strategy should get 
    let strategy_emissions = (merry_emissions * 3) / 2;

    // This emissions are for Merry (user 4), who deposited directly into the pool a double amount than
    // user 2 and user 3.
    // this means that is expected that the emissions for Merry to be equal to the emissions for the
    // strategy
    let expected_usdc=soroswap_router
        .router_get_amounts_out(
            &strategy_emissions, 
            &vec![&e, blnd.address().clone(), usdc.address().clone()])
        .get(1).unwrap();

    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    // Check Helthy Strategy USDC Balance in the pool
    assert_approx_eq_rel(
        (strategy_b_tokens * b_rate) / SCALAR_12,
        starting_balance * 3 + expected_strategy_profit,
        0_00001000,
    );   

    // withdraw from blend strategy for user_2
    // each of the users are expected to receive half of the profit of user_4 + the profit for the sold emissions:

    // print expected usdc earned by sold emissions
    println!("Expected USDC earned by the strategy by sold emissions {}", expected_usdc);
    println!("Expected USDC earned by each user  by sold emissions {}", expected_usdc/3);
    let expected_withdraw_amount = starting_balance + expected_users_profit + expected_usdc / 3 + 1; // one stroop in rounding calculations 

    println!("Expected withdraw amount for users {}", expected_withdraw_amount);

    // we harvest first
    strategy_client.harvest(&keeper);

    // -> verify over withdraw fails
    let result =
        strategy_client.try_withdraw(&(expected_withdraw_amount + 1), &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
    let result =
        strategy_client.try_withdraw(&(expected_withdraw_amount + 1 ), &user_3, &user_3); 
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
    println!("Expected withdraw amount for users {}", expected_withdraw_amount);

    // Only user 2 will withdraw its amount
    let remain_underlying = strategy_client.withdraw(&expected_withdraw_amount, &user_2, &user_2);
    // -> verify withdraw auth
    assert_eq!(
        e.auths()[0],
        (
            user_2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    strategy.clone(),
                    Symbol::new(&e, "withdraw"),
                    vec![
                        &e,
                        expected_withdraw_amount.into_val(&e),
                        user_2.to_val(),
                        user_2.to_val(),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    // -> verify healthy balances
    assert_eq!(usdc_client.balance(&user_2), expected_withdraw_amount);
    assert_eq!(usdc_client.balance(&user_3), 0);
    assert_eq!(strategy_client.balance(&user_2), 0);
    assert_eq!(remain_underlying, 0);   
    assert_eq!(strategy_client.balance(&user_3), expected_withdraw_amount);

    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    // Check Helthy Strategy USDC Balance in the pool
    assert_approx_eq_rel(
        (strategy_b_tokens * b_rate) / SCALAR_12,
        starting_balance * 3 + expected_strategy_profit + expected_usdc - expected_withdraw_amount,
        0_0000010,
    );    
    
    // -> verify withdraw from empty vault fails
    let result = strategy_client.try_withdraw(&min_dust, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    /*
         * Harvest
        
        Because we already did harvest when doing withdraw, we will need to invest again for user 4
        Then wait one week, user 4 will claim and then we wil harvest
        * -> claim emissions for the strategy
        * -> Swaps them into the underlying asset
        * -> Re invest this claimed usdc into the blend pool

    */
    // deposit again for user 4, the same amount of the strategy investment
    
    // let user_4_b_tokens = pool_client.get_positions(&user_4).supply.get(0).unwrap();
    // assert_eq!(user_4_b_tokens, 0);

    
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    
    let user_4_starting_balance = usdc_client.balance(&user_4);
    let user_4_new_investment = (strategy_b_tokens * b_rate) / SCALAR_12;
    usdc_client.mint(&user_4, &user_4_starting_balance);
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 0, // deposit
                address: usdc.address().clone(),
                amount: user_4_new_investment,
            },
        ],
    );

    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let user_4_b_tokens = pool_client.get_positions(&user_4).supply.get(0).unwrap();
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    assert_approx_eq_rel(
        user_4_b_tokens,
        strategy_b_tokens,
        0_0000010,
    );

    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    // Check Helthy Strategy USDC Balance in the pool
    assert_approx_eq_rel(
        (strategy_b_tokens * b_rate) / SCALAR_12,
        starting_balance * 3 + expected_strategy_profit + expected_usdc - expected_withdraw_amount,
        0_0100000,
    );

    // wait one week
    println!(" emitter distribution {}",  blend_fixture.emitter.distribute());
    println!(" pool backstop distribution {}",  blend_fixture.backstop.distribute());
    println!(" pool gulp {}",  pool_client.gulp_emissions());

    e.jump(ONE_DAY_LEDGERS * 14);

    //withdraw all funds directly from pool for user_4
    let user_4_before = usdc_client.balance(&user_4);
    println!("User 4 Balance before withdrawal {}", usdc_client.balance(&user_4));
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 1, // withdraw
                address: usdc.address().clone(),
                amount: user_4_new_investment * 10, // We just put a big amount to take everything
            },
        ],
    );
    let user_4_final_balance = usdc_client.balance(&user_4);
    println!("User 4 Balance after withdrawal {}", usdc_client.balance(&user_4));
    let new_user_4_profit = user_4_final_balance - user_4_before - user_4_new_investment ;
    println!("User 4 Balance new profit {}", new_user_4_profit);
    


     // We verify that the strategy now holds the new_expected_usdc
    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;

    // Check Helthy Strategy USDC Balance in the pool
    assert_approx_eq_rel(
        (strategy_b_tokens * b_rate) / SCALAR_12,
        starting_balance * 3 + expected_strategy_profit  + expected_usdc - expected_withdraw_amount + new_user_4_profit,
        0_0000010,
    );


    // harvest

    // Claim emissions for user_4 (that deposited directly on the pool). We do this to guess the emissions for the strategy
    // print blnd_client.balance(&user_4);
    let blnd_balance_before = blnd_client.balance(&user_4);
    println!("User 4 BLND Balance before claim {}", blnd_balance_before);
    let reserve_token_ids = vec![&e, 0, 1, 2, 3];
    let amounts_claimed = pool_client.claim(&user_4, &reserve_token_ids, &user_4);
    println!("Merry amounts_claimed {}", amounts_claimed);
    let merry_emissions = blnd_client.balance(&user_4) - blnd_balance_before;
    println!("Merry emissions {}", merry_emissions);
    assert_eq!(amounts_claimed, merry_emissions);

    // This emissions are for Merry (user 4), who deposited directly into the pool a double amount than
    // user 2 and user 3.
    // this means that is expected that the emissions for Merry to be equal to the emissions for the
    // strategy
    let strategy_emissions = merry_emissions;

    let new_expected_usdc=soroswap_router
        .router_get_amounts_out(
            &strategy_emissions, 
            &vec![&e, blnd.address().clone(), usdc.address().clone()])
        .get(1).unwrap();

    println!("Expected USDC {}", new_expected_usdc);

    let initial_blnd_strategy_balance = blnd_client.balance(&strategy);
    let initial_usdc_pool_balance = usdc_client.balance(&pool);
    assert_eq!(initial_blnd_strategy_balance, 0);
    println!("Strategy BLND Balance before harvest {}", blnd_client.balance(&strategy));
    println!("Strategy USDC Balance before harvest {}", usdc_client.balance(&strategy));
    println!("Pool USDC Balance before harvest {}", initial_usdc_pool_balance);

    let user_3_starting_balance = strategy_client.balance(&user_3);
    println!("Strategy USER 3 Balance before harvest {}", user_3_starting_balance);    
    println!("=======       HARVEST  =======");

    
    strategy_client.harvest(&keeper);

    /*
        TODO:
            - Verify harvest  event, Should provide the correct harvested_BLND amount

            that should be equal to merry_emissions
        
            event::emit_harvest(
            &e,
            String::from_str(&e, STRATEGY_NAME),
            harvested_blend,
            from,
        );

    */

   


    // Get Strategy btokens & brate to get the USDC balance of the strategy in the pool
    let strategy_b_tokens = pool_client.get_positions(&strategy).supply.get(0).unwrap();
    let b_rate = pool_client.get_reserve(&usdc.address().clone()).data.b_rate;
    // Check Helthy Strategy USDC Balance in the pool
    assert_approx_eq_rel(
        (strategy_b_tokens * b_rate) / SCALAR_12,
        starting_balance * 3 + expected_strategy_profit  + expected_usdc - expected_withdraw_amount + new_user_4_profit + new_expected_usdc,
        0_0100000,
    );
    /*
            Now that we only have one user that has a balance in the strategy
            we are expecting that the user_3 balance increased in expected_usdc
    */
    let user_3_after_balance = strategy_client.balance(&user_3);
    println!("Strategy USER 3 after harvest {}", user_3_after_balance);
    assert_approx_eq_rel(
        user_3_after_balance,
        user_3_starting_balance + new_expected_usdc/2, //this new expected usdc is shared with the initial depositor
        0_0000100,
    );

    println!("Strategy USER 3 Increased in {}", user_3_after_balance - user_3_starting_balance);
    println!("Strategy BLND Balance after harvest {}", blnd_client.balance(&strategy));
    println!("Strategy USDC Balance after harvest {}", usdc_client.balance(&strategy));
    println!("Pool USDC Balance after harvest {}", usdc_client.balance(&pool));

    let usdc_pool_increased_in_harvest= usdc_client.balance(&pool) - initial_usdc_pool_balance;
    println!("Pool USDC Increased in  {}", usdc_pool_increased_in_harvest);

    assert_eq!(usdc_pool_increased_in_harvest, new_expected_usdc);


    // get keeper
    let old_keeper = strategy_client.get_keeper();
    assert_eq!(old_keeper, keeper);
    // set keeper to a new address
    let new_keeper = Address::generate(&e);
    strategy_client.set_keeper(&keeper, &new_keeper);
    assert_eq!(strategy_client.get_keeper(), new_keeper);

    // try to harvest with the new keeper
    let harvest_result = strategy_client.try_harvest(&new_keeper);
    assert_eq!(harvest_result, Ok(Ok(())));

    // try to harvest with the old keeper
    let harvest_result = strategy_client.try_harvest(&keeper);
    assert_eq!(harvest_result, Err(Ok(StrategyError::NotAuthorized)));

    // get keeper
    let keeper = strategy_client.get_keeper();
    assert_eq!(keeper, new_keeper);
}
