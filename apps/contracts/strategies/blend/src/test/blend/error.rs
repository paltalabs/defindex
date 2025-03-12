#![cfg(test)]
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{
    create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils, ONE_DAY_IN_SECONDS,
};
use crate::StrategyReserves;
use crate::reserves;

use crate::BlendStrategyClient;
use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{Address, Env, IntoVal};

#[test]
fn deposit_below_min_dust() {
    // Setting up the environment
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    // Setting up the users
    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 100_0000000;
    usdc_client.mint(&user_2, &starting_balance);

    //Trying to deposit below the min dust
    let deposit_below_min_dust = strategy_client.try_deposit(&9999i128, &user_2);

    assert_eq!(
        deposit_below_min_dust,
        Err(Ok(StrategyError::AmountBelowMinDust))
    );
}

#[test]
fn deposit_zero_and_negative_amount() {
    // Setting up the environment
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    // Setting up the users
    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 100_0000000;
    usdc_client.mint(&user_2, &starting_balance);

    //Trying to deposit below the min dust
    let deposit_negative = strategy_client.try_deposit(&-100_000i128, &user_2);

    assert_eq!(deposit_negative, Err(Ok(StrategyError::NegativeNotAllowed)));

    let deposit_zero = strategy_client.try_deposit(&0i128, &user_2);

    assert_eq!(deposit_zero, Err(Ok(StrategyError::AmountBelowMinDust)));
}

#[test]
fn harvest_from_random_address() {
    // Setting up the environment
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();

    // Setting up the users
    e.set_default_info();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
    let blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    // Setting up soroswap pool
    let pool_admin = Address::generate(&e);
    let amount_a = 2_000;
    let amount_b = 2_000;
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 100_0_000_000i128;
    usdc_client.mint(&user, &starting_balance);

    strategy_client.deposit(&starting_balance, &user);

    e.jump_time(ONE_DAY_IN_SECONDS * 365);

    usdc_client.mint(&user, &1_0_000_000);

    strategy_client.deposit(&1_0_000_000, &user);

    //Trying to harvest from random address
    let balance_before_harvest = strategy_client.balance(&user);
    let harvest_from_random_address = strategy_client.try_harvest(&strategy_client.address);
    let balance_after_harvest = strategy_client.balance(&user);

    assert_eq!(balance_before_harvest, balance_after_harvest);
    assert_eq!(harvest_from_random_address, Ok(Ok(())));
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")] // Unauthorized
fn unauthorized_harvest() {
    // Setting up the environment
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();

    // Setting up the users
    e.set_default_info();
    let admin = Address::generate(&e);
    let user = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
    let blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    // Setting up soroswap pool
    let pool_admin = Address::generate(&e);
    let amount_a = 2_000;
    let amount_b = 2_000;

    blnd_client.mock_all_auths().mint(&pool_admin, &amount_a);
    usdc_client.mock_all_auths().mint(&pool_admin, &amount_b);
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
    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );

    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 100_0_000_000i128;
    usdc_client.mint(&user, &starting_balance);

    strategy_client.deposit(&starting_balance, &user);

    e.jump_time(ONE_DAY_IN_SECONDS * 365);

    usdc_client.mint(&user, &1_0_000_000);

    strategy_client.deposit(&1_0_000_000, &user);
    let user_2 = Address::generate(&e);

    //Trying to harvest from random address

    //Validate the error

    let try_harvest_result = strategy_client.mock_auths(
        &[MockAuth {
            address: &user,
            invoke: &MockAuthInvoke {
                contract: &strategy_client.address.clone(),
                fn_name: "harvest",
                args: (user_2.clone(),).into_val(&e),
                sub_invokes: &[],
            },
        }],
    ).try_harvest(&user_2);

    assert_eq!(try_harvest_result, Err(Err(soroban_sdk::InvokeError::Abort)));

    //Panic with Unauthorized
    let _result = strategy_client.mock_auths(
        &[MockAuth {
            address: &user,
            invoke: &MockAuthInvoke {
                contract: &strategy_client.address.clone(),
                fn_name: "harvest",
                args: (user_2.clone(),).into_val(&e),
                sub_invokes: &[],
            },
        }],
    ).harvest(&user_2);
}

#[test]
fn withdraw_insufficient_balance() {
    // Setting up the environment
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    // Setting up the users

    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let result = strategy_client.try_withdraw(&200_0_000_000, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    let starting_balance = 10_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);
    strategy_client.deposit(&starting_balance, &user_2);

    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, starting_balance);
    assert_eq!(user_balance, 0);

    let result = strategy_client.try_withdraw(&200_0_000_000, &Address::generate(&e), &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    /*
    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, 0);
    assert_eq!(user_balance, starting_balance); */
}

#[test]
fn withdraw_zero_and_negative() {
    // Setting up the environment
    let e = Env::default();
    e.mock_all_auths();
    e.set_default_info();

    // Setting up the users

    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 10_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);
    strategy_client.deposit(&starting_balance, &user_2);

    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, starting_balance);
    assert_eq!(user_balance, 0);

    let result = strategy_client.try_withdraw(&-200_0_000_000, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::NegativeNotAllowed)));

    let result = strategy_client.try_withdraw(&0, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::AmountBelowMinDust)));

    let result = strategy_client.try_withdraw(&9_000, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::AmountBelowMinDust)));
}

#[test]
fn unauthorized_withdraw() {
    // Setting up the environment
    let e = Env::default();
    e.set_default_info();

    // Setting up the users
    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);
    let user_3 = Address::generate(&e);

    // Setting up the assets
    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    // Setting up the token clients
    let blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    // Setting up soroswap pool
    let pool_admin = Address::generate(&e);
    let amount_a = 100000000_0_000_000;
    let amount_b = 50000000_0_000_000;
    blnd_client.mock_all_auths().mint(&pool_admin, &amount_a);
    usdc_client.mock_all_auths().mint(&pool_admin, &amount_b);

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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 10_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);
    strategy_client
        .mock_auths(&[MockAuth {
            address: &user_2,
            invoke: &MockAuthInvoke {
                contract: &strategy_client.address.clone(),
                fn_name: "deposit",
                args: (starting_balance.clone(), user_2.clone()).into_val(&e),
                sub_invokes: &[MockAuthInvoke {
                    contract: &usdc_client.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user_2.clone(),
                        strategy_client.address.clone(),
                        starting_balance.clone(),
                    )
                        .into_val(&e),
                    sub_invokes: &[],
                }],
            },
        }])
        .deposit(&starting_balance, &user_2);

    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, starting_balance);
    assert_eq!(user_balance, 0);

    let withdraw_amount = 8_0_000_000i128;
    let result = strategy_client
        .mock_auths(&[MockAuth {
            address: &user_3,
            invoke: &MockAuthInvoke {
                contract: &strategy_client.address.clone(),
                fn_name: "withdraw",
                args: (withdraw_amount.clone(), user_2.clone(), user_2.clone()).into_val(&e),
                sub_invokes: &[MockAuthInvoke {
                    contract: &usdc_client.address.clone(),
                    fn_name: "transfer",
                    args: (
                        strategy_client.address.clone(),
                        user_2.clone(),
                        withdraw_amount.clone(),
                    )
                        .into_val(&e),
                    sub_invokes: &[],
                }],
            },
        }])
        .try_withdraw(&withdraw_amount, &user_2, &user_2);

    assert_eq!(result, Err(Err(soroban_sdk::InvokeError::Abort)));
}

#[test]
fn arithmetic_error_deposit() {
    // Setting up the environment
    let e = Env::default();
    e.set_default_info();
    let admin = Address::generate(&e);

    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());

    let blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    let pool_admin = Address::generate(&e);
    let amount_a = 100000000_0_000_000;
    let amount_b = 50000000_0_000_000;
    blnd_client.mock_all_auths().mint(&pool_admin, &amount_a);
    usdc_client.mock_all_auths().mint(&pool_admin, &amount_b);

    let soroswap_router = create_soroswap_pool(
        &e,
        &pool_admin,
        &blnd.address(),
        &usdc.address(),
        &amount_a,
        &amount_b,
    );
    let blend_fixture = BlendFixture::deploy(&e, &admin, &blnd.address(), &usdc.address());

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &0u32,
        &blnd.address(),
        &soroswap_router.address,
    );

    let reserve = StrategyReserves {
        total_shares: 0,
        total_b_tokens: 0,
        b_rate: 0,
    };
    
    let from = Address::generate(&e);
    let underlying_amount = i128::MAX;
    let b_tokens_amount = 1;

    
    let result = e.as_contract(&&strategy, || reserves::deposit(&e, reserve, &from, underlying_amount, b_tokens_amount ));

    assert_eq!(result, Err(StrategyError::ArithmeticError));
}