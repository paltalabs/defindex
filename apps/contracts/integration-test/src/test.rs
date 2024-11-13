#![cfg(test)]
extern crate std;
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{BytesN, Val};
use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
    Vec,
    vec as sorobanvec,
    String
};
use std::vec;

use crate::hodl_strategy::{create_strategy_contract, StrategyContractClient};
use crate::token;

// pub(crate) fn create_asset_params(test: &DeFindexFactoryTest) -> Vec<AssetStrategySet> {
//     sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token0.address.clone(),
//             strategies: sorobanvec![
//                 &test.env,
//                 Strategy {
//                     address: test.strategy_contract_token0.address.clone(),
//                     name: String::from_str(&test.env, "Strategy 1"),
//                     paused: false,
//                 }
//             ],
//         },
//         AssetStrategySet {
//             address: test.token1.address.clone(),
//             strategies: sorobanvec![
//                 &test.env,
//                 Strategy {
//                     address: test.strategy_contract_token1.address.clone(),
//                     name: String::from_str(&test.env, "Strategy 1"),
//                     paused: false,
//                 }
//             ],
//         }
//     ]
// }

pub struct DeFindexFactoryTest<'a> {
    env: Env,
    // factory_contract: DeFindexFactoryClient<'a>,
    // admin: Address,
    // defindex_receiver: Address,
    // defindex_wasm_hash: BytesN<32>,
    // emergency_manager: Address,
    // fee_receiver: Address,
    // manager: Address,
    // token0_admin_client: SorobanTokenAdminClient<'a>,
    // token0: SorobanTokenClient<'a>,
    token_admin_client: SorobanTokenAdminClient<'a>,
    // token1: SorobanTokenClient<'a>,
    strategy_contract: StrategyContractClient<'a>,
    // strategy_contract_token1: StrategyContractClient<'a>,
}

impl<'a> DeFindexFactoryTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        // let factory_contract = create_defindex_factory(&env);
        
        // let admin = Address::generate(&env);
        // let defindex_receiver = Address::generate(&env);

        // let defindex_wasm_hash = env.deployer().upload_contract_wasm(defindex_vault_contract::WASM);

        // let emergency_manager = Address::generate(&env);
        // let fee_receiver = Address::generate(&env);
        // let manager = Address::generate(&env);

        // let token0_admin = Address::generate(&env);
        // let token0 = create_token_contract(&env, &token0_admin);

        let token_admin = Address::generate(&env);
        // let token1 = create_token_contract(&env, &token1_admin);
        
        // let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());
        // let token1_admin_client = get_token_admin_client(&env, &token1.address.clone());
        let (token, token_admin_client) = token::create_token(&env, &token_admin);

        // // TODO: Add a strategy adapter, this is a mockup
        let strategy_contract = create_strategy_contract(&env, &token.address, &Vec::new(&env));
        // let strategy_contract_token1 = create_strategy_contract(&env, &token1.address, &Vec::new(&env));
        env.budget().reset_unlimited();

        DeFindexFactoryTest {
            env,
            // factory_contract,
            // admin,
            // defindex_receiver,
            // defindex_wasm_hash,
            // emergency_manager,
            // fee_receiver,
            // manager,
            // token0_admin_client,
            // token0,
            token_admin_client,
            // token1,
            strategy_contract,
            // strategy_contract_token1
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