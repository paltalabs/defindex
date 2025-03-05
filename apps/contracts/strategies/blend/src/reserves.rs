use defindex_strategy_core::StrategyError;
use soroban_fixed_point_math::{i128, FixedPoint};
use soroban_sdk::{contracttype, panic_with_error, Address, Env};

use crate::{constants::SCALAR_9, storage};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyReserves {
    /// The total deposited amount of the underlying asset
    pub total_shares: i128,
    /// The total bToken deposits owned by the strategy depositors.
    pub total_b_tokens: i128,
    /// The reserve's last bRate
    pub b_rate: i128,
}

// Useful functions to handle with reserves and b tokens
// taken from https://github.com/script3/fee-vault/blob/433ae359b24f15dee66fc624fa09479890e249f5/src/reserve_vault.rs#L24

impl StrategyReserves {
    /// Converts a b_token amount to shares rounding down
    pub fn b_tokens_to_shares_down(&self, amount: i128) -> Result<i128, StrategyError> {
        if self.total_shares == 0 || self.total_b_tokens == 0 {
            return Ok(amount);
        } else {
            return amount
                .fixed_mul_floor(self.total_shares, self.total_b_tokens)
                .ok_or_else(|| StrategyError::ArithmeticError);
        }
    }

    /// Converts a b_token amount to shares rounding up
    pub fn b_tokens_to_shares_up(&self, amount: i128) -> Result<i128, StrategyError> {
        if self.total_shares == 0 || self.total_b_tokens == 0 {
            return Ok(amount);
        }
        amount
            .fixed_mul_ceil(self.total_shares, self.total_b_tokens)
            .ok_or_else(|| StrategyError::ArithmeticError)
    }

    /// Coverts a share amount to a b_token amount rounding down
    pub fn shares_to_b_tokens_down(&self, amount: i128) -> Result<i128, StrategyError> {
        amount
            .fixed_div_floor(self.total_shares, self.total_b_tokens)
            .ok_or_else(|| StrategyError::DivisionByZero)
    }

    pub fn update_rate(
        &mut self,
        underlying_amount: i128,
        b_tokens_amount: i128,
    ) -> Result<(), StrategyError> {
        // Calculate the new bRate - 9 decimal places of precision
        // Update the reserve's bRate
        let new_rate = underlying_amount
            .fixed_div_floor(b_tokens_amount, SCALAR_9)
            .ok_or_else(|| StrategyError::ArithmeticError)?;

        self.b_rate = new_rate;

        Ok(())
    }
}

/// Accounts for a deposit into the Blend pool.
///
/// This function updates the strategy reserves and user/vault shares after a deposit
/// has been made. It calculates the new user/vault shares, updates the total
/// shares owned by the strategy, and adjusts the reserves based on the deposited
/// underlying asset and bTokens.
///
/// # Process
/// 1. Validate that `underlying_amount` and `b_tokens_amount` are above zero.
/// 2. Update the strategy's rate based on the deposit.
/// 3. Retrieve the caller's existing vault shares.
/// 4. Compute the new shares to be minted based on bTokens deposited.
/// 5. Update total strategy shares and bTokens held.
/// 6. Store the updated strategy reserves and vault shares.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `reserves` - The current strategy reserves.
/// * `from` - The address of the depositor (user/vault)
/// * `underlying_amount` - The amount of the underlying asset deposited.
/// * `b_tokens_amount` - The amount of bTokens received from the deposit.
///
/// # Returns
/// * `Result<(i128, StrategyReserves), StrategyError>` - A tuple containing the updated
///   vault shares of the depositor and the updated strategy reserves.
pub fn deposit(
    e: &Env,
    mut reserves: StrategyReserves,
    from: &Address,
    underlying_amount: i128,
    b_tokens_amount: i128,
) -> Result<(i128, StrategyReserves), StrategyError> {
    if underlying_amount <= 0 {
        panic_with_error!(e, StrategyError::UnderlyingAmountBelowMin);
    }

    if b_tokens_amount <= 0 {
        panic_with_error!(e, StrategyError::BTokensAmountBelowMin);
    }

    reserves.update_rate(underlying_amount, b_tokens_amount)?;

    let old_vault_shares = storage::get_vault_shares(&e, &from);
    let new_minted_shares: i128 = reserves.b_tokens_to_shares_down(b_tokens_amount)?;

    reserves.total_shares = reserves
        .total_shares
        .checked_add(new_minted_shares)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;
    reserves.total_b_tokens = reserves
        .total_b_tokens
        .checked_add(b_tokens_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    let new_vault_shares = old_vault_shares
        .checked_add(new_minted_shares)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    storage::set_strategy_reserves(&e, reserves.clone());
    storage::set_vault_shares(&e, &from, new_vault_shares);
    Ok((new_vault_shares, reserves))
}

/// Accounts for a deposit into the Blend pool.
///
/// This function updates the strategy reserves and vault shares after a deposit
/// has been made. It calculates the new user/vault shares, updates the total
/// shares owned by the strategy, and adjusts the reserves based on the deposited
/// underlying asset and bTokens.
///
/// # Process
/// 1. Validate that `underlying_amount` and `b_tokens_amount` are above zero.
/// 2. Update the strategy's rate based on the deposit.
/// 3. Retrieve the caller (vault)'s existing shares (vault_shares)
/// 4. Compute the new shares to be minted based on bTokens deposited.
/// 5. Update total strategy shares and bTokens held.
/// 6. Store the updated strategy reserves and user/vault shares.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `reserves` - The current strategy reserves.
/// * `from` - The address of the depositor.
/// * `underlying_amount` - The amount of the underlying asset deposited.
/// * `b_tokens_amount` - The amount of bTokens received from the deposit.
///
/// # Returns
/// * `Result<(i128, StrategyReserves), StrategyError>` - A tuple containing the updated
///   shares of the depositor and the updated strategy reserves.
pub fn withdraw(
    e: &Env,
    mut reserves: StrategyReserves,
    from: &Address,
    underlying_amount: i128,
    b_tokens_amount: i128,
) -> Result<(i128, StrategyReserves), StrategyError> {
    if underlying_amount <= 0 {
        return Err(StrategyError::UnderlyingAmountBelowMin);
    }
    if b_tokens_amount <= 0 {
        return Err(StrategyError::BTokensAmountBelowMin);
    }

    let mut vault_shares = storage::get_vault_shares(&e, &from);
    let share_amount = reserves.b_tokens_to_shares_up(b_tokens_amount)?;

    if reserves.total_shares < share_amount || reserves.total_b_tokens < b_tokens_amount {
        return Err(StrategyError::InsufficientBalance);
    }

    reserves.total_shares = reserves
        .total_shares
        .checked_sub(share_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;
    reserves.total_b_tokens = reserves
        .total_b_tokens
        .checked_sub(b_tokens_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    if share_amount > vault_shares {
        return Err(StrategyError::InsufficientBalance);
    }

    vault_shares = vault_shares
        .checked_sub(share_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    storage::set_strategy_reserves(&e, reserves.clone());
    storage::set_vault_shares(&e, &from, vault_shares);

    Ok((vault_shares, reserves))
}

/// Updates strategy reserves after reinvesting rewards.
///
/// This function accounts for newly earned rewards by updating the total bTokens
/// held by the strategy. It assumes the rewards have already been reinvested
/// into the Blend pool and only updates the reserves accordingly.
///
/// # Process
/// 1. Validate that `underlying_amount` and `b_tokens_amount` are positive.
/// 2. Update the reserve rate using the newly acquired underlying assets and bTokens.
/// 3. Increase the total bTokens stored in the strategy reserves.
/// 4. Store the updated reserves in persistent storage.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `reserves` - The current strategy reserves.
/// * `underlying_amount` - The amount of the underlying asset obtained from rewards.
/// * `b_tokens_amount` - The amount of bTokens minted from the reinvestment.
///
/// # Returns
/// * `Result<(), StrategyError>` - Returns `Ok(())` if successful, otherwise an error.
///
/// # Errors
/// * `StrategyError::InvalidArgument` - If `underlying_amount` or `b_tokens_amount` are not positive.
/// * `StrategyError::UnderflowOverflow` - If an arithmetic operation fails due to an overflow/underflow.
pub fn harvest(
    e: &Env,
    mut reserves: StrategyReserves,
    underlying_amount: i128,
    b_tokens_amount: i128,
) -> Result<(), StrategyError> {
    if underlying_amount <= 0 {
        panic_with_error!(e, StrategyError::UnderlyingAmountBelowMin);
    }

    if b_tokens_amount <= 0 {
        panic_with_error!(e, StrategyError::BTokensAmountBelowMin);
    }

    let _ = reserves.update_rate(underlying_amount, b_tokens_amount)?;

    reserves.total_b_tokens = reserves
        .total_b_tokens
        .checked_add(b_tokens_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    storage::set_strategy_reserves(&e, reserves);

    Ok(())
}
