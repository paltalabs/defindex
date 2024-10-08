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

// DeFindex Hodl Strategy Contract
mod hodl_strategy {
  soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm");
  pub type StrategyContractClient<'a> = Client<'a>;
}

use hodl_strategy::StrategyContractClient;

fn create_strategy_contract<'a>(e: & Env, asset: &Address, init_args: &Vec<Val>) -> StrategyContractClient<'a> {
    let address = &e.register_contract_wasm(None, hodl_strategy::WASM);
    let strategy = StrategyContractClient::new(e, address); 
    strategy.initialize(asset, init_args);
    strategy
}

// DeFindex Factory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_factory.optimized.wasm");
    pub type FactoryClient<'a> = Client<'a>;
}
  
use factory::FactoryClient;

fn create_defindex_factory<'a>(e: &Env) -> FactoryClient<'a> {
    FactoryClient::new(e, &e.register_contract_wasm(None, factory::WASM))
}

// DeFindex Vault Contract
// mod defindex_vault_contract {
//   soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm");
// }

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

// pub(crate) fn create_asset_params(test: &DeFindexIntegrationTest) -> Vec<AssetAllocation> {
//     sorobanvec![
//         &test.env,
//         AssetAllocation {
//             address: test.token0.address.clone(),
//             ratio: 1i128,
//             strategies: sorobanvec![
//                 &test.env,
//                 Strategy {
//                     address: test.strategy_address.clone(),
//                     name: String::from_str(&test.env, "Strategy 1"),
//                     paused: false,
//                     ratio: 1i128
//                 }
//             ],
//         }
//     ]
// }

pub struct DeFindexIntegrationTest<'a> {
    env: Env,
    strategy_contract_token_0: StrategyContractClient<'a>,
    strategy_contract_token_1: StrategyContractClient<'a>,
    // factory_contract: DeFindexFactoryClient<'a>,
    // admin: Address,
    // defindex_receiver: Address,
    // defindex_wasm_hash: BytesN<32>,
    // emergency_manager: Address,
    // fee_receiver: Address,
    // manager: Address,
    token0_admin_client: SorobanTokenAdminClient<'a>,
    token0: SorobanTokenClient<'a>,
    token1_admin_client: SorobanTokenAdminClient<'a>,
    token1: SorobanTokenClient<'a>,
    // strategy_address: Address,
}

impl<'a> DeFindexIntegrationTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        
        // TOKENS SETUP
        let token0_admin = Address::generate(&env);
        let token0 = create_token_contract(&env, &token0_admin);

        let token1_admin = Address::generate(&env);
        let token1 = create_token_contract(&env, &token1_admin);
        
        let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());
        let token1_admin_client = get_token_admin_client(&env, &token1.address.clone());

        // 2 create two strategy contract for each token
        let strategy_contract_token_0 = create_strategy_contract(&env, &token0.address, &sorobanvec![&env,]);
        let strategy_contract_token_1 = create_strategy_contract(&env, &token1.address, &sorobanvec![&env,]);

        // 3 create the factory contract


        // 4 deploy a defindex using the factory




        // let factory_contract = create_defindex_factory(&env);
        
        // let admin = Address::generate(&env);
        // let defindex_receiver = Address::generate(&env);

        // let defindex_wasm_hash = env.deployer().upload_contract_wasm(defindex_vault_contract::WASM);

        // let emergency_manager = Address::generate(&env);
        // let fee_receiver = Address::generate(&env);
        // let manager = Address::generate(&env);


        // // TODO: Add a strategy adapter, this is a mockup
        // let strategy_address = Address::generate(&env);

        DeFindexIntegrationTest {
            env,
            strategy_contract_token_0,
            strategy_contract_token_1,
            // factory_contract,
            // admin,
            // defindex_receiver,
            // defindex_wasm_hash,
            // emergency_manager,
            // fee_receiver,
            // manager,
            token0_admin_client,
            token0,
            token1_admin_client,
            token1,
            // strategy_address
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

// mod admin;
// mod initialize;
// mod create_defindex;

