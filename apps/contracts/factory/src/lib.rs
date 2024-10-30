#![no_std]

mod defindex;
mod events;
mod storage;
mod error;

use soroban_sdk::{
    contract, contractimpl, Address, BytesN, Env, Map, String, Vec
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
    /// Initializes the factory contract with the given parameters.
    /// 
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `admin` - The address of the contract administrator, who can manage settings.
    /// * `defindex_receiver` - The default address designated to receive a portion of fees.
    /// * `fee_rate` - The initial annual fee rate (in basis points).
    /// * `defindex_wasm_hash` - The hash of the DeFindex Vault's WASM file for deploying new vaults.
    /// 
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, otherwise an error.
    fn initialize(
        e: Env, 
        admin: Address,
        defindex_receiver: Address,
        fee_rate: u32,
        defindex_wasm_hash: BytesN<32>
    ) -> Result<(), FactoryError>;

    /// Creates a new DeFindex Vault with specified parameters.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `emergency_manager` - The address assigned emergency control over the vault.
    /// * `fee_receiver` - The address designated to receive fees from the vault.
    /// * `vault_share` - The percentage share of fees allocated to the vault's fee receiver.
    /// * `vault_name` - The name of the vault.
    /// * `vault_symbol` - The symbol of the vault.
    /// * `manager` - The address assigned as the vault manager.
    /// * `assets` - A vector of `AssetAllocation` structs that define the assets managed by the vault.
    /// * `salt` - A salt used for ensuring unique addresses for each deployed vault.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the new vault, or an error if unsuccessful.
    fn create_defindex_vault(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_share: u32,
        vault_name: String,
        vault_symbol: String,
        manager: Address,
        assets: Vec<AssetAllocation>,
        salt: BytesN<32>
    ) -> Result<Address, FactoryError>;

    /// Creates a new DeFindex Vault with specified parameters and makes the first deposit to set ratios.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `emergency_manager` - The address assigned emergency control over the vault.
    /// * `fee_receiver` - The address designated to receive fees from the vault.
    /// * `vault_share` - The percentage share of fees allocated to the vault's fee receiver.
    /// * `vault_name` - The name of the vault.
    /// * `vault_symbol` - The symbol of the vault.
    /// * `manager` - The address assigned as the vault manager.
    /// * `assets` - A vector of `AssetAllocation` structs that define the assets managed by the vault.
    /// * `amounts` - A vector of `AssetAmounts` structs that define the initial deposit amounts.
    /// * `salt` - A salt used for ensuring unique addresses for each deployed vault.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the new vault, or an error if unsuccessful.
    fn create_defindex_vault_deposit(
        e: Env, 
        caller: Address,
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_share: u32,
        vault_name: String,
        vault_symbol: String,
        manager: Address,
        assets: Vec<AssetAllocation>,
        amounts: Vec<i128>,
        salt: BytesN<32>
    ) -> Result<Address, FactoryError>;

    // --- Admin Functions ---
    
    /// Sets a new admin address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_admin` - The new administrator's address.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_new_admin(e: Env, new_admin: Address) -> Result<(), FactoryError>;

    /// Updates the default receiver address for the DeFindex portion of fees.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_fee_receiver` - The address of the new fee receiver.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_defindex_receiver(e: Env, new_fee_receiver: Address) -> Result<(), FactoryError>;

    /// Updates the default fee rate for new vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_fee_rate` - The new annual fee rate in basis points.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_fee_rate(e: Env, new_fee_rate: u32) -> Result<(), FactoryError>;
    
    // --- Read Methods ---

    /// Retrieves the current admin's address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the admin's address or an error if not found.
    fn admin(e: Env) -> Result<Address, FactoryError>;
    
    /// Retrieves the current DeFindex receiver's address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the DeFindex receiver's address or an error if not found.
    fn defindex_receiver(e: Env) -> Result<Address, FactoryError>;

    /// Retrieves a map of all deployed DeFindex vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Map<u32, Address>, FactoryError>` - Returns a map with vault identifiers and addresses or an error if retrieval fails.
    fn deployed_defindexes(e: Env) -> Result<Map<u32, Address>, FactoryError>;

    /// Retrieves the current fee rate.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<u32, FactoryError>` - Returns the fee rate in basis points or an error if not found.
    fn fee_rate(e: Env) -> Result<u32, FactoryError>;
}

#[contract]
struct DeFindexFactory;

#[contractimpl]
impl FactoryTrait for DeFindexFactory {

    /// Initializes the factory contract with the given parameters.
    /// 
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `admin` - The address of the contract administrator, who can manage settings.
    /// * `defindex_receiver` - The default address designated to receive a portion of fees.
    /// * `fee_rate` - The initial annual fee rate (in basis points).
    /// * `defindex_wasm_hash` - The hash of the DeFindex Vault's WASM file for deploying new vaults.
    /// 
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, otherwise an error.
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

    /// Creates a new DeFindex Vault with specified parameters.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `emergency_manager` - The address assigned emergency control over the vault.
    /// * `fee_receiver` - The address designated to receive fees from the vault.
    /// * `vault_share` - The percentage share of fees allocated to the vault's fee receiver.
    /// * `manager` - The address assigned as the vault manager.
    /// * `assets` - A vector of `AssetAllocation` structs that define the assets managed by the vault.
    /// * `salt` - A salt used for ensuring unique addresses for each deployed vault.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the new vault, or an error if unsuccessful.
    fn create_defindex_vault(
        e: Env, 
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_share: u32,
        vault_name: String,
        vault_symbol: String,
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
            &vault_name,
            &vault_symbol,
        );

        add_new_defindex(&e, defindex_address.clone());
        events::emit_create_defindex_vault(&e, emergency_manager, fee_receiver, manager, vault_share, assets);
        Ok(defindex_address)
    }

    /// Creates a new DeFindex Vault with specified parameters and makes the first deposit to set ratios.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `emergency_manager` - The address assigned emergency control over the vault.
    /// * `fee_receiver` - The address designated to receive fees from the vault.
    /// * `vault_share` - The percentage share of fees allocated to the vault's fee receiver.
    /// * `vault_name` - The name of the vault.
    /// * `vault_symbol` - The symbol of the vault.
    /// * `manager` - The address assigned as the vault manager.
    /// * `assets` - A vector of `AssetAllocation` structs that define the assets managed by the vault.
    /// * `amounts` - A vector of `AssetAmounts` structs that define the initial deposit amounts.
    /// * `salt` - A salt used for ensuring unique addresses for each deployed vault.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the new vault, or an error if unsuccessful.
    fn create_defindex_vault_deposit(
        e: Env, 
        caller: Address,
        emergency_manager: Address, 
        fee_receiver: Address, 
        vault_share: u32,
        vault_name: String,
        vault_symbol: String,
        manager: Address,
        assets: Vec<AssetAllocation>,
        amounts: Vec<i128>,
        salt: BytesN<32>
    ) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);
        caller.require_auth();

        if assets.len() != amounts.len() {
            return Err(FactoryError::AssetLengthMismatch);
        }

        let current_contract = e.current_contract_address();

        let defi_wasm_hash = get_defi_wasm_hash(&e)?;
        let defindex_address = create_contract(&e, defi_wasm_hash, salt);

        let defindex_receiver = get_defindex_receiver(&e);

        let defindex_client = defindex::Client::new(&e, &defindex_address);

        defindex_client.initialize(
            &assets,
            &manager,
            &emergency_manager,
            &fee_receiver,
            &vault_share,
            &defindex_receiver,
            &current_contract,
            &vault_name,
            &vault_symbol,
        );

        let mut amounts_min = Vec::new(&e);
        for _ in 0..amounts.len() {
            amounts_min.push_back(0i128);
        }

        defindex_client.deposit(
            &amounts,
            &amounts_min,
            &caller
        );

        add_new_defindex(&e, defindex_address.clone());
        events::emit_create_defindex_vault(&e, emergency_manager, fee_receiver, manager, vault_share, assets);
        Ok(defindex_address)
    }

    // --- Admin Functions ---
    
    /// Sets a new admin address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_admin` - The new administrator's address.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_new_admin(e: Env, new_admin: Address) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_admin(&e, &new_admin);
        events::emit_new_admin(&e, new_admin);
        Ok(())
    }

    /// Updates the default receiver address for the DeFindex portion of fees.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_fee_receiver` - The address of the new fee receiver.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_defindex_receiver(e: Env, new_fee_receiver: Address) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_defindex_receiver(&e, &new_fee_receiver);
        events::emit_new_defindex_receiver(&e, new_fee_receiver);
        Ok(())
    }

    /// Updates the default fee rate for new vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_fee_rate` - The new annual fee rate in basis points.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_fee_rate(e: Env, fee_rate: u32) -> Result<(), FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        let admin = get_admin(&e);
        admin.require_auth();

        put_fee_rate(&e, &fee_rate);
        events::emit_new_fee_rate(&e, fee_rate);
        Ok(())
    }

    // --- Read Methods ---

    /// Retrieves the current admin's address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the admin's address or an error if not found.
    fn admin(e: Env) -> Result<Address, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_admin(&e))
    }

    /// Retrieves the current DeFindex receiver's address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the DeFindex receiver's address or an error if not found.
    fn defindex_receiver(e: Env) -> Result<Address, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_defindex_receiver(&e))
    }
    
    /// Retrieves a map of all deployed DeFindex vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Map<u32, Address>, FactoryError>` - Returns a map with vault identifiers and addresses or an error if retrieval fails.
    fn deployed_defindexes(e: Env) -> Result<Map<u32, Address>, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        get_deployed_defindexes(&e)
    }

    /// Retrieves the current fee rate.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<u32, FactoryError>` - Returns the fee rate in basis points or an error if not found.
    fn fee_rate(e: Env) -> Result<u32, FactoryError> {
        check_initialized(&e)?;
        extend_instance_ttl(&e);
        Ok(get_fee_rate(&e))
    }
}

mod test;