#![no_std]

mod defindex;
mod events;
mod storage;
mod error;

use soroban_sdk::{
    contract, contractimpl, Address, BytesN, Env, Map, Vec
};
use error::FactoryError;
use defindex::{create_contract, AssetAllocation};
use storage::{ add_new_defindex, extend_instance_ttl, get_admin, get_defi_wasm_hash, get_defindex_receiver, get_deployed_defindexes, get_fee_rate, has_admin, put_admin, put_defi_wasm_hash, put_defindex_receiver, put_fee_rate };

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
        fee_rate: u32,
        defindex_wasm_hash: BytesN<32>
    ) -> Result<(), FactoryError>;

    fn create_defindex_vault(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_share: u32,
        manager: Address,
        assets: Vec<AssetAllocation>,
        salt: BytesN<32>
    ) -> Result<Address, FactoryError>;

    // Admin functions
    fn set_new_admin(e: Env, new_admin: Address) -> Result<(), FactoryError>;
    fn set_defindex_receiver(e: Env, new_fee_receiver: Address) -> Result<(), FactoryError>;
    fn set_fee_rate(e: Env, new_fee_rate: u32) -> Result<(), FactoryError>;
    
    // Read Methods
    fn admin(e: Env) -> Result<Address, FactoryError>;
    fn defindex_receiver(e: Env) -> Result<Address, FactoryError>;
    fn deployed_defindexes(e: Env) -> Result<Map<u32, Address>, FactoryError>;
    fn fee_rate(e: Env) -> Result<u32, FactoryError>;
}

#[contract]
struct DeFindexFactory;

#[contractimpl]
impl FactoryTrait for DeFindexFactory {

    fn initialize(
        e: Env, 
        admin: Address, 
        defindex_receiver: Address,
        fee_rate: u32,
        defi_wasm_hash: BytesN<32>
    ) -> Result<(), FactoryError> {
        if has_admin(&e) {
            return Err(FactoryError::AlreadyInitialized);
        }

        put_admin(&e, &admin);
        put_defindex_receiver(&e, &defindex_receiver);
        put_defi_wasm_hash(&e, defi_wasm_hash);
        put_fee_rate(&e, &fee_rate);

        events::emit_initialized(&e, admin, defindex_receiver, fee_rate);
        extend_instance_ttl(&e);
        Ok(())
    }

    fn create_defindex_vault(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_share: u32,
        manager: Address,
        assets: Vec<AssetAllocation>,
        salt: BytesN<32>
    ) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);

        let current_contract = e.current_contract_address();

        let defi_wasm_hash = get_defi_wasm_hash(&e)?;
        let defindex_address = create_contract(&e, defi_wasm_hash, salt);

        let defindex_receiver = get_defindex_receiver(&e);

        defindex::Client::new(&e, &defindex_address).initialize(
            &assets,
            &manager,
            &emergency_manager,
            &fee_receiver,
            &vault_share,
            &defindex_receiver,
            &current_contract,
        );

        add_new_defindex(&e, defindex_address.clone());
        events::emit_create_defindex_vault(&e, emergency_manager, fee_receiver, manager, vault_share, assets);
        Ok(defindex_address)
    }

    fn set_new_admin(e: Env, new_admin: Address) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_admin(&e, &new_admin);
        events::emit_new_admin(&e, new_admin);
        Ok(())
    }

    fn set_defindex_receiver(e: Env, new_fee_receiver: Address) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_defindex_receiver(&e, &new_fee_receiver);
        events::emit_new_defindex_receiver(&e, new_fee_receiver);
        Ok(())
    }

    fn set_fee_rate(e: Env, fee_rate: u32) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_fee_rate(&e, &fee_rate);
        events::emit_new_fee_rate(&e, fee_rate);
        Ok(())
    }

    fn admin(e: Env) -> Result<Address, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_admin(&e))
    }

    fn defindex_receiver(e: Env) -> Result<Address, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_defindex_receiver(&e))
    }
    
    fn deployed_defindexes(e: Env) -> Result<Map<u32, Address>, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        get_deployed_defindexes(&e)
    }

    fn fee_rate(e: Env) -> Result<u32, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_fee_rate(&e))
    }
}

mod test;