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
use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke, Events};
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Symbol, Vec, Val, symbol_short, String, FromVal};
use crate::test::std::println;
use crate::STRATEGY_NAME;

#[test]
fn min_amount_out() {
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
    let reserve_blnd = 100000000_0_000_000; // 100M
    let reserve_usdc = 50000000_0_000_000; // 50M
    blnd_client.mint(&pool_admin, &reserve_blnd);
    usdc_client.mint(&pool_admin, &reserve_usdc);
    let soroswap_router = create_soroswap_pool(
        &e,
        &pool_admin,
        &blnd.address(),
        &usdc.address(),
        &reserve_blnd,
        &reserve_usdc,
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

    // user deposits 1k usdc
    let amount_usdc = 1000_000000000;
    usdc_client.mint(&user_2, &amount_usdc);
    let deposit_result = strategy_client.deposit(&amount_usdc, &user_2);
    assert_eq!(deposit_result, amount_usdc - 1000);

    // We wait for one week
    println!("Jumping 7 days");
    e.jump(ONE_DAY_LEDGERS * 7);

    // This was got by running pool_client.claim previously
    let emission_to_receive = 151200000000i128;

    let amount_out = soroswap_router.get_amount_out(&emission_to_receive, &reserve_blnd, &reserve_usdc);
    println!("Amount out: {:?}", amount_out);

    let pool_usdc_balance_before_harvest = usdc_client.balance(&pool);
    println!("Pool USDC balance before harvest: {:?}", pool_usdc_balance_before_harvest);

    // Try to harvest with amount out + 1
    let min_amount_out = amount_out + 1;
    println!("Min amount out: {:?}", min_amount_out);

    let min_amount_out_bytes = Bytes::from_slice(&e, &min_amount_out.to_be_bytes());
    let harvest_result = strategy_client.try_harvest(&keeper, &Some(min_amount_out_bytes.clone()));
    println!("Harvest result with high min_amount: {:?}", harvest_result);
    assert!(harvest_result.is_err());
    
    let pool_usdc_balance_after_failed_harvest = usdc_client.balance(&pool);
    assert_eq!(pool_usdc_balance_after_failed_harvest, pool_usdc_balance_before_harvest);
    
    // Harvest successfully
    let min_amount_out = amount_out ;
    println!("Min amount out: {:?}", min_amount_out);
    
    // Convert min_amount_out to Bytes
    let min_amount_bytes = Bytes::from_slice(&e, &min_amount_out.to_be_bytes());

    // Harvest
    let harvest_result = strategy_client.try_harvest(&keeper, &Some(min_amount_bytes.clone()));
    println!("Harvest result with high min_amount: {:?}", harvest_result);
    
    // Pool balance should be higher than before
    let pool_usdc_balance_after_harvest = usdc_client.balance(&pool);
    println!("Pool USDC balance after harvest: {:?}", pool_usdc_balance_after_harvest);
    assert_eq!(pool_usdc_balance_after_harvest, pool_usdc_balance_before_harvest + min_amount_out);
}

