#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::storage::ONE_DAY_IN_LEDGERS;
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils};
use crate::BlendStrategyClient;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env};

// use crate::test::std;
use crate::test::std::println;

// There is a discrepancy of in the funds the strategy actually holds and the funds it tells you it holds when asking for the balance
// This is because the bRate is not up to date with the actual amount of funds in the strategy
#[test]
fn missing_balance() {
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
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
    let amount_a = 100_000_000_0_000_000;
    let amount_b = 50_000_000_0_000_000;
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

    println!("--- First Deposit ---");
    println!("USER 2 Deposits: {}", starting_balance);
    let deposit_result_0 = strategy_client.deposit(&starting_balance, &user_2);
    assert_eq!(deposit_result_0, starting_balance);
    assert_eq!(
        usdc_client.balance(&pool),
        starting_balance + pool_usdc_balace_start
    );

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
        starting_balance * 2 + pool_usdc_balace_start
    );

    let vault_positions = pool_client.get_positions(&strategy);
    assert_eq!(vault_positions.supply.get(0).unwrap(), starting_balance * 2);

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

    assert_eq!(
        usdc_client.balance(&pool),
        starting_balance * 2 + pool_usdc_balace_start + user_4_starting_balance
    );

    // admin borrow back to 50% util rate
    println!("--- ADMIN Borrowing from Blend ---");
    let borrow_amount = (user_4_starting_balance + starting_balance * 2) / 2;
    // pool_client.submit(
    //     &admin,
    //     &admin,
    //     &admin,
    //     &vec![
    //         &e,
    //         Request {
    //             request_type: 4,
    //             address: usdc.address().clone(),
    //             amount: borrow_amount,
    //         },
    //     ],
    // );

    // println!("admin borrows {}", borrow_amount);

    // assert_eq!(
    //     usdc_client.balance(&pool),
    //     starting_balance * 2 + pool_usdc_balace_start + user_4_starting_balance - borrow_amount
    // );

    // /*
    //  * Allow 1 week to pass
    //  */
    // e.jump(ONE_DAY_IN_LEDGERS * 7);

    // /*
    //  * Withdraw from pool
    //  * -> withdraw all funds from pool for user_4
    //  * -> withdraw (excluding dust) from blend strategy for user_2 and user_3
    //  * -> verify a withdraw from an uninitialized vault fails
    //  * -> verify a withdraw from an empty vault fails
    //  * -> verify an over withdraw fails
    //  */

    // // withdraw all funds from pool for user_4
    // println!("USER 4 Withdraws from Blend Pool: 200_1_917_808");
    // pool_client.submit(
    //     &user_4,
    //     &user_4,
    //     &user_4,
    //     &vec![
    //         &e,
    //         Request {
    //             request_type: 1,
    //             address: usdc.address().clone(),
    //             amount: user_4_starting_balance * 2,
    //         },
    //     ],
    // );
    // let user_4_final_balance = usdc_client.balance(&user_4);
    // let user_4_profit = user_4_final_balance - user_4_starting_balance; //1_917_808
    // println!("USER 4 Profit: {}", user_4_profit);

    // // withdraw from blend strategy for user_2 and user_3
    // // they are expected to receive half of the profit of user_4
    // let expected_user_2_profit = user_4_profit / 2;
    // let withdraw_amount = starting_balance + expected_user_2_profit;
    // println!("Withdraw Amount USER 2 and USER 3: {}", withdraw_amount);
    // // withdraw_amount = 100_0958904

    // println!("Withdrawing USER 2: 1000958904");
    // strategy_client.withdraw(&withdraw_amount, &user_2, &user_2);

    // // -> verify withdraw
    // assert_eq!(usdc_client.balance(&user_2), withdraw_amount);
    // assert_eq!(strategy_client.balance(&user_2), 0);

    // // harvest
    // let blnd_strategy_balance = blnd_client.balance(&strategy);
    // assert_eq!(blnd_strategy_balance, 0);

    // println!("--- Harvesting ---");
    // strategy_client.harvest(&user_3);

    // // After harvest the swapped blend is 22_5_668_156
    // // Before harvest the USER 3 balance is 100_0_958_904
    // // After harvest the USER 3 balance is 122_6_627_059

    // let blnd_strategy_balance = blnd_client.balance(&strategy);
    // assert_eq!(blnd_strategy_balance, 0);

    // let usdc_strategy_balance = usdc_client.balance(&strategy);
    // assert_eq!(usdc_strategy_balance, 0);

    // let user_3_strategy_balance = strategy_client.balance(&user_3);
    // assert_eq!(user_3_strategy_balance, 1226627059);

    // println!("-----------------------------------------------");
    // println!("--- Simulating Distributing fees ---");
    // println!("-----------------------------------------------");
    // let fee_amount = 20000i128;
    // println!(
    //     "It sends {} USDC from the USER 3 aka the vault to the fee receiver",
    //     fee_amount
    // );
    // let user_3_balance_prev_sim = strategy_client.balance(&user_3);
    // println!(
    //     "USER 3 strategy balance previous simulation {}",
    //     user_3_balance_prev_sim
    // );
    // let fee_receiver = Address::generate(&e);
    // strategy_client.withdraw(&fee_amount, &user_3, &fee_receiver);

    // let user_3_balance_after_sim = strategy_client.balance(&user_3);
    // assert_eq!(
    //     (user_3_balance_prev_sim - fee_amount),
    //     user_3_balance_after_sim
    // );

    // println!(
    //     "USER 3 strategy balance after sim {}",
    //     user_3_balance_after_sim
    // );
    // strategy_client.withdraw(&(user_3_balance_after_sim * 2), &user_3, &user_3);
}
