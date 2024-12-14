use soroban_sdk::{
    testutils::Address as _, vec, Address, BytesN, Env, String
};

fn pair_contract_wasm(e: &Env) -> BytesN<32> {
    soroban_sdk::contractimport!(
        file = "../soroswap_pair.wasm"
    );
    e.deployer().upload_contract_wasm(WASM)
}

// SoroswapFactory Contract
mod factory {
    soroban_sdk::contractimport!(file = "../soroswap_factory.wasm");
    pub type SoroswapFactoryClient<'a> = Client<'a>;
}
pub use factory::SoroswapFactoryClient;

pub fn create_soroswap_factory<'a>(e: &Env, setter: &Address) -> SoroswapFactoryClient<'a> {
    let pair_hash = pair_contract_wasm(&e);
    let factory_address = &e.register(factory::WASM, ());
    let factory = SoroswapFactoryClient::new(e, factory_address);
    factory.initialize(&setter, &pair_hash);
    factory
}

// SoroswapRouter Contract
mod router {
    soroban_sdk::contractimport!(file = "../soroswap_router.wasm");
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
 
pub fn create_soroswap_pool<'a>(e: &Env, router: &SoroswapRouterClient, to: &Address, token_a: &Address, token_b: &Address, amount_a: &i128, amount_b: &i128) -> (i128, i128, i128) {
    router.add_liquidity(
        token_a, 
        token_b, 
        &amount_a, 
        &amount_b, 
        &0i128, 
        &0i128, 
        &to, 
        &(e.ledger().timestamp() + 3600)
    )
}

// SoroswapRouter Contract
mod aggregator {
    soroban_sdk::contractimport!(file = "../soroswap_aggregator.wasm");
    pub type SoroswapAggregatorClient<'a> = Client<'a>;
}
pub use aggregator::{SoroswapAggregatorClient, Adapter};

pub fn create_soroswap_aggregator<'a>(e: &Env, admin: &Address, router: &Address) -> SoroswapAggregatorClient<'a> {
    let aggregator_address = &e.register(aggregator::WASM, ());
    let aggregator = SoroswapAggregatorClient::new(e, aggregator_address);
    
    let adapter_vec = vec![
        e,
        Adapter {
            protocol_id: String::from_str(e, "soroswap"),
            address: router.clone(),
            paused: false,
        }
    ];

    aggregator.initialize(&admin, &adapter_vec);    
    aggregator
}