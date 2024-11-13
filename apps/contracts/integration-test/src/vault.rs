// DeFindex Vault Contract
pub mod defindex_vault_contract {
    soroban_sdk::contractimport!(file = "../target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm");

    pub type VaultContractClient<'a> = Client<'a>;
}

pub use defindex_vault_contract::{AssetStrategySet as VaultAssetStrategySet, Strategy as VaultStrategy};
