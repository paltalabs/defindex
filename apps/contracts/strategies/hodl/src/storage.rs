use soroban_sdk::{contracttype, Env, Address};

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    UnderlyingAsset,
    Balance(Address)
}

const DAY_IN_LEDGERS: u32 = 17280;
pub const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

// Underlying asset
pub fn set_underlying_asset(e: &Env, address: &Address) {
    e.storage().instance().set(&DataKey::UnderlyingAsset, &address);
}

pub fn get_underlying_asset(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::UnderlyingAsset).unwrap()
}