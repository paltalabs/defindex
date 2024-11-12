mod factory_contract {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_factory.optimized.wasm");
    pub type DeFindexFactoryClient<'a> = Client<'a>;
}

use factory_contract::DeFindexFactoryClient;
use soroban_sdk::Env;

// DeFindex Factory Contract
pub fn create_factory_contract<'a>(e: &Env) -> DeFindexFactoryClient<'a> {
    let address = &e.register_contract_wasm(None, factory_contract::WASM);
    let factory = DeFindexFactoryClient::new(e, address); 
    factory
}