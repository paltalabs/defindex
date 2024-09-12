use soroban_sdk::{contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyParams {
    pub name: String,
    pub address: Address,
}

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Tokens(u32),       // Token Addresses by index
    Ratios(u32),       // Ratios corresponding to tokens
    TotalTokens,       // Total number of tokens
    Strategy(u32),     // Strategy addresses by index
    StrategyName(u32), // Strategy names by index
    TotalStrategies,   // Total number of strategies
    IdleFunds,
    DeFindexReceiver
}

// Token Management
pub fn set_token(e: &Env, index: u32, token: &Address) {
    e.storage().instance().set(&DataKey::Tokens(index), token);
}

pub fn get_token(e: &Env, index: u32) -> Address {
    e.storage().instance().get(&DataKey::Tokens(index)).unwrap()
}

pub fn get_tokens(e: &Env) -> Vec<Address> {
    let total_tokens = get_total_tokens(e);
    let mut tokens = Vec::new(e);
    for i in 0..total_tokens {
        tokens.push_back(get_token(e, i));
    }
    tokens
}

pub fn set_ratio(e: &Env, index: u32, ratio: u32) {
    e.storage().instance().set(&DataKey::Ratios(index), &ratio);
}

pub fn get_ratio(e: &Env, index: u32) -> u32 {
    e.storage().instance().get(&DataKey::Ratios(index)).unwrap()
}

pub fn set_total_tokens(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalTokens, &n);
}

pub fn get_total_tokens(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalTokens).unwrap()
}

// Strategy Management
pub fn set_strategy(e: &Env, index: u32, strategy: &Address) {
    e.storage().instance().set(&DataKey::Strategy(index), strategy);
}

pub fn get_strategy(e: &Env, index: u32) -> Address {
    e.storage().instance().get(&DataKey::Strategy(index)).unwrap()
}

pub fn get_strategies(e: &Env) -> Vec<Address> {
    let total_strategies = get_total_strategies(e);
    let mut strategies = Vec::new(e);
    for i in 0..total_strategies {
        strategies.push_back(get_strategy(e, i));
    }
    strategies
}

pub fn set_strategy_name(e: &Env, index: u32, name: &String) {
    e.storage().instance().set(&DataKey::StrategyName(index), name);
}

pub fn get_strategy_name(e: &Env, index: u32) -> String {
    e.storage().instance().get(&DataKey::StrategyName(index)).unwrap()
}

pub fn set_total_strategies(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalStrategies, &n);
}

pub fn get_total_strategies(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalStrategies).unwrap()
}

// Idle Funds Management
pub fn set_idle_funds(e: &Env, n: &i128) {
    e.storage().instance().set(&DataKey::IdleFunds, n);
}

pub fn get_idle_funds(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::IdleFunds).unwrap()
}

// PaltaLabs Fee Receiver
pub fn set_defindex_receiver(e: &Env, address: &Address) {
    e.storage().instance().set(&DataKey::DeFindexReceiver, address);
}

pub fn get_defindex_receiver(e: &Env) -> i128 {
    e.storage().instance().get(&DataKey::DeFindexReceiver).unwrap()
}