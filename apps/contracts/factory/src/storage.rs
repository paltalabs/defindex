use crate::error::FactoryError;
use soroban_sdk::{contracttype, Address, BytesN, Env, TryFromVal, Val};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    DeFindexWasmHash,
    DeFindexReceiver,
    TotalVaults,
    VaultAddressNIndexed(u32),
    FeeRate,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const PERSISTENT_BUMP_AMOUNT: u32 = 60 * DAY_IN_LEDGERS;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/// Fetch an entry in persistent storage that has a default value if it doesn't exist
fn get_persistent_extend_or_error<V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &DataKey,
    error: FactoryError,
) -> Result<V, FactoryError> {
    if let Some(result) = e.storage().persistent().get(key) {
        e.storage().persistent().extend_ttl(
            key,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
        result
    } else {
        return Err(error);
    }
}

pub fn get_vault_wasm_hash(e: &Env) -> Result<BytesN<32>, FactoryError> {
    let key = DataKey::DeFindexWasmHash;
    get_persistent_extend_or_error(&e, &key, FactoryError::NotInitialized)
}

pub fn put_vault_wasm_hash(e: &Env, vault_wasm_hash: BytesN<32>) {
    let key = DataKey::DeFindexWasmHash;
    e.storage().persistent().set(&key, &vault_wasm_hash);
    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT)
}

// Storing deployed vaults
pub fn get_total_vaults(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalVaults).unwrap_or(0)
}

pub fn put_total_vaults(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalVaults, &n);
}

pub fn get_vault_by_index(e: &Env, n: u32) -> Result<Address, FactoryError> {
    let key = DataKey::VaultAddressNIndexed(n);
    get_persistent_extend_or_error(&e, &key, FactoryError::IndexDoesNotExist)
}

pub fn add_new_vault(e: &Env, vault_address: Address) {
    let total_vaults = get_total_vaults(e);
    
    let key = DataKey::VaultAddressNIndexed(total_vaults);
    e.storage().persistent().set(&key, &vault_address);
    e.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);

    put_total_vaults(e, total_vaults.checked_add(1).unwrap());
}


// Admin
pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

pub fn put_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(e: &Env) -> Result<Address, FactoryError> {
    e.storage().instance().get(&DataKey::Admin).ok_or(FactoryError::NotInitialized)?
}

// Fee Receiver
pub fn put_defindex_receiver(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::DeFindexReceiver, address);
}

pub fn get_defindex_receiver(e: &Env) -> Result<Address, FactoryError> {
    e.storage()
        .instance()
        .get(&DataKey::DeFindexReceiver)
        .ok_or(FactoryError::NotInitialized)?
}

// Fee Rate BPS (MAX BPS = 10000)
pub fn put_defindex_fee(e: &Env, value: &u32) {
    e.storage().instance().set(&DataKey::FeeRate, value);
}

pub fn get_fee_rate(e: &Env) -> Result<u32, FactoryError> {
    e.storage().instance().get(&DataKey::FeeRate).ok_or(FactoryError::NotInitialized)?
}
