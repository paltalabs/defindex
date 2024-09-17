use soroban_sdk::{contracttype, Address, Env, String, Vec};

use crate::models::{Asset, Strategy};

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Asset(u32),      // Asset Addresse by index
    Ratios(u32),     // Ratios corresponding to tokens
    TotalAssets,     // Total number of tokens
    Strategy(u32),   // Strategy by index
    TotalStrategies, // Total number of strategies
    IdleFunds,
    DeFindexReceiver,
}

// Assets Management
pub fn set_asset(e: &Env, index: u32, asset: &Asset) {
    e.storage().instance().set(&DataKey::Asset(index), asset);
}

pub fn get_asset(e: &Env, index: u32) -> Asset {
    e.storage().instance().get(&DataKey::Asset(index)).unwrap()
}

pub fn set_total_assets(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalAssets, &n);
}

pub fn get_total_assets(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalAssets).unwrap()
}

pub fn get_assets(e: &Env) -> Vec<Asset> {
    let total_assets = get_total_assets(e);
    let mut assets = Vec::new(e);
    for i in 0..total_assets {
        assets.push_back(get_asset(e, i));
    }
    assets
}

// Strategy Management
pub fn set_strategy(e: &Env, index: u32, strategy: &Strategy) {
    e.storage()
        .instance()
        .set(&DataKey::Strategy(index), strategy);
}

pub fn get_strategy(e: &Env, index: u32) -> Strategy {
    e.storage()
        .instance()
        .get(&DataKey::Strategy(index))
        .unwrap()

    // TODO implement errors like this
    // match e.storage().instance().get(&DataKey::Adapter(protocol_id)) {
    //     Some(adapter) => Ok(adapter),
    //     None => Err(AggregatorError::ProtocolNotFound),
    // }
}

pub fn set_total_strategies(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalStrategies, &n);
}

pub fn get_total_strategies(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&DataKey::TotalStrategies)
        .unwrap()
    // TODO not use unwrap
}

pub fn get_strategies(e: &Env) -> Vec<Strategy> {
    let total_strategies = get_total_strategies(e);
    let mut strategies = Vec::new(e);
    for i in 0..total_strategies {
        strategies.push_back(get_strategy(e, i));
    }
    strategies
}

// Idle Funds Management
fn set_idle_funds(e: &Env, amount: i128) {
    e.storage().instance().set(&DataKey::IdleFunds, &amount);
}

pub fn get_idle_funds(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::IdleFunds).unwrap()
}

pub fn receive_idle_funds(e: &Env, amount: i128) {
    let balance = get_idle_funds(e);

    let new_balance = balance
        .checked_add(amount)
        .expect("Integer overflow occurred while adding balance.");

    set_idle_funds(e, new_balance);
}

pub fn spend_idle_funds(e: &Env, amount: i128) {
    let balance = get_idle_funds(e);
    if balance < amount {
        panic!("insufficient balance");
    }
    set_idle_funds(e, balance - amount);
}

// DeFindex Fee Receiver
pub fn set_defindex_receiver(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::DeFindexReceiver, address);
}

pub fn get_defindex_receiver(e: &Env) -> i128 {
    e.storage()
        .instance()
        .get(&DataKey::DeFindexReceiver)
        .unwrap()
}
