#![allow(unused)]
use soroban_sdk::{contracttype, contracterror, xdr::ToXdr, Address, Bytes, BytesN, Env, Vec};

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env, // Pass in the current environment as an argument
    defindex_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    salt: BytesN<32>,
) -> Address {

    e.deployer()
        .with_current_contract(e.crypto().sha256(&salt.into())) 
        .deploy(defindex_wasm_hash)
}