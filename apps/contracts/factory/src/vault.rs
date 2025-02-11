#![allow(unused)]
use soroban_sdk::{contracterror, contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env, Val, Vec};

use crate::storage::get_total_vaults;

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env,                        // Pass in the current environment as an argument
    defindex_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    constructor_args: Vec<Val>,
) -> Address {
    let total_vaults = get_total_vaults(e);

    // Generate a salt BytesN<32> with the total vault value
    let salt = {
        let mut salt_bytes = [0u8; 32];
        let total_vaults_bytes = total_vaults.to_be_bytes();
        let len = total_vaults_bytes.len();
        salt_bytes[..len].copy_from_slice(&total_vaults_bytes);
        BytesN::from_array(e, &salt_bytes)
    };

    e.deployer()
        .with_current_contract(salt)
        .deploy_v2(defindex_wasm_hash, constructor_args)
}
