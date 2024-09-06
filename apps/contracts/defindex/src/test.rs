#![cfg(test)]
extern crate std;
use crate::storage::StrategyParams;
use crate::{DeFindexVault, DeFindexVaultClient};
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
    Vec,
    vec as sorobanvec,
    String
};
use std::vec;

// DeFindex Vault Contract
fn create_defindex_vault<'a>(e: &Env) -> DeFindexVaultClient<'a> {
    DeFindexVaultClient::new(e, &e.register_contract(None, DeFindexVault {}))
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

pub(crate) fn create_strategy_params(test: &DeFindexVaultTest) -> Vec<StrategyParams> {
    sorobanvec![
        &test.env,
        StrategyParams {
            name: String::from_str(&test.env, "Strategy 1"),
            address: test.adapter_address.clone(),
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
    manager: Address,
    adapter_address: Address,
}

impl<'a> DeFindexVaultTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let defindex_contract = create_defindex_vault(&env);
        
        let emergency_manager = Address::generate(&env);
        let fee_receiver = Address::generate(&env);
        let manager = Address::generate(&env);
        
        let token0_admin = Address::generate(&env);
        let token0 = create_token_contract(&env, &token0_admin);

        let token1_admin = Address::generate(&env);
        let token1 = create_token_contract(&env, &token1_admin);
        
        let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());
        let token1_admin_client = get_token_admin_client(&env, &token1.address.clone());

        // token1_admin_client.mint(to, amount);
        
        //TODO: Adapter mockup (should be an strategy later on)
        let adapter_address = Address::generate(&env);

        DeFindexVaultTest {
            env,
            defindex_contract,
            token0_admin_client,
            token0,
            token1_admin_client,
            token1,
            emergency_manager,
            fee_receiver,
            manager,
            adapter_address
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