mod factory_contract {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_factory.optimized.wasm");
    pub type DeFindexFactoryClient<'a> = Client<'a>;
}

pub use factory_contract::{AssetStrategySet, Strategy, DeFindexFactoryClient};
use soroban_sdk::{Address, BytesN, Env};

// DeFindex Factory Contract
pub fn create_factory_contract<'a>(e: &Env, admin: &Address, defindex_receiver: &Address, defindex_fee: &u32, vault_wasm_hash: &BytesN<32>) -> DeFindexFactoryClient<'a> {
    let address = &e.register_contract_wasm(None, factory_contract::WASM);
    let factory = DeFindexFactoryClient::new(e, address); 

    factory.initialize(admin, defindex_receiver, defindex_fee, vault_wasm_hash);
    factory
}