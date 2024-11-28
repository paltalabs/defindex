use soroban_sdk::{contracttype, Address, Env};

use crate::{positions::VaultPosition, reserves::StrategyReserves};

#[contracttype]
pub struct Config {
    pub asset: Address,
    pub pool: Address,
    pub reserve_id: u32,
}

#[derive(Clone)]
#[contracttype]

pub enum DataKey {
    Initialized,
    Config,
    Reserves,
    VaultPos(Address) // Vaults Positions
}

const DAY_IN_LEDGERS: u32 = 17280;
pub const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
const LEDGER_BUMP: u32 = 120 * DAY_IN_LEDGERS;
const LEDGER_THRESHOLD: u32 = LEDGER_BUMP - 20 * DAY_IN_LEDGERS;

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

// Config
pub fn set_config(e: &Env, config: Config) {
    e.storage().instance().set(&DataKey::Config, &config);
}

pub fn get_config(e: &Env) -> Config {
    e.storage().instance().get(&DataKey::Config).unwrap()
}

// Vault Position
pub fn set_vault_position(e: &Env, address: &Address, vault_position: VaultPosition) {
    let key = DataKey::VaultPos(address.clone());
    e.storage().persistent().set(&key, &vault_position);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
}

pub fn get_vault_position(e: &Env, address: &Address) -> VaultPosition {
    e.storage().persistent().get(&DataKey::VaultPos(address.clone())).unwrap_or(
        VaultPosition {
            deposited: 0,
            withdrawn: 0,
            b_tokens: 0,
        }
    )
}

// Strategy Reserves
pub fn set_strategy_reserves(e: &Env, new_reserves: StrategyReserves) {
    e.storage().instance().set(&DataKey::Reserves, &new_reserves);
}

pub fn get_strategy_reserves(e: &Env) -> StrategyReserves {
    e.storage().instance().get(&DataKey::Reserves).unwrap_or(
        StrategyReserves {
            total_deposited: 0,
            total_b_tokens: 0,
            b_rate: 0,
        }
    )
}

