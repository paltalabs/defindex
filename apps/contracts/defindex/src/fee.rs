use soroban_sdk::{Address, Env, Map, Symbol, Vec};

use crate::{funds::fetch_total_managed_funds, storage::get_factory, token::VaultToken, utils::calculate_dftokens_from_asset_amounts, ContractError};

/// Fetches the current fee rate from the factory contract.
/// The fee rate is expressed in basis points (BPS).
pub fn fetch_fee_rate(e: &Env) -> u32 {
  let factory_address = get_factory(e);
  // Interacts with the factory contract to get the fee rate.
  e.invoke_contract(
    &factory_address,
    &Symbol::new(&e, "fee_rate"), 
    Vec::new(&e)
  )
}

pub fn assess_fees(e: &Env, time_elapsed: u64, fee_rate: u32) -> Result<i128, ContractError> {

    let total_managed_funds = fetch_total_managed_funds(e); // Get total managed funds per asset
    let df_token_supply = VaultToken::total_supply(e.clone()); // Get total supply of dfTokens
    
    let seconds_per_year = 31_536_000; // 365 days in seconds

    let mut total_fees_per_asset: Map<Address, i128> = Map::new(&e);

    // Iterate over each asset in the vault
    for (asset_address, amount) in total_managed_funds.iter() {
        // Fetch current managed funds for each asset
        let current_asset_value = amount;

        // Calculate the fee for this asset based on the fee rate and time elapsed
        let asset_fee = (current_asset_value * fee_rate as i128 * time_elapsed as i128) / (seconds_per_year * 10_000);

        total_fees_per_asset.set(asset_address.clone(), asset_fee);

        // Now convert the asset fee into dfTokens based on the total value of the vault
        // let df_tokens_to_mint = convert_fee_to_dftokens(&e, asset_fee, asset_address)?;
        
        // Add the dfTokens to the total fees to mint
        // total_fees_in_dftokens += df_tokens_to_mint;
    }

    let total_fees_in_dftokens = calculate_dftokens_from_asset_amounts(e, total_fees_per_asset)?;

    // fetch_total_managed_funds should correspond to the total supply of dfTokens
    // `total_fees_per_asset` (is the same Map as fetch_total_managed_funds but with the amounts corresponding to fees) is a map of asset addresses and their corresponding fees in dfTokens
    // for example
    // total_fees_per_asset = {
    //   usdc: 1000,
    //   xlm: 10200,
    // };
    // and this should be represented as dfTokens... how? need a helper function to convert the fees to dfTokens

    // now it should be possible to calculate the total fees in dfTokens
    // fetch_total_managed_funds returns a map of address and i128 all of the funds there should correspond to the total supply of dfTokens
    // so we can calculate the amount of dfTokens that corresponds to the total_fees_per_asset which is also a Map<Address, i128>    

    // Mint the total fees as dfTokens
    // mint_fees(e, total_fees_in_dftokens)?;

    // Update the last fee assessment timestamp
    // update_last_fee_assessment(e);

    Ok(total_fees_in_dftokens)
}

/// Converts the asset fee into dfTokens based on the asset's value.
pub fn convert_fee_to_dftokens(e: &Env, asset_fee: i128, asset_address: Address) -> Result<i128, ContractError> {
    let df_token_supply = VaultToken::total_supply(e.clone());

    // Calculate the value of dfTokens relative to the total managed funds
    let total_managed_funds = fetch_total_managed_funds(e);
    let total_vault_value = calculate_total_vault_value(&e, &total_managed_funds);

    // Convert the asset fee into dfTokens
    let df_token_value = (df_token_supply * asset_fee) / total_vault_value;

    Ok(df_token_value)
}

/// Calculates the total value of the vault based on managed funds.
pub fn calculate_total_vault_value(e: &Env, total_managed_funds: &Map<Address, i128>) -> i128 {
    let mut total_value = 0i128;
    for (asset_address, amount) in total_managed_funds.iter() {
        total_value += amount;
    }
    total_value
}