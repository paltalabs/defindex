#![cfg(test)]

// Standard library imports
use soroban_sdk::{testutils::Address as _, Address, Env};

// External crate imports
use sep_41_token::testutils::MockTokenClient;

// Local module imports
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{
    create_blend_pool, register_blend_strategy, BlendFixture, EnvTestUtils
};
use crate::BlendStrategyClient;

#[allow(dead_code)]
pub struct GenericBlendData<'a> {
  pub e: Env,
  pub blnd: Address,
  pub usdc: Address,
  pub xlm: Address,
  pub blnd_client: MockTokenClient<'a>,
  pub usdc_client: MockTokenClient<'a>,
  pub xlm_client: MockTokenClient<'a>,
  pub pool_admin: Address,
  pub soroswap_router: Address,
  pub pool: Address,
  pub address: Address,
  pub reserve_blnd: i128,
  pub reserve_usdc: i128,
  pub keeper: Address,
}

pub fn create_generic_strategy() -> GenericBlendData<'static> {
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

    let reward_threshold = 100i128;
    let strategy_address = register_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        reward_threshold,
        &keeper,
    );
    GenericBlendData {
        e: e.clone(),
        blnd: blnd.address(),
        usdc: usdc.address(),
        xlm: xlm.address(),
        blnd_client,
        usdc_client,
        xlm_client,
        pool_admin,
        soroswap_router: soroswap_router.address,
        pool,
        address: strategy_address,
        reserve_blnd: amount_a,
        reserve_usdc: amount_b,
        keeper,
    }
}

pub fn mint_and_deposit_to_strategy(e: &GenericBlendData, user: &Address, amount: i128) {
    e.usdc_client.mint(user, &amount);

    let strategy_client = BlendStrategyClient::new(&e.e, &e.address);
    strategy_client.deposit(&amount, user);
}