#![allow(unused)]
use soroban_sdk::{auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, contracterror, contracttype, vec, xdr::ToXdr, Address, Bytes, BytesN, Env, Symbol, Val, Vec};

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

    let new_contract_address = e.deployer()
        .with_current_contract(salt.clone())
        .deployed_address();

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: new_contract_address,
                fn_name: Symbol::new(&e, "__constructor"),
                args: constructor_args.clone(),
            },
            sub_invocations: vec![&e],
        }),
    ]);

    e.deployer()
        .with_current_contract(salt)
        .deploy_v2(defindex_wasm_hash, constructor_args)
}
