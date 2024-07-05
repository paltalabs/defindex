use soroban_sdk::{
    contracttype, BytesN, Env, Val, TryFromVal
};
use crate::error::FactoryError;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    DefindexWasmHash,
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

pub fn get_defi_wasm_hash(e: &Env) -> Result<BytesN<32>, FactoryError>{
    let key = DataKey::DefindexWasmHash;
    get_persistent_extend_or_error(&e, &key, FactoryError::NotInitialized)
}

pub fn put_defi_wasm_hash(e: &Env, pair_wasm_hash: BytesN<32>) {
    let key = DataKey::DefindexWasmHash;
    e.storage().persistent().set(&key, &pair_wasm_hash);
    e.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT)
}