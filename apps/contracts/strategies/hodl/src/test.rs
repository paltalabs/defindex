#![cfg(test)]
use crate::{HodlStrategy, HodlStrategyClient, StrategyError};

use soroban_sdk::token::{TokenClient, StellarAssetClient};

use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
};

// Base Strategy Contract
fn create_hodl_strategy<'a>(e: &Env) -> HodlStrategyClient<'a> {
    HodlStrategyClient::new(e, &e.register_contract(None, HodlStrategy {}))
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

pub struct HodlStrategyTest<'a> {
    env: Env,
    strategy: HodlStrategyClient<'a>,
    token: TokenClient<'a>,
    user: Address,
}

impl<'a> HodlStrategyTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();

        let strategy = create_hodl_strategy(&env);
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

mod initialize;
mod deposit;