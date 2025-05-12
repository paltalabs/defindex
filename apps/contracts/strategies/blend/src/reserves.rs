use crate::{
    blend_pool, constants::SCALAR_12, storage::{self, Config}
};

use defindex_strategy_core::StrategyError;
use soroban_fixed_point_math::{i128, FixedPoint};
use soroban_sdk::{contracttype, panic_with_error, Address, Env};


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
// taken from https://github.com/script3/fee-vault/

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

    /// Coverts a b_token amount to an underlying token amount rounding down
    pub fn b_tokens_to_underlying_down(&self, amount: i128) -> Result<i128, StrategyError> {
        return amount
            .fixed_mul_floor(self.b_rate, SCALAR_12)
            .ok_or_else(|| StrategyError::ArithmeticError);
    }

    pub fn update_rate(
        &mut self,
        e: &Env,
        config: &Config,
    ) {
        let new_rate = blend_pool::reserve_b_rate(e, &config);
        self.b_rate = new_rate;
        return;
    }
}

/// Get the strategy reserve from storage and update the bRate
///
/// ### Arguments
/// * `e` - The execution environment
/// * `config` - The configuration parameters for the strategy
///
/// ### Returns
/// * `StrategyReserves` - The updated reserve vault
pub fn get_strategy_reserve_updated(
    e: &Env, 
    config: &Config,    
) -> StrategyReserves {
    let mut reserve = storage::get_strategy_reserves(&e);
    reserve.update_rate(&e, &config);
    reserve
}

/// Updates the vault shares for a user with a positive value or panic.
///
/// ### Arguments
/// * `e` - The execution environment.
/// * `from` - The address of the user.
/// * `vault_shares` - The amount of vault shares to set.
/// 
/// ### Returns
/// * `Result<i128, StrategyError>` - The updated vault shares.
/// 
pub fn set_validated_vault_shares(
    e: &Env,
    from: &Address,
    vault_shares: i128
) -> Result<i128, StrategyError> {
    if vault_shares >= 0 {
        storage::set_vault_shares(&e, &from, vault_shares);
        Ok(vault_shares)
    } else {
        Err(StrategyError::OnlyPositiveAmountAllowed)
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
/// 1. Gets the current strategy reserves with updated bRate.
/// 2. Validate that `b_tokens_amount` is above zero.
/// 3. Retrieve the caller's existing vault shares.
/// 4. Compute the new shares to be minted based on bTokens deposited.
/// 5. Update total strategy shares and bTokens held.
/// 6. Store the updated strategy reserves and vault shares.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `from` - The address of the depositor (user/vault)
/// * `b_tokens_amount` - The amount of bTokens received from the deposit.
/// * `config` - The configuration parameters for the strategy.
///
/// # Returns
/// * `Result<(i128, StrategyReserves), StrategyError>` - A tuple containing the updated
///   vault shares of the depositor and the updated strategy reserves.
pub fn deposit(
    e: &Env,
    from: &Address,
    b_tokens_amount: i128,
    reserves: &StrategyReserves,
) -> Result<(i128, StrategyReserves), StrategyError> {
    let mut reserves = reserves.clone();
    if b_tokens_amount <= 0 {
        return Err(StrategyError::BTokensAmountBelowMin);
    }

    let old_vault_shares = storage::get_vault_shares(&e, &from);
    
    let new_minted_shares: i128 = reserves.b_tokens_to_shares_down(b_tokens_amount)?;

    if new_minted_shares <= 0 {
        panic_with_error!(e, StrategyError::InvalidSharesMinted);
    }
    
    // for the first depositor, the protocol will take out 1000 stroop units from the user shares in order to
    // avoid inflation attacks
    let new_vault_minted_shares = if reserves.total_shares == 0 {
        
        if new_minted_shares <= 1000 {
            panic_with_error!(e, StrategyError::InvalidSharesMinted);
        }

        new_minted_shares.checked_sub(1000).ok_or_else(|| StrategyError::UnderflowOverflow)?
    } else {
        new_minted_shares
    };
    
    reserves.total_shares = reserves
        .total_shares
        .checked_add(new_minted_shares)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;
    reserves.total_b_tokens = reserves
        .total_b_tokens
        .checked_add(b_tokens_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    // The vault shares are updated with the new vault minted shares (For the first depositor will be 1000 less)
    let new_vault_shares = old_vault_shares
        .checked_add(new_vault_minted_shares)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    storage::set_strategy_reserves(&e, reserves.clone());
    set_validated_vault_shares(e, from, new_vault_shares)?;
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
/// 1. Get the current strategy reserves with updated bRate.
/// 2. Validate that `b_tokens_amount` is above zero.
/// 3. Retrieve the caller's existing vault shares.
/// 4. Compute the shares to be burned based on bTokens withdrawn.
/// 5. Update total strategy shares and bTokens held.
/// 6. Store the updated strategy reserves and user/vault shares.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `from` - The address of the depositor.
/// * `b_tokens_amount` - The amount of bTokens received from the deposit.
/// * `config` - The configuration parameters for the strategy.
///
/// # Returns
/// * `Result<(i128, StrategyReserves), StrategyError>` - A tuple containing the updated
///   shares of the depositor and the updated strategy reserves.
pub fn withdraw(
    e: &Env,
    from: &Address,
    b_tokens_amount: i128,
    reserves: &StrategyReserves
) -> Result<(i128, StrategyReserves), StrategyError> {

    let mut reserves = reserves.clone();

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
    set_validated_vault_shares(e, from, vault_shares)?;
    Ok((vault_shares, reserves))
}

/// Updates strategy reserves after reinvesting rewards.
///
/// This function accounts for newly earned rewards by updating the total bTokens
/// held by the strategy. It assumes the rewards have already been reinvested
/// into the Blend pool and only updates the reserves accordingly.
///
/// # Process
/// 1. Validates that `b_tokens_amount` is positive.
/// 2. Retrieves the updated reserves with the new `b_rate` directly from the Blend contract.
/// 3. Increases the total bTokens stored in the strategy reserves based on the reinvestment amount.
/// 4. Stores the updated reserves in persistent storage.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `b_tokens_amount` - The amount of bTokens minted from the reinvestment.
///
/// # Returns
/// * `Result<StrategyReserves, StrategyError>` - Returns the strategy reserves if successful, otherwise an error.
///
/// # Errors
/// * `StrategyError::InvalidArgument` - If `b_tokens_amount` is not positive.
/// * `StrategyError::UnderflowOverflow` - If an arithmetic operation fails due to overflow/underflow.
pub fn harvest(
    e: &Env,
    b_tokens_amount: i128,
    config: &Config,
) -> Result<StrategyReserves, StrategyError> {
    let mut reserves = get_strategy_reserve_updated(e, &config);

    if b_tokens_amount <= 0 {
        panic_with_error!(e, StrategyError::BTokensAmountBelowMin);
    }

    reserves.total_b_tokens = reserves
        .total_b_tokens
        .checked_add(b_tokens_amount)
        .ok_or_else(|| StrategyError::UnderflowOverflow)?;

    storage::set_strategy_reserves(&e, reserves.clone());

    Ok(reserves)
}
