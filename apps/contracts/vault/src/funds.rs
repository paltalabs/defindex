use soroban_sdk::token::TokenClient;
use soroban_sdk::{Address, Env, Map, Vec};

use common::models::AssetStrategySet;
use crate::models::{StrategyInvestment, CurrentAssetInvestmentAllocation};
use crate::storage::get_assets;
use crate::strategies::get_strategy_client;

// Funds for AssetStrategySet

/// Fetches the idle funds for a given asset. Idle funds refer to the balance of the asset
/// that is currently not invested in any strategies.
///
/// # Arguments
/// * `e` - The current environment instance.
/// * `asset` - The asset for which idle funds are being fetched.
///
/// # Returns
/// * The idle balance (i128) of the asset in the current contract address.
pub fn fetch_idle_funds_for_asset(e: &Env, asset: &Address) -> i128 {
    TokenClient::new(e, &asset).balance(&e.current_contract_address())
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
pub fn fetch_invested_funds_for_strategy(e: &Env, strategy_address: &Address) -> i128 {
    let strategy_client = get_strategy_client(e, strategy_address.clone());
    strategy_client.balance(&e.current_contract_address())
}

// // Investment Allocation in Strategies
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct StrategyInvestment {
//     pub strategy: Address,
//     pub amount: i128,
// }


// return total invested funds but also a vec of StrategyInvestment
pub fn fetch_invested_funds_for_asset(e: &Env, asset: &AssetStrategySet) -> (i128, Vec<StrategyInvestment>){
    let mut invested_funds = 0;
    let mut strategy_investments: Vec<StrategyInvestment> = Vec::new(e);
    for strategy in asset.strategies.iter() {
        let strategy_balance = fetch_invested_funds_for_strategy(e, &strategy.address);
        invested_funds += strategy_balance;
        strategy_investments.push_back(StrategyInvestment {
            strategy: strategy.address.clone(),
            amount: strategy_balance,
        });
    }
    (invested_funds, strategy_investments)
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

/// Fetches the total managed funds for all assets. This includes both idle and invested funds.
/// It returns a map where the key is the asset's address and the value is the total managed balance
/// (idle + invested). With this map we can calculate the current managed funds ratio.
///
/// # Arguments
/// * `e` - The current environment instance.
///
/// # Returns
/// * A map where each entry represents an asset's address and its total managed balance.


// // Current Asset Investment Allocation
// #[contracttype]
// #[derive(Clone, Debug, Eq, PartialEq)]
// pub struct CurrentAssetInvestmentAllocation {
//     pub asset: Address,
//     pub total_amount: i128,
//     pub idle_amount: i128,
//     pub invested_amount: i128,
//     pub strategy_investments: Vec<StrategyInvestment>,
// }
pub fn fetch_total_managed_funds(e: &Env) -> Map<Address, CurrentAssetInvestmentAllocation> {
    let assets = get_assets(e);
    let mut map: Map<Address, CurrentAssetInvestmentAllocation> = Map::new(e);
    for asset in assets {
        let idle_amount = fetch_idle_funds_for_asset(e, &asset.address);
        let (invested_amount, strategy_investments) = fetch_invested_funds_for_asset(e, &asset);
        let total_amount = idle_amount + invested_amount;
        map.set(
            asset.address.clone(),
            CurrentAssetInvestmentAllocation {
                asset: asset.address.clone(),
                total_amount,
                idle_amount,
                invested_amount,
                strategy_investments,
            },
        );
    }
    map
}
