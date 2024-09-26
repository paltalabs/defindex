use defindex_strategy_core::DeFindexStrategyClient;
use soroban_sdk::{Env, Address};

use crate::{models::{Asset, Strategy}, storage::{get_asset, get_assets, get_total_assets, set_asset}, ContractError};

pub fn get_strategy_client(e: &Env, address: Address) -> DeFindexStrategyClient {
    DeFindexStrategyClient::new(&e, &address)
}

/// Finds the asset corresponding to the given strategy address.
pub fn get_strategy_asset(e: &Env, strategy_address: &Address) -> Result<Asset, ContractError> {
    let assets = get_assets(e);

    for asset in assets.iter() {
        if asset.strategies.iter().any(|strategy| &strategy.address == strategy_address) {
            return Ok(asset);
        }
    }

    Err(ContractError::StrategyNotFound)
}

/// Finds the strategy struct corresponding to the given strategy address within the given asset.
pub fn get_strategy_struct(strategy_address: &Address, asset: &Asset) -> Result<Strategy, ContractError> {
    asset
        .strategies
        .iter()
        .find(|strategy| &strategy.address == strategy_address && !strategy.paused)
        .ok_or(ContractError::StrategyNotFound)
}

/// Pauses a strategy by setting its `paused` field to `true`.
/// Finds the asset that contains the strategy and updates the storage.
pub fn pause_strategy(e: &Env, strategy_address: Address) -> Result<(), ContractError> {
    let total_assets = get_total_assets(e);

    // Iterate through all assets to find the one that contains the strategy
    for i in 0..total_assets {
        let mut asset = get_asset(e, i);

        // Check if this asset contains the strategy
        for (j, strategy) in asset.strategies.iter().enumerate() {
            if strategy.address == strategy_address {
                // Pause the strategy by modifying its `paused` field
                let mut updated_strategy = strategy.clone();
                updated_strategy.paused = true;

                // Update the strategy in the asset
                asset.strategies.set(j as u32, updated_strategy);

                // Save the updated asset back into storage
                set_asset(e, i, &asset);

                return Ok(());
            }
        }
    }

    // If no strategy is found, return an error
    Err(ContractError::StrategyNotFound)
}

/// Unpauses a strategy by setting its `paused` field to `false`.
/// Finds the asset that contains the strategy and updates the storage.
pub fn unpause_strategy(e: &Env, strategy_address: Address) -> Result<(), ContractError> {
    let total_assets = get_total_assets(e);

    // Iterate through all assets to find the one that contains the strategy
    for i in 0..total_assets {
        let mut asset = get_asset(e, i);

        // Check if this asset contains the strategy
        for (j, strategy) in asset.strategies.iter().enumerate() {
            if strategy.address == strategy_address {
                // Unpause the strategy by modifying its `paused` field
                let mut updated_strategy = strategy.clone();
                updated_strategy.paused = false;

                // Update the strategy in the asset
                asset.strategies.set(j as u32, updated_strategy);

                // Save the updated asset back into storage
                set_asset(e, i, &asset);

                return Ok(());
            }
        }
    }

    // If no strategy is found, return an error
    Err(ContractError::StrategyNotFound)
}