use soroban_sdk::{Address, Env, Map, Vec};

use crate::{
    models::Investment,
    strategies::{get_strategy_asset, invest_in_strategy},
    utils::check_nonnegative_amount,
    ContractError,
};

pub fn prepare_investment(
    e: &Env,
    investments: Vec<Investment>,
    idle_funds: Map<Address, i128>,
) -> Result<Map<Address, i128>, ContractError> {
    let mut total_investment_per_asset: Map<Address, i128> = Map::new(e);

    for investment in investments.iter() {
        let strategy_address = &investment.strategy;
        let amount_to_invest = investment.amount;
        check_nonnegative_amount(amount_to_invest.clone())?;

        // Find the corresponding asset for the strategy
        let asset = get_strategy_asset(&e, strategy_address)?;

        // Track investment per asset
        let current_investment = total_investment_per_asset
            .get(asset.address.clone())
            .unwrap_or(0);
        let updated_investment = current_investment
            .checked_add(amount_to_invest)
            .ok_or(ContractError::Overflow)?;

        total_investment_per_asset.set(asset.address.clone(), updated_investment);

        // Check if total investment exceeds idle funds
        let idle_balance = idle_funds.get(asset.address.clone()).unwrap_or(0);
        if updated_investment > idle_balance {
            return Err(ContractError::NotEnoughIdleFunds);
        }
    }

    Ok(total_investment_per_asset)
}

pub fn execute_investment(e: &Env, investments: Vec<Investment>) -> Result<(), ContractError> {
    for investment in investments.iter() {
        let strategy_address = &investment.strategy;
        let amount_to_invest = &investment.amount;

        invest_in_strategy(e, strategy_address, amount_to_invest)?
    }

    Ok(())
}
