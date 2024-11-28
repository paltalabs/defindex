use soroban_fixed_point_math::{i128, FixedPoint};
use soroban_sdk::contracttype;

use crate::constants::SCALAR_9;

#[contracttype]
pub struct StrategyReserves {
    /// The total deposited amount of the underlying asset
    pub total_deposited: i128,
    /// The total bToken deposits owned by the strategy depositors.
    pub total_b_tokens: i128,
    /// The reserve's last bRate
    pub b_rate: i128,
}

impl StrategyReserves {
    pub fn add(&mut self, amount: i128, b_tokens: i128) {
        // Calculate the new bRate - 9 decimal places of precision
        // Update the reserve's bRate
        self.b_rate = new_rate(amount, b_tokens);
        
        self.total_b_tokens += b_tokens;
        self.total_deposited += amount;
    }

    pub fn remove(&mut self, amount: i128, b_tokens: i128) {
        // Calculate the new bRate - 9 decimal places of precision
        // Update the reserve's bRate
        self.b_rate = new_rate(amount, b_tokens);
        
        self.total_b_tokens -= b_tokens;
    }
}

fn new_rate(amount: i128, b_tokens: i128) -> i128 {
    amount
        .fixed_div_floor(b_tokens, SCALAR_9)
        .unwrap()
}