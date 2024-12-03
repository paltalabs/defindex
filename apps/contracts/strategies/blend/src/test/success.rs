#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::constants::MIN_DUST;
use crate::storage::DAY_IN_LEDGERS;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils};
use crate::BlendStrategyClient;
use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
use soroban_sdk::{vec, Address, Env, IntoVal, Symbol};
use crate::test::std;

#[test]
fn success() {
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
    let _blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    let blend_fixture = BlendFixture::deploy(&e, &admin, &blnd.address(), &usdc.address());

    // usdc (0) and xlm (1) charge a fixed 10% borrow rate with 0% backstop take rate
    // admin deposits 200m tokens and borrows 100m tokens for a 50% util rate
    // emits to each reserve token evently, and starts emissions
    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&e, &pool);
    let strategy = create_blend_strategy(&e, &usdc.address(), &pool, &0u32, &blnd.address(), &Address::generate(&e));
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

    let user_3_balance = usdc_client.balance(&user_2);
    assert_eq!(user_3_balance, starting_balance);


    strategy_client.deposit(&starting_balance, &user_2);
    // -> verify deposit auth
    
    assert_eq!(
        e.auths()[0],
        (
            user_2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    strategy.clone(),
                    Symbol::new(&e, "deposit"),
                    vec![
                        &e,
                        starting_balance.into_val(&e),
                        user_2.to_val(),
                    ]
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

    strategy_client.deposit(&starting_balance, &user_3);

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

    // user_4 deposit directly into pool
    let merry_starting_balance = 200_0000000;
    usdc_client.mint(&user_4, &merry_starting_balance);
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 0,
                address: usdc.address().clone(),
                amount: merry_starting_balance,
            },
        ],
    );

    // admin borrow back to 50% util rate
    let borrow_amount = (merry_starting_balance + starting_balance * 2) / 2;
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

    /*
     * Allow 1 week to pass
     */
    e.jump(DAY_IN_LEDGERS * 7);

    /*
     * Withdraw from pool
     * -> withdraw all funds from pool for user_4
     * -> withdraw (excluding dust) from blend strategy for user_2 and user_3
     * -> verify a withdraw from an uninitialized vault fails
     * -> verify a withdraw from an empty vault fails
     * -> verify an over withdraw fails
     */

    // withdraw all funds from pool for user_4
    pool_client.submit(
        &user_4,
        &user_4,
        &user_4,
        &vec![
            &e,
            Request {
                request_type: 1,
                address: usdc.address().clone(),
                amount: merry_starting_balance * 2,
            },
        ],
    );
    let user_5_final_balance = usdc_client.balance(&user_4);
    let user_5_profit = user_5_final_balance - merry_starting_balance;

    // withdraw from blend strategy for user_2 and user_3
    // they are expected to receive half of the profit of user_4
    let expected_user_4_profit = user_5_profit / 2;
    let withdraw_amount = starting_balance + expected_user_4_profit;
    // withdraw_amount = 100_0958904

    // -> verify over withdraw fails
    let result = strategy_client.try_withdraw(&(withdraw_amount + 100_000_000_0000000), &user_3);
    assert_eq!(result, Err(Ok(StrategyError::InvalidArgument))); // TODO: Check which is the one failing

    strategy_client.withdraw(&withdraw_amount, &user_2);
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
                    ]
                )),
                sub_invocations: std::vec![]
            }
        )
    );

    strategy_client.withdraw(&withdraw_amount, &user_3);

    // -> verify withdraw
    assert_eq!(usdc_client.balance(&user_2), withdraw_amount);
    assert_eq!(usdc_client.balance(&user_3), withdraw_amount);
    assert_eq!(strategy_client.balance(&user_2), 0);
    assert_eq!(strategy_client.balance(&user_3), 0);

    // -> verify withdraw from empty vault fails
    let result = strategy_client.try_withdraw(&MIN_DUST, &user_3);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    // TODO: Finish harvest testings, pending soroswap router setup with a blend token pair with the underlying asset
    /*
     * Harvest
     * -> claim emissions for the strategy
     * -> Swaps them into the underlying asset
     * -> Re invest this claimed usdc into the blend pool
     */

    // harvest
    // strategy_client.harvest(&usdc, &user_2, &expected_fees);

    // -> verify harvest
    
}
