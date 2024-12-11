use soroban_sdk::{contract, contractimpl, unwrap::UnwrapOptimized, Address, Env, Vec};

use sep_40_oracle::{Asset, PriceData, PriceFeedTrait};

use crate::storage;

/// ### Mock Price Feed Oracle
///
/// Mock Contract to set and fetch prices from a SEP-40 compliant Price Feed Oracle.
///
/// ### Dev
/// For testing purposes only!
#[contract]
pub struct MockOracle;

trait MockPriceFeed {
    /// Set the data for the mock price feed oracle.
    fn set_data(
        env: Env,
        admin: Address,
        base: Asset,
        assets: Vec<Asset>,
        decimals: u32,
        resolution: u32,
    );

    /// Sets the mocked price for an asset at a given timestamp.
    ///
    /// The prices are defined in the same order as the assets the oracle supports.
    fn set_price(env: Env, prices: Vec<i128>, timestamp: u64);

    /// Sets a stable "lastprice" such that the price timestamp is always the current ledger.
    ///
    /// The prices are defined in the same order as the assets the oracle supports.
    ///
    /// This price will be ignored if `set_price` is called, until `set_price_stable` is called again.
    fn set_price_stable(env: Env, prices: Vec<i128>);
}

#[contractimpl]
impl MockPriceFeed for MockOracle {
    fn set_data(
        env: Env,
        admin: Address,
        base: Asset,
        assets: Vec<Asset>,
        decimals: u32,
        resolution: u32,
    ) {
        if let Some(old_admin) = storage::get_admin_option(&env) {
            old_admin.require_auth();
        } else {
            admin.require_auth();
        }
        storage::set_admin(&env, &admin);
        storage::set_base(&env, &base);
        storage::set_assets(&env, &assets);
        storage::set_decimals(&env, decimals);
        storage::set_resolution(&env, resolution);

        // set asset index's
        for (i, asset) in assets.into_iter().enumerate() {
            storage::set_asset_index(&env, &asset, i as u32);
        }
        storage::extend_instance(&env);
    }

    fn set_price(env: Env, prices: Vec<i128>, timestamp: u64) {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        storage::extend_instance(&env);
        storage::set_last_timestamp(&env, timestamp);
        set_prices(&env, prices, timestamp);
    }

    fn set_price_stable(env: Env, prices: Vec<i128>) {
        let admin = storage::get_admin(&env);
        admin.require_auth();

        storage::extend_instance(&env);
        storage::set_last_timestamp(&env, 0);
        set_prices(&env, prices, 0);
    }
}

#[contractimpl]
impl PriceFeedTrait for MockOracle {
    fn base(env: Env) -> Asset {
        storage::get_base(&env)
    }

    fn assets(env: Env) -> Vec<Asset> {
        storage::get_assets(&env)
    }

    fn decimals(env: Env) -> u32 {
        storage::get_decimals(&env)
    }

    fn resolution(env: Env) -> u32 {
        storage::get_resolution(&env)
    }

    fn price(env: Env, asset: Asset, timestamp: u64) -> Option<PriceData> {
        get_price_data_asset(&env, &asset, timestamp)
    }

    fn prices(env: Env, asset: Asset, records: u32) -> Option<Vec<PriceData>> {
        let mut prices: Vec<PriceData> = Vec::new(&env);
        let resolution = storage::get_resolution(&env) as u64;
        let mut timestamp = storage::get_last_timestamp(&env);
        if timestamp == 0 || resolution == 0 {
            return None;
        }

        let mut records = records;
        if records > 20 {
            records = 20;
        }
        for _ in 0..records {
            let price_data = get_price_data_asset(&env, &asset, timestamp);
            if price_data.is_none() {
                break;
            }
            prices.push_back(price_data.unwrap_optimized());
            timestamp -= resolution;
        }

        if prices.len() == 0 {
            return None;
        }
        Some(prices)
    }

    fn lastprice(env: Env, asset: Asset) -> Option<PriceData> {
        let timestamp = storage::get_last_timestamp(&env);
        get_price_data_asset(&env, &asset, timestamp)
    }
}

//********** Helpers **********//

/// Set the prices for all assets at a given timestamp.
fn set_prices(env: &Env, prices: Vec<i128>, timestamp: u64) {
    for (i, price) in prices.iter().enumerate() {
        let asset = i as u8;
        // store the new price
        storage::set_price(env, asset, price, timestamp);
    }
}

/// Get the price for a given asset at at given timestamp.
///
/// Returns the most recent ledger if the timestamp is stable (0)
fn get_price_data_asset(env: &Env, asset: &Asset, timestamp: u64) -> Option<PriceData> {
    let asset_index = storage::get_asset_index(&env, &asset);
    let price = storage::get_price(&env, asset_index, timestamp);
    match price {
        Some(price) => {
            let mut data_timestamp = timestamp;
            if data_timestamp == 0 {
                data_timestamp = env.ledger().timestamp();
            }
            Some(PriceData {
                price,
                timestamp: data_timestamp,
            })
        }
        None => None,
    }
}
