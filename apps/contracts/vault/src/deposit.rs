use common::models::AssetStrategySet;
use soroban_sdk::{panic_with_error, token::TokenClient, Address, Env, Vec, Map};

use crate::{
    storage::get_assets,
    token::{internal_mint, VaultToken},
    utils::{calculate_deposit_amounts_and_shares_to_mint, check_nonnegative_amount},
    ContractError, MINIMUM_LIQUIDITY,
    models::{CurrentAssetInvestmentAllocation},
};

/// Common logic for processing deposits.
pub fn process_deposit(
    e: &Env,
    assets: &Vec<AssetStrategySet>,
    total_managed_funds: &Map<Address, CurrentAssetInvestmentAllocation>,
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
        calculate_single_asset_shares(
            e, 
            amounts_desired, 
            &total_managed_funds,
            total_supply)?
    } else {
        if total_supply == 0 {
            (amounts_desired.clone(), amounts_desired.iter().sum())
        } else {
            calculate_deposit_amounts_and_shares_to_mint(
                &e, 
                &assets, 
                &total_managed_funds,
                amounts_desired, 
                amounts_min)?
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
    total_managed_funds: &Map<Address, CurrentAssetInvestmentAllocation>,
    total_supply: i128,
) -> Result<(Vec<i128>, i128), ContractError> {
    let shares = if total_supply == 0 {
        amounts_desired.get(0).unwrap()
    } else {
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