use soroban_sdk::{Env, Symbol, Vec};

use crate::{
    access::AccessControl,
    constants::{MAX_BPS, SECONDS_PER_YEAR},
    events,
    storage::{
        get_defindex_protocol_fee_receiver, get_factory, get_last_fee_assesment, get_vault_fee,
        set_last_fee_assesment,
    },
    token::{internal_mint, VaultToken},
    ContractError,
};

/// Fetches the current fee rate from the factory contract.
/// The fee rate is expressed in basis points (BPS).
pub fn fetch_defindex_fee(e: &Env) -> u32 {
    let factory_address = get_factory(e);
    // Interacts with the factory contract to get the fee rate.
    e.invoke_contract(
      &factory_address,
      &Symbol::new(&e, "defindex_fee"), 
      Vec::new(&e)
    )
}

/// Calculates the required fees in dfTokens based on the current APR fee rate.
fn calculate_fees(e: &Env, time_elapsed: u64, fee_rate: u32) -> Result<i128, ContractError> {
    let total_supply = VaultToken::total_supply(e.clone());

    // fee_rate as i128 * total_supply * time_elapsed / SECONDS_PER_YEAR * MAX_BPS - fee_rate as i128 * time_elapsed;
    let numerator = (fee_rate as i128)
        .checked_mul(total_supply)
        .unwrap()
        .checked_mul(time_elapsed as i128)
        .unwrap();
    let denominator = SECONDS_PER_YEAR
        .checked_mul(MAX_BPS)
        .unwrap()
        .checked_sub((fee_rate as i128).checked_mul(time_elapsed as i128).unwrap())
        .unwrap();
    let fees = numerator.checked_div(denominator).unwrap();
    
    Ok(fees)
}

/// Collects and mints fees in dfTokens, distributing them to the appropriate fee receivers.
pub fn collect_fees(e: &Env) -> Result<(), ContractError> {
    let current_timestamp = e.ledger().timestamp();
    // If last_fee_assesment was not set yet, this will be set to the current timestamp
    let last_fee_assessment = get_last_fee_assesment(e);

    // Update the last fee assessment timestamp
    // Set it now to Avoid Reentrancy Attack
    set_last_fee_assesment(e, &current_timestamp);

    let time_elapsed = current_timestamp.checked_sub(last_fee_assessment).unwrap();

    // If no time has passed since the last fee assessment, no fees are collected
    if time_elapsed == 0 {
        return Ok(());
    }

    // Fetch the individual fees for DeFindex and Vault, then calculate the total rate
    let defindex_fee = fetch_defindex_fee(e);
    let vault_fee = get_vault_fee(e);
    let total_fee_rate = defindex_fee.checked_add(vault_fee).unwrap();

    // Calculate the total fees in dfTokens based on the combined fee rate
    let total_fees = calculate_fees(e, time_elapsed, total_fee_rate)?;

    // Mint and distribute the fees proportionally
    mint_fees(e, total_fees, defindex_fee, vault_fee)?;


    Ok(())
}

/// Mints dfTokens for fees and distributes them to the vault fee receiver and DeFindex receiver.
fn mint_fees(e: &Env, total_fees: i128, defindex_fee: u32, vault_fee: u32) -> Result<(), ContractError> {
    let access_control = AccessControl::new(&e);

    let vault_fee_receiver = access_control.get_fee_receiver()?;
    let defindex_protocol_receiver = get_defindex_protocol_fee_receiver(e);

    // Calculate shares for each receiver based on their fee proportion
    let total_fee_bps = defindex_fee as i128 + vault_fee as i128;
    let defindex_shares = (total_fees * defindex_fee as i128) / total_fee_bps;
    let vault_shares = total_fees - defindex_shares;

    // Mint shares for both receivers
    internal_mint(e.clone(), vault_fee_receiver.clone(), vault_shares);
    internal_mint(
        e.clone(),
        defindex_protocol_receiver.clone(),
        defindex_shares,
    );

    events::emit_fees_minted_event(
        e,
        defindex_protocol_receiver,
        defindex_shares,
        vault_fee_receiver,
        vault_shares,
    );
    Ok(())
}
