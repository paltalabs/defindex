mod factory_contract {
    soroban_sdk::contractimport!(
        file = "../target/wasm32v1-none/release/defindex_factory.optimized.wasm"
    );
    pub type DeFindexFactoryClient<'a> = Client<'a>;
}

pub use factory_contract::{AssetStrategySet, DeFindexFactoryClient, Strategy};
use soroban_sdk::{Address, BytesN, Env};

// DeFindex Factory Contract
pub fn create_factory_contract<'a>(
    e: &Env,
    admin: &Address,
    defindex_receiver: &Address,
    defindex_fee: &u32,
    vault_wasm_hash: &BytesN<32>,
) -> DeFindexFactoryClient<'a> {
    let args = (
        admin.clone(),
        defindex_receiver.clone(),
        defindex_fee.clone(),
        vault_wasm_hash.clone(),
    );

    let address = &e.register(factory_contract::WASM, args);
    let factory = DeFindexFactoryClient::new(e, address);

    factory
}
