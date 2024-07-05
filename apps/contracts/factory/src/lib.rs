#![no_std]

mod defindex;
mod storage;
mod error;

use soroban_sdk::{
    contract,
    contractimpl,
    Address, BytesN, Env,
    Vec,
};
use error::FactoryError;
use defindex::{create_contract, AdapterParams};
use storage::*;

pub trait SoroswapFactoryTrait {
    fn initialize(e: Env, defindex_wasm_hash: BytesN<32>) -> Result<(), FactoryError>;

    fn create_defindex(e: Env, adapters: Vec<AdapterParams>) -> Result<Address, FactoryError>;
}


#[contract]
struct SoroswapFactory;

#[contractimpl]
impl SoroswapFactoryTrait for SoroswapFactory {

fn initialize(e: Env, defi_wasm_hash: BytesN<32>) -> Result<(), FactoryError> {
    put_defi_wasm_hash(&e, defi_wasm_hash);

    extend_instance_ttl(&e);
    Ok(())
}

fn create_defindex(e: Env, adapters: Vec<AdapterParams>) -> Result<Address, FactoryError> {
    extend_instance_ttl(&e);

    let defi_wasm_hash = get_defi_wasm_hash(&e)?;
    let defindex_address = create_contract(&e, defi_wasm_hash, adapters.clone());

    defindex::Client::new(&e, &defindex_address).initialize(
        &adapters
    );

    Ok(defindex_address)
}


}
