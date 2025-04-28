#![cfg(test)]
use crate::{HodlStrategy, HodlStrategyClient};

use soroban_sdk::token::{StellarAssetClient, TokenClient};

use soroban_sdk::{testutils::Address as _, Address, Env};
use soroban_sdk::{vec, Val, Vec};

// Base Strategy Contract
pub fn _create_hodl_strategy<'a>(e: &Env, asset: &Address) -> HodlStrategyClient<'a> {
    let init_args: Vec<Val> = vec![e];

    let args = (asset, init_args);
    HodlStrategyClient::new(e, &e.register(HodlStrategy, args))
}

// Create Test Token
pub(crate) fn _create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(
        e,
        &e.register_stellar_asset_contract_v2(admin.clone())
            .address(),
    )
}

pub struct _HodlStrategyTest<'a> {
    env: Env,
    token: TokenClient<'a>,
    user: Address,
    user1: Address,
}

impl<'a> _HodlStrategyTest<'a> {
    fn _setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let token = _create_token_contract(&env, &admin);
        let user = Address::generate(&env);
        let user1 = Address::generate(&env);

        // Mint 1,000,000,000 to user
        StellarAssetClient::new(&env, &token.address).mint(&user, &1_000_000_000);
        // Mint 1,000,000,000 to user1
        StellarAssetClient::new(&env, &token.address).mint(&user1, &1_000_000_000);

        _HodlStrategyTest {
            env,
            token,
            user,
            user1,
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

mod unsafe_hodl;
