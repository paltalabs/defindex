use soroban_sdk::token::TokenClient;
use soroban_sdk::{Address, Env, Map};

use crate::models::AssetAllocation;
use crate::storage::get_assets;
use crate::strategies::get_strategy_client;

// Funds for AssetAllocation 

/// Fetches the idle funds for a given asset. Idle funds refer to the balance of the asset
/// that is currently not invested in any strategies.
/// 
/// # Arguments
/// * `e` - The current environment instance.
/// * `asset` - The asset for which idle funds are being fetched.
/// 
/// # Returns
/// * The idle balance (i128) of the asset in the current contract address.
fn fetch_idle_funds_for_asset(e: &Env, asset: &AssetAllocation) -> i128 {
    TokenClient::new(e, &asset.address).balance(&e.current_contract_address())
}


/// Fetches the total funds that are invested for a given asset. 
/// It iterates through all the strategies associated with the asset and sums their balances.
/// 
/// # Arguments
/// * `e` - The current environment instance.
/// * `asset` - The asset for which invested funds are being fetched.
/// 
/// # Returns
/// * The total invested balance (i128) of the asset across all strategies.
fn fetch_invested_funds_for_asset(e: &Env, asset: &AssetAllocation) -> i128 {
    let mut invested_funds = 0;
    for strategy in asset.strategies.iter() {
        let strategy_client = get_strategy_client(e, strategy.address);
        invested_funds += strategy_client.balance(&e.current_contract_address());
    }
    invested_funds
}


// Pub functions

/// Fetches the current idle funds for all assets managed by the contract. 
/// It returns a map where the key is the asset's address and the value is the idle balance.
/// 
/// # Arguments
/// * `e` - The current environment instance.
/// 
/// # Returns
/// * A map where each entry represents an asset's address and its corresponding idle balance.
pub fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128> {
    let assets = get_assets(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for asset in assets {
        map.set(asset.address.clone(), fetch_idle_funds_for_asset(e, &asset));
    }
    map
}

/// Fetches the current invested funds for all assets managed by the contract. 
/// It returns a map where the key is the asset's address and the value is the invested balance.
/// 
/// # Arguments
/// * `e` - The current environment instance.
/// 
/// # Returns
/// * A map where each entry represents an asset's address and its corresponding invested balance.
pub fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128> {
    let assets = get_assets(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for asset in assets {
        map.set(
            asset.address.clone(),
            fetch_invested_funds_for_asset(e, &asset),
        );
    }
    map
}

/// Fetches the total managed funds for all assets. This includes both idle and invested funds.
/// It returns a map where the key is the asset's address and the value is the total managed balance 
/// (idle + invested). With this map we can calculate the current managed funds ratio.
/// 
/// # Arguments
/// * `e` - The current environment instance.
/// 
/// # Returns
/// * A map where each entry represents an asset's address and its total managed balance.
pub fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128> {
    let assets = get_assets(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for asset in assets {
        let idle_funds = fetch_idle_funds_for_asset(e, &asset);
        let invested_funds = fetch_invested_funds_for_asset(e, &asset);
        map.set(asset.address.clone(), idle_funds + invested_funds);
    }
    map
}
