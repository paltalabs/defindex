use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Initialized,
    UnderlyingAsset,
    BlendPool,
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

// Blend Pool Address
pub fn set_blend_pool(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::BlendPool, &address);
}

pub fn get_blend_pool(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::BlendPool).unwrap()
}