#![allow(unused)]
use soroban_sdk::{contracterror, contracttype, xdr::ToXdr, Address, Bytes, BytesN, Env, Val, Vec};

// Define a function to create a new contract instance
pub fn create_contract(
    e: &Env, // Pass in the current environment as an argument
    defindex_wasm_hash: BytesN<32>, // Pass in the hash of the token contract's WASM file
    constructor_args: Vec<Val>,
    salt: BytesN<32>,
) -> Address {

    e.deployer()
        .with_current_contract(salt) 
        .deploy_v2(defindex_wasm_hash, constructor_args)
}