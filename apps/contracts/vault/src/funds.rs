use soroban_sdk::token::TokenClient;
use soroban_sdk::{Address, Env, Vec};

use crate::models::{CurrentAssetInvestmentAllocation, StrategyAllocation};
use crate::storage::{get_assets, get_report};
use crate::strategies::get_strategy_client;
use crate::report;
use crate::ContractError;
use common::models::AssetStrategySet;

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

/// Retrieves the total funds invested in a specified strategy, excluding current locked fees, with an option to lock new fees.
///
/// This function performs a cross-contract call to the strategy contract to fetch the current investment balance.
/// It always returns the balance minus the current locked fees, representing the actively invested funds available 
/// to the user. If `lock_fees` is `true`, it updates the report and locks new fees before calculating the result; 
/// if `false`, it uses the existing locked fees without updating the report.
///
/// # Arguments
/// * `e` - The current environment instance, providing access to the contract's storage and functions.
/// * `strategy_address` - The address of the strategy whose investment balance is to be retrieved.
/// * `lock_fees` - A boolean flag indicating whether to update the report and lock new fees before calculating 
///   the balance. If `true`, new fees are locked; if `false`, only existing locked fees are subtracted.
///
/// # Returns
/// * `Result<i128, ContractError>` - Returns the total invested funds in the strategy (excluding locked fees) as 
///   an `i128`. Returns a `ContractError` if an error occurs (e.g., overflow, underflow, or report update failure).
///
pub fn fetch_strategy_invested_funds(e: &Env, strategy_address: &Address, lock_fees: bool) -> Result<i128, ContractError> {
    let strategy_client = get_strategy_client(e, strategy_address.clone());
    let strategy_invested_funds = strategy_client.balance(&e.current_contract_address());
    
    if !lock_fees {
        return Ok(strategy_invested_funds
            .checked_sub(
                get_report(e, strategy_address).locked_fee
            ).unwrap_or(0))
    } else {
        let report = report::update_report_and_lock_fees(e, strategy_address, strategy_invested_funds)?;
        Ok(strategy_invested_funds.checked_sub(report.locked_fee).unwrap_or(0))
    }
}

/// Calculates the total funds invested in strategies for a given asset and provides a detailed breakdown of allocations.
///
/// This function aggregates the balances of all strategies linked to the specified asset, always excluding current locked fees 
/// from the total. The `lock_fees` flag determines whether new fees are locked during the calculation: if `true`, the report 
/// for each strategy is updated and new fees are locked before excluding them; if `false`, only existing locked fees are excluded.
/// The function returns both the total invested amount (net of locked fees) and a detailed allocation of funds for each strategy.
///
/// # Arguments
/// * `e` - The current environment instance.
/// * `asset_strategy_set` - The asset and its associated set of strategies to evaluate.
/// * `lock_fees` - A flag indicating whether to update strategy reports and lock new fees before calculating balances. 
///   If `true`, new fees are locked; if `false`, only existing locked fees are subtracted.
///
/// # Returns
/// A tuple containing:
/// * `i128`: The total funds invested across all strategies, excluding locked fees.
/// * `Vec<StrategyAllocation>`: A vector with the allocation details for each strategy.
pub fn fetch_invested_funds_for_asset(
    e: &Env,
    asset_strategy_set: &AssetStrategySet,
    lock_fees: bool,
) -> Result<(i128, Vec<StrategyAllocation>), ContractError> {
    let mut invested_funds: i128 = 0;
    let mut strategy_allocations: Vec<StrategyAllocation> = Vec::new(e);
    for strategy in asset_strategy_set.strategies.iter() {
        let strategy_balance = fetch_strategy_invested_funds(e, &strategy.address, lock_fees)?;
        invested_funds = invested_funds.checked_add(strategy_balance).unwrap();
        strategy_allocations.push_back(StrategyAllocation {
            strategy_address: strategy.address.clone(),
            amount: strategy_balance,
            paused: strategy.paused
        });
    }
    Ok((invested_funds, strategy_allocations))
}

/// Fetches the total managed funds for all assets, including both idle and invested funds.
/// The `lock_fees` flag determines whether to exclude locked fees from the invested funds when calculating the total.
///
/// This function returns a vector where each entry represents an asset's total managed balance 
/// (idle + invested) in the same order as the assets are listed.
///
/// # Arguments
/// * `e` - The current environment instance.
/// * `lock_fees` - A flag indicating whether to exclude locked fees from the invested funds. 
///   If `true`, the invested funds will exclude any locked fees; otherwise, they include locked fees.
///
/// # Returns
/// * A vector where each entry represents an asset's total managed balance, including idle and invested funds. 
///   Each entry is a `CurrentAssetInvestmentAllocation` containing the total balance, idle funds, invested funds, and the strategy allocations for the asset.
pub fn fetch_total_managed_funds(
    e: &Env,
    lock_fees: bool,
) -> Result<Vec<CurrentAssetInvestmentAllocation>, ContractError> {
    let assets = get_assets(e)?;
    let mut allocations: Vec<CurrentAssetInvestmentAllocation> = Vec::new(e);
    for asset in &assets {
        let idle_amount = fetch_idle_funds_for_asset(e, &asset.address);
        let (invested_amount, strategy_allocations) =
            fetch_invested_funds_for_asset(e, &asset, lock_fees)?;
        let total_amount = idle_amount.checked_add(invested_amount).unwrap();
        allocations.push_back(CurrentAssetInvestmentAllocation {
            asset: asset.address.clone(),
            total_amount,
            idle_amount,
            invested_amount,
            strategy_allocations,
        });
    }
    Ok(allocations)
}
