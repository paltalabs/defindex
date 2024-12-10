use soroban_sdk::{Env, Vec, panic_with_error};

use crate::{
    models::{AssetInvestmentAllocation, StrategyAllocation},
    strategies::invest_in_strategy,
    utils::{check_nonnegative_amount},
    ContractError,
    funds::{
        fetch_invested_funds_for_asset, fetch_strategy_invested_funds,
    },
};
use common::models::AssetStrategySet;

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
    e: Env, 
    assets: Vec<AssetStrategySet>,
    asset_investments: Vec<Option<AssetInvestmentAllocation>>
) -> Result<(), ContractError> {

    // Iterate over each asset investment allocation
    for (i, asset_investment_opt) in asset_investments.iter().enumerate() {
        if let Some(asset_investment) = asset_investment_opt { // Proceed only if allocation is defined
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
            for (j, strategy_investment_opt) in asset_investment.strategy_allocations.iter().enumerate() {
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
                        &strategy_investment.amount)?;
                }
            }
        }
    }
    Ok(())
}

/// Generate investment allocations and execute them.
pub fn generate_and_execute_investments(
    e: &Env,
    amounts: &Vec<i128>,
    assets: &Vec<AssetStrategySet>,
) -> Result<(), ContractError> {
    let mut asset_investments = Vec::new(&e);

    for (i, amount) in amounts.iter().enumerate() {
        let asset = assets.get(i as u32).unwrap();
        let (asset_invested_funds, _) = fetch_invested_funds_for_asset(&e, &asset);

        let mut strategy_allocations = Vec::new(&e);
        let mut remaining_amount = amount;

        for (j, strategy) in asset.strategies.iter().enumerate() {
            let strategy_invested_funds = fetch_strategy_invested_funds(&e, &strategy.address);

            let mut invest_amount = if asset_invested_funds > 0 {
                (amount * strategy_invested_funds) / asset_invested_funds
            } else {
                0
            };

            if j == asset.strategies.len() as usize - 1 {
                invest_amount = remaining_amount;
            }

            remaining_amount -= invest_amount;

            strategy_allocations.push_back(Some(StrategyAllocation {
                strategy_address: strategy.address.clone(),
                amount: invest_amount,
            }));
        }

        asset_investments.push_back(Some(AssetInvestmentAllocation {
            asset: asset.address.clone(),
            strategy_allocations,
        }));
    }

    check_and_execute_investments(e.clone(), assets.clone(), asset_investments)?;
    Ok(())
}
