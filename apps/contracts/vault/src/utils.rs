use soroban_sdk::{panic_with_error, Address, Env, Map, Vec};

use crate::{
    models::{CurrentAssetInvestmentAllocation},
    access::{AccessControl, AccessControlTrait, RolesDataKey},
    token::VaultToken,
    ContractError,
};
use common::models::AssetStrategySet;

pub const DAY_IN_LEDGERS: u32 = 17280;

pub fn bump_instance(e: &Env) {
    let max_ttl = e.storage().max_ttl();
    e.storage()
        .instance()
        .extend_ttl(max_ttl - DAY_IN_LEDGERS, max_ttl);
}

pub fn check_initialized(e: &Env) -> Result<(), ContractError> {
    //TODO: Should also check if adapters/strategies have been set
    let access_control = AccessControl::new(&e);
    if access_control.has_role(&RolesDataKey::Manager) {
        Ok(())
    } else {
        panic_with_error!(&e, ContractError::NotInitialized);
    }
}

pub fn check_nonnegative_amount(amount: i128) -> Result<(), ContractError> {
    if amount < 0 {
        Err(ContractError::NegativeNotAllowed)
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
///
/// # Returns
/// * `Map<Address, i128>` - A map of asset addresses to their respective amounts, proportional to the `shares_amount`.
///
/// # Errors
/// * Panics with `ContractError::ArithmeticError` if there are any issues with multiplication or division,
///   such as overflow or division by zero.
pub fn calculate_asset_amounts_per_vault_shares(
    env: &Env,
    shares_amount: i128,
    total_managed_funds: &Map<Address, CurrentAssetInvestmentAllocation>,
) -> Result<Map<Address, i128>, ContractError> {
    let mut asset_amounts: Map<Address, i128> = Map::new(env);

    // Fetch the total supply of vault shares and the total managed funds for each asset
    let total_shares_supply = VaultToken::total_supply(env.clone());

    // if shares amount over total supply, return error AmountOverTotalSupply
    if shares_amount > total_shares_supply {
        return Err(ContractError::AmountOverTotalSupply);
    }

    // Iterate over each asset and calculate the corresponding amount based on shares_amount
    for (asset_address, current_asset_allocation) in total_managed_funds.iter() {
        // Calculate the proportional asset amount per the given number of shares
        let asset_amount = if total_shares_supply != 0 {
            current_asset_allocation.total_amount
                .checked_mul(shares_amount)
                .ok_or(ContractError::ArithmeticError)?
                .checked_div(total_shares_supply)
                .ok_or(ContractError::ArithmeticError)?
        } else {
            return Err(ContractError::AmountOverTotalSupply);
        };

        // Set the calculated asset amount for the given asset address
        asset_amounts.set(asset_address.clone(), asset_amount);
    }

    return Ok(asset_amounts);
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
    total_managed_funds: &Map<Address, CurrentAssetInvestmentAllocation>,
    assets: &Vec<AssetStrategySet>,
    amounts_desired: &Vec<i128>,
    i: &u32,
) -> (Vec<i128>, i128) {
    // we have to calculate the optimal amount to deposit for the rest of the assets
    // we need the total amount managed by this vault in order for the deposit to be proportional
    // reserve (total manage funds) of the asset we are enforcing
    let reserve_target = total_managed_funds
        .get(assets.get(*i).unwrap_or_else(|| panic_with_error!(&e, ContractError::WrongAmountsLength)).address)
        .unwrap_or_else(|| panic_with_error!(&e, ContractError::WrongAmountsLength))
        .total_amount;

    // If reserve target is zero, we cannot calculate the optimal amounts
    if reserve_target == 0 {
        panic_with_error!(&e, ContractError::InsufficientManagedFunds);
    }

    let amount_desired_target = amounts_desired.get(*i).unwrap_or_else(|| panic_with_error!(&e, ContractError::WrongAmountsLength));

    let mut optimal_amounts = Vec::new(e);
    
    for (j, asset) in assets.iter().enumerate() {
        if j == (*i as usize) {
            optimal_amounts.push_back(amount_desired_target);
        } else {
            let reserve = total_managed_funds
                            .get(asset.address).unwrap_or_else(|| panic_with_error!(&e, ContractError::WrongAmountsLength))
                            .total_amount;
            let amount = reserve.checked_mul(amount_desired_target)
                .unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError))
                .checked_div(reserve_target)
                .unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError));
            optimal_amounts.push_back(amount);  
        }
    }

    //TODO: calculate the shares to mint = total_supply * amount_desired_target  / reserve_target
    let shares_to_mint =
        VaultToken::total_supply(e.clone())
        .checked_mul(amount_desired_target)
        .unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError))
        .checked_div(reserve_target)
        .unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError));
        
    (optimal_amounts, shares_to_mint)
}
/// Calculates the optimal amounts to deposit for a set of assets, along with the shares to mint.
/// This function iterates over a list of assets and checks if the desired deposit amounts
/// match the optimal deposit strategy, based on current managed funds and asset ratios.
///
/// If the desired amount for a given asset cannot be achieved due to constraints (e.g., it's below the minimum amount),
/// the function attempts to find an optimal solution by adjusting the amounts of subsequent assets.
///
/// # Arguments
/// * `e` - The current environment.
/// * `assets` - A vector of assets for which deposits are being calculated.
/// * `amounts_desired` - A vector of desired amounts for each asset.
/// * `amounts_min` - A vector of minimum amounts for each asset, below which deposits are not allowed.
///
/// # Returns
/// A tuple containing:
/// * A vector of optimal amounts to deposit for each asset.
/// * The number of shares to mint based on the optimal deposits.
///
/// # Errors
/// If no valid deposit configuration can be found that satisfies the minimum amounts for all assets, the function
/// will return an error.
///
/// # Panics
/// The function may panic if it encounters invalid states (e.g., insufficient amounts) but TODO: these should
/// be replaced with proper error handling.
pub fn calculate_deposit_amounts_and_shares_to_mint(
    e: &Env,
    assets: &Vec<AssetStrategySet>,
    total_managed_funds: &Map<Address, CurrentAssetInvestmentAllocation>,
    amounts_desired: &Vec<i128>,
    amounts_min: &Vec<i128>,
) -> Result<(Vec<i128>, i128), ContractError> {

    for i in 0..assets.len() {
        // Calculate the optimal amounts and shares to mint for asset `i`.
        let (optimal_amounts, shares_to_mint) = calculate_optimal_amounts_and_shares_with_enforced_asset(
            &e,
            &total_managed_funds,
            &assets,
            &amounts_desired,
            &i,
        );

        let mut should_skip = false;

        for j in i + 1..assets.len() {
            // Retrieve the desired and minimum amounts, returning an error if unavailable.
            let desired_amount = amounts_desired.get(j).ok_or(ContractError::WrongAmountsLength)?;
            let min_amount = amounts_min.get(j).ok_or(ContractError::WrongAmountsLength)?;
            let optimal_amount = optimal_amounts.get(j).ok_or(ContractError::WrongAmountsLength)?;

            // Check if optimal amount meets the desired or minimum requirements.
            if optimal_amount <= desired_amount {
                if optimal_amount < min_amount {
                    return Err(ContractError::InsufficientAmount); // Insufficient amount error.
                }
            } else { // if not, we should try the next asset as enforced asset
                should_skip = true;
                // If we have already analized all assets as enforced (i), return an error.
                if i == assets.len() - 1 {
                    return Err(ContractError::NoOptimalAmounts); // probably enforcing 0? We should never reach this point
                }
                break;
            }
        }

        // If valid amounts found, return the results; otherwise, skip to the next asset.
        if !should_skip {
            return Ok((optimal_amounts, shares_to_mint));
        }
    }

    // Return an error if no valid deposit configuration is found.
    Err(ContractError::NoOptimalAmounts)
}
