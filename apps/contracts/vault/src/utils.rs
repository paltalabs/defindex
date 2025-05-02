use common::models::{AssetStrategySet, Strategy};
use soroban_sdk::{panic_with_error, Env, Map, Vec};

use crate::{
    //access::{AccessControl, AccessControlTrait, RolesDataKey},
    models::CurrentAssetInvestmentAllocation,
    token::VaultToken,
    ContractError,
};


pub fn validate_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::AmountNotAllowed)
    } else {
        Ok(())
    }
}

fn divide_rounding_up(a: i128, b: i128) -> Result<i128, ContractError> {
    let result = a.checked_div(b).ok_or(ContractError::ArithmeticError)?;
    if a % b != 0 {
        Ok(result.checked_add(1).ok_or(ContractError::ArithmeticError)?)
    } else {
        Ok(result)
    }
}

pub fn validate_assets( e: &Env, assets: &Vec<AssetStrategySet>){
    if assets.len() == 0 || assets.is_empty(){
        panic_with_error!(&e, ContractError::NoAssetAllocation);
    }
    let mut asset_addresses = Map::new(&e);

    for (_, asset) in assets.iter().enumerate(){
        if asset_addresses.contains_key(asset.address.clone()){
            panic_with_error!(&e, ContractError::DuplicatedAsset);
        }
        asset_addresses.set(asset.address.clone(), true);
        validate_strategies(e, &asset.strategies);
    }
}

pub fn validate_strategies(e: &Env, strategies: &Vec<Strategy>){
    let mut strategy_addresses = Map::new(&e);
    for strategy in strategies.iter() {
        if strategy_addresses.contains_key(strategy.address.clone()){
            panic_with_error!(&e, ContractError::DuplicatedStrategy);
        }
        strategy_addresses.set(strategy.address.clone(), true);
    }
}

// /// From an amount, calculates how much to withdraw from each strategy;
// /// returns a map of strategy address to token amount
// pub fn calculate_withdrawal_amounts(
//     e: &Env,
//     amount: i128,
//     asset: AssetStrategySet,
// ) -> Map<Address, i128> {
//     let mut withdrawal_amounts = Map::<Address, i128>::new(e);

//     let (total_invested_in_strategies, _) = fetch_invested_funds_for_asset(&e, &asset);

//     for strategy in asset.strategies.iter() {
//         // TODO: if strategy is paused but still holds assets on it shouldnt we withdraw them?
//         if strategy.paused {
//             continue;
//         }

//         let strategy_invested_funds = fetch_strategy_invested_funds(e, &strategy.address);

//         let strategy_share_of_withdrawal =
//             (amount * strategy_invested_funds) / total_invested_in_strategies;

//         withdrawal_amounts.set(strategy.address.clone(), strategy_share_of_withdrawal);
//     }

//     withdrawal_amounts
// }

/// Calculates the corresponding amounts of each asset per given number of vault shares.
/// This function takes the number of vault shares (`shares_amount`) and computes how much of each asset in the vault
/// corresponds to those shares, based on the total managed funds and the total supply of vault shares.
///
/// # Arguments
/// * `env` - Reference to the current environment.
/// * `shares_amount` - The number of vault shares for which the equivalent asset amounts are being calculated.
/// * `total_managed_funds` - A vector of `CurrentAssetInvestmentAllocation` representing the current managed funds for each asset.
///
/// # Returns
/// * `Vec<i128>` - A vector of amounts corresponding to each asset, proportional to the `shares_amount`.
///
/// # Errors
/// * Returns `ContractError::ArithmeticError` if there are any issues with multiplication or division,
///   such as overflow or division by zero.
/// * Returns `ContractError::AmountOverTotalSupply` if the `shares_amount` exceeds the total supply of vault shares.
pub fn calculate_asset_amounts_per_vault_shares( 
    env: &Env,
    shares_amount: i128,
    total_managed_funds: &Vec<CurrentAssetInvestmentAllocation>,
) -> Result<Vec<i128>, ContractError> {
    let mut asset_amounts = Vec::new(env);

    // Fetch the total supply of vault shares
    let total_shares_supply = VaultToken::total_supply(env.clone());

    // Check if the requested shares amount exceeds the total supply
    if shares_amount > total_shares_supply {
        return Err(ContractError::AmountOverTotalSupply);
    }

    // Iterate over each asset in the total managed funds and calculate the proportional amount
    for asset_allocation in total_managed_funds.iter() {
        // Calculate the proportional asset amount for the given number of shares
        let asset_amount = if total_shares_supply != 0 {
            asset_allocation
                .total_amount
                .checked_mul(shares_amount)
                .ok_or(ContractError::ArithmeticError)?
                .checked_div(total_shares_supply)
                .ok_or(ContractError::ArithmeticError)?
        } else {
            return Err(ContractError::AmountOverTotalSupply);
        };

        // Append the calculated asset amount to the vector
        asset_amounts.push_back(asset_amount);
    }

    Ok(asset_amounts)
}

pub fn calculate_optimal_amounts_and_shares_with_enforced_asset(
    e: &Env,
    total_managed_funds: &Vec<CurrentAssetInvestmentAllocation>,
    amounts_desired: &Vec<i128>,
    enforced_asset_index: u32,
) -> (Vec<i128>, i128) {
    // Reserve (total managed funds) of the enforced asset
    let reserve_target = total_managed_funds
        .get(enforced_asset_index)
        .unwrap_or_else(|| panic_with_error!(e, ContractError::WrongAmountsLength))
        .total_amount;

    let amount_desired_target = amounts_desired
        .get(enforced_asset_index)
        .unwrap_or_else(|| panic_with_error!(e, ContractError::WrongAmountsLength));

    if amount_desired_target == 0 {
        panic_with_error!(e, ContractError::InsufficientAmount);
    }

    let mut optimal_amounts = Vec::new(e);

    for (i, asset_allocation) in total_managed_funds.iter().enumerate() {
        if i == enforced_asset_index as usize {
            // Use the full desired amount for the enforced asset
            optimal_amounts.push_back(amount_desired_target);
        } else {
            // Calculate the proportional allocation for non-enforced assets
            let reserve = asset_allocation.total_amount;
            let amount = match divide_rounding_up(
                reserve
                    .checked_mul(amount_desired_target)
                    .unwrap_or_else(|| panic_with_error!(e, ContractError::ArithmeticError)),
                reserve_target
            ){
                Ok(amount) => amount,
                Err(_) => panic_with_error!(e, ContractError::ArithmeticError)
            };
            optimal_amounts.push_back(amount);
        }
    }

    // Calculate shares to mint = (total_supply * amount_desired_target) / reserve_target
    let shares_to_mint = VaultToken::total_supply(e.clone())
        .checked_mul(amount_desired_target)
        .unwrap_or_else(|| panic_with_error!(e, ContractError::ArithmeticError))
        .checked_div(reserve_target)
        .unwrap_or_else(|| panic_with_error!(e, ContractError::ArithmeticError));

    (optimal_amounts, shares_to_mint)
}

pub fn calculate_deposit_amounts_and_shares_to_mint(
    e: &Env,
    total_managed_funds: &Vec<CurrentAssetInvestmentAllocation>,
    amounts_desired: &Vec<i128>,
    amounts_min: &Vec<i128>,
) -> Result<(Vec<i128>, i128), ContractError> {
    for i in 0..total_managed_funds.len() {
        // Skip zero balance assets
        if total_managed_funds.get(i).unwrap().total_amount == 0 {
            continue;
        }

        let (optimal_amounts, shares_to_mint) =
            calculate_optimal_amounts_and_shares_with_enforced_asset(
                e,
                total_managed_funds,
                amounts_desired,
                i,
            );

        let mut should_skip = false;

        // Check ALL other assets
        for j in 0..total_managed_funds.len() {
            // Skip the reference asset and zero balance assets
            if j == i || total_managed_funds.get(j).unwrap().total_amount == 0 {
                continue;
            }

            let desired_amount = amounts_desired
                .get(j)
                .ok_or(ContractError::WrongAmountsLength)?;
            let min_amount = amounts_min
                .get(j)
                .ok_or(ContractError::WrongAmountsLength)?;
            let optimal_amount = optimal_amounts
                .get(j)
                .ok_or(ContractError::WrongAmountsLength)?;

            if optimal_amount <= desired_amount {
                if optimal_amount < min_amount {
                    should_skip = true;
                    break;  // Try next enforced asset instead of returning error
                }
            } else {
                should_skip = true;
                break;
            }
        }

        if !should_skip {
            return Ok((optimal_amounts, shares_to_mint));
        }
    }

    Err(ContractError::NoOptimalAmounts)
}