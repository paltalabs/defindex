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
use defindex::{create_contract, StrategyParams};
use storage::{ extend_instance_ttl, get_admin, get_defi_wasm_hash, get_defindex_receiver, has_admin, put_admin, put_defi_wasm_hash, put_defindex_receiver };

fn check_initialized(e: &Env) -> Result<(), FactoryError> {
    if !has_admin(e) {
        return Err(FactoryError::NotInitialized);
    } 
    Ok(())
}

pub trait FactoryTrait {
    fn initialize(
        e: Env, 
        admin: Address,
        defindex_receiver: Address,
        defindex_wasm_hash: BytesN<32>
    ) -> Result<(), FactoryError>;

    fn create_defindex_vault(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        manager: Address,
        tokens: Vec<Address>,
        ratios: Vec<u32>,
        strategies: Vec<StrategyParams>
    ) -> Result<Address, FactoryError>;

    // Admin functions
    fn set_new_admin(e: Env, new_admin: Address) -> Result<(), FactoryError>;
    fn get_admin(e: Env) -> Result<Address, FactoryError>;

    fn set_fee_receiver(e: Env, new_fee_receiver: Address) -> Result<(), FactoryError>;
    fn get_defindex_receiver(e: Env) -> Result<Address, FactoryError>;
}

#[contract]
struct DeFindexFactory;

#[contractimpl]
impl FactoryTrait for DeFindexFactory {

    fn initialize(
        e: Env, 
        admin: Address, 
        defindex_receiver: Address, 
        defi_wasm_hash: BytesN<32>
    ) -> Result<(), FactoryError> {
        if has_admin(&e) {
            return Err(FactoryError::AlreadyInitialized);
        }

        put_admin(&e, &admin);
        put_defindex_receiver(&e, &defindex_receiver);
        put_defi_wasm_hash(&e, defi_wasm_hash);

        extend_instance_ttl(&e);
        Ok(())
    }

    fn create_defindex_vault(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        manager: Address,
        tokens: Vec<Address>,
        ratios: Vec<u32>,
        strategies: Vec<StrategyParams>
    ) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);

        let defi_wasm_hash = get_defi_wasm_hash(&e)?;
        let defindex_address = create_contract(&e, defi_wasm_hash);

        let defindex_receiver = get_defindex_receiver(&e);

        defindex::Client::new(&e, &defindex_address).initialize(
            &emergency_manager,
            &fee_receiver,
            &manager,
            &defindex_receiver,
            &tokens,
            &ratios,
            &strategies
        );

        Ok(defindex_address)
    }

    fn set_new_admin(e: Env, new_admin: Address) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_admin(&e, &new_admin);
        Ok(())
    }

    fn get_admin(e: Env) -> Result<Address, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_admin(&e))
    }

    fn set_fee_receiver(e: Env, new_fee_receiver: Address) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_defindex_receiver(&e, &new_fee_receiver);
        Ok(())
    }

    fn get_defindex_receiver(e: Env) -> Result<Address, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_defindex_receiver(&e))
    }
}

mod test;