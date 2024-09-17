use soroban_sdk::{Address, Env, Map};

use crate::{storage::{get_assets}, ContractError};
pub const DAY_IN_LEDGERS: u32 = 17280;

pub fn bump_instance(e: &Env) {
  let max_ttl = e.storage().max_ttl();
  e.storage()
      .instance()
      .extend_ttl(max_ttl - DAY_IN_LEDGERS, max_ttl);
}

/// Converts dfToken amount into corresponding token amounts based on their ratio.
pub fn calculate_withdrawal_amounts(
  e: &Env,
  df_token_amount: i128,  // The amount of dfTokens to withdraw
) -> Result<Map<Address, i128>, ContractError> {
  let mut withdrawal_amounts = Map::<Address, i128>::new(e);
  let assets = get_assets(e);

  let total_ratio= assets.iter().fold(0, |acc, asset| acc + asset.ratio);

  // Iterate through all assets and calculate how much of each should be withdrawn
  for (i, asset) in assets.iter().enumerate() {
      // Calculate how much of this token corresponds to the dfToken amount
      let token_withdraw_amount = (df_token_amount * asset.ratio) / total_ratio;  // Proportional to the total ratio sum
      withdrawal_amounts.set(asset.address.clone(), token_withdraw_amount);
  }

  Ok(withdrawal_amounts)
}