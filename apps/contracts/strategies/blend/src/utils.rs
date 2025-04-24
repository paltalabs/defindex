use defindex_strategy_core::StrategyError;
use soroban_fixed_point_math::FixedPoint;

use crate::{constants::SCALAR_9, reserves::StrategyReserves};


/// Converts a given amount of shares to the corresponding amount of underlying assets.
///
/// This function first converts the shares to bTokens using the `shares_to_b_tokens_down` method
/// from the `reserves`. It then uses the `b_rate` to convert the bTokens to the underlying assets.
///
/// # Arguments
///
/// * `shares` - The amount of shares to be converted.
/// * `reserves` - The strategy reserves containing the total shares, total bTokens, and b_rate.
///
/// # Returns
///
/// * `Result<i128, StrategyError>` - The amount of underlying assets corresponding to the given shares,
///   or an error if the conversion fails.
///
pub fn shares_to_underlying(shares: i128, reserves: StrategyReserves) -> Result<i128, StrategyError> {
  let total_shares = reserves.total_shares;
  let total_b_tokens = reserves.total_b_tokens;

  if total_shares == 0 || total_b_tokens == 0 {
      // No shares or bTokens in the strategy
      return Ok(0i128);
  }
  // Calculate the bTokens corresponding to the vault's shares
  let vault_b_tokens = reserves.shares_to_b_tokens_down(shares)?;
  // Use the b_rate to convert bTokens to underlying assets
  reserves.b_tokens_to_underlying_down(vault_b_tokens)
}

pub fn calculate_optimal_deposit_amount(
  deposit_amount: i128,
  reserves: &StrategyReserves,
) -> Result<i128, StrategyError> {
  // Step 1: Calculate the amount of bTokens that would be minted based on the deposit_amount
  let b_tokens_minted = deposit_amount * SCALAR_9 / reserves.b_rate;

  // If the reserves are empty, the optimal_b_token amount is equal to the b_tokens_minted
  let optimal_b_token_amount = if reserves.total_shares == 0 {
      b_tokens_minted
  } else {
      // Step 2: Calculate the amount of shares that would be minted based on the bTokens minted
      let shares_minted = reserves.b_tokens_to_shares_down(b_tokens_minted)?;
      if shares_minted == 0 {
          return Err(StrategyError::InvalidSharesMinted);
      }
      // Step 3: Based on the shares to-be minted, calculate the optimal bToken amount
      // Note: This should round up, as the shares calculation round down
      (shares_minted * reserves.total_b_tokens - 1) / reserves.total_shares + 1
  };

  if optimal_b_token_amount <= 0 {
      return Err(StrategyError::BTokensAmountBelowMin);
  }

  // Step 4: Now calculate the optimal deposit amount to deposit to Blend. This should also round up
  let optimal_deposit_amt = (optimal_b_token_amount * reserves.b_rate - 1) / SCALAR_9 + 1;

  Ok(optimal_deposit_amt)
}


pub fn calculate_optimal_withdraw_amount(
  withdraw_amount: i128,
  reserves: &StrategyReserves
) -> Result<i128, StrategyError> {
  let b_tokens_burnt = withdraw_amount.fixed_mul_ceil(SCALAR_9, reserves.b_rate)
    .ok_or(StrategyError::ArithmeticError)?;
  let shares_burnt = reserves.b_tokens_to_shares_up(b_tokens_burnt)?;
  let optimal_b_tokens = shares_burnt.fixed_mul_floor(reserves.total_b_tokens, reserves.total_shares) // Changed to floor
    .ok_or(StrategyError::ArithmeticError)?;
  let optimal_withdraw_amount = optimal_b_tokens.fixed_mul_floor(reserves.b_rate, SCALAR_9) // Changed to floor
    .ok_or(StrategyError::ArithmeticError)?;
  Ok(optimal_withdraw_amount)
}