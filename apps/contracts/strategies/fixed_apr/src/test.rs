#![cfg(test)]
extern crate std;
use crate::{FixAprStrategy, FixAprStrategyClient, StrategyError};

use soroban_sdk::token::{StellarAssetClient, TokenClient};

use soroban_sdk::{vec, Val, Vec};
use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
    IntoVal,
};

use std::vec as stdvec;

// Base Strategy Contract
pub fn create_fixapr_strategy<'a>(e: &Env, asset: &Address, apr_bps: u32, token: &Address) -> FixAprStrategyClient<'a> {
    let init_args: Vec<Val>= vec![e, apr_bps.into_val(e)];

    let args = (asset, init_args);
    let client = FixAprStrategyClient::new(e, &e.register(FixAprStrategy, args));
    
    // Mint 100M to the strategy
    let starting_amount = 100_000_000_000_0_000_000i128;
    StellarAssetClient::new(e, token).mint(&client.address, &starting_amount);

    client
}

// Create Test Token
pub(crate) fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

pub struct FixAprStrategyTest<'a> {
    env: Env,
    token: TokenClient<'a>,
}

impl<'a> FixAprStrategyTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = create_token_contract(&env, &admin);

        FixAprStrategyTest {
            env,
            token,
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