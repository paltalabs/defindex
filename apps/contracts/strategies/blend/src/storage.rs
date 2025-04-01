use crate::reserves::StrategyReserves;
use defindex_strategy_core::StrategyError;
use soroban_sdk::{contracttype, Address, Env, Vec};

#[contracttype]
pub struct Config {
    pub asset: Address,
    pub pool: Address,
    pub reserve_id: u32,
    pub blend_token: Address,
    pub router: Address,
    pub claim_ids: Vec<u32>,
    pub reward_threshold: i128
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,
    Reserves,
    VaultPos(Address), // Vaults Positions
    Keeper,
}

pub const ONE_DAY_LEDGERS: u32 = 17280; 

pub const INSTANCE_BUMP_AMOUNT: u32 = 30 * ONE_DAY_LEDGERS;
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - ONE_DAY_LEDGERS;

const PERSISTENT_BUMP_AMOUNT: u32 = 120 * ONE_DAY_LEDGERS;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = PERSISTENT_BUMP_AMOUNT - 20 * ONE_DAY_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

// Config
pub fn set_config(e: &Env, config: Config) {
    e.storage().instance().set(&DataKey::Config, &config);
}

pub fn get_config(e: &Env) -> Result<Config, StrategyError> {
    e.storage()
        .instance()
        .get(&DataKey::Config)
        .ok_or(StrategyError::NotInitialized)?
}

/// This function sets the vault shares associated with a specific user address.
/// Shares are stored with 7 decimal places of precision to ensure accurate tracking.  
/// The function also extends the time-to-live (TTL) for persistent storage.
pub fn set_vault_shares(e: &Env, address: &Address, shares: i128) {
    let key = DataKey::VaultPos(address.clone());
    e.storage().persistent().set::<DataKey, i128>(&key, &shares);
}

/// Get the number of strategy shares a user owns. Shares are stored with 7 decimal places of precision.
pub fn get_vault_shares(e: &Env, address: &Address) -> i128 {
    let result = e
        .storage()
        .persistent()
        .get::<DataKey, i128>(&DataKey::VaultPos(address.clone()));
    match result {
        Some(shares) => {
            shares
        }
        None => 0,
    }
}

// Strategy Reserves
pub fn set_strategy_reserves(e: &Env, new_reserves: StrategyReserves) {
    e.storage()
        .instance()
        .set(&DataKey::Reserves, &new_reserves);
}

pub fn get_strategy_reserves(e: &Env) -> StrategyReserves {
    e.storage()
        .instance()
        .get(&DataKey::Reserves)
        .unwrap_or(StrategyReserves {
            total_shares: 0,
            total_b_tokens: 0,
            b_rate: 0,
        })
}

pub fn set_keeper(e: &Env, keeper: &Address) {
    e.storage().instance().set(&DataKey::Keeper, &keeper);
}

pub fn get_keeper(e: &Env) -> Result<Address, StrategyError> {
    e.storage()
        .instance()
        .get(&DataKey::Keeper)
        .ok_or(StrategyError::NotInitialized)
}
