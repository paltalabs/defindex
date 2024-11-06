use soroban_sdk::{contracttype, Address, Env, Vec};

use crate::models::AssetAllocation;

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}


#[derive(Clone)]
#[contracttype]
enum DataKey {
    AssetAllocation(u32), // AssetAllocation Addresse by index
    TotalAssets,          // Total number of tokens
    DeFindexProtocolFeeReceiver,
    Factory,
    LastFeeAssessment,
    VaultShare,
}



// Assets Management
pub fn set_asset(e: &Env, index: u32, asset: &AssetAllocation) {
    e.storage()
        .instance()
        .set(&DataKey::AssetAllocation(index), asset);
}

pub fn get_asset(e: &Env, index: u32) -> AssetAllocation {
    e.storage()
        .instance()
        .get(&DataKey::AssetAllocation(index))
        .unwrap()
}

pub fn set_total_assets(e: &Env, n: u32) {
    e.storage().instance().set(&DataKey::TotalAssets, &n);
}

pub fn get_total_assets(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::TotalAssets).unwrap()
}

pub fn get_assets(e: &Env) -> Vec<AssetAllocation> {
    let total_assets = get_total_assets(e);
    let mut assets = Vec::new(e);
    for i in 0..total_assets {
        assets.push_back(get_asset(e, i));
    }
    assets
}

// DeFindex Fee Receiver
pub fn set_defindex_protocol_fee_receiver(e: &Env, address: &Address) {
    e.storage()
        .instance()
        .set(&DataKey::DeFindexProtocolFeeReceiver, address);
}

pub fn get_defindex_protocol_fee_receiver(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::DeFindexProtocolFeeReceiver)
        .unwrap()
}

// DeFindex Factory
pub fn set_factory(e: &Env, address: &Address) {
    e.storage().instance().set(&DataKey::Factory, address);
}

pub fn get_factory(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Factory).unwrap()
}

// Last Fee Assesment
pub fn set_last_fee_assesment(e: &Env, timestamp: &u64) {
    e.storage()
        .instance()
        .set(&DataKey::LastFeeAssessment, timestamp);
}

pub fn get_last_fee_assesment(e: &Env) -> u64 {
    e.storage()
        .instance()
        .get(&DataKey::LastFeeAssessment)
        .unwrap()
}

// Vault Share
pub fn set_vault_fee(e: &Env, vault_fee: &u32) {
    e.storage().instance().set(&DataKey::VaultShare, vault_fee);
}

pub fn get_vault_share(e: &Env) -> u32 {
    e.storage().instance().get(&DataKey::VaultShare).unwrap()
}
