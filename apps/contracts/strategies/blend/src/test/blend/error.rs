#![cfg(test)]
use crate::storage::DAY_IN_LEDGERS;
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils};
use crate::BlendStrategyClient;
use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{Address, Env};

extern crate std;

#[test]
fn deposit_below_min_dust() {
    // Setting up the environment
    let e = Env::default();
    e.budget().reset_unlimited();
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
    
    assert_eq!(deposit_below_min_dust, Err(Ok(StrategyError::AmountBelowMinDust)));
}

#[test]
fn deposit_zero_and_negative_amount(){
    // Setting up the environment
    let e = Env::default();
    e.budget().reset_unlimited();
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
fn harvest_from_random_address(){
    // Setting up the environment
    let e = Env::default();
    e.budget().reset_unlimited();
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

    let starting_balance = 100_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);

    strategy_client.deposit(&starting_balance, &user_2);

    e.jump(DAY_IN_LEDGERS * 22);
    usdc_client.mint(&user_2, &starting_balance);

    strategy_client.deposit(&1_0_000_000, &user_2);


    //Trying to harvest from random address

    let harvest_from_random_address = strategy_client.try_harvest(&Address::generate(&e));
    std::println!("{:?}", harvest_from_random_address);
    let funds = strategy_client.balance(&user_2);
    std::println!("{:?}", funds);
    /*   
    assert_eq!(harvest_from_random_address, Err(Ok(StrategyError::ProtocolAddressNotFound))); */
}

#[test]
fn insufficient_balance(){
    // Setting up the environment
    let e = Env::default();
    e.budget().reset_unlimited();
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

    let result = strategy_client.try_withdraw(&200_0_000_000 , &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));
    
    let starting_balance = 10_0_000_000i128;
    usdc_client.mint(&user_2, &starting_balance);
    strategy_client.deposit(&starting_balance, &user_2);
    
    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, starting_balance);
    assert_eq!(user_balance, 0); 


    let result = strategy_client.try_withdraw(&200_0_000_000 , &Address::generate(&e), &user_2);
    assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    /* 
    let balance = strategy_client.balance(&user_2);
    let user_balance = usdc_client.balance(&user_2);
    assert_eq!(balance, 0);
    assert_eq!(user_balance, starting_balance); */
}

#[test]
fn withdraw_zero_and_negative(){
    // Setting up the environment
    let e = Env::default();
    e.budget().reset_unlimited();
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

    let result = strategy_client.try_withdraw(&-200_0_000_000 , &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::NegativeNotAllowed)));

    let result = strategy_client.try_withdraw(&0 , &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::AmountBelowMinDust)));

    let result = strategy_client.try_withdraw(&9_000 , &user_2, &user_2);
    assert_eq!(result, Err(Ok(StrategyError::AmountBelowMinDust)));
}
