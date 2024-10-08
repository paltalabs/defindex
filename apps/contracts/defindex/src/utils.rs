use soroban_sdk::{panic_with_error, Address, Env, Map, Vec};

use crate::{
    access::{AccessControl, AccessControlTrait, RolesDataKey}, funds::{fetch_invested_funds_for_asset, fetch_invested_funds_for_strategy, fetch_total_managed_funds}, models::{AssetAllocation, Investment}, storage::get_assets, token::VaultToken, ContractError

};

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

/// Converts dfToken amount into corresponding token amounts for each strategy based on their ratio.
/// returns a map of strategy address to token amount
pub fn calculate_withdrawal_amounts(
    e: &Env,
    df_token_amount: i128, // The amount of dfTokens to withdraw
) -> Map<Address, i128> {
    let mut withdrawal_amounts = Map::<Address, i128>::new(e);
    let assets = get_assets(e);

    // Step 1: Calculate asset amounts for the given df_token_amount
    let asset_amounts = calculate_asset_amounts_for_dftokens(e, df_token_amount);

    // Step 2: Iterate through each asset and calculate the amounts for each strategy
    for asset in assets.iter() {
        // Get the total invested funds for the current asset
        let total_invested_for_asset = asset_amounts.get(asset.address.clone()).unwrap_or(0);

        // Get total funds in each strategy for this asset
        let total_invested_in_strategies: i128 = fetch_invested_funds_for_asset(&e, &asset);

        // Step 3: Calculate the withdrawal amount for each strategy within this asset
        for strategy in asset.strategies.iter() {
            if strategy.paused {
                continue; // Skip paused strategies
            }
            // Fetch invested funds for the strategy
            let strategy_invested_funds = fetch_invested_funds_for_strategy(e, &strategy.address);

            // Calculate the strategy's share of the withdrawal amount based on its proportion
            let strategy_share_of_withdrawal = (total_invested_for_asset * strategy_invested_funds) / total_invested_in_strategies;

            // Set the calculated amount to be withdrawn from this strategy
            withdrawal_amounts.set(strategy.address.clone(), strategy_share_of_withdrawal);
        }
    }

    withdrawal_amounts
}

pub fn calculate_asset_amounts_for_dftokens(
    env: &Env,
    df_token_amount: i128,
) -> Map<Address, i128> {
    let mut asset_amounts: Map<Address, i128> = Map::new(&env);
    let total_supply = VaultToken::total_supply(env.clone());
    let total_managed_funds = fetch_total_managed_funds(&env);

    // Iterate over each asset and calculate the corresponding amount based on df_token_amount
    for (asset_address, amount) in total_managed_funds.iter() {
        let asset_amount = (amount * df_token_amount) / total_supply;
        asset_amounts.set(asset_address.clone(), asset_amount);
    }

    asset_amounts
}

pub fn calculate_dftokens_from_asset_amounts(
    env: &Env,
    asset_amounts: Map<Address, i128>, // The input asset amounts
) -> Result<i128, ContractError> {
    let total_supply = VaultToken::total_supply(env.clone()); // Total dfToken supply
    let total_managed_funds = fetch_total_managed_funds(&env); // Fetch all managed assets

    // Initialize the minimum dfTokens corresponding to each asset
    let mut min_df_tokens: Option<i128> = None;

    // Iterate over each asset in the input map
    for (asset_address, input_amount) in asset_amounts.iter() {
        // Get the total managed amount for this asset
        let managed_amount = total_managed_funds.get(asset_address.clone()).unwrap_or(0);

        // Ensure the managed amount is not zero to prevent division by zero
        if managed_amount == 0 {
            return Err(ContractError::InsufficientManagedFunds);
        }

        // Calculate the dfTokens corresponding to this asset's amount
        let df_tokens_for_asset = (input_amount * total_supply) / managed_amount;

        // If this is the first asset or if the calculated df_tokens_for_asset is smaller, update the minimum df_tokens
        if let Some(current_min_df_tokens) = min_df_tokens {
            min_df_tokens = Some(current_min_df_tokens.min(df_tokens_for_asset));
        } else {
            min_df_tokens = Some(df_tokens_for_asset);
        }
    }

    // Return the minimum dfTokens across all assets
    min_df_tokens.ok_or(ContractError::NoAssetsProvided)
}

pub fn calculate_optimal_amounts_and_shares_with_enforced_asset(
    e: &Env,
    total_managed_funds: &Map<Address, i128>,
    assets: &Vec<AssetAllocation>,
    amounts_desired: &Vec<i128>,
    i: &u32,
) -> (Vec<i128>, i128) {
    // we have to calculate the optimal amount to deposit for the rest of the assets
    // we need the total amount managed by this vault in order for the deposit to be proportional
    // reserve (total manage funds) of the asset we are enforcing
    let reserve_target = total_managed_funds
        .get(assets.get(*i).unwrap().address)
        .unwrap(); // i128
    if reserve_target == 0 {
        // return sum of amounts desired as shares
        return (amounts_desired.clone(), amounts_desired.iter().sum()); // first shares will be equal to the first amounts_desired
                                                                        // TODO, this amounts desired might be too little?
                                                                        // this might be the first deposit... in this case, the ratio will be enforced by the first depositor
                                                                        // TODO: might happen that the reserve_target is zero because everything is in one asset!?
                                                                        // in this case we ned to check the ratio
        // TODO VERY DANGEROUS.
    }
    let amount_desired_target = amounts_desired.get(*i).unwrap(); // i128

    let mut optimal_amounts = Vec::new(e);

    for (j, (_asset_address, reserve)) in total_managed_funds.iter().enumerate() {
        if j == (*i as usize) {
            optimal_amounts.push_back(amount_desired_target);
        } else {
            // amount = amount_desired_target * reserve[j] / reserve_target
            // factor is (amount_desired_target  / reserve_target;)
            let amount = reserve * amount_desired_target / reserve_target;
            optimal_amounts.push_back(amount);
        }
    }
    //TODO: calculate the shares to mint = total_supply * amount_desired_target  / reserve_target
    let shares_to_mint = VaultToken::total_supply(e.clone()) * amount_desired_target / reserve_target;
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
    assets: &Vec<AssetAllocation>,
    amounts_desired: &Vec<i128>,
    amounts_min: &Vec<i128>,
) -> (Vec<i128>, i128) {
    // Retrieve the total managed funds for each asset as a Map<Address, i128>.
    let total_managed_funds = fetch_total_managed_funds(e); 

    // Iterate over each asset in the assets vector.
    for i in 0..assets.len() {
        // Calculate the optimal amounts and the number of shares to mint for the given asset at index `i`.
        let (optimal_amounts, shares_to_mint) =
            calculate_optimal_amounts_and_shares_with_enforced_asset(
                &e,
                &total_managed_funds,
                &assets,
                &amounts_desired,
                &i,
            );

        // Flag to skip the current asset if necessary.
        let mut should_skip = false;

        // Check if the calculated optimal amounts meet the desired or minimum requirements.
        for j in i + 1..assets.len() {
            // If the optimal amount for asset[j] is less than the desired amount,
            // but at least greater than the minimum amount, it is acceptable.
            if optimal_amounts.get(j).unwrap() <= amounts_desired.get(j).unwrap() {
                // If the optimal amount is below the minimum, we cannot proceed with this asset.
                if optimal_amounts.get(j).unwrap() < amounts_min.get(j).unwrap() {
                    panic!("insufficient amount"); // TODO: Replace panic with an error return.
                }
            } else {
                // If the optimal amount exceeds the desired amount, we skip the current asset {i}.
                should_skip = true;

                // If we've reached the last asset and still don't find a solution, throw an error.
                if j == assets.len() - 1 {
                    panic!("didn't find optimal amounts"); // TODO: Replace panic with an error return.
                }
                break;
            }
        }

        // If we should skip this asset, continue to the next one.
        if should_skip {
            continue;
        } else {
            // Return the calculated optimal amounts and shares to mint.
            return (optimal_amounts, shares_to_mint);
        }
    }

    // If no solution was found after iterating through all assets, throw an error.
    panic!("didn't find optimal amounts");
}

// Calculates the amounts to invest in each strategy based on the current ratio of invested funds.
// The function returns a Vec<Investment> where each element contains a strategy address and the amount to be invested.
//
// # Parameters
// - `e`: The environment object, containing all relevant contract data.
//
// # Returns
// A Vec of `Investment` structs, each containing a strategy address and the calculated amount to invest.
// pub fn calculate_investments_based_on_ratios(
//     e: &Env,
// ) -> Result<Vec<Investment>, ContractError> {
//     // i should get all managed funds
//     let total_managed_funds = get_total_managed_funds(e);

    
//     // Create a vector to store the investment amounts for each strategy
//     let mut investments: Vec<Investment> = Vec::new(e);

//     Ok(investments)
// }