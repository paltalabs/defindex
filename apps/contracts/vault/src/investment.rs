use soroban_sdk::{Address, Env, Map, Vec, panic_with_error};

use crate::{
    models::{AssetStrategySet, AssetInvestmentAllocation},
    strategies::invest_in_strategy,
    utils::{check_nonnegative_amount},
    ContractError,
};

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
            if asset.strategies.len() != asset_investment.strategy_investments.len() {
                panic_with_error!(&e, ContractError::WrongStrategiesLength);
            }

            // NOTE: We can avoid this check as it if total idle funds exceed idle funds, this will fail
            // when trying to transfer

            // // Calculate total intended investment for this asset
            // let total_asset_investment: i128 = asset_investment.investments.iter()
            //     .filter_map(|strategy| strategy.as_ref().map(|s| s.amount.unwrap_or(0)))
            //     .sum();

            // // Verify total intended investment does not exceed idle funds for this asset
            // if total_asset_investment > fetch_idle_funds_for_asset(&e, &asset_investment.asset) {
            //     panic_with_error!(&e, ContractError::InsufficientIdleFunds);
            // }

            // Process each defined strategy investment for the current asset
            for (j, strategy_investment_opt) in asset_investment.strategy_investments.iter().enumerate() {
                if let Some(strategy_investment) = strategy_investment_opt {
                    // Validate amount is non-negative
                    check_nonnegative_amount(strategy_investment.amount)?;

                    // Ensure the strategy is active before proceeding
                    let strategy = asset.strategies.get(j as u32).unwrap();
                    if strategy_investment.amount > 0 && strategy.paused {
                        panic_with_error!(&e, ContractError::StrategyPaused);
                    }

                    //Reduce idle funds for this asset


                    // Execute the investment if checks pass
                    invest_in_strategy(&e, &strategy.address, &strategy_investment.amount)?;
                }
            }
        }
    }
    Ok(())
}