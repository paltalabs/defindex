use soroban_sdk::{contracttype, Address, Env, Vec, panic_with_error};

use common::models::AssetStrategySet;

use crate::report::Report;
use crate::error::ContractError;
const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
const LEDGER_BUMP: u32 = 120 * DAY_IN_LEDGERS;
const LEDGER_THRESHOLD: u32 = LEDGER_BUMP - 20 * DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

#[derive(Clone)]
#[contracttype]
enum DataKey {
    TotalAssets,           // Total number of tokens
    AssetStrategySet(u32), // AssetStrategySet Addresse by index
    DeFindexProtocolFeeReceiver,
    DeFindexFactory,
    Upgradable,
    VaultFee,
    SoroswapRouter,
    DeFindexProtocolFeeRate,
    Factory,
    Report(Address),
}

// AssetStrategySet(index)
pub fn set_asset(e: &Env, index: u32, asset: &AssetStrategySet) {
    e.storage()
        .instance()
        .set(&DataKey::AssetStrategySet(index), asset);
}

pub fn get_asset(e: &Env, index: u32) -> Result<AssetStrategySet, ContractError> {
    e.storage()
        .instance()
        .get(&DataKey::AssetStrategySet(index))
        .ok_or(ContractError::NotInitialized)
}

pub fn get_assets(e: &Env) -> Result<Vec<AssetStrategySet>, ContractError> {
    let total_assets = get_total_assets(e)?;
    let mut assets = Vec::new(e);
    for i in 0..total_assets {
        assets.push_back(get_asset(e, i)?);
    }

    Ok(assets)
}

pub fn set_total_assets(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalAssets, &n);
}

pub fn get_total_assets(e: &Env) -> Result<u32, ContractError> {
    e.storage().instance().get(&DataKey::TotalAssets).ok_or(ContractError::NotInitialized)
}

// DeFindex Fee Receiver
pub fn set_defindex_protocol_fee_receiver(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::DeFindexProtocolFeeReceiver, address);
}

pub fn get_defindex_protocol_fee_receiver(e: &Env) -> Result<Address, ContractError> {
    e.storage()
        .instance()
        .get(&DataKey::DeFindexProtocolFeeReceiver)
        .ok_or(ContractError::NotInitialized)
}

// DeFindex Fee BPS
pub fn set_defindex_protocol_fee_rate(e: &Env, value: &u32) {
    e.storage()
        .instance()
        .set(&DataKey::DeFindexProtocolFeeRate, value);
}

pub fn get_defindex_protocol_fee_rate(e: &Env) -> u32 {
    e.storage()
        .instance()
        .get(&DataKey::DeFindexProtocolFeeRate)
        .unwrap()
}

// DeFindex Factory
pub fn set_factory(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::DeFindexFactory, address);
}

// Soroswap Router
pub fn set_soroswap_router(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::SoroswapRouter, address);
}

pub fn get_soroswap_router(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::SoroswapRouter)
        .unwrap()
}


// Vault Share
pub fn set_vault_fee(e: &Env, vault_fee: &u32) {
    if vault_fee > &9000u32 {
        panic_with_error!(&e, ContractError::MaximumFeeExceeded);
    }
    e.storage().instance().set(&DataKey::VaultFee, vault_fee);
}

pub fn get_vault_fee(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::VaultFee).unwrap()
}

// Strategy Previous Balance
pub fn set_report(e: &Env, strategy_address: &Address, report: &Report) {
    let key = DataKey::Report(strategy_address.clone());
    e.storage()
        .persistent()
        .set::<DataKey, Report>(&key, report);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
}

pub fn get_report(e: &Env, strategy_address: &Address) -> Report {
    let key = DataKey::Report(strategy_address.clone());
    let result = e.storage().persistent().get::<DataKey, Report>(&key);
    match result {
        Some(report) => {
            e.storage()
                .persistent()
                .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
            report
        }
        None => Report {
            prev_balance: 0,
            gains_or_losses: 0,
            locked_fee: 0,
        },
    }
}

// Upgradable
pub fn set_is_upgradable(e: &Env, value: &bool) {
    e.storage().instance().set(&DataKey::Upgradable, value);
}

pub fn is_upgradable(e: &Env) -> bool {
    e.storage().instance().get(&DataKey::Upgradable).unwrap_or(true)
}