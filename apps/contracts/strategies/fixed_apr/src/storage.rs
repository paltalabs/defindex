use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Initialized,
    UnderlyingAsset,
    Balance(Address),
    YieldBalance(Address),
    Apr,
    LastHarvestTime(Address),
}

const DAY_IN_LEDGERS: u32 = 17280;
pub const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

// Underlying asset
pub fn set_underlying_asset(e: &Env, address: &Address) {
    e.storage().instance().set(&DataKey::UnderlyingAsset, &address);
}

pub fn get_underlying_asset(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::UnderlyingAsset).unwrap()
}

// Apr
pub fn set_apr(e: &Env, apr: u32) {
    e.storage().instance().set(&DataKey::Apr, &apr);
}

pub fn get_apr(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::Apr).unwrap()
}

// Last harvest time
pub fn set_last_harvest_time(e: &Env, timestamp: u64, from: Address) {
    e.storage().instance().set(&DataKey::LastHarvestTime(from), &timestamp);
}

pub fn get_last_harvest_time(e: &Env, from: Address) -> u64 {
    e.storage().instance().get(&DataKey::LastHarvestTime(from)).unwrap_or(0)
}