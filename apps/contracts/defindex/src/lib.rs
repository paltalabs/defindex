#![no_std]
use access::{AccessControl, AccessControlTrait, RolesDataKey};
use interface::AdminInterfaceTrait;
use soroban_sdk::{
    contract, contractimpl, panic_with_error, Address, Env, String, Vec
};
use soroban_token_sdk::metadata::TokenMetadata;
use crate::interface::VaultTrait;

mod access;
mod error;
mod interface;
mod models;
mod storage;
mod test;
mod token;
mod utils;

pub use error::ContractError;

use storage::{
    get_adapter, get_share, get_total_adapters, set_adapter,
    set_share, set_total_adapters,
};

use models::AdapterParams;
use defindex_adapter_interface::DeFindexAdapterClient;
use token::write_metadata;

fn check_initialized(e: &Env) -> Result<(), ContractError> {
    //TODO: Should also check if adapters/strategies have been set
    let access_control = AccessControl::new(&e);
    if access_control.has_role(&RolesDataKey::Manager) {
        Ok(())
    } else {
        panic_with_error!(&e, ContractError::NotInitialized);
    }
}

pub fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {
    fn initialize(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        manager: Address, 
        adapters: Vec<AdapterParams>
    ) -> Result<(), ContractError> {
        let access_control = AccessControl::new(&e);
        if access_control.has_role(&RolesDataKey::Manager) {
            panic_with_error!(&e, ContractError::AlreadyInitialized);
        }

        access_control.set_role(&RolesDataKey::EmergencyManager, &emergency_manager);
        access_control.set_role(&RolesDataKey::FeeReceiver, &fee_receiver);
        access_control.set_role(&RolesDataKey::Manager, &manager);

        //TODO: Name of the tokens should be created by the strategy taken place?
        let decimal: u32 = 7;
        let name: String = String::from_str(&e, "dfToken");
        let symbol: String = String::from_str(&e, "DFT");
    
        write_metadata(
            &e,
            TokenMetadata {
                decimal ,
                name,
                symbol,
            },
        );

        set_total_adapters(&e, &adapters.len());

        for adapter in adapters.iter() {
            set_share(&e, adapter.index, adapter.share);
            set_adapter(&e, adapter.index, &adapter.address);
        }

        Ok(())
    }

    fn deposit(e: Env, amount: i128, from: Address) -> Result<(), ContractError> {
        check_initialized(&e)?;
        check_nonnegative_amount(amount)?;
        from.require_auth();

        let total_adapters = get_total_adapters(&e);
        let total_amount_used: i128 = 0;

        for i in 0..total_adapters {
            let adapter_share = get_share(&e, i);

            let adapter_address = get_adapter(&e, i);
            let adapter_client = DeFindexAdapterClient::new(&e, &adapter_address);

            let adapter_amount = if i == (total_adapters - 1) {
                amount - total_amount_used
            } else {
                amount
                    .checked_mul(adapter_share.into())
                    .and_then(|prod| prod.checked_div(100))
                    .ok_or(ContractError::ArithmeticError)?
            };

            adapter_client.deposit(&adapter_amount, &from);
            //should run deposit functions on adapters
        }

        Ok(())
    }

    fn withdraw(
        e: Env,
        from: Address,
    ) -> Result<(), ContractError>{
        check_initialized(&e)?;
        from.require_auth();
        let total_adapters = get_total_adapters(&e);

        for i in 0..total_adapters {
            let adapter_address = get_adapter(&e, i);
            let adapter_client = DeFindexAdapterClient::new(&e, &adapter_address);

            adapter_client.withdraw(&from);
        }

        Ok(())
    }

    fn emergency_withdraw(
        e: Env,
        from: Address,
    ) -> Result<(), ContractError>{
        check_initialized(&e)?;
        from.require_auth();
        let total_adapters = get_total_adapters(&e);

        for i in 0..total_adapters {
            let adapter_address = get_adapter(&e, i);
            let adapter_client = DeFindexAdapterClient::new(&e, &adapter_address);

            adapter_client.withdraw(&from);
        }

        Ok(())
    }

    fn get_adapter_address(e: Env) -> Address {
        get_adapter(&e, 0)
    }

    fn current_invested_funds(e: Env) -> i128 {
        0i128
    }
}

#[contractimpl]
impl AdminInterfaceTrait for DeFindexVault {  
    fn set_fee_receiver(e: Env, caller: Address, fee_receiver: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_fee_receiver(&caller, &fee_receiver)
    }
  
    fn get_fee_receiver(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_fee_receiver()
    }
  
    fn set_manager(e: Env, manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_manager(&manager)
    }
  
    fn get_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_manager()
    }
  
    fn set_emergency_manager(e: Env, emergency_manager: Address) {
        let access_control = AccessControl::new(&e);
        access_control.set_emergency_manager(&emergency_manager)
    }
  
    fn get_emergency_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_emergency_manager()
    }
}