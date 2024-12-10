use sep_40_oracle::Asset;
use soroban_sdk::{panic_with_error, unwrap::UnwrapOptimized, Address, Env, Symbol, Vec};

use crate::error::PriceOracleError;

const ONE_DAY_LEDGERS: u32 = 17280; // assumes 5s a ledger

const LEDGER_THRESHOLD: u32 = ONE_DAY_LEDGERS * 30; // ~ 30 days
const LEDGER_BUMP: u32 = LEDGER_THRESHOLD + ONE_DAY_LEDGERS; // ~ 31 days

pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(LEDGER_THRESHOLD, LEDGER_BUMP);
}

//********** Instance storage ***********//

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "admin"))
        .unwrap_optimized()
}

pub fn get_admin_option(env: &Env) -> Option<Address> {
    env.storage().instance().get(&Symbol::new(env, "admin"))
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "admin"), &admin);
}

pub fn get_resolution(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "res"))
        .unwrap_optimized()
}

pub fn set_resolution(env: &Env, res: u32) {
    env.storage().instance().set(&Symbol::new(env, "res"), &res);
}

pub fn get_decimals(env: &Env) -> u32 {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "decimals"))
        .unwrap_optimized()
}

pub fn set_decimals(env: &Env, decimals: u32) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "decimals"), &decimals);
}

pub fn get_base(env: &Env) -> Asset {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "base"))
        .unwrap_optimized()
}

pub fn set_base(env: &Env, asset: &Asset) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "base"), asset);
}

pub fn get_assets(env: &Env) -> Vec<Asset> {
    env.storage()
        .instance()
        .get(&Symbol::new(env, "assets"))
        .unwrap_or_else(|| Vec::new(env))
}

pub fn set_assets(env: &Env, assets: &Vec<Asset>) {
    env.storage()
        .instance()
        .set(&Symbol::new(env, "assets"), assets);
}

pub fn get_asset_index(env: &Env, asset: &Asset) -> u8 {
    let index: Option<u32>;
    match asset {
        Asset::Stellar(address) => {
            index = env.storage().instance().get(&address);
        }
        Asset::Other(symbol) => {
            index = env.storage().instance().get(&symbol);
        }
    }
    match index {
        Some(index) => index as u8,
        None => panic_with_error!(env, PriceOracleError::AssetMissing),
    }
}

pub fn set_asset_index(env: &Env, asset: &Asset, index: u32) {
    match asset {
        Asset::Stellar(address) => {
            env.storage().instance().set(&address, &index);
        }
        Asset::Other(symbol) => {
            env.storage().instance().set(&symbol, &index);
        }
    }
}

//********** Temporary storage ***********//

pub fn get_price(env: &Env, asset: u8, timestamp: u64) -> Option<i128> {
    let data_key = (timestamp as u128) << 64 | asset as u128;
    let result = env.storage().temporary().get(&data_key);
    // keep stable prices alive
    if result.is_some() && timestamp == 0 {
        env.storage()
            .temporary()
            .extend_ttl(&data_key, LEDGER_THRESHOLD, LEDGER_BUMP);
    }
    result
}

pub fn set_price(env: &Env, asset: u8, price: i128, timestamp: u64) {
    let data_key = (timestamp as u128) << 64 | asset as u128;
    env.storage().temporary().set(&data_key, &price);
    env.storage()
        .temporary()
        .extend_ttl(&data_key, LEDGER_BUMP, LEDGER_BUMP);
}

pub fn get_last_timestamp(env: &Env) -> u64 {
    env.storage()
        .temporary()
        .get(&Symbol::new(env, "timestamp"))
        .unwrap_or_else(|| 0)
}

pub fn set_last_timestamp(env: &Env, timestamp: u64) {
    env.storage()
        .temporary()
        .set(&Symbol::new(env, "timestamp"), &timestamp);
    env.storage()
        .temporary()
        .extend_ttl(&Symbol::new(env, "timestamp"), LEDGER_BUMP, LEDGER_BUMP);
}
