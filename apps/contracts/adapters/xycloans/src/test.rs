#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger}, xdr::{Asset, ContractIdPreimage}, Bytes, BytesN, Env, String
};

// Xycloans Pool Contract
mod xycloans_pool {
    soroban_sdk::contractimport!(file = "../external_wasms/xycloans/xycloans_pool.wasm");
    pub type XycloansPoolClient<'a> = Client<'a>;
}
use xycloans_pool::XycloansPoolClient;

// Xycloans Pool Contract
pub fn create_xycloans_pool<'a>(e: &Env) -> XycloansPoolClient<'a> {
    let pool_address = &e.register_contract_wasm(None, xycloans_pool::WASM);
    let pool = XycloansPoolClient::new(e, pool_address);
    pool
}

// Xycloans Adapter Contract
fn create_xycloans_adapter<'a>(e: &Env) -> XycloansAdapterClient<'a> {
    XycloansAdapterClient::new(e, &e.register_contract(None, XycloansAdapter {}))
}

// Token Contract
mod token {
    soroban_sdk::contractimport!(file = "../external_wasms/token_contract.wasm");
    pub type TokenClient<'a> = Client<'a>;
}
use token::TokenClient;

pub fn create_token_contract<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    TokenClient::new(&e, &e.register_stellar_asset_contract(admin.clone()))
}

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../external_wasms/soroswap/soroswap_pair.optimized.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../external_wasms/soroswap/soroswap_factory.optimized.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register_contract_wasm(None, factory::WASM);
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
mod router {
    soroban_sdk::contractimport!(file = "../external_wasms/soroswap/soroswap_router.optimized.wasm");
    pub type SoroswapRouterClient<'a> = Client<'a>;
}
use router::SoroswapRouterClient;

// SoroswapRouter Contract
pub fn create_soroswap_router<'a>(e: &Env) -> SoroswapRouterClient<'a> {
    let router_address = &e.register_contract_wasm(None, router::WASM);
    let router = SoroswapRouterClient::new(e, router_address);
    router
}

pub struct XycloansAdapterTest<'a> {
    env: Env,
    xycloans_pool: XycloansPoolClient<'a>,
    router_contract: SoroswapRouterClient<'a>,
    adapter_contract: XycloansAdapterClient<'a>,
    xycloans_admin: Address,
    soroswap_admin: Address,
    defindex_admin: Address,
    user: Address,
    token_0: TokenClient<'a>,
    token_1: TokenClient<'a>,
}

impl<'a> XycloansAdapterTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();
        let xycloans_pool = create_xycloans_pool(&env);
        let router_contract = create_soroswap_router(&env);
        let adapter_contract = create_xycloans_adapter(&env);

        let initial_user_balance: i128 = 20_000_000_000_000_000_000;

        let xycloans_admin = Address::generate(&env);
        let soroswap_admin = Address::generate(&env);
        let defindex_admin = Address::generate(&env);
        let user = Address::generate(&env);
        assert_ne!(xycloans_admin, soroswap_admin);
        assert_ne!(soroswap_admin, defindex_admin);
        assert_ne!(defindex_admin, user);
    
        let token_0 = create_token_contract(&env, &soroswap_admin);
        let token_1 = create_token_contract(&env, &soroswap_admin);
        token_0.mint(&soroswap_admin, &initial_user_balance);
        token_1.mint(&soroswap_admin, &initial_user_balance);
        token_1.mint(&user, &initial_user_balance);

        let factory_contract = create_soroswap_factory(&env, &soroswap_admin);
        env.budget().reset_unlimited();

        let ledger_timestamp = 100;
        let desired_deadline = 1000;

        assert!(desired_deadline > ledger_timestamp);

        env.ledger().with_mut(|li| {
            li.timestamp = ledger_timestamp;
        });

        let amount_0: i128 = 1_000_000_000_000_000_000;
        let amount_1: i128 = 4_000_000_000_000_000_000;
        let expected_liquidity: i128 = 2_000_000_000_000_000_000;

        // Check initial user value of every token:
        assert_eq!(token_0.balance(&user), 0);
        assert_eq!(token_1.balance(&user), initial_user_balance);

        router_contract.initialize(&factory_contract.address);

        assert_eq!(factory_contract.pair_exists(&token_0.address, &token_1.address), false);
        let (added_token_0_0, added_token_1_0, added_liquidity_0_1) = router_contract.add_liquidity(
            &token_0.address, //     token_a: Address,
            &token_1.address, //     token_b: Address,
            &amount_0, //     amount_a_desired: i128,
            &amount_1, //     amount_b_desired: i128,
            &0, //     amount_a_min: i128,
            &0 , //     amount_b_min: i128,
            &soroswap_admin, //     to: Address,
            &desired_deadline//     deadline: u64,
        );

        static MINIMUM_LIQUIDITY: i128 = 1000;
    
        assert_eq!(added_token_0_0, amount_0);
        assert_eq!(added_token_1_0, amount_1);

        assert_eq!(added_liquidity_0_1, expected_liquidity.checked_sub(MINIMUM_LIQUIDITY).unwrap());
    
        assert_eq!(token_1.balance(&soroswap_admin), 16_000_000_000_000_000_000);
        assert_eq!(token_0.balance(&soroswap_admin), 19_000_000_000_000_000_000);

        // Initialize xycloans pool
        xycloans_pool.initialize(&token_0.address);

        // Initialize xycloans adapter
        // adapter_contract.initialize(&router_contract.address, &xycloans_pool.address, &token_0.address, &token_1.address);

        XycloansAdapterTest {
            env,
            xycloans_pool,
            router_contract,
            adapter_contract,
            xycloans_admin,
            soroswap_admin,
            defindex_admin,
            user,
            token_0,
            token_1
        }
    }
}

mod initialize;
mod deposit;
