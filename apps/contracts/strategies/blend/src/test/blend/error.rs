#![cfg(test)]
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{
    create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils, ONE_DAY_IN_SECONDS,
};
use crate::reserves;

use crate::BlendStrategyClient;
use crate::storage;
use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, MockAuth, MockAuthInvoke};
use soroban_sdk::{Address, Env, IntoVal, Bytes};

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
    let keeper = Address::generate(&e);

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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 100_0000000;
    usdc_client.mint(&user_2, &starting_balance);

    //Trying to deposit below the min dust
    let deposit_negative = strategy_client.try_deposit(&-100_000i128, &user_2);

    assert_eq!(deposit_negative, Err(Ok(StrategyError::OnlyPositiveAmountAllowed)));

    let deposit_zero = strategy_client.try_deposit(&0i128, &user_2);

    assert_eq!(deposit_zero, Err(Ok(StrategyError::OnlyPositiveAmountAllowed)));
}

#[test]
fn deposit_first_depositor_less_than_1000() {
    // Setting up the environment
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    // Setting up the users
    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);
    let user_3 = Address::generate(&e);
    let keeper = Address::generate(&e);
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 100_0000000;
    usdc_client.mint(&user_2, &starting_balance);
    usdc_client.mint(&user_3, &starting_balance);

    //Trying to deposit below the min dust
    let deposit_1000 = strategy_client.try_deposit(&1000i128, &user_2);
    assert_eq!(deposit_1000, Err(Ok(StrategyError::InvalidSharesMinted)));

    // with 1001 is possible
    strategy_client.deposit(&1001i128, &user_2);
    // check balance of user 2
    let balance = strategy_client.balance(&user_2);
    assert_eq!(balance, 1001 - 1000);

    // as contract get reserves
    e.as_contract(&strategy, || {

        let config = storage::get_config(&e).unwrap();
        let reserves=reserves::get_strategy_reserve_updated(&e, &config);
        assert_eq!(reserves.total_shares, 1001);
    }
    );


    // use 3 is the second depositor and he can even deposit 1 unit

    let deposit_1 = strategy_client.try_deposit(&99i128, &user_3);
    assert!(deposit_1.is_ok(), "Expected successful deposit for user 3, got {:?}", deposit_1);

    e.as_contract(&strategy, || {

        let config = storage::get_config(&e).unwrap();
        let reserves=reserves::get_strategy_reserve_updated(&e, &config);
        assert_eq!(reserves.total_shares, 1100);
        assert_eq!(reserves.total_b_tokens, 1100);
        let user_3_shares = storage::get_vault_shares(&e, &user_3);

        // new shares minted are
        // amount * total shares / total b tokens
        // (99*1101)/1101 = 99
        assert_eq!(user_3_shares, 99);
    }
    );
    

    // check balance of user 3
    let balance = strategy_client.balance(&user_3);
    assert_eq!(balance, 99);

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
    let keeper = Address::generate(&e);

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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
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
    let harvest_from_random_address = strategy_client.try_harvest(&strategy_client.address, &None::<Bytes>);
    let balance_after_harvest = strategy_client.balance(&user);

    assert_eq!(balance_before_harvest, balance_after_harvest);
    assert_eq!(harvest_from_random_address,  Err(Ok(StrategyError::NotAuthorized)));
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
    let keeper = Address::generate(&e);
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
    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
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
                args: (keeper.clone(), &None::<Bytes>).into_val(&e),
                sub_invokes: &[],
            },
        }],
    ).try_harvest(&keeper, &None::<Bytes>);

    assert_eq!(try_harvest_result, Err(Err(soroban_sdk::InvokeError::Abort)));

    let harvest_result = strategy_client.try_harvest(&keeper, &None::<Bytes>);
    assert_eq!(harvest_result, Ok(Ok(())));

    //Panic with Unauthorized
    let _result = strategy_client.mock_auths(
        &[MockAuth {
            address: &user,
            invoke: &MockAuthInvoke {
                contract: &strategy_client.address.clone(),
                fn_name: "harvest",
                args: (user_2.clone(), &None::<Bytes>).into_val(&e),
                sub_invokes: &[],
            },
        }],
    ).harvest(&user_2, &None::<Bytes>);
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
    let keeper = Address::generate(&e);
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let result = strategy_client.try_withdraw(&200_0_000_000, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    let starting_balance = 10_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);
    strategy_client.deposit(&starting_balance, &user_2);

    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, starting_balance - 1000);
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
    let keeper = Address::generate(&e);
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    let starting_balance = 10_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);
    strategy_client.deposit(&starting_balance, &user_2);

    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, starting_balance -1000);
    assert_eq!(user_balance, 0);

    // Check negative
    let result = strategy_client.try_withdraw(&-200_0_000_000, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::OnlyPositiveAmountAllowed)));

    // Check zero
    let result = strategy_client.try_withdraw(&0, &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::OnlyPositiveAmountAllowed)));

    // Small is possible, No errors
    let result = strategy_client.try_withdraw(&9_000, &user_2, &user_2);
    assert!(result.is_ok(), "Expected successful withdrawal for small amount, got {:?}", result);}

#[test]
fn unauthorized_withdraw() {
    // Setting up the environment
    let e = Env::default();
    e.set_default_info();

    // Setting up the users
    let admin = Address::generate(&e);
    let user_2 = Address::generate(&e);
    let user_3 = Address::generate(&e);
    let keeper = Address::generate(&e);
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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
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
    assert_eq!(balance, starting_balance - 1000);
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
    let keeper = Address::generate(&e);

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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );

    
    let from = Address::generate(&e);
    let b_tokens_amount = 0;

    let config = e.as_contract(&strategy, || storage::get_config(&e)).unwrap();
    let result = e.as_contract(&strategy, || reserves::deposit(&e, &from, b_tokens_amount, &config));

    assert_eq!(result, Err(StrategyError::BTokensAmountBelowMin));
}

#[test]
fn set_keeper_unauthorized() {
    // Setting up the environment
    let e = Env::default();
    e.set_default_info();
    let admin = Address::generate(&e);
    let keeper = Address::generate(&e);
    let attacker = Address::generate(&e);
    let new_keeper = Address::generate(&e);

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

    let pool = create_blend_pool(&e, &blend_fixture, &admin, &usdc_client, &xlm_client, &blnd_client);
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    
    let strategy_client = BlendStrategyClient::new(&e, &strategy);
    
    // Verify the current keeper
    let current_keeper = strategy_client.get_keeper();
    assert_eq!(current_keeper, keeper);
    
    // Attacker tries to set a new keeper without proper authorization
    // We don't use mock_all_auths() here to test the authorization check
    let result = strategy_client.mock_auths(
        &[MockAuth {
            address: &attacker,
            invoke: &MockAuthInvoke {
                contract: &strategy_client.address.clone(),
                fn_name: "set_keeper",
                args: (keeper.clone(), new_keeper.clone()).into_val(&e),
                sub_invokes: &[],
            },
        }],
    ).try_set_keeper(&new_keeper);
    
    // Should abort as failed in require_auth
    assert_eq!(result, Err(Err(soroban_sdk::InvokeError::Abort)));
    
    // Verify the keeper hasn't changed
    assert_eq!(strategy_client.get_keeper(), keeper);
}
