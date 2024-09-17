use soroban_sdk::token::TokenClient;
use soroban_sdk::{Address, Env, Map};

use crate::models::Asset;
use crate::storage::{get_assets, get_total_strategies};
use crate::strategies::get_strategy_client;

// Helper functions
fn get_idle_funds_for_asset(e: &Env, asset: &Asset) -> i128 {
    TokenClient::new(e, &asset.address).balance(&e.current_contract_address())
}

fn get_invested_funds_for_asset(e: &Env, asset: &Asset) -> i128 {
    let total_strategies = get_total_strategies(e);
    let mut invested_funds = 0;
    for i in 0..total_strategies {
        let strategy_client = get_strategy_client(e, i);
        // TODO: Every strategy will work with an specific asset!
        invested_funds += strategy_client.balance(&e.current_contract_address());
    }
    invested_funds
}

// Pub functions

pub fn get_current_idle_funds(e: &Env) -> Map<Address, i128> {
    let assets = get_assets(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for asset in assets {
        map.set(asset.address.clone(), get_idle_funds_for_asset(e, &asset));
    }
    map
}

pub fn get_current_invested_funds(e: &Env) -> Map<Address, i128> {
    let assets = get_assets(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for asset in assets {
        map.set(
            asset.address.clone(),
            get_invested_funds_for_asset(e, &asset),
        );
    }
    map
}

pub fn get_total_managed_funds(e: &Env) -> Map<Address, i128> {
    let assets = get_assets(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for asset in assets {
        let idle_funds = get_idle_funds_for_asset(e, &asset);
        let invested_funds = get_invested_funds_for_asset(e, &asset);
        map.set(asset.address.clone(), idle_funds + invested_funds);
    }
    map
}
