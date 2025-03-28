use soroban_sdk::{
    contracttype, panic_with_error, token::TokenClient, Address, Env
};
use crate::{
    access::AccessControl,
    constants::SCALAR_BPS,
    storage::{
        get_defindex_protocol_fee_rate,
        get_defindex_protocol_fee_receiver,
        get_report,
        set_report,
        get_vault_fee
    },
    strategies::unwind_from_strategy,
    ContractError
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Report {
    pub prev_balance: i128,
    pub gains_or_losses: i128,
    pub locked_fee: i128,
}

impl Report {
    /// Locks a portion of the current gains or losses as a fee, based on the specified fee rate. 
    /// This separates the fee from the current gains/losses and resets the gains/losses to zero.
    /// 
    /// The fee is calculated using the formula: 
    /// (gains_or_losses * fee_rate) / SCALAR_BPS, where SCALAR_BPS is a constant (10,000) 
    /// representing basis points. This operation should be performed or withdrawal as this 
    ///  ensures users always get the least amount possible, preventing the pool from losing value
    /// from any strategy.
    /// 
    /// By adjusting the `fee_rate`, the manager can influence the reported APY of a strategy by locking
    /// a fee when the APY exceeds a threshold, thus reducing the displayed gains. The calculated fee is 
    /// added to `locked_fee`, and the `gains_or_losses` are reset to zero.
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
        if self.gains_or_losses <= 0 {
            return Ok(());
        }
        let gains_or_losses = self.gains_or_losses;
        let numerator = gains_or_losses.checked_mul(fee_rate as i128).unwrap();
        let total_fee = numerator.checked_div(SCALAR_BPS).unwrap();

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
    /// `locked_fee`, the function panics with an `InsufficientFeesToRelease` error.
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
    /// Panics with `ContractError::InsufficientFeesToRelease` if the requested `amount` exceeds the 
    /// available `locked_fee`.
    ///
    /// # Examples
    /// If `locked_fee = 10` and `gains_or_losses = 0`, calling `release_fee(e, 5)` will set 
    /// `locked_fee = 5` and `gains_or_losses = 5`. If `amount = 15`, it will panic with 
    /// `InsufficientFeesToRelease`.
    pub fn release_fee(&mut self, e: &Env, amount: i128) -> Result<(), ContractError> {
        if self.locked_fee < amount {
            panic_with_error!(e, ContractError::InsufficientFeesToRelease);
        }
        self.locked_fee = self.locked_fee.checked_sub(amount).ok_or(ContractError::Underflow)?;
        self.gains_or_losses = self.gains_or_losses.checked_add(amount).ok_or(ContractError::Overflow)?;
        Ok(())
    }

    /// Updates the current balance, calculates the gains or losses, and updates the total 
    /// accumulated gains or losses for the contract. The previous balance is updated for the next report.
    ///
    /// If this is the first report (i.e., `prev_balance` is 0), the current balance is used as the 
    /// reference for the previous balance and no gains or losses are recorded.
    ///
    /// # Arguments
    /// * `current_balance` - The current balance to compare against the previous balance to calculate the gains or losses.
    ///
    /// # Returns
    /// * `Result<(), ContractError>` - Returns `Ok(())` on success, or a `ContractError::Underflow` 
    ///   if there is an underflow when calculating the current gains/losses, or `ContractError::Overflow` 
    ///   if the accumulated gains/losses exceed the allowable range.
    ///
    /// # Examples
    /// If `prev_balance = 1000` and `current_balance = 1200`, the `current_gains_or_losses` will be 200, 
    /// and the `gains_or_losses` will be updated accordingly. The `prev_balance` will also be updated to `1200`.
    pub fn report(&mut self, current_balance: i128) -> Result<(), ContractError> {
        // Use current balance as previous balance if this is the first report (prev_balance is 0)
        let prev_balance = if self.prev_balance == 0 {
            current_balance
        } else {
            self.prev_balance
        };

        // Calculate gains or losses and handle potential underflow
        let current_gains_or_losses = current_balance
            .checked_sub(prev_balance)
            .ok_or(ContractError::Underflow)?;

        // Update the accumulated gains or losses, handle overflow
        self.gains_or_losses = self.gains_or_losses
            .checked_add(current_gains_or_losses)
            .ok_or(ContractError::Overflow)?;

        // Update the previous balance for the next report
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

/// Updates the strategy report and locks fees based on the current strategy balance and fee rate.
/// 
/// This function retrieves the current strategy report, updates it with the current invested funds,
/// and locks any applicable fees based on the vault's fee rate. The updated report is then saved back 
/// to the storage to reflect the most recent financial data for the strategy.
///
/// # Arguments
/// * `e` - The environment context that provides access to storage and functions.
/// * `strategy_address` - The address of the strategy for which the report and fee will be updated.
/// * `strategy_invested_funds` - The current balance of funds invested in the strategy to update the report with.
///
/// # Returns
/// * `Result<Report, ContractError>` - Returns `Ok(report)` with the updated report on success, or
///   a `ContractError` if an error occurs during the update process, such as an overflow/underflow 
///   or invalid fee calculation.
pub fn update_report_and_lock_fees(
    e: &Env, 
    strategy_address: &Address,
    strategy_invested_funds: i128
) -> Result<Report, ContractError> {

    let mut report = get_report(&e, &strategy_address);
    report.report(strategy_invested_funds)?;
    report.lock_fee(get_vault_fee(&e))?;
    set_report(&e, &strategy_address, &report);

    Ok(report)
}

pub fn distribute_strategy_fees(e: &Env, strategy_address: &Address, access_control: &AccessControl, asset: &Address) -> Result<i128, ContractError> {
    let report = get_report(e, strategy_address);
    
    let defindex_fee = get_defindex_protocol_fee_rate(&e);
    let defindex_protocol_receiver = get_defindex_protocol_fee_receiver(&e)?;
    let vault_fee_receiver = access_control.get_fee_receiver()?;

    let fees_to_distribute = report.locked_fee;

    if fees_to_distribute > 0 {
        // Calculate shares for each receiver based on their fee proportion
        let numerator = fees_to_distribute.checked_mul(defindex_fee as i128).unwrap();
        let defindex_fee_amount = numerator.checked_div(SCALAR_BPS).unwrap();

        let vault_fee_amount = fees_to_distribute.checked_sub(defindex_fee_amount).ok_or(ContractError::Underflow)?;

        let mut report = unwind_from_strategy(
            &e,
            &strategy_address,
            &fees_to_distribute,
            &e.current_contract_address(),
        )?;

        // Transfer fees to the respective receivers
        let asset_client = TokenClient::new(&e, &asset);
        asset_client.transfer( &e.current_contract_address(), &vault_fee_receiver, &vault_fee_amount);
        asset_client.transfer( &e.current_contract_address(), &defindex_protocol_receiver, &defindex_fee_amount);

        report.locked_fee = 0;
        set_report(&e, &strategy_address, &report);

    }

    Ok(fees_to_distribute)
}