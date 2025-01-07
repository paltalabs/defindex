use soroban_sdk::{panic_with_error, Address, Env, Map, Vec};

use crate::{
    models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, StrategyAllocation},
    strategies::invest_in_strategy,
    utils::check_nonnegative_amount,
    ContractError,
};
use common::models::AssetStrategySet;
use crate::events;

/// Executes investment allocations for a set of assets based on the provided investment strategies.
///
/// This function ensures that the specified assets and strategies match the contract's known configuration,
/// then validates and processes the investment allocations for each asset and its strategies. It assumes
/// that the caller is responsible for ensuring the correctness of investment ratios and does not check the
/// current state of the strategies or existing investments.
///
/// # Arguments
/// * `e` - The current environment reference.
/// * `assets` - A vector of `AssetStrategySet` representing the assets and their associated strategies
///   managed by this vault.
/// * `asset_investments` - A vector of optional investment allocations for each asset.
///
/// # Returns
/// * `Result<(), ContractError>` - Returns `Ok(())` if all investments are successful, or an appropriate
///   `ContractError` if validation or execution fails.
///
/// # Function Details
/// 1. **Iterates Over Asset Investments**: Loops through each asset's investment allocation, processing only
///    defined allocations.
/// 2. **Validation**:
///    - Confirms that the asset's address matches the expected address in the allocation.
///    - Checks that the number of strategies in the asset matches the provided allocation.
/// 3. **Processes Strategy Investments**:
///    - Ensures that investment amounts are non-negative.
///    - Verifies that strategies are active before investing.
///    - Executes the investment for valid allocations by calling `invest_in_strategy`.
///
/// # Errors
/// * `ContractError::WrongAssetAddress` - If the asset's address does not match the allocation.
/// * `ContractError::WrongStrategiesLength` - If the number of strategies in the asset and allocation do not match.
/// * `ContractError::StrategyPaused` - If an allocation targets a paused strategy.
///
/// # Notes
/// - The function relies on the assets being ordered consistently with the investment allocations.
/// - It allows the caller to update investment ratios freely, without verifying the current state of investments
///   or strategies.
pub fn check_and_execute_investments(
    e: &Env,
    assets: &Vec<AssetStrategySet>,
    asset_investments: &Vec<Option<AssetInvestmentAllocation>>,
) -> Result<(), ContractError> {
    // Iterate over each asset investment allocation
    for (i, asset_investment_opt) in asset_investments.iter().enumerate() {
        if let Some(asset_investment) = asset_investment_opt {
            // Proceed only if allocation is defined
            let asset = assets.get(i as u32).unwrap();

            // Verify the asset address matches the specified investment allocation
            if asset.address != asset_investment.asset {
                panic_with_error!(&e, ContractError::WrongAssetAddress);
            }

            // Ensure the number of strategies aligns between asset and investment
            if asset.strategies.len() != asset_investment.strategy_allocations.len() {
                panic_with_error!(&e, ContractError::WrongStrategiesLength);
            }

            // Process each defined strategy investment for the current asset
            for (j, strategy_investment_opt) in
                asset_investment.strategy_allocations.iter().enumerate()
            {
                if let Some(strategy_investment) = strategy_investment_opt {
                    // Validate amount is non-negative
                    check_nonnegative_amount(strategy_investment.amount)?;

                    // Ensure the strategy is active before proceeding
                    let strategy = asset.strategies.get(j as u32).unwrap();
                    if strategy_investment.amount > 0 && strategy.paused {
                        panic_with_error!(&e, ContractError::StrategyPaused);
                    }

                    // Execute the investment if checks pass
                    invest_in_strategy(
                        &e,
                        &asset.address,
                        &strategy.address,
                        &strategy_investment.amount,
                    )?;
                }
            }
        }
    }
    events::emit_execute_investment_event(e, assets.clone(), asset_investments.clone());
    Ok(())
}

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
                let invest_amount = if j == asset.strategies.len() as usize - 1 {
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
                strategy_allocations.push_back(if invest_amount > 0 {
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
