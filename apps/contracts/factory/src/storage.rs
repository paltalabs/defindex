use soroban_sdk::{
    contracttype, Address, BytesN, Env, Map, TryFromVal, Val
};
use crate::error::FactoryError;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Admin,
    DeFindexWasmHash,
    DeFindexReceiver,
    DeFindexesMap,
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
    error: FactoryError
) -> Result<V, FactoryError> {
    if let Some(result) = e.storage().persistent().get(key) {
        e.storage()
            .persistent()
            .extend_ttl(key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        result
    } else {
        return Err(error);
    }
}

pub fn get_vault_wasm_hash(e: &Env) -> Result<BytesN<32>, FactoryError>{
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

// Storing deployed defindexes
pub fn get_deployed_defindexes(e: &Env) -> Result<Map<u32, Address>, FactoryError> {
    let key = DataKey::DeFindexesMap;
    get_persistent_extend_or_error(&e, &key, FactoryError::EmptyMap)
}

fn put_deployed_defindexes(e: &Env, deployed_defindexes: Map<u32, Address>) {
    let key = DataKey::DeFindexesMap;
    e.storage().persistent().set(&key, &deployed_defindexes);
    e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT)
}

pub fn add_new_defindex(e: &Env, defindex_address: Address) {
    let mut deployed_defindexes: Map<u32, Address> = get_deployed_defindexes(&e).unwrap_or(Map::new(&e));
    let new_id = deployed_defindexes.len() as u32;
    deployed_defindexes.set(new_id, defindex_address);
    put_deployed_defindexes(&e, deployed_defindexes);
}

// Admin
pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Admin)
}

pub fn put_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Admin).unwrap()
}

// Fee Receiver
pub fn put_defindex_receiver(e: &Env, address: &Address) {
    e.storage().instance().set(&DataKey::DeFindexReceiver, address);
}

pub fn get_defindex_receiver(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::DeFindexReceiver).unwrap()
}

// Fee Rate BPS (MAX BPS = 10000)
pub fn put_defindex_fee(e: &Env, value: &u32) {
    e.storage().instance().set(&DataKey::FeeRate, value);
}

pub fn get_fee_rate(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::FeeRate).unwrap()
}