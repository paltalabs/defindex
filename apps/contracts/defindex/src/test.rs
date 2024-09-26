#![cfg(test)]
extern crate std;
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{testutils::Address as _, vec as sorobanvec, Address, Env, String, Vec};
use std::vec;

// DeFindex Hodl Strategy Contract
mod hodl_strategy {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm");
    pub type HodlStrategyClient<'a> = Client<'a>;
}
use hodl_strategy::HodlStrategyClient;

fn create_hodl_strategy<'a>(e: & Env, asset: & Address) -> HodlStrategyClient<'a> {
    let contract_address = &e.register_contract_wasm(None, hodl_strategy::WASM);
    let hodl_strategy = HodlStrategyClient::new(e, contract_address); 
    hodl_strategy.initialize(&asset, &sorobanvec![&e]);
    hodl_strategy
}

// DeFindex Vault Contract
pub mod defindex_vault {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm");
    pub type DeFindexVaultClient<'a> = Client<'a>;
}
use defindex_vault::{DeFindexVaultClient, Strategy};

fn create_defindex_vault<'a>(
    e: & Env
) -> DeFindexVaultClient<'a> {
    let address = &e.register_contract_wasm(None, defindex_vault::WASM);
    let client = DeFindexVaultClient::new(e, address);
    client
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> SorobanTokenClient<'a> {
    SorobanTokenClient::new(e, &e.register_stellar_asset_contract(admin.clone()))
}

pub(crate) fn get_token_admin_client<'a>(
    e: &Env,
    address: &Address,
) -> SorobanTokenAdminClient<'a> {
    SorobanTokenAdminClient::new(e, address)
}

pub(crate) fn create_strategy_params(test: &DeFindexVaultTest) -> Vec<Strategy> {
    sorobanvec![
        &test.env,
        Strategy {
            name: String::from_str(&test.env, "Strategy 1"),
            address: test.strategy_client.address.clone(),
            paused: false
        }
    ]
}

pub struct DeFindexVaultTest<'a> {
    env: Env,
    defindex_contract: DeFindexVaultClient<'a>,
    token0_admin_client: SorobanTokenAdminClient<'a>,
    token0: SorobanTokenClient<'a>,
    token1_admin_client: SorobanTokenAdminClient<'a>,
    token1: SorobanTokenClient<'a>,
    emergency_manager: Address,
    fee_receiver: Address,
    defindex_receiver: Address,
    manager: Address,
    strategy_client: HodlStrategyClient<'a>,
}

impl<'a> DeFindexVaultTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        // env.mock_all_auths();
        let defindex_contract = create_defindex_vault(&env);

        let emergency_manager = Address::generate(&env);
        let fee_receiver = Address::generate(&env);
        let defindex_receiver = Address::generate(&env);
        let manager = Address::generate(&env);

        let token0_admin = Address::generate(&env);
        let token0 = create_token_contract(&env, &token0_admin);

        let token1_admin = Address::generate(&env);
        let token1 = create_token_contract(&env, &token1_admin);

        let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());
        let token1_admin_client = get_token_admin_client(&env, &token1.address.clone());

        // token1_admin_client.mint(to, amount);

        let strategy_client = create_hodl_strategy(&env, &token0.address);

        DeFindexVaultTest {
            env,
            defindex_contract,
            token0_admin_client,
            token0,
            token1_admin_client,
            token1,
            emergency_manager,
            fee_receiver,
            defindex_receiver,
            manager,
            strategy_client,
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

mod admin;
mod initialize;
mod withdraw;
