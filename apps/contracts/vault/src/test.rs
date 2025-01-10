#![cfg(test)]
extern crate std;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke}, token::{StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient}, vec as sorobanvec, Address, Env, IntoVal, Map, String, Val, Vec
};
use std::vec;

use soroswap_setup::{
    create_soroswap_factory, create_soroswap_pool, create_soroswap_router, SoroswapRouterClient,
};

// DeFindex Hodl Strategy Contract
pub mod hodl_strategy {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm"
    );
    pub type HodlStrategyClient<'a> = Client<'a>;
}
use hodl_strategy::HodlStrategyClient;

pub fn create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val> = sorobanvec![e];
    let args = (asset, init_args);
    HodlStrategyClient::new(e, &e.register(hodl_strategy::WASM, args))
}
pub mod fixed_strategy {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/fixed_apr_strategy.optimized.wasm"
    );
    pub type FixedStrategyClient<'a> = Client<'a>;
}
use fixed_strategy::FixedStrategyClient;

pub fn create_fixed_strategy<'a>(e: &Env, asset: &Address) -> FixedStrategyClient<'a> {
    let apr_bps = 1000u32;
    let init_args: Vec<Val> = sorobanvec![e, apr_bps.into_val(e)];
    let args = (asset, init_args);
    FixedStrategyClient::new(e, &e.register(fixed_strategy::WASM, args))
}

// DeFindex Vault Contract
pub mod defindex_vault {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm"
    );
    pub type DeFindexVaultClient<'a> = Client<'a>;
}
use defindex_vault::{AssetStrategySet, DeFindexVaultClient, Strategy};

pub fn create_defindex_vault<'a>(
    e: &Env,
    assets: Vec<AssetStrategySet>,
    roles: Map<u32, Address>,
    vault_fee: u32,
    defindex_protocol_receiver: Address,
    defindex_protocol_rate: u32,
    factory: Address,
    soroswap_router: Address,
    name_symbol: Map<String, String>,
    upgradable: bool,
) -> DeFindexVaultClient<'a> {
    let args = (
        assets,
        roles,
        vault_fee,
        defindex_protocol_receiver,
        defindex_protocol_rate,
        factory,
        soroswap_router,
        name_symbol,
        upgradable
    );
    let address = &e.register(defindex_vault::WASM, args);
    let client = DeFindexVaultClient::new(e, address);
    client
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> SorobanTokenClient<'a> {
    SorobanTokenClient::new(
        e,
        &e.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}

pub(crate) fn get_token_admin_client<'a>(
    e: &Env,
    address: &Address,
) -> SorobanTokenAdminClient<'a> {
    SorobanTokenAdminClient::new(e, address)
}

pub(crate) fn create_strategy_params_token_0(test: &DeFindexVaultTest) -> Vec<Strategy> {
    sorobanvec![
        &test.env,
        Strategy {
            name: String::from_str(&test.env, "Strategy 1"),
            address: test.strategy_client_token_0.address.clone(),
            paused: false,
        }
    ]
}

pub(crate) fn create_strategy_params_token_1(test: &DeFindexVaultTest) -> Vec<Strategy> {
    sorobanvec![
        &test.env,
        Strategy {
            name: String::from_str(&test.env, "Strategy 1"),
            address: test.strategy_client_token_1.address.clone(),
            paused: false,
        }
    ]
}

pub fn mock_mint(
    env: &Env,
    token_admin_client: &SorobanTokenAdminClient,
    token_admin: &Address,
    to: &Address,
    amount: &i128,
) {
    token_admin_client
        .mock_auths(&[MockAuth {
            address: &token_admin,
            invoke: &MockAuthInvoke {
                contract: &token_admin_client.address.clone(),
                fn_name: "mint",
                args: sorobanvec![&env, to.into_val(env), amount.into_val(env)],
                sub_invokes: &[],
            },
        }])
        .mint(&to, &amount);
}

pub struct DeFindexVaultTest<'a> {
    env: Env,
    defindex_factory: Address,
    token_0_admin_client: SorobanTokenAdminClient<'a>,
    token_0: SorobanTokenClient<'a>,
    token_1_admin_client: SorobanTokenAdminClient<'a>,
    token_1: SorobanTokenClient<'a>,
    emergency_manager: Address,
    vault_fee_receiver: Address,
    defindex_protocol_receiver: Address,
    manager: Address,
    rebalance_manager: Address,
    strategy_client_token_0: HodlStrategyClient<'a>,
    strategy_client_token_1: HodlStrategyClient<'a>,
    fixed_strategy_client_token_0: FixedStrategyClient<'a>,
    fixed_strategy_client_token_1: FixedStrategyClient<'a>,
    soroswap_router: SoroswapRouterClient<'a>,
    // soroswap_factory: SoroswapFactoryClient<'a>,
    // soroswap_pair: Address,
}

impl<'a> DeFindexVaultTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        // env.mock_all_auths();

        // Mockup, should be the factory contract
        let defindex_factory = Address::generate(&env);
        env.budget().reset_unlimited();
        let emergency_manager = Address::generate(&env);
        let vault_fee_receiver = Address::generate(&env);
        let defindex_protocol_receiver = Address::generate(&env);
        let manager = Address::generate(&env);
        let rebalance_manager = Address::generate(&env);

        let token_0_admin = Address::generate(&env);
        let token_0 = create_token_contract(&env, &token_0_admin);

        let token_1_admin = Address::generate(&env);
        let token_1 = create_token_contract(&env, &token_1_admin);

        let token_0_admin_client = get_token_admin_client(&env, &token_0.address.clone());
        let token_1_admin_client = get_token_admin_client(&env, &token_1.address.clone());

        // token_1_admin_client.mint(to, amount);
        env.budget().reset_unlimited();
        let strategy_client_token_0 = create_hodl_strategy(&env, &token_0.address.clone());
        let strategy_client_token_1 = create_hodl_strategy(&env, &token_1.address.clone());

        let fixed_strategy_client_token_0 = create_fixed_strategy(&env, &token_0.address.clone());
        let fixed_strategy_client_token_1 = create_fixed_strategy(&env, &token_1.address.clone());
        env.budget().reset_unlimited();
        // Soroswap Setup
        let soroswap_admin = Address::generate(&env);

        let amount_0: i128 = 1_000_000_000_000_000_000;
        let amount_1: i128 = 4_000_000_000_000_000_000;

        mock_mint(
            &env,
            &token_0_admin_client,
            &token_0_admin,
            &soroswap_admin,
            &amount_0,
        );
        mock_mint(
            &env,
            &token_1_admin_client,
            &token_1_admin,
            &soroswap_admin,
            &amount_1,
        );

        let soroswap_factory = create_soroswap_factory(&env, &soroswap_admin);
        let soroswap_router = create_soroswap_router(&env, &soroswap_factory.address);

        env.budget().reset_unlimited();

        create_soroswap_pool(
            &env,
            &soroswap_router,
            &soroswap_admin,
            &token_0.address,
            &token_1.address,
            &amount_0,
            &amount_1,
        );
        // let soroswap_pair = soroswap_factory.get_pair(&token_0.address, &token_1.address);

        env.budget().reset_unlimited();

        DeFindexVaultTest {
            env,
            defindex_factory,
            token_0_admin_client,
            token_0,
            token_1_admin_client,
            token_1,
            emergency_manager,
            vault_fee_receiver,
            defindex_protocol_receiver,
            manager,
            rebalance_manager,
            strategy_client_token_0,
            strategy_client_token_1,
            fixed_strategy_client_token_0,
            fixed_strategy_client_token_1,
            soroswap_router,
        }
    }

    pub(crate) fn generate_random_users(e: &Env, users_count: u32) -> vec::Vec<Address> {
        let mut users = vec![];
        for _c in 0..users_count {
            users.push(Address::generate(e));
        }
        users
    }
}

mod soroswap_setup;
mod vault;
