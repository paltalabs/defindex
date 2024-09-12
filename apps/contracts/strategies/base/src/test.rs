#![cfg(test)]
extern crate std;
use crate::{BaseStrategy, BaseStrategyClient};
use soroban_sdk::token::{
    StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient,
};
use soroban_sdk::{IntoVal, Val};
use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
    Vec,
};
use std::vec;

// Base Strategy Contract
fn create_base_strategy<'a>(e: &Env) -> BaseStrategyClient<'a> {
    BaseStrategyClient::new(e, &e.register_contract(None, BaseStrategy {}))
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


pub struct BaseStrategyTest<'a> {
    env: Env,
    strategy: BaseStrategyClient<'a>,
    token0_admin_client: SorobanTokenAdminClient<'a>,
    token0: SorobanTokenClient<'a>,
}

impl<'a> BaseStrategyTest<'a> {
    fn setup() -> Self {

        let env = Env::default();
        env.mock_all_auths();
        let strategy = create_base_strategy(&env);
        
        let token0_admin = Address::generate(&env);
        let token0 = create_token_contract(&env, &token0_admin);
        let token0_admin_client = get_token_admin_client(&env, &token0.address.clone());

        let init_fn_args: Vec<Val> = (0,).into_val(&env);
        strategy.initialize(&token0.address, &init_fn_args);

        BaseStrategyTest {
            env,
            strategy,
            token0_admin_client,
            token0,
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

#[test]
fn test_deposit_and_withdrawal_flow() {
    let test = BaseStrategyTest::setup();
    let users = BaseStrategyTest::generate_random_users(&test.env, 1);

    let amount: i128 = 10_000_000;
    // Minting token 0 to the user
    test.token0_admin_client.mint(&users[0], &amount);

    // Reading user 0 balance
    let balance = test.token0.balance(&users[0]);
    assert_eq!(balance, amount);

    // Depositing token 0 to the strategy from user
    test.strategy.deposit(&amount, &users[0]);

    // Reading user 0 balance
    let balance = test.token0.balance(&users[0]);
    assert_eq!(balance, 0);

    // Reading strategy balance
    let balance = test.token0.balance(&test.strategy.address);
    assert_eq!(balance, amount);

    // Reading user balance on strategy contract
    let user_balance = test.strategy.balance(&users[0]);
    assert_eq!(user_balance, amount);

    // Withdrawing token 0 from the strategy to user
    test.strategy.withdraw(&amount, &users[0]);

    // Reading user 0 balance
    let balance = test.token0.balance(&users[0]);
    assert_eq!(balance, amount);

    // Reading strategy balance
    let balance = test.token0.balance(&test.strategy.address);
    assert_eq!(balance, 0);

    // Reading user balance on strategy contract
    let user_balance = test.strategy.balance(&users[0]);
    assert_eq!(user_balance, 0);
}