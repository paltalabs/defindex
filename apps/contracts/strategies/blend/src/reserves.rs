use defindex_strategy_core::StrategyError;
use soroban_fixed_point_math::{i128, FixedPoint};
use soroban_sdk::{contracttype, panic_with_error, Address, Env};

use crate::{constants::SCALAR_9, storage};

#[contracttype]
pub struct StrategyReserves {
    /// The total deposited amount of the underlying asset
    pub total_shares: i128,
    /// The total bToken deposits owned by the strategy depositors.
    pub total_b_tokens: i128,
    /// The reserve's last bRate
    pub b_rate: i128,
}

impl StrategyReserves {
    /// Converts a b_token amount to shares rounding down
    pub fn b_tokens_to_shares_down(&self, amount: i128) -> i128 {
        if self.total_shares == 0 || self.total_b_tokens == 0 {
            return amount;
        }
        amount
            .fixed_mul_floor(self.total_shares, self.total_b_tokens)
            .unwrap()
    }

    /// Converts a b_token amount to shares rounding up
    pub fn b_tokens_to_shares_up(&self, amount: i128) -> i128 {
        if self.total_shares == 0 || self.total_b_tokens == 0 {
            return amount;
        }
        amount
            .fixed_mul_ceil(self.total_shares, self.total_b_tokens)
            .unwrap()
    }

    /// Coverts a share amount to a b_token amount rounding down
    pub fn shares_to_b_tokens_down(&self, amount: i128) -> i128 {
        amount
            .fixed_div_floor(self.total_shares, self.total_b_tokens)
            .unwrap()
    }

    pub fn update_rate(&mut self, amount: i128, b_tokens: i128) {
        // Calculate the new bRate - 9 decimal places of precision
        // Update the reserve's bRate
        let new_rate = amount
            .fixed_div_floor(b_tokens, SCALAR_9)
            .unwrap();

        self.b_rate = new_rate;
    }

}

/// Deposit into the reserve vault. This function expects the deposit to have already been made
/// into the pool, and accounts for the deposit in the reserve vault.
pub fn deposit(
    e: &Env,
    mut reserves: StrategyReserves,
    from: &Address,
    underlying_amount: i128,
    b_tokens_amount: i128,
) -> i128 {
    if underlying_amount <= 0 {
        panic_with_error!(e, StrategyError::InvalidArgument); //TODO: create a new error type for this
    }

    if b_tokens_amount <= 0 {
        panic_with_error!(e, StrategyError::InvalidArgument); //TODO: create a new error type for this
    }

    reserves.update_rate(underlying_amount, b_tokens_amount);
    
    let mut vault_shares = storage::get_vault_shares(&e, &from);
    let share_amount: i128 = reserves.b_tokens_to_shares_down(b_tokens_amount);
    
    reserves.total_shares += share_amount;
    reserves.total_b_tokens += b_tokens_amount;

    vault_shares += share_amount;
    
    storage::set_strategy_reserves(&e, reserves);
    storage::set_vault_shares(&e, &from, vault_shares);
    share_amount
}

/// Withdraw from the reserve vault. This function expects the withdraw to have already been made
/// from the pool, and only accounts for the withdraw from the reserve vault.
pub fn withdraw(
    e: &Env,
    mut reserves: StrategyReserves,
    from: &Address,
    underlying_amount: i128,
    b_tokens_amount: i128,
) -> i128 {
    if underlying_amount <= 0 {
        panic_with_error!(e, StrategyError::InvalidArgument);
    }
    if b_tokens_amount <= 0 {
        panic_with_error!(e, StrategyError::InvalidArgument);
    }

    reserves.update_rate(underlying_amount, b_tokens_amount);

    let mut vault_shares = storage::get_vault_shares(&e, &from);
    let share_amount = reserves.b_tokens_to_shares_up(b_tokens_amount);

    if reserves.total_shares < share_amount || reserves.total_b_tokens < b_tokens_amount {
        panic_with_error!(e, StrategyError::InvalidArgument);
    }

    reserves.total_shares -= share_amount;
    reserves.total_b_tokens -= b_tokens_amount;
    
    if share_amount > vault_shares {
        panic_with_error!(e, StrategyError::InvalidArgument);
    }

    vault_shares -= share_amount;
    storage::set_strategy_reserves(&e, reserves);
    storage::set_vault_shares(&e, &from, vault_shares);

    share_amount
}