// Import necessary types from the Soroban SDK
#![allow(unused)]
use soroban_sdk::{contracttype, contracterror, xdr::ToXdr, Address, Bytes, BytesN, Env, Vec};

soroban_sdk::contractimport!(
    file = "../target/wasm32-unknown-unknown/release/defindex.optimized.wasm"
);

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env, // Pass in the current environment as an argument
    defindex_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    adapters: Vec<AdapterParams>
) -> Address {
    
    let mut salt = Bytes::new(e);

    // Append the bytes of the address and name to the salt
    salt.append(&adapters.clone().to_xdr(e)); 

    // Use the deployer() method of the current environment to create a new contract instance
    e.deployer()
        .with_current_contract(e.crypto().sha256(&salt)) // Use the salt as a unique identifier for the new contract instance
        .deploy(defindex_wasm_hash) // Deploy the new contract instance using the given pair_wasm_hash value
}