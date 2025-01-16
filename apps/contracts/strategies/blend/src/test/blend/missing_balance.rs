#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::constants::MIN_DUST;
use crate::storage::DAY_IN_LEDGERS;
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils};
use crate::BlendStrategyClient;
use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use soroban_sdk::{vec, Address, Env, IntoVal, Symbol};

use crate::test::std;
use crate::test::std::println;

// There is a discrepancy of in the funds the strategy actually holds and the funds it tells you it holds when asking for the balance
// This is because the bRate is not up to date with the actual amount of funds in the strategy
#[test]
fn missing_balance() {
    let e = Env::default();
    e.budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);
    let user_3 = Address::generate(&e);
    let user_4 = Address::generate(&e);

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

    // usdc (0) and xlm (1) charge a fixed 10% borrow rate with 0% backstop take rate
    // admin deposits 200m tokens and borrows 100m tokens for a 50% util rate
    // emits to each reserve token evently, and starts emissions
    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&e, &pool);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    /*
     * Deposit into pool
     * -> deposit 100 into blend strategy for each user_2 and user_3
     * -> deposit 200 into pool for user_4
     * -> admin borrow from pool to return to 50% util rate
     * -> verify a deposit into an uninitialized vault fails
     */
    let pool_usdc_balace_start = usdc_client.balance(&pool);
    let starting_balance = 100_0000000;
    usdc_client.mint(&user_2, &starting_balance);
    usdc_client.mint(&user_3, &starting_balance);

    let user_3_balance = usdc_client.balance(&user_3);
    assert_eq!(user_3_balance, starting_balance);

    println!("-----------------------------------------------");
    println!("---- Amounts before anything ----");
    println!("Blend pool is at 100k USDC");
    println!("Blend Pool USDC Balance {}", usdc_client.balance(&pool));
    println!("Strategy is empty");
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 2 USDC Balance {}", usdc_client.balance(&user_2));
    println!("USER 3 USDC Balance {}", usdc_client.balance(&user_3));
    println!("-----------------------------------------------");

    println!("--- First Deposit ---");
    println!("USER 2 Deposits: {}", starting_balance);
    let deposit_result_0 = strategy_client.deposit(&starting_balance, &user_2);
    assert_eq!(deposit_result_0, starting_balance);
    // -> verify deposit auth

    assert_eq!(
        e.auths()[0],
        (
            user_2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    strategy.clone(),
                    Symbol::new(&e, "deposit"),
                    vec![&e, starting_balance.into_val(&e), user_2.to_val(),]
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
                }]
            }
        )
    );

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance +100 {}", usdc_client.balance(&pool));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 2 USDC Balance -100 {}", usdc_client.balance(&user_2));
    println!("USER 3 USDC Balance {}", usdc_client.balance(&user_3));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    println!("USER 2 strategy balance {}", strategy_client.balance(&user_2));
    println!("-----------------------------------------------");


    println!("--- Second Deposit ---");
    println!("USER 3 Deposits: {}", starting_balance);
    let deposit_result_1 = strategy_client.deposit(&starting_balance, &user_3);
    assert_eq!(deposit_result_1, starting_balance);

    // verify deposit (pool b_rate still 1 as no time has passed)
    assert_eq!(usdc_client.balance(&user_2), 0);
    assert_eq!(usdc_client.balance(&user_3), 0);
    assert_eq!(strategy_client.balance(&user_2), starting_balance);
    assert_eq!(strategy_client.balance(&user_3), starting_balance);
    assert_eq!(
        usdc_client.balance(&pool),
        pool_usdc_balace_start + starting_balance * 2
    );
    let vault_positions = pool_client.get_positions(&strategy);
    assert_eq!(vault_positions.supply.get(0).unwrap(), starting_balance * 2);

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance +100 {}", usdc_client.balance(&pool));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 3 USDC Balance -100 {}", usdc_client.balance(&user_3));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    println!("USER 2 strategy balance {}", strategy_client.balance(&user_2));
    println!("USER 3 strategy balance {}", strategy_client.balance(&user_3));
    println!("-----------------------------------------------");

    // user_4 deposit directly into pool
    println!("--- Depositing Directly into Blend ---");
    let user_4_starting_balance = 200_0000000;
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

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance +200 {}", usdc_client.balance(&pool));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    println!("USER 2 strategy balance {}", strategy_client.balance(&user_2));
    println!("USER 3 strategy balance {}", strategy_client.balance(&user_3));
    println!("-----------------------------------------------");


    // admin borrow back to 50% util rate
    println!("--- ADMIN Borrowing from Blend ---");
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

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance -200 {}", usdc_client.balance(&pool));
    println!("ADMIN USDC Balance +200 {}", usdc_client.balance(&admin));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    println!("USER 2 strategy balance {}", strategy_client.balance(&user_2));
    println!("USER 3 strategy balance {}", strategy_client.balance(&user_3));
    println!("-----------------------------------------------");


    /*
     * Allow 1 week to pass
     */
    e.jump(DAY_IN_LEDGERS * 7);

    println!("--- 7 Days have passed ---");

    /*
     * Withdraw from pool
     * -> withdraw all funds from pool for user_4
     * -> withdraw (excluding dust) from blend strategy for user_2 and user_3
     * -> verify a withdraw from an uninitialized vault fails
     * -> verify a withdraw from an empty vault fails
     * -> verify an over withdraw fails
     */

    // withdraw all funds from pool for user_4
    println!("USER 4 Withdraws from Blend Pool: 200_1_917_808");
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 1,
                address: usdc.address().clone(),
                amount: user_4_starting_balance * 2,
            },
        ],
    );
    let user_4_final_balance = usdc_client.balance(&user_4);
    let user_4_profit = user_4_final_balance - user_4_starting_balance; //1_917_808
    println!("USER 4 Profit: {}", user_4_profit);

    // withdraw from blend strategy for user_2 and user_3
    // they are expected to receive half of the profit of user_4
    let expected_user_2_profit = user_4_profit / 2;
    let withdraw_amount = starting_balance + expected_user_2_profit;
    println!("Withdraw Amount USER 2 and USER 3: {}", withdraw_amount);
    // withdraw_amount = 100_0958904

    // -> verify over withdraw fails
    let result =
        strategy_client.try_withdraw(&(withdraw_amount + 100_000_000_0000000), &user_3, &user_3);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance {}", usdc_client.balance(&pool));
    println!("Strategy USDC Balance {}", usdc_client.balance(&strategy));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 2 USDC Balance {}", usdc_client.balance(&user_2));
    println!("USER 3 USDC Balance {}", usdc_client.balance(&user_3));
    println!("USER 4 USDC Balance {}", usdc_client.balance(&user_4));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    println!("ADMIN strategy balance {}", strategy_client.balance(&admin));
    println!("USER 2 strategy balance {}", strategy_client.balance(&user_2));
    println!("USER 3 strategy balance {}", strategy_client.balance(&user_3));
    println!("USER 4 strategy balance {}", strategy_client.balance(&user_4));
    println!("-----------------------------------------------");

    println!("Withdrawing USER 2: 1000958904");
    strategy_client.withdraw(&withdraw_amount, &user_2, &user_2);
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
                        withdraw_amount.into_val(&e),
                        user_2.to_val(),
                        user_2.to_val(),
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    // -> verify withdraw
    assert_eq!(usdc_client.balance(&user_2), withdraw_amount);
    assert_eq!(strategy_client.balance(&user_2), 0);

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance {}", usdc_client.balance(&pool));
    println!("Strategy USDC Balance {}", usdc_client.balance(&strategy));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 2 USDC Balance {}", usdc_client.balance(&user_2));
    println!("USER 3 USDC Balance {}", usdc_client.balance(&user_3));
    println!("USER 4 USDC Balance {}", usdc_client.balance(&user_4));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    println!("ADMIN strategy balance {}", strategy_client.balance(&admin));
    println!("USER 2 strategy balance {}", strategy_client.balance(&user_2));
    println!("USER 3 strategy balance {}", strategy_client.balance(&user_3));
    println!("USER 4 strategy balance {}", strategy_client.balance(&user_4));
    println!("-----------------------------------------------");


    // -> verify withdraw from empty vault fails
    let result = strategy_client.try_withdraw(&MIN_DUST, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    // TODO: Finish harvest testings, pending soroswap router setup with a blend token pair with the underlying asset
    /*
     * Harvest
     * -> claim emissions for the strategy
     * -> Swaps them into the underlying asset
     * -> Re invest this claimed usdc into the blend pool
     */

    // harvest
    let blnd_strategy_balance = blnd_client.balance(&strategy);
    assert_eq!(blnd_strategy_balance, 0);

    println!("--- Harvesting ---");
    strategy_client.harvest(&user_3);

    // After harvest the swapped blend is 22_5_668_156
    // Before harvest the USER 3 balance is 100_0_958_904
    // After harvest the USER 3 balance is 122_6_627_059

    let blnd_strategy_balance = blnd_client.balance(&strategy);
    assert_eq!(blnd_strategy_balance, 0);

    let usdc_strategy_balance = usdc_client.balance(&strategy);
    assert_eq!(usdc_strategy_balance, 0);

    let user_3_strategy_balance = strategy_client.balance(&user_3);
    assert_eq!(user_3_strategy_balance, 1226627059);

    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance {}", usdc_client.balance(&pool));
    println!("Strategy USDC Balance {}", usdc_client.balance(&strategy));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 2 USDC Balance {}", usdc_client.balance(&user_2));
    println!("USER 3 USDC Balance {}", usdc_client.balance(&user_3));
    println!("USER 4 USDC Balance {}", usdc_client.balance(&user_4));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    let user_3_balance_prev_sim = strategy_client.balance(&user_3);
    println!("USER 3 strategy balance {}", user_3_balance_prev_sim);
    println!("-----------------------------------------------");
    

    println!("--- Simulating Distributing fees ---");
    println!("It sends 2275859 USDC from the USER 3 aka the vault to the fee receiver");
    let fee_receiver = Address::generate(&e);
    strategy_client.withdraw(&2275859i128, &user_3, &fee_receiver);
    println!("-----------------------------------------------");
    println!("Blend Pool USDC Balance {}", usdc_client.balance(&pool));
    println!("Strategy USDC Balance {}", usdc_client.balance(&strategy));
    println!("ADMIN USDC Balance {}", usdc_client.balance(&admin));
    println!("USER 2 USDC Balance {}", usdc_client.balance(&user_2));
    println!("USER 3 USDC Balance {}", usdc_client.balance(&user_3));
    println!("USER 4 USDC Balance {}", usdc_client.balance(&user_4));
    println!("-----------------------------------------------");
    println!("---- Strategy Balances for users ----");
    let user_3_balance_after_sim = strategy_client.balance(&user_3);
    println!("USER 3 strategy balance {}", user_3_balance_after_sim);
    println!("-----------------------------------------------");

    println!("If we subtract the 2275859 USDC from the USER 3 balance we get");
    println!("{}", user_3_balance_prev_sim - 2275859i128);
    println!("Which should be the same as the USER 3 balance after the distributing fees simulation");
    println!("But is not, the balance after is: {}", user_3_balance_after_sim);
    println!("And there is a difference of: {}", (user_3_balance_prev_sim - 2275859i128) - user_3_balance_after_sim);

    let blend_pool_position = pool_client.get_positions(&strategy_client.address);
    println!("Blend Pool Position for strategy: {:?}", blend_pool_position);
    println!("Is the same amount as the strategy knows, so we are good here");

    // println!("--- Withdrawing ALL user 3 funds ---");
    // strategy_client.withdraw(&200_000_000_000i128, &user_3, &user_3); // if i withdraw more than i have it will just withdraw the maximum possible, which is the real USER 3 Balance of 1224351200 so in some part of the code when the conversion from b_tokens to shares to underlying asset is done it is not working properly
    // println!("Blend Pool USDC Balance {}", usdc_client.balance(&pool));
    // println!("---- Strategy Balances for users ----");
    // let user_3_balance_after_withdraw = strategy_client.balance(&user_3);
    // println!("USER 3 strategy balance {}", user_3_balance_after_withdraw);
    // println!("-----------------------------------------------");

    println!("Deposits");
    strategy_client.deposit(&100_000_000_0i128, &user_2);
    let user_3_balance = strategy_client.balance(&user_3);
    println!("USER 3 strategy balance {}", user_3_balance);



}

// Reserves BEFORE withdrawal of everything
// bRate = 1000958798
// total_shares = 998144621
// total_b_tokens = 1223178290
// vault_shares = 998144621
// (((vault_shares * total_b_tokens) / total_shares) * bRate) / 1000000000
// https://es.symbolab.com/solver/step-by-step/%5Cfrac%7B%5Cfrac%7B%5Cleft(998144621%5Ccdot1223178290%5Cright)%7D%7B998144621%7D%5Ccdot1000958904%7D%7B1000000000%7D?or=input


// Reserves AFTER withdrawal, bRate here is the updated one and the one that gives you the actual amount that the strategy holds for the user
// bRate = 1000958903
// total_shares_after = 0
// total_b_tokens_after = 0
// vault_shares_after = 0

// https://es.symbolab.com/solver/step-by-step/%5Cfrac%7B%5Cfrac%7B%5Cleft(998144621%5Ccdot1223178290%5Cright)%7D%7B998144621%7D%5Ccdot1000958903%7D%7B1000000000%7D?or=input
// The discrepancy in the lost tokens appear to be from the bRate conversion, since this are not fully up to date with what the blend_pool rate actually is, we have no control over this...

// with the new bRate after the withdrawal of all underlying asset we get a bRate that makes sense with the actual amount in the strategy