use soroban_sdk::{Address, Env, Map, Symbol, Vec};

use crate::{access::AccessControl, constants::{MAX_BPS, SECONDS_PER_YEAR}, events, funds::fetch_total_managed_funds, storage::{get_defindex_receiver, get_factory, get_last_fee_assesment, get_vault_share, set_last_fee_assesment}, token::internal_mint, utils::calculate_dftokens_from_asset_amounts, ContractError};

/// Fetches the current fee rate from the factory contract.
/// The fee rate is expressed in basis points (BPS).
fn fetch_fee_rate(e: &Env) -> u32 {
  let factory_address = get_factory(e);
  // Interacts with the factory contract to get the fee rate.
  e.invoke_contract(
    &factory_address,
    &Symbol::new(&e, "fee_rate"), 
    Vec::new(&e)
  )
}

fn calculate_fees(e: &Env, time_elapsed: u64, fee_rate: u32) -> Result<i128, ContractError> {

    let total_managed_funds = fetch_total_managed_funds(e); // Get total managed funds per asset
    
    let seconds_per_year = SECONDS_PER_YEAR; // 365 days in seconds

    let mut total_fees_per_asset: Map<Address, i128> = Map::new(&e);

    // Iterate over each asset in the vault
    for (asset_address, amount) in total_managed_funds.iter() {
        // Fetch current managed funds for each asset
        let current_asset_value = amount;

        // Calculate the fee for this asset based on the fee rate and time elapsed
        let asset_fee = (current_asset_value * fee_rate as i128 * time_elapsed as i128) / (seconds_per_year * MAX_BPS);

        total_fees_per_asset.set(asset_address.clone(), asset_fee);

    }

    let total_fees_in_dftokens = calculate_dftokens_from_asset_amounts(e, total_fees_per_asset)?;

    Ok(total_fees_in_dftokens)
}

pub fn collect_fees(e: &Env) -> Result<(), ContractError> {
    let current_timestamp = e.ledger().timestamp();
    let last_fee_assessment = get_last_fee_assesment(e); 

    let time_elapsed = current_timestamp.checked_sub(last_fee_assessment).unwrap();

    if time_elapsed == 0 {
        return Ok(());
    }

    let fee_rate = fetch_fee_rate(e);

    let total_fees = calculate_fees(e, time_elapsed, fee_rate)?;

    // Mint the total fees as dfTokens
    mint_fees(e, total_fees)?;

    // Update the last fee assessment timestamp
    set_last_fee_assesment(e, &current_timestamp);

    Ok(())
}

fn mint_fees(e: &Env, total_fees: i128) -> Result<(), ContractError> {
    let access_control = AccessControl::new(&e);
    
    let vault_fee_receiver = access_control.get_fee_receiver()?;
    let defindex_receiver = get_defindex_receiver(e);

    let vault_share_bps = get_vault_share(e);

    let vault_shares = (total_fees * vault_share_bps as i128) / MAX_BPS;
    
    let defindex_shares = total_fees - vault_shares;

    internal_mint(e.clone(), vault_fee_receiver.clone(), vault_shares);
    internal_mint(e.clone(), defindex_receiver.clone(), defindex_shares);

    events::emit_fees_minted_event(e, defindex_receiver, defindex_shares, vault_fee_receiver, vault_shares);
    Ok(())
}