use soroban_sdk::{contracttype, panic_with_error, Env};

use crate::{constants::MAX_BPS, ContractError};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub prev_balance: i128,
    pub gains_or_losses: i128,
    pub locked_fee: i128,
}

impl Report {
    pub fn lock_fee(&mut self, fee_rate: u32) {
        let gains_or_losses = self.gains_or_losses;
        let numerator = gains_or_losses
            .checked_mul(fee_rate as i128)
            .unwrap();
        let total_fee = numerator.checked_div(MAX_BPS).unwrap();

        self.locked_fee += total_fee;
        self.gains_or_losses = 0;
    }
    
    pub fn release_fee(&mut self, e: &Env, amount: i128) {
        if self.locked_fee < amount {
            panic_with_error!(e, ContractError::InsufficientManagedFunds);
        }
        self.locked_fee -= amount;
        self.gains_or_losses += amount;
    }

    pub fn report(&mut self, current_balance: i128) {
        let prev_balance = if self.prev_balance == 0 {
            current_balance
        } else {
            self.prev_balance
        };
        
        let gains_or_losses = current_balance - prev_balance;
        self.gains_or_losses += gains_or_losses;
        self.prev_balance = current_balance;
    }

    pub fn reset(&mut self) {
        self.prev_balance = 0;
        self.gains_or_losses = 0;
        self.locked_fee = 0;
    }
}