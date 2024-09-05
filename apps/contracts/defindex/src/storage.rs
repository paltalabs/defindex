use soroban_sdk::{contracttype, Address, Env, String};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Initialized,
    TotalAdapters,
    IdleFunds,
    Shares(u32),
    Adapters(u32),
}

pub fn set_initialized(e: &Env, initialized: bool) {
    e.storage().instance().set(&DataKey::Initialized, &initialized);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

pub fn set_share(e: &Env, index: u32, share: u32) {
    e.storage().instance().set(&DataKey::Shares(index), &share);
}

pub fn get_share(e: &Env, index: u32) -> u32 {
    e.storage().instance().get(&DataKey::Shares(index)).unwrap()
}

pub fn set_adapter(e: &Env, index: u32, adapter: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::Adapters(index), &adapter);
}

pub fn get_adapter(e: &Env, index: u32) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::Adapters(index))
        .unwrap()
}

pub fn set_total_adapters(e: &Env, n: &u32) {
    e.storage().instance().set(&DataKey::TotalAdapters, n);
}

pub fn get_total_adapters(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalAdapters).unwrap()
}

pub fn set_idle_funds(e: &Env, n: &i128) {
    e.storage().instance().set(&DataKey::IdleFunds, n);
}

pub fn get_idle_funds(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::IdleFunds).unwrap()
}