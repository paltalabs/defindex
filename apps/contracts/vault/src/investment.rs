use soroban_sdk::{Address, Env, Map, Vec};

use crate::{
    models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, StrategyAllocation},
    ContractError,
};
use common::models::AssetStrategySet;

/// Generates investment allocations for a set of assets and their associated strategies.
///
/// This function calculates the distribution of funds across strategies for each asset based
/// on the current state of strategy investments. The allocations are returned as a vector,
/// where each entry corresponds to an asset's investment allocation or `None` if no allocation
/// is required.
///
/// # Arguments
/// - `e` - Reference to the current environment.
/// - `assets` - A vector of `AssetStrategySet` objects representing the assets and their strategies.
/// - `total_managed_funds` - A map containing the current allocation of funds across all strategies for each asset.
/// - `amounts` - A vector of amounts representing the funds to be allocated for each asset.
///
/// # Returns
/// - `Ok(Vec<Option<AssetInvestmentAllocation>>)` - A vector of investment allocations where each entry
///   represents an asset's strategy allocations. If an asset does not require allocation, its entry is `None`.
/// - `Err(ContractError)` - If any errors occur during the allocation process, such as invalid data or calculations.
///
/// # Function Flow
/// 1. **Iterate Over Assets**: For each asset in the provided list:
///    - Skip assets with zero amounts or no prior investments.
///    - Calculate the allocation of funds across strategies proportionally based on the current state.
/// 2. **Proportional Distribution**:
///    - For each strategy within an asset, determine the proportional investment based on its existing allocation.
///    - Ensure that all amounts are correctly calculated without overflows or division errors.
/// 3. **Prepare Allocation**:
///    - Append the calculated strategy allocations to the resulting vector.
///    - Include `None` for assets with no required allocations.
/// 4. **Return Results**: Return the vector containing the investment allocations.
///
/// # Notes
/// - This function does not execute the investments; it only prepares the allocations.
/// - It assumes that the provided `total_managed_funds` contains valid and complete data.
pub fn generate_investment_allocations(
    e: &Env,
    assets: &Vec<AssetStrategySet>,
    total_managed_funds: &Map<Address, CurrentAssetInvestmentAllocation>,
    amounts: &Vec<i128>,
) -> Result<Vec<Option<AssetInvestmentAllocation>>, ContractError> {
    let mut asset_investments = Vec::new(&e);

    for (i, amount) in amounts.iter().enumerate() {
        let asset = assets.get(i as u32).unwrap();

        let current_asset_allocation = total_managed_funds.get(asset.address.clone()).unwrap();
        let asset_invested_funds = current_asset_allocation.invested_amount;

        // We only consider assets that have a non zero allocation
        // if the amount already invested in the asset is 0,
        // this means that there is no previous investment in the asset, so we can just
        // invest, and we need to wait for the manager to execute a manual investment of the idle assets
        // on the strategies.
        if amount > 0 && asset_invested_funds > 0 {
            // here the asset will be distributed amont the different strategies considering the current raio
            // of investment in each strategy.
            let mut strategy_allocations = Vec::new(&e);
            let mut remaining_amount = amount;

            for (j, strategy) in asset.strategies.iter().enumerate() {
                // Determine the investment amount for the strategy
                let invest_amount = if j == (asset.strategies.len() as usize).checked_sub(1).unwrap()  {
                    remaining_amount
                } else {
                    let strategy_invested_funds = current_asset_allocation
                        .strategy_allocations
                        .get(j as u32)
                        .unwrap()
                        .amount;

                    amount
                        .checked_mul(strategy_invested_funds)
                        .and_then(|v| v.checked_div(asset_invested_funds))
                        .unwrap()
                };

                // Update the remaining amount
                remaining_amount -= invest_amount;

                // Add the strategy allocation
                strategy_allocations.push_back(if invest_amount > 0 && strategy.paused == false {
                    Some(StrategyAllocation {
                        strategy_address: strategy.address.clone(),
                        amount: invest_amount,
                    })
                } else {
                    None
                });
            }

            // Add the asset investment allocation
            asset_investments.push_back(Some(AssetInvestmentAllocation {
                asset: asset.address.clone(),
                strategy_allocations,
            }));
        } else {
            asset_investments.push_back(None); // No investments to be executed for this asset
        }
    }
    Ok(asset_investments)
}
