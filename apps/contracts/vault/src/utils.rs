use soroban_sdk::{panic_with_error, Env, Vec};

use crate::{
    //access::{AccessControl, AccessControlTrait, RolesDataKey},
    models::CurrentAssetInvestmentAllocation,
    token::VaultToken,
    ContractError,
};

pub const DAY_IN_LEDGERS: u32 = 17280;

pub fn bump_instance(e: &Env) {
    let max_ttl = e.storage().max_ttl();
    let new_ttl = max_ttl.checked_sub(DAY_IN_LEDGERS).unwrap_or_else(|| panic_with_error!(e, ContractError::Underflow));
    e.storage().instance().extend_ttl(new_ttl, max_ttl);
}

pub fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

pub fn check_min_amount(amount: i128, min_amount: i128) -> Result<(), ContractError> {
    if amount < min_amount {
        Err(ContractError::InsufficientAmount)
    } else {
        Ok(())
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

// pub fn calculate_dftokens_from_asset_amounts(
//     env: &Env,
//     asset_amounts: Map<Address, i128>, // The input asset amounts
//     total_managed_funds: Map<Address, i128>, // The total managed funds for each asset
// ) -> Result<i128, ContractError> {
//     let total_supply = VaultToken::total_supply(env.clone()); // Total dfToken supply

//     // Initialize the minimum dfTokens corresponding to each asset
//     let mut min_df_tokens: Option<i128> = None;

//     // Iterate over each asset in the input map
//     for (asset_address, input_amount) in asset_amounts.iter() {
//         // Get the total managed amount for this asset
//         let managed_amount = total_managed_funds.get(asset_address.clone()).unwrap_or(0);

//         // Ensure the managed amount is not zero to prevent division by zero
//         if managed_amount == 0 {
//             return Err(ContractError::InsufficientManagedFunds);
//         }

//         // Calculate the dfTokens corresponding to this asset's amount
//         let df_tokens_for_asset = (input_amount * total_supply) / managed_amount;

//         // If this is the first asset or if the calculated df_tokens_for_asset is smaller, update the minimum df_tokens
//         if let Some(current_min_df_tokens) = min_df_tokens {
//             min_df_tokens = Some(current_min_df_tokens.min(df_tokens_for_asset));
//         } else {
//             min_df_tokens = Some(df_tokens_for_asset);
//         }
//     }

//     // Return the minimum dfTokens across all assets
//     min_df_tokens.ok_or(ContractError::NoAssetsProvided)
// }

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

    // If reserve target is zero, we cannot calculate the optimal amounts
    if reserve_target == 0 {
        panic_with_error!(e, ContractError::InsufficientManagedFunds);
    }

    let amount_desired_target = amounts_desired
        .get(enforced_asset_index)
        .unwrap_or_else(|| panic_with_error!(e, ContractError::WrongAmountsLength));

    let mut optimal_amounts = Vec::new(e);

    for (i, asset_allocation) in total_managed_funds.iter().enumerate() {
        if i == enforced_asset_index as usize {
            // Use the full desired amount for the enforced asset
            optimal_amounts.push_back(amount_desired_target);
        } else {
            // Calculate the proportional allocation for non-enforced assets
            let reserve = asset_allocation.total_amount;
            let amount = reserve
                .checked_mul(amount_desired_target)
                .unwrap_or_else(|| panic_with_error!(e, ContractError::ArithmeticError))
                .checked_div(reserve_target)
                .unwrap_or_else(|| panic_with_error!(e, ContractError::ArithmeticError));
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
        // Calculate the optimal amounts and shares to mint for the enforced asset
        let (optimal_amounts, shares_to_mint) =
            calculate_optimal_amounts_and_shares_with_enforced_asset(
                e,
                total_managed_funds,
                amounts_desired,
                i,
            );

        let mut should_skip = false;

        for j in i + 1..total_managed_funds.len() {
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
                    return Err(ContractError::InsufficientAmount);
                }
            } else {
                should_skip = true;

                // If all assets have been analyzed and no valid solution is found, return an error
                if i == total_managed_funds.len().checked_sub(1).ok_or(ContractError::Underflow)? {
                    return Err(ContractError::NoOptimalAmounts);
                }
                break;
            }
        }

        if !should_skip {
            return Ok((optimal_amounts, shares_to_mint));
        }
    }

    Err(ContractError::NoOptimalAmounts)
}