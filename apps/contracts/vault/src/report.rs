use soroban_sdk::{contracttype, panic_with_error, Address, Env};

use crate::{access::AccessControl, constants::MAX_BPS, funds::fetch_strategy_invested_funds, storage::{get_defindex_protocol_fee_rate, get_defindex_protocol_fee_receiver, get_report, set_report}, strategies::unwind_from_strategy, ContractError};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub prev_balance: i128,
    pub gains_or_losses: i128,
    pub locked_fee: i128,
}

impl Report {
    pub fn lock_fee(&mut self, fee_rate: u32) -> Result<(), ContractError> {
        let gains_or_losses = self.gains_or_losses;
        let numerator = gains_or_losses.checked_mul(fee_rate as i128).unwrap();
        let total_fee = numerator.checked_div(MAX_BPS).unwrap();

        self.locked_fee = self.locked_fee.checked_add(total_fee).ok_or(ContractError::Overflow)?;
        self.gains_or_losses = 0;
        Ok(())
    }

    pub fn release_fee(&mut self, e: &Env, amount: i128) -> Result<(), ContractError> {
        if self.locked_fee < amount {
            panic_with_error!(e, ContractError::InsufficientManagedFunds);
        }
        self.locked_fee = self.locked_fee.checked_sub(amount).ok_or(ContractError::Underflow)?;
        self.gains_or_losses = self.gains_or_losses.checked_add(amount).ok_or(ContractError::Overflow)?;
        Ok(())
    }

    pub fn report(&mut self, current_balance: i128) -> Result<(), ContractError> {
        let prev_balance = if self.prev_balance == 0 {
            current_balance
        } else {
            self.prev_balance
        };

        let gains_or_losses = current_balance.checked_sub(prev_balance).ok_or(ContractError::Underflow)?;
        self.gains_or_losses = self.gains_or_losses.checked_add(gains_or_losses).ok_or(ContractError::Overflow)?;
        self.prev_balance = current_balance;
        Ok(())
    }

    pub fn reset(&mut self) {
        self.prev_balance = 0;
        self.gains_or_losses = 0;
        self.locked_fee = 0;
    }
}

pub fn distribute_strategy_fees(e: &Env, strategy_address: &Address, access_control: &AccessControl) -> Result<i128, ContractError> {
    let mut report = get_report(&e, &strategy_address);
    let strategy_balance = fetch_strategy_invested_funds(e, strategy_address, false)?;
    report.report(strategy_balance)?;
    
    let defindex_fee = get_defindex_protocol_fee_rate(&e);
    report.lock_fee(defindex_fee)?;

    let defindex_protocol_receiver = get_defindex_protocol_fee_receiver(&e)?;
    let vault_fee_receiver = access_control.get_fee_receiver()?;

    let mut fees_distributed: i128 = 0;

    if report.locked_fee > 0 {
        // Calculate shares for each receiver based on their fee proportion
        let numerator = report.locked_fee.checked_mul(defindex_fee as i128).unwrap();
        let defindex_fee_amount = numerator.checked_div(MAX_BPS).unwrap();

        let vault_fee_amount = report.locked_fee.checked_sub(defindex_fee_amount).ok_or(ContractError::Underflow)?;

        unwind_from_strategy(
            &e,
            &strategy_address,
            &defindex_fee_amount,
            &defindex_protocol_receiver,
        )?;
        let mut report = unwind_from_strategy(
            &e,
            &strategy_address,
            &vault_fee_amount,
            &vault_fee_receiver,
        )?;
        
        fees_distributed = report.locked_fee;
        report.locked_fee = 0;
        set_report(&e, &strategy_address, &report);
    }

    Ok(fees_distributed)
}