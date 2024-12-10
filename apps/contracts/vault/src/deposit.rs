use common::models::AssetStrategySet;
use soroban_sdk::{panic_with_error, token::TokenClient, Address, Env, Vec};

use crate::{
    funds::{
        fetch_invested_funds_for_asset, fetch_invested_funds_for_strategy,
        fetch_total_managed_funds,
    },
    investment::check_and_execute_investments,
    models::{AssetInvestmentAllocation, StrategyAllocation},
    storage::get_assets,
    token::{internal_mint, VaultToken},
    utils::{calculate_deposit_amounts_and_shares_to_mint, check_nonnegative_amount},
    ContractError, MINIMUM_LIQUIDITY,
};

/// Common logic for processing deposits.
pub fn process_deposit(
    e: &Env,
    assets: &Vec<AssetStrategySet>,
    amounts_desired: &Vec<i128>,
    amounts_min: &Vec<i128>,
    from: &Address,
) -> Result<(Vec<i128>, i128), ContractError> {
    let assets_length = assets.len();

    // Validate inputs
    if assets_length != amounts_desired.len() || assets_length != amounts_min.len() {
        panic_with_error!(&e, ContractError::WrongAmountsLength);
    }

    for amount in amounts_desired.iter() {
        check_nonnegative_amount(amount)?;
    }

    let total_supply = VaultToken::total_supply(e.clone());
    let (amounts, shares_to_mint) = if assets_length == 1 {
        calculate_single_asset_shares(e, amounts_desired, total_supply)?
    } else {
        if total_supply == 0 {
            (amounts_desired.clone(), amounts_desired.iter().sum())
        } else {
            calculate_deposit_amounts_and_shares_to_mint(&e, &assets, amounts_desired, amounts_min)?
        }
    };

    // Transfer assets
    for (i, amount) in amounts.iter().enumerate() {
        if amount < amounts_min.get(i as u32).unwrap() {
            panic_with_error!(&e, ContractError::InsufficientAmount);
        }
        if amount > 0 {
            let asset = assets.get(i as u32).unwrap();
            let asset_client = TokenClient::new(&e, &asset.address);
            asset_client.transfer(&from, &e.current_contract_address(), &amount);
        }
    }

    // Mint shares
    mint_shares(e, &total_supply, shares_to_mint, from.clone())?;

    Ok((amounts, shares_to_mint))
}

/// Calculate shares for single-asset deposits.
fn calculate_single_asset_shares(
    e: &Env,
    amounts_desired: &Vec<i128>,
    total_supply: i128,
) -> Result<(Vec<i128>, i128), ContractError> {
    let shares = if total_supply == 0 {
        amounts_desired.get(0).unwrap()
    } else {
        let total_managed_funds = fetch_total_managed_funds(&e);
        VaultToken::total_supply(e.clone())
            .checked_mul(amounts_desired.get(0).unwrap())
            .unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError))
            .checked_div(
                total_managed_funds
                    .get(get_assets(&e).get(0).unwrap().address.clone())
                    .unwrap().total_amount,
            )
            .unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError))
    };
    Ok((amounts_desired.clone(), shares))
}

/// Mint vault shares.
fn mint_shares(
    e: &Env,
    total_supply: &i128,
    shares_to_mint: i128,
    from: Address,
) -> Result<(), ContractError> {
    if *total_supply == 0 {
        if shares_to_mint < MINIMUM_LIQUIDITY {
            panic_with_error!(&e, ContractError::InsufficientAmount);
        }
        internal_mint(e.clone(), e.current_contract_address(), MINIMUM_LIQUIDITY);
        internal_mint(
            e.clone(),
            from.clone(),
            shares_to_mint.checked_sub(MINIMUM_LIQUIDITY).unwrap(),
        );
    } else {
        internal_mint(e.clone(), from, shares_to_mint);
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
            let strategy_invested_funds = fetch_invested_funds_for_strategy(&e, &strategy.address);

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
