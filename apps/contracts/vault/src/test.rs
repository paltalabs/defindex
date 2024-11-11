#![cfg(test)]
extern crate std;
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{testutils::Address as _, vec as sorobanvec, Address, Env, String, Vec};
use std::vec;

// DeFindex Hodl Strategy Contract
pub mod hodl_strategy {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm"
    );
    pub type HodlStrategyClient<'a> = Client<'a>;
}
use hodl_strategy::HodlStrategyClient;

fn create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let contract_address = &e.register_contract_wasm(None, hodl_strategy::WASM);
    let hodl_strategy = HodlStrategyClient::new(e, contract_address);
    hodl_strategy.initialize(&asset, &sorobanvec![&e]);
    hodl_strategy
}

// DeFindex Vault Contract
pub mod defindex_vault {
    soroban_sdk::contractimport!(
        file = "../target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm"
    );
    pub type DeFindexVaultClient<'a> = Client<'a>;
}
use defindex_vault::{DeFindexVaultClient, Strategy};

fn create_defindex_vault<'a>(e: &Env) -> DeFindexVaultClient<'a> {
    let address = &e.register_contract_wasm(None, defindex_vault::WASM);
    let client = DeFindexVaultClient::new(e, address);
    client
}

// DeFindex Factory Contract
// pub mod defindex_factory {
//     soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_factory.optimized.wasm");
//     pub type DeFindexFactoryClient<'a> = Client<'a>;
// }
// use defindex_factory::DeFindexFactoryClient;

// fn create_defindex_factory<'a>(
//     e: & Env
// ) -> DeFindexFactoryClient<'a> {
//     let address = &e.register_contract_wasm(None, defindex_factory::WASM);
//     let client = DeFindexFactoryClient::new(e, address);
//     client
// }

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

pub(crate) fn create_strategy_params_token0(test: &DeFindexVaultTest) -> Vec<Strategy> {
    sorobanvec![
        &test.env,
        Strategy {
            name: String::from_str(&test.env, "Strategy 1"),
            address: test.strategy_client_token0.address.clone(),
            paused: false,
        }
    ]
}

pub(crate) fn create_strategy_params_token1(test: &DeFindexVaultTest) -> Vec<Strategy> {
    sorobanvec![
        &test.env,
        Strategy {
            name: String::from_str(&test.env, "Strategy 1"),
            address: test.strategy_client_token1.address.clone(),
            paused: false,
        }
    ]
}

pub struct DeFindexVaultTest<'a> {
    env: Env,
    defindex_factory: Address,
    defindex_contract: DeFindexVaultClient<'a>,
    token0_admin_client: SorobanTokenAdminClient<'a>,
    token0: SorobanTokenClient<'a>,
    token0_admin: Address,
    token1_admin_client: SorobanTokenAdminClient<'a>,
    token1: SorobanTokenClient<'a>,
    token1_admin: Address,
    emergency_manager: Address,
    vault_fee_receiver: Address,
    defindex_protocol_receiver: Address,
    manager: Address,
    strategy_client_token0: HodlStrategyClient<'a>,
    strategy_client_token1: HodlStrategyClient<'a>,
}

impl<'a> DeFindexVaultTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        // env.mock_all_auths();

        // Mockup, should be the factory contract
        let defindex_factory = Address::generate(&env);

        let defindex_contract = create_defindex_vault(&env);

        let emergency_manager = Address::generate(&env);
        let vault_fee_receiver = Address::generate(&env);
        let defindex_protocol_receiver = Address::generate(&env);
        let manager = Address::generate(&env);

        let token0_admin = Address::generate(&env);
        let token0 = create_token_contract(&env, &token0_admin);

        let token1_admin = Address::generate(&env);
        let token1 = create_token_contract(&env, &token1_admin);

        let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());
        let token1_admin_client = get_token_admin_client(&env, &token1.address.clone());

        // token1_admin_client.mint(to, amount);

        let strategy_client_token0 = create_hodl_strategy(&env, &token0.address);
        let strategy_client_token1 = create_hodl_strategy(&env, &token1.address);

        env.budget().reset_unlimited();
        
        DeFindexVaultTest {
            env,
            defindex_factory,
            defindex_contract,
            token0_admin_client,
            token0,
            token0_admin,
            token1_admin_client,
            token1,
            token1_admin,
            emergency_manager,
            vault_fee_receiver,
            defindex_protocol_receiver,
            manager,
            strategy_client_token0,
            strategy_client_token1,
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

mod initialize;
mod deposit;
mod admin;
mod invest;
// mod emergency_withdraw;
// mod rebalance;
// mod withdraw;
