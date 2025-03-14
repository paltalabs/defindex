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
    /// Locks a portion of the gains or losses as a fee based on the specified fee rate, 
    /// separating it from the current gains/losses and resetting the gains/losses to zero.
    /// 
    /// The fee is calculated as (gains_or_losses * fee_rate) / SCALAR_BPS, where SCALAR_BPS 
    /// is a constant equal to 10,000 (representing basis points). 
    /// This should be done before any deposit and any withdraw from any strategy. 
    ///
    /// By increasing or decreasing the fee_rate, the manager can manipulate the APY of a strategy
    /// by locking fees when the APY exceeds a certain threshold, effectively reducing the reported gains. The calculated fee is added to 
    /// `locked_fee`, and `gains_or_losses` is reset to 0.
    ///
    /// # Arguments
    /// * `fee_rate` - The fee rate (as a u32, in basis points) to apply to the current gains or losses.
    ///
    /// # Returns
    /// * `Result<(), ContractError>` - Returns `Ok(())` on success, or a `ContractError::Overflow` 
    ///   if an arithmetic overflow occurs during fee calculation or locked fee update.
    ///
    /// # Examples
    /// If `gains_or_losses = 50`, `fee_rate = 1000` (10%), and `SCALAR_BPS = 10,000`, 
    /// then `total_fee = (50 * 1000) / 10,000 = 5`. This fee is added to `locked_fee`, 
    /// and `gains_or_losses` is reset to 0.
    pub fn lock_fee(&mut self, fee_rate: u32) -> Result<(), ContractError> {
        let gains_or_losses = self.gains_or_losses;
        let numerator = gains_or_losses.checked_mul(fee_rate as i128).unwrap();
        let total_fee = numerator.checked_div(MAX_BPS).unwrap();

        self.locked_fee = self.locked_fee.checked_add(total_fee).ok_or(ContractError::Overflow)?;
        self.gains_or_losses = 0;
        Ok(())
    }

    /// Releases a specified amount of previously locked fees back to the users, reducing the manager's gains 
    /// and adding the amount to the current gains or losses.
    /// 
    /// This function is used when the APY is too low, allowing the manager to return some or all of the fees 
    /// that were previously locked via `lock_fee` to the users. The released amount is subtracted from 
    /// `locked_fee` and added to `gains_or_losses`. If the requested amount exceeds the available 
    /// `locked_fee`, the function panics with an `InsufficientManagedFunds` error.
    ///
    /// # Arguments
    /// * `e` - A reference to the environment (`Env`), used for error handling.
    /// * `amount` - The amount of locked fees (as an i128) to release back to the users.
    ///
    /// # Returns
    /// * `Result<(), ContractError>` - Returns `Ok(())` on success, or a `ContractError` if:
    ///   - An underflow occurs when subtracting from `locked_fee` (`ContractError::Underflow`).
    ///   - An overflow occurs when adding to `gains_or_losses` (`ContractError::Overflow`).
    ///
    /// # Panics
    /// Panics with `ContractError::InsufficientManagedFunds` if the requested `amount` exceeds the 
    /// available `locked_fee`.
    ///
    /// # Examples
    /// If `locked_fee = 10` and `gains_or_losses = 0`, calling `release_fee(e, 5)` will set 
    /// `locked_fee = 5` and `gains_or_losses = 5`. If `amount = 15`, it will panic with 
    /// `InsufficientManagedFunds`.
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

    /// Resets the strategy's financial tracking fields to zero, intended for use only when rescuing funds 
    /// after fully unwinding a strategy.
    ///
    /// This function clears the `prev_balance`, `gains_or_losses`, and `locked_fee` fields, effectively 
    /// resetting the state of the strategy. It is designed to be called in emergency situations where all 
    /// funds have been withdrawn from the strategy, ensuring no residual values remain in the tracking 
    /// variables.
    ///
    /// # Notes
    /// - This function does not return a result as it performs a simple state reset without the possibility 
    ///   of failure.
    /// - Should only be invoked after all funds have been unwound from the strategy to avoid losing track 
    ///   of active balances or fees.
    ///
    pub fn reset(&mut self) {
        self.prev_balance = 0;
        self.gains_or_losses = 0;
        self.locked_fee = 0;
    }
}

pub fn update_and_lock_fees(e: &Env, strategy_address: &Address) -> Result<Report, ContractError> {
    let mut report = get_report(&e, &strategy_address);
    let strategy_balance = fetch_strategy_invested_funds(e, strategy_address, false)?;
    report.report(strategy_balance)?;

    let defindex_fee = get_defindex_protocol_fee_rate(&e);
    report.lock_fee(defindex_fee)?;

    Ok(report)
}

pub fn distribute_strategy_fees(e: &Env, strategy_address: &Address, access_control: &AccessControl) -> Result<i128, ContractError> {
    let report = get_report(e, strategy_address);
    
    let defindex_fee = get_defindex_protocol_fee_rate(&e);
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