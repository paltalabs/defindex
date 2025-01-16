use soroban_sdk::{testutils::Address as _, Address, BytesN, Env};

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(file = "../external_wasms/soroswap/soroswap_pair.optimized.wasm");
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(
        file = "../external_wasms/soroswap/soroswap_factory.optimized.wasm"
    );
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
use factory::SoroswapFactoryClient;

pub fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register(factory::WASM, ());
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
mod router {
    soroban_sdk::contractimport!(
        file = "../external_wasms/soroswap/soroswap_router.optimized.wasm"
    );
    pub type SoroswapRouterClient<'a> = Client<'a>;
}
pub use router::SoroswapRouterClient;

// SoroswapRouter Contract
pub fn create_soroswap_router<'a>(e: &Env, factory: &Address) -> SoroswapRouterClient<'a> {
    let router_address = &e.register(router::WASM, ());
    let router = SoroswapRouterClient::new(e, router_address);
    router.initialize(factory);
    router
}

pub fn create_soroswap_pool<'a>(
    e: &Env,
    to: &Address,
    token_a: &Address,
    token_b: &Address,
    amount_a: &i128,
    amount_b: &i128,
) -> SoroswapRouterClient<'a> {
    e.mock_all_auths();
    let soroswap_admin = Address::generate(&e);
    let factory = create_soroswap_factory(&e, &soroswap_admin);
    let router = create_soroswap_router(&e, &factory.address);

    router.add_liquidity(
        token_a,
        token_b,
        &amount_a,
        &amount_b,
        &0i128,
        &0i128,
        &to,
        &(e.ledger().timestamp() + 3600),
    );

    router
}
