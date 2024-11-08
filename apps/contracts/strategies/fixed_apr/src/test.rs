#![cfg(test)]
use crate::{FixAprStrategy, FixAprStrategyClient, StrategyError};

use soroban_sdk::token::{TokenClient, StellarAssetClient};

use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
};

// Base Strategy Contract
fn create_fixapr_strategy<'a>(e: &Env) -> FixAprStrategyClient<'a> {
    FixAprStrategyClient::new(e, &e.register_contract(None, FixAprStrategy {}))
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

pub struct FixAprStrategyTest<'a> {
    env: Env,
    strategy: FixAprStrategyClient<'a>,
    token: TokenClient<'a>,
    user: Address,
}

impl<'a> FixAprStrategyTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();

        let strategy = create_fixapr_strategy(&env);
        let admin = Address::generate(&env);
        let token = create_token_contract(&env, &admin);
        let user = Address::generate(&env);

        // Mint 1,000,000,000 to user
        StellarAssetClient::new(&env, &token.address).mint(&user, &1_000_000_000);

        FixAprStrategyTest {
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
mod events;
mod withdraw;