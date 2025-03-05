#![no_std]

mod error;
mod constants;
mod events;
mod storage;
mod vault;

use common::models::AssetStrategySet;
use error::FactoryError;
use soroban_sdk::{
    contract, contractimpl, vec, Address, BytesN, Env, IntoVal, Map, String, Symbol, Val, Vec,
};
use storage::{
    add_new_vault, extend_instance_ttl, get_admin, get_defindex_receiver,
    get_total_vaults, get_vault_by_index, get_fee_rate, get_vault_wasm_hash, put_admin,
    put_defindex_fee, put_defindex_receiver, put_vault_wasm_hash,
};
pub use vault::create_contract;


pub trait FactoryTrait {
    /// Initializes the factory contract with the given parameters.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `admin` - The address of the contract administrator, who can manage settings.
    /// * `defindex_receiver` - The default address designated to receive a portion of fees.
    /// * `defindex_fee` - The initial annual fee rate (in basis points).
    /// * `vault_wasm_hash` - The hash of the DeFindex Vault's WASM file for deploying new vaults.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, otherwise an error.
    fn __constructor(
        e: Env,
        admin: Address,
        defindex_receiver: Address,
        defindex_fee: u32,
        vault_wasm_hash: BytesN<32>,
    ) -> Result<(), FactoryError>;

    /// Creates a new DeFindex Vault with specified parameters.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `roles` - A `Map` containing role identifiers (`u32`) and their corresponding `Address` assignments.
    ///              Example roles include the manager and fee receiver.
    /// * `vault_fee` - The fee rate in basis points (1 basis point = 0.01%) allocated to the fee receiver.
    /// * `assets` - A vector of `AssetStrategySet` structs defining the strategies and assets managed by the vault.
    /// * `soroswap_router` - The `Address` of the Soroswap router, which facilitates swaps within the vault.
    /// * `name_symbol` - A `Map` containing the vault's name and symbol metadata (e.g., "name" -> "MyVault", "symbol" -> "MVLT").
    /// * `upgradable` - A boolean flag indicating whether the deployed vault contract should support upgrades.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the newly created vault if successful, or an error if creation fails.
    fn create_defindex_vault(
        e: Env,
        roles: Map<u32, Address>,
        vault_fee: u32,
        assets: Vec<AssetStrategySet>,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
    ) -> Result<Address, FactoryError>;

    /// Creates a new DeFindex Vault with specified parameters and makes the first deposit to set ratios.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `emergency_manager` - The address assigned emergency control over the vault.
    /// * `fee_receiver` - The address designated to receive fees from the vault.
    /// * `vault_fee` - The percentage share of fees allocated to the vault's fee receiver.
    /// * `vault_name` - The name of the vault.
    /// * `vault_symbol` - The symbol of the vault.
    /// * `manager` - The address assigned as the vault manager.
    /// * `assets` - A vector of `AssetStrategySet` structs that define the assets managed by the vault.
    /// * `amounts` - A vector of `AssetAmounts` structs that define the initial deposit amounts.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the new vault, or an error if unsuccessful.
    fn create_defindex_vault_deposit(
        e: Env,
        caller: Address,
        roles: Map<u32, Address>,
        vault_fee: u32,
        assets: Vec<AssetStrategySet>,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
        amounts: Vec<i128>,
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
    /// * `defindex_fee` - The new annual fee rate in basis points.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_defindex_fee(e: Env, defindex_fee: u32) -> Result<(), FactoryError>;

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

    /// Retrieves the total number of deployed DeFindex vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<u32, FactoryError>` - Returns the total number of deployed vaults or an error if retrieval fails.
    fn total_vaults(e: Env) -> Result<u32, FactoryError>;

    /// Retrieves a specific vault address by its index.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `index` - The index of the vault to retrieve (0-based).
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the vault address at the specified index or an error if not found.
    fn get_vault_by_index(e: Env, index: u32) -> Result<Address, FactoryError>;

    /// Retrieves the current fee rate.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<u32, FactoryError>` - Returns the fee rate in basis points or an error if not found.
    fn defindex_fee(e: Env) -> Result<u32, FactoryError>;

    /// Updates the vault WASM hash used for deploying new vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `new_vault_wasm_hash` - The new hash of the vault's WASM file.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_vault_wasm_hash(e: Env, new_vault_wasm_hash: BytesN<32>) -> Result<(), FactoryError>;

    /// Retrieves the current vault WASM hash.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<BytesN<32>, FactoryError>` - Returns the current vault WASM hash or an error if not found.
    fn vault_wasm_hash(e: Env) -> Result<BytesN<32>, FactoryError>;
}

#[contract]
struct DeFindexFactory;

// Private helper function for vault creation
fn create_vault_internal(
    e: &Env,
    roles: Map<u32, Address>,
    vault_fee: u32,
    assets: Vec<AssetStrategySet>,
    soroswap_router: Address,
    name_symbol: Map<String, String>,
    upgradable: bool,
) -> Result<Address, FactoryError> {
    let current_contract = e.current_contract_address();
    let vault_wasm_hash = get_vault_wasm_hash(e)?;
    let defindex_receiver = get_defindex_receiver(e)?;
    let defindex_fee = get_fee_rate(e)?;

    let mut init_args: Vec<Val> = vec![e];
    init_args.push_back(assets.to_val());
    init_args.push_back(roles.to_val());
    init_args.push_back(vault_fee.into_val(e));
    init_args.push_back(defindex_receiver.to_val());
    init_args.push_back(defindex_fee.into_val(e));
    init_args.push_back(current_contract.to_val());
    init_args.push_back(soroswap_router.to_val());
    init_args.push_back(name_symbol.to_val());
    init_args.push_back(upgradable.into_val(e));

    let defindex_address = create_contract(e, vault_wasm_hash, init_args);
    add_new_vault(e, defindex_address.clone());

    events::emit_create_defindex_vault(
        &e,
        roles,
        vault_fee,
        assets,
    );

    Ok(defindex_address)
}

// Private helper function for deposits
fn perform_initial_deposit(
    e: &Env,
    vault_address: &Address,
    caller: &Address,
    amounts: &Vec<i128>,
) {
    let mut amounts_min = Vec::new(e);
    for _ in 0..amounts.len() {
        amounts_min.push_back(0i128);
    }

    let mut deposit_args: Vec<Val> = vec![e];
    deposit_args.push_back(amounts.to_val());
    deposit_args.push_back(amounts_min.to_val());
    deposit_args.push_back(caller.to_val());
    deposit_args.push_back(false.into_val(e));

    e.invoke_contract::<Val>(vault_address, &Symbol::new(e, "deposit"), deposit_args);
}

#[contractimpl]
impl FactoryTrait for DeFindexFactory {
    /// Initializes the factory contract with the given parameters.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `admin` - The address of the contract administrator who can manage settings.
    /// * `defindex_receiver` - The default address designated to receive the DeFindex portion of fees.
    /// * `defindex_fee` - The initial fee rate in basis points (1 basis point = 0.01%).
    /// * `vault_wasm_hash` - The hash of the DeFindex Vault's WASM file used for deploying new vaults.
    ///
    /// # Behavior
    /// 1. Sets the initial admin address
    /// 2. Sets the initial DeFindex fee receiver address
    /// 3. Sets the initial vault WASM hash
    /// 4. Sets the initial DeFindex fee rate
    /// 5. Extends the contract instance's time-to-live
    fn __constructor(
        e: Env,
        admin: Address,
        defindex_receiver: Address,
        defindex_fee: u32,
        vault_wasm_hash: BytesN<32>,
    ) -> Result<(), FactoryError> {
        put_admin(&e, &admin);
        put_defindex_receiver(&e, &defindex_receiver);
        put_vault_wasm_hash(&e, vault_wasm_hash);
        extend_instance_ttl(&e);
        match put_defindex_fee(&e, &defindex_fee) {
            Ok(_) => Ok(()),
            Err(err) => return Err(err),
        }
    }

    /// Creates a new DeFindex Vault with the specified parameters.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `roles` - A `Map` containing role identifiers (`u32`) and their corresponding `Address` assignments.
    ///              Example roles include the manager and fee receiver.
    /// * `vault_fee` - The fee rate in basis points (1 basis point = 0.01%) allocated to the fee receiver.
    /// * `assets` - A vector of `AssetStrategySet` structs defining the strategies and assets managed by the vault.
    /// * `soroswap_router` - The `Address` of the Soroswap router, which facilitates swaps within the vault.
    /// * `name_symbol` - A `Map` containing the vault's name and symbol metadata (e.g., "name" -> "MyVault", "symbol" -> "MVLT").
    /// * `upgradable` - A boolean flag indicating whether the deployed vault contract should support upgrades.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the newly created vault if successful, or an error if creation fails.
    fn create_defindex_vault(
        e: Env,
        roles: Map<u32, Address>,
        vault_fee: u32,
        assets: Vec<AssetStrategySet>,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
    ) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);
        
        let vault_address = create_vault_internal(
            &e,
            roles.clone(),
            vault_fee,
            assets.clone(),
            soroswap_router,
            name_symbol,
            upgradable,
        )?;

        Ok(vault_address)
    }

    /// Creates a new DeFindex Vault with specified parameters and performs an initial deposit to set asset ratios.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `caller` - The address of the caller, who must authenticate and provide the initial deposit.
    /// * `roles` - A `Map` containing role identifiers (`u32`) and their corresponding `Address` assignments.
    ///             Example roles include the manager and fee receiver.
    /// * `vault_fee` - The fee rate in basis points (1 basis point = 0.01%) allocated to the fee receiver.
    /// * `assets` - A vector of `AssetStrategySet` structs defining the strategies and assets managed by the vault.
    /// * `soroswap_router` - The `Address` of the Soroswap router, which facilitates swaps within the vault.
    /// * `name_symbol` - A `Map` containing the vault's name and symbol metadata (e.g., "name" -> "MyVault", "symbol" -> "MVLT").
    /// * `upgradable` - A boolean flag indicating whether the deployed vault contract should support upgrades.
    /// * `amounts` - A vector of `i128` values representing the initial deposit amounts for each asset in the vault.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the address of the newly created vault if successful, or an error if creation fails.
    
    fn create_defindex_vault_deposit(
        e: Env,
        caller: Address,
        roles: Map<u32, Address>,
        vault_fee: u32,
        assets: Vec<AssetStrategySet>,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
        amounts: Vec<i128>,
    ) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);
        caller.require_auth();

        if assets.len() != amounts.len() {
            return Err(FactoryError::AssetLengthMismatch);
        }

        let vault_addreess = create_vault_internal(
            &e,
            roles.clone(),
            vault_fee,
            assets.clone(),
            soroswap_router,
            name_symbol,
            upgradable,
        )?;

        perform_initial_deposit(&e, &vault_addreess, &caller, &amounts);

        Ok(vault_addreess)
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
        extend_instance_ttl(&e);
        let admin = get_admin(&e)?;
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
        extend_instance_ttl(&e);
        let admin = get_admin(&e)?;
        admin.require_auth();

        put_defindex_receiver(&e, &new_fee_receiver);
        events::emit_new_defindex_receiver(&e, new_fee_receiver);
        Ok(())
    }

    /// Updates the default fee rate for new vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `defindex_fee` - The new annual fee rate in basis points.
    ///
    /// # Returns
    /// * `Result<(), FactoryError>` - Returns Ok(()) if successful, or an error if not authorized.
    fn set_defindex_fee(e: Env, defindex_fee: u32) -> Result<(), FactoryError> {
        extend_instance_ttl(&e);
        let admin = get_admin(&e)?;
        admin.require_auth();

        match put_defindex_fee(&e, &defindex_fee) {
            Ok(_) => events::emit_new_defindex_fee(&e, defindex_fee),
            Err(err) => return Err(err),
        }
        Ok(())
    }

    fn set_vault_wasm_hash(e: Env, new_vault_wasm_hash: BytesN<32>) -> Result<(), FactoryError> {
        extend_instance_ttl(&e);
        let admin = get_admin(&e)?;
        admin.require_auth();

        put_vault_wasm_hash(&e, new_vault_wasm_hash.clone());
        events::emit_new_vault_wasm_hash(&e, new_vault_wasm_hash);
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
        extend_instance_ttl(&e);
        Ok(get_admin(&e)?)
    }

    /// Retrieves the current DeFindex receiver's address.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the DeFindex receiver's address or an error if not found.
    fn defindex_receiver(e: Env) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);
        Ok(get_defindex_receiver(&e)?)
    }

    /// Retrieves the total number of deployed DeFindex vaults.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<u32, FactoryError>` - Returns the total number of deployed vaults or an error if retrieval fails.
    fn total_vaults(e: Env) -> Result<u32, FactoryError> {
        extend_instance_ttl(&e);
        let vaults = get_total_vaults(&e);
        Ok(vaults)
    }

    /// Retrieves a specific vault address by its index.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    /// * `index` - The index of the vault to retrieve (0-based).
    ///
    /// # Returns
    /// * `Result<Address, FactoryError>` - Returns the vault address at the specified index or an error if not found.
    fn get_vault_by_index(e: Env, index: u32) -> Result<Address, FactoryError> {
        extend_instance_ttl(&e);

        get_vault_by_index(&e, index)
    }

    /// Retrieves the current fee rate.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<u32, FactoryError>` - Returns the fee rate in basis points or an error if not found.
    fn defindex_fee(e: Env) -> Result<u32, FactoryError> {
        extend_instance_ttl(&e);
        Ok(get_fee_rate(&e)?)
    }

    /// Retrieves the WASM hash of the vault contract.
    ///
    /// # Arguments
    /// * `e` - The environment in which the contract is running.
    ///
    /// # Returns
    /// * `Result<BytesN<32>, FactoryError>` - Returns the 32-byte WASM hash of the vault contract 
    ///   or an error if the contract is not properly initialized.
    ///
    /// # Behavior
    /// 1. Assumes that the contract is initialized by the constructor.
    /// 2. Extends the instance's time-to-live (TTL) by invoking `extend_instance_ttl(&e)`.
    /// 3. Retrieves and returns the vault WASM hash by calling `get_vault_wasm_hash(&e)`.
    fn vault_wasm_hash(e: Env) -> Result<BytesN<32>, FactoryError> {
        extend_instance_ttl(&e);
        get_vault_wasm_hash(&e)
    }
}

mod test;
