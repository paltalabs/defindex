#![cfg(test)]
use crate::{BlendStrategy, BlendStrategyClient, StrategyError};

use soroban_sdk::token::{TokenClient, StellarAssetClient};

use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
};

// mod blend_pool_module {
//     soroban_sdk::contractimport!(file = "../external_wasms/blend/blend_pool.wasm");
//     pub type BlendPoolContractClient<'a> = Client<'a>;
// }

// use blend_pool_module::BlendPoolContractClient;

// // fn initialize(admin: address, name: string, oracle: address, bstop_rate: u32, max_postions: u32, backstop_id: address, blnd_id: address)

// fn create_blend_pool_contract<'a>(e: &Env, asset: &Address, init_args: &Vec<Val>) -> BlendPoolContractClient<'a> {
//     let address = &e.register_contract_wasm(None, hodl_strategy::WASM);
//     let strategy = BlendPoolContractClient::new(e, address); 
//     strategy.initialize(asset, init_args);
//     strategy
// }  

// Blend Strategy Contract
fn create_blend_strategy<'a>(e: &Env) -> BlendStrategyClient<'a> {
    BlendStrategyClient::new(e, &e.register_contract(None, BlendStrategy {}))
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

pub struct HodlStrategyTest<'a> {
    env: Env,
    strategy: BlendStrategyClient<'a>,
    token: TokenClient<'a>,
    user: Address,
}

impl<'a> HodlStrategyTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();

        let strategy = create_blend_strategy(&env);
        let admin = Address::generate(&env);
        let token = create_token_contract(&env, &admin);
        let user = Address::generate(&env);

        // Mint 1,000,000,000 to user
        StellarAssetClient::new(&env, &token.address).mint(&user, &1_000_000_000);

        HodlStrategyTest {
            env,
            strategy,
            token,
            user
        }
    }
    
    // pub(crate) fn generate_random_users(e: &Env, users_count: u32) -> vec::Vec<Address> {
    //     let mut users = vec![];
    //     for _c in 0..users_count {
    //         users.push(Address::generate(e));
    //     }
    //     users
    // }
}

// mod initialize;
// mod deposit;
// mod events;
// mod withdraw;