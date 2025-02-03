use soroban_sdk::{Env, Vec};

use crate::{
    models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, StrategyAllocation},
    ContractError,
};

/// Generates investment allocations for a set of assets and their associated strategies.
///
/// This function calculates the distribution of funds across strategies for each asset based
/// on the current state of strategy investments. The allocations are returned as a vector,
/// where each entry corresponds to an asset's investment allocation or `None` if no allocation
/// is required.
///
/// # Arguments
/// - `e` - Reference to the current environment.
/// - `total_managed_funds` - A vector of `CurrentAssetInvestmentAllocation` objects, representing
///   the current allocation of funds for each asset and its associated strategies.
/// - `amounts` - A vector of amounts representing the funds to be allocated for each asset. Each index
///   in this vector corresponds to the same index in the `total_managed_funds`.
///
/// # Returns
/// - `Ok(Vec<Option<AssetInvestmentAllocation>>)` - A vector of investment allocations where each entry
///   represents an asset's strategy allocations. If an asset does not require allocation, its entry is `None`.
/// - `Err(ContractError)` - If any errors occur during the allocation process, such as invalid data or calculations.
///
/// # Function Flow
/// 1. **Iterate Over Assets**:
///    - For each entry in `total_managed_funds`:
///      - Match the corresponding amount from the `amounts` vector.
///      - Skip assets with zero amounts or no prior investments.
///      - Calculate the allocation of funds across strategies proportionally based on the current state.
/// 2. **Proportional Distribution**:
///    - For each strategy within an asset, determine the proportional investment based on its existing allocation.
///    - Ensure that all amounts are correctly calculated without overflows or division errors.
/// 3. **Prepare Allocation**:
///    - Append the calculated strategy allocations to the resulting vector.
///    - Include `None` for assets with no required allocations.
/// 4. **Return Results**:
///    - Return the vector containing the investment allocations.
///
/// # Notes
/// - This function does not execute the investments; it only prepares the allocations.
/// - It assumes that the provided `total_managed_funds` contains valid and complete data.
/// - The function ensures that the last strategy allocation balances any remaining amounts to avoid rounding issues.
///
/// # Example
/// ```rust
/// let total_managed_funds = vec![
///     CurrentAssetInvestmentAllocation {
///         asset: Address::from_str("asset_1"),
///         invested_amount: 100_000,
///         strategy_allocations: vec![
///             StrategyAllocation {
///                 strategy_address: Address::from_str("strategy_1"),
///                 amount: 50_000,
///             },
///             StrategyAllocation {
///                 strategy_address: Address::from_str("strategy_2"),
///                 amount: 50_000,
///             },
///         ],
///     },
///     CurrentAssetInvestmentAllocation {
///         asset: Address::from_str("asset_2"),
///         invested_amount: 200_000,
///         strategy_allocations: vec![
///             StrategyAllocation {
///                 strategy_address: Address::from_str("strategy_3"),
///                 amount: 200_000,
///             },
///         ],
///     },
/// ];
/// let amounts = vec![50_000, 100_000];
///
/// let allocations = generate_investment_allocations(&env, &total_managed_funds, &amounts)?;
/// ```
///
/// This example demonstrates how to generate investment allocations for assets with proportional
/// distribution across their strategies.
pub fn generate_investment_allocations(
    e: &Env,
    total_managed_funds: &Vec<CurrentAssetInvestmentAllocation>,
    amounts: &Vec<i128>,
) -> Result<Vec<Option<AssetInvestmentAllocation>>, ContractError> {
    let mut asset_investments = Vec::new(&e);

    // Iterate through the total managed funds and match it with the corresponding amount
    for (i, current_asset_allocation) in total_managed_funds.iter().enumerate() {
        let amount = amounts.get(i as u32).unwrap_or(0);
        let asset_invested_funds = current_asset_allocation.invested_amount;

        // Skip assets with zero allocation or no previous investments
        if amount > 0 && asset_invested_funds > 0 {
            let mut strategy_allocations = Vec::new(&e);
            let mut remaining_amount = amount;

            for (j, strategy_allocation) in current_asset_allocation.strategy_allocations.iter().enumerate() {
                // Calculate the investment amount for the strategy
                let invest_amount = if j == (current_asset_allocation.strategy_allocations.len() as usize).checked_sub(1).unwrap() {
                    remaining_amount
                } else {
                    let strategy_invested_funds = strategy_allocation.amount;

                    amount
                        .checked_mul(strategy_invested_funds)
                        .and_then(|v| v.checked_div(asset_invested_funds))
                        .unwrap_or(0)
                };

                // Update the remaining amount
                remaining_amount = remaining_amount.checked_sub(invest_amount).ok_or(ContractError::Underflow)?;

                // Add the strategy allocation if it has a non-zero amount
                strategy_allocations.push_back(if invest_amount > 0 && strategy_allocation.paused == false {
                    Some(StrategyAllocation {
                        strategy_address: strategy_allocation.strategy_address.clone(),
                        amount: invest_amount,
                        paused: strategy_allocation.paused
                    })
                } else {
                    None
                });
            }

            // Add the asset investment allocation
            asset_investments.push_back(Some(AssetInvestmentAllocation {
                asset: current_asset_allocation.asset.clone(),
                strategy_allocations,
            }));
        } else {
            asset_investments.push_back(None); // No investments to be executed for this asset
        }
    }

    Ok(asset_investments)
}
