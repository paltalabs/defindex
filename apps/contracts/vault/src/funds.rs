use soroban_sdk::token::TokenClient;
use soroban_sdk::{Address, Env, Map, Vec};

use common::models::AssetStrategySet;
use crate::models::{StrategyAllocation, CurrentAssetInvestmentAllocation};
use crate::storage::{get_assets, get_report};
use crate::strategies::get_strategy_client;

/// Retrieves the idle funds for a given asset.
/// 
/// Idle funds represent the balance of the asset that is held by the current contract 
/// but not actively allocated to any strategies.
///
/// # Arguments
/// * `e` - The current environment instance.
/// * `asset_address` - The address of the asset for which idle funds are being calculated.
///
/// # Returns
/// The idle funds of the asset as an `i128`, representing the unallocated balance.
pub fn fetch_idle_funds_for_asset(e: &Env, asset: &Address) -> i128 {
    TokenClient::new(e, &asset).balance(&e.current_contract_address())
}

/// Retrieves the total funds invested in a specified strategy, excluding any locked fees.
/// 
/// This function performs a cross-contract call to the strategy to fetch the current balance 
/// of the investment. It then subtracts any locked fees from the total to provide an accurate 
/// representation of the funds that are actively invested and available to the user.
///
/// # Arguments
/// * `e` - The current environment instance.
/// * `strategy_address` - The address of the strategy whose investment balance is to be retrieved.
///
/// # Returns
/// The total invested funds in the strategy as an `i128`, excluding locked fees.
pub fn fetch_strategy_invested_funds(e: &Env, strategy_address: &Address) -> i128 {
    let strategy_client = get_strategy_client(e, strategy_address.clone());
    let strategy_invested_funds = strategy_client.balance(&e.current_contract_address());

    let report = get_report(e, strategy_address);
    strategy_invested_funds.checked_sub(report.locked_fee).unwrap_or(0)
}


/// Calculates the total funds invested in strategies for a given asset and 
/// provides a detailed breakdown of allocations.
///
/// This function aggregates the balances of all strategies linked to the specified 
/// asset and returns both the total invested amount and a detailed allocation.
///
/// # Arguments
/// * `e` - The current environment instance.
/// * `asset_strategy_set` - The asset and its associated set of strategies to evaluate.
///
/// # Returns
/// A tuple containing:
/// * `i128`: The total funds invested across all strategies.
/// * `Vec<StrategyAllocation>`: A vector with the allocation details for each strategy.
pub fn fetch_invested_funds_for_asset(e: &Env, asset_strategy_set: &AssetStrategySet) -> (i128, Vec<StrategyAllocation>){
    let mut invested_funds = 0;
    let mut strategy_allocations: Vec<StrategyAllocation> = Vec::new(e);
    for strategy in asset_strategy_set.strategies.iter() {
        let strategy_balance = fetch_strategy_invested_funds(e, &strategy.address);
        invested_funds += strategy_balance;
        strategy_allocations.push_back(StrategyAllocation {
            strategy_address: strategy.address.clone(),
            amount: strategy_balance,
        });
    }
    (invested_funds, strategy_allocations)
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
pub fn fetch_total_managed_funds(e: &Env) -> Map<Address, CurrentAssetInvestmentAllocation> {
    let assets = get_assets(e);
    let mut map: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(e);
    for asset in assets {
        let idle_amount = fetch_idle_funds_for_asset(e, &asset.address);
        let (invested_amount, strategy_allocations) = fetch_invested_funds_for_asset(e, &asset);
        let total_amount = idle_amount + invested_amount;
        map.set(
            asset.address.clone(),
            CurrentAssetInvestmentAllocation {
                asset: asset.address.clone(),
                total_amount,
                idle_amount,
                invested_amount,
                strategy_allocations,
            },
        );
    }
    map
}

/*
    User experience functions. The following functions are not being used inernally in 
    the contrac, but are intender to be used by the client for a better user experience.
    They create maps for better redeability

*/


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
        map.set(asset.address.clone(), fetch_idle_funds_for_asset(e, &asset.address));
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
        let (invested_funds, _) = fetch_invested_funds_for_asset(e, &asset);
        map.set(
            asset.address.clone(),
            invested_funds
        );
    }
    map
}
