use soroban_sdk::{Address, Env, Map};

use crate::{storage::{get_ratio, get_tokens}, ContractError};
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
  let tokens = get_tokens(e);

  // Calculate the total sum of all ratios
  let total_ratio: u32 = tokens
      .iter()
      .enumerate()
      .map(|(i, _)| get_ratio(e, i.try_into().unwrap()))
      .sum();

  // Iterate through all tokens and calculate how much of each should be withdrawn
  for (i, token_address) in tokens.iter().enumerate() {
      let ratio = get_ratio(e, i.try_into().unwrap());

      // Calculate how much of this token corresponds to the dfToken amount
      let token_withdraw_amount = (df_token_amount * (ratio as i128)) / (total_ratio as i128);  // Proportional to the total ratio sum
      withdrawal_amounts.set(token_address.clone(), token_withdraw_amount);
  }

  Ok(withdrawal_amounts)
}