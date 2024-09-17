use soroban_sdk::{panic_with_error, Address, Env, Map, Vec};

use crate::{
    access::{AccessControl, AccessControlTrait, RolesDataKey},
    funds::get_total_managed_funds,
    models::Asset,
    storage::get_assets,
    ContractError,
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

/// Converts dfToken amount into corresponding token amounts based on their ratio.
pub fn calculate_withdrawal_amounts(
    e: &Env,
    df_token_amount: i128, // The amount of dfTokens to withdraw
) -> Result<Map<Asset, i128>, ContractError> {
    let mut withdrawal_amounts = Map::<Asset, i128>::new(e);
    let assets = get_assets(e);

    let total_ratio = assets.iter().fold(0, |acc, asset| acc + asset.ratio);

    // Iterate through all assets and calculate how much of each should be withdrawn
    for (i, asset) in assets.iter().enumerate() {
        // Calculate how much of this token corresponds to the dfToken amount
        let token_withdraw_amount = (df_token_amount * asset.ratio) / total_ratio; // Proportional to the total ratio sum
        withdrawal_amounts.set(asset, token_withdraw_amount);
    }

    Ok(withdrawal_amounts)
}

pub fn get_optimal_amounts_and_shares_to_mint_enforcing_asset_i(
    e: &Env,
    total_managed_funds: &Map<Address, i128>,
    assets: &Vec<Asset>,
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
    let shares_to_mint = 0; // TODO: calculate the shares to mint = total_supply * amount_desired_target  / reserve_target
    (optimal_amounts, shares_to_mint)
}

pub fn calculate_deposit_amounts_and_shares_to_mint(
    e: &Env,
    assets: &Vec<Asset>,
    amounts_desired: &Vec<i128>,
    amounts_min: &Vec<i128>,
) -> (Vec<i128>, i128) {
    // here we have already 3 vectors with same length, and all vectors are corrclty organized.
    // meaning that amounts_desired[i] is the amount desired for asset[i] and amounts_min[i] is the minimum amount for asset[i]

    // for each index, we calculate the optimal amount to deposit for the rest of the assets
    // and then we see if it is possible to deposit that amount, considering the amounts_min
    // if it is not possible, we calculate the optimal amount considering the next asset and so on
    // if it is not possible to deposit the optimal amount for the last asset, we throw an error

    let total_managed_funds = get_total_managed_funds(e); // Map<Address, i128>// a number for each asset
    for i in 0..assets.len() {
        // TODO dont enforce asset i if ratio of asset i is 0... in this case we need to enforce the next one
        let (optimal_amounts, shares_to_mint) =
            get_optimal_amounts_and_shares_to_mint_enforcing_asset_i(
                &e,
                &total_managed_funds,
                &assets,
                &amounts_desired,
                &i,
            );

        // if optimal _amounts[i]  is less than amounts_desired[i], but greater than amouints_min[i], then we cfalculate with the next one

        // Flag to indicate if we should skip the current asset {i} and continue
        let mut should_skip = false;

        for j in i + 1..assets.len() {
            // if optimal_amounts.get(j)  is less than amounts_desired.get(j), then we check if is at least more than the minimum, if yes, this might work!
            if optimal_amounts.get(j).unwrap() <= amounts_desired.get(j).unwrap() {
                // if not, this will never work, because the optimal amount with that amount_min
                if optimal_amounts.get(j).unwrap() < amounts_min.get(j).unwrap() {
                    panic!("insufficient amount"); // TODO transform panic in error
                }
                // if not, this is great. we continue, hoping this will be the answer
            } else {
                // If the optimal amount is greater to the desired amount, we skip the current asset {i}
                should_skip = true;
                // if we are in the last asset, we should throw an error
                if j == assets.len() - 1 {
                    panic!("didnt find optimal amounts"); // TODO transform panic in error
                }
                break; // Skip further checks for this asset {i}
            }
        }
        if should_skip {
            continue;
        } else {
            return (optimal_amounts, shares_to_mint);
        }
    }
    panic!("didnt finfd");
}
