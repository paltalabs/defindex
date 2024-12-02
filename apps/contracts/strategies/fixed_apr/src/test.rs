#![cfg(test)]
extern crate std;
use crate::{FixAprStrategy, FixAprStrategyClient, StrategyError};

use soroban_sdk::token::TokenClient;

use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
};

use std::vec as stdvec;

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
    strategy_admin: Address,
}

impl<'a> FixAprStrategyTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();

        let strategy = create_fixapr_strategy(&env);
        let admin = Address::generate(&env);
        let token = create_token_contract(&env, &admin);

        let strategy_admin = Address::generate(&env);

        FixAprStrategyTest {
            env,
            strategy,
            token,
            strategy_admin
        }
    }
    
    pub(crate) fn generate_random_users(e: &Env, users_count: u32) -> stdvec::Vec<Address> {
        let mut users = stdvec![];
        for _c in 0..users_count {
            users.push(Address::generate(e));
        }
        users
    }
}

mod fixed_apr;