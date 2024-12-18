#![no_std]
use constants::MAX_BPS;
use report::Report;
use soroban_sdk::{
    contract, contractimpl, panic_with_error,
    token::TokenClient,
    Address, Env, Map, String, Vec,
};
use soroban_token_sdk::metadata::TokenMetadata;

mod access;
mod aggregator;
mod constants;
mod deposit;
mod error;
mod events;
mod funds;
mod interface;
mod investment;
mod models;
mod report;
mod storage;
mod strategies;
mod test;
mod token;
mod utils;

use access::{AccessControl, AccessControlTrait, RolesDataKey};
use aggregator::{internal_swap_exact_tokens_for_tokens, internal_swap_tokens_for_exact_tokens};
use deposit::process_deposit;
use funds::{fetch_current_idle_funds, fetch_current_invested_funds, fetch_total_managed_funds}; 
use interface::{AdminInterfaceTrait, VaultManagementTrait, VaultTrait};
use investment::{check_and_execute_investments, generate_investment_allocations};
use models::{
    Instruction, OptionalSwapDetailsExactIn,
    OptionalSwapDetailsExactOut, CurrentAssetInvestmentAllocation,
    ActionType, AssetInvestmentAllocation,
};
use storage::{
    extend_instance_ttl, get_assets, get_defindex_protocol_fee_rate, get_defindex_protocol_fee_receiver, get_report, get_vault_fee, set_asset, set_defindex_protocol_fee_rate, set_defindex_protocol_fee_receiver, set_factory, set_report, set_total_assets, set_vault_fee
};
use strategies::{
    get_strategy_asset, get_strategy_client,
    get_strategy_struct, invest_in_strategy, pause_strategy, unpause_strategy,
    unwind_from_strategy,
};
use token::{internal_burn, write_metadata};
use utils::{
    calculate_asset_amounts_per_vault_shares,
    check_initialized,
    check_nonnegative_amount,
};

use common::models::AssetStrategySet;
use defindex_strategy_core::DeFindexStrategyClient;

static MINIMUM_LIQUIDITY: i128 = 1000;

pub use error::ContractError;

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {
    /// Initializes the DeFindex Vault contract with the required parameters.
    ///
    /// This function sets the roles for manager, emergency manager, vault fee receiver, and manager.
    /// It also stores the list of assets to be managed by the vault, including strategies for each asset.
    ///
    /// # Arguments
    /// - `assets`: List of asset allocations for the vault, including strategies associated with each asset.
    /// - `manager`: Primary vault manager with permissions for vault control.
    /// - `emergency_manager`: Address with emergency access for emergency control over the vault.
    /// - `vault_fee_receiver`: Address designated to receive the vault fee receiver's portion of management fees.
    /// - `vault_fee`: Vault-specific fee percentage in basis points (typically set at 0-2% APR).
    /// - `defindex_protocol_receiver`: Address receiving DeFindex’s protocol-wide fee in basis points (0.5% APR).
    /// - `factory`: Factory contract address for deployment linkage.
    /// - `vault_name`: Name of the vault token to be displayed in metadata.
    /// - `vault_symbol`: Symbol representing the vault’s token.
    ///
    /// # Returns
    /// - `Result<(), ContractError>`: Returns `Ok(())` if initialization succeeds, or a `ContractError` if
    ///   any setup fails (e.g., strategy mismatch with asset).
    ///
    /// # Errors
    /// - `ContractError::AlreadyInitialized`: If the vault has already been initialized.
    /// - `ContractError::StrategyDoesNotSupportAsset`: If a strategy within an asset does not support the asset’s contract.
    ///
    fn __constructor(
        e: Env,
        assets: Vec<AssetStrategySet>,
        manager: Address,
        emergency_manager: Address,
        vault_fee_receiver: Address,
        vault_fee: u32,
        defindex_protocol_receiver: Address,
        defindex_protocol_rate: u32,
        factory: Address,
        vault_name: String,
        vault_symbol: String,
    ) {
        let access_control = AccessControl::new(&e);

        access_control.set_role(&RolesDataKey::EmergencyManager, &emergency_manager);
        access_control.set_role(&RolesDataKey::VaultFeeReceiver, &vault_fee_receiver);
        access_control.set_role(&RolesDataKey::Manager, &manager);

        // Set Vault Fee (in basis points)
        set_vault_fee(&e, &vault_fee);

        // Set Paltalabs Fee Receiver
        set_defindex_protocol_fee_receiver(&e, &defindex_protocol_receiver);
        set_defindex_protocol_fee_rate(&e, &defindex_protocol_rate);

        // Set the factory address
        set_factory(&e, &factory);

        // Store Assets Objects
        let total_assets = assets.len();

        // fails if the total assets is 0
        if total_assets == 0 {
            panic_with_error!(&e, ContractError::NoAssetAllocation);
        }

        set_total_assets(&e, total_assets as u32);
        for (i, asset) in assets.iter().enumerate() {
            // for every asset, we need to check that the list of strategies indeed support this asset

            for strategy in asset.strategies.iter() {
                let strategy_client = DeFindexStrategyClient::new(&e, &strategy.address);
                if strategy_client.asset() != asset.address {
                    panic_with_error!(&e, ContractError::StrategyDoesNotSupportAsset);
                }
            }
            set_asset(&e, i as u32, &asset);
        }

        // Metadata for the contract's token (unchanged)
        // TODO: Name should be concatenated with some other name giving when initializing. Check how soroswap pairs token are called.
        let decimal: u32 = 7;

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name: vault_name,
                symbol: vault_symbol,
            },
        );
    }

    /// Handles user deposits into the DeFindex Vault and optionally allocates investments automatically.
    ///
    /// This function processes a deposit by transferring each specified asset amount from the user's address to
    /// the vault, allocating assets according to the vault's defined strategy ratios, and minting vault shares that
    /// represent the user's proportional share in the vault. Additionally, if the `invest` parameter is set to `true`,
    /// the function will immediately generate and execute investment allocations based on the vault's strategy configuration.
    ///
    /// # Parameters
    /// * `e` - The current environment reference (`Env`), for access to the contract state and utilities.
    /// * `amounts_desired` - A vector specifying the user's intended deposit amounts for each asset.
    /// * `amounts_min` - A vector of minimum deposit amounts required for the transaction to proceed.
    /// * `from` - The address of the user making the deposit.
    /// * `invest` - A boolean flag indicating whether to immediately invest the deposited funds into the vault's strategies:
    ///     - `true`: Generate and execute investments after the deposit.
    ///     - `false`: Leave the deposited funds as idle assets in the vault.
    ///
    /// # Returns
    /// * `Result<(Vec<i128>, i128), ContractError>` - Returns the actual deposited `amounts` and `shares_to_mint` if successful,
    ///   otherwise a `ContractError`.
    ///
    /// # Function Flow
    /// 1. **Fee Collection**: Collects accrued fees before processing the deposit.
    /// 2. **Validation**: Checks that the lengths of `amounts_desired` and `amounts_min` match the vault's assets.
    /// 3. **Share Calculation**: Calculates `shares_to_mint` based on the vault's total managed funds and the deposit amount.
    /// 4. **Asset Transfer**: Transfers each specified amount from the user’s address to the vault as idle funds.
    /// 5. **Vault Shares Minting**: Mints vault shares for the user to represent their ownership in the vault.
    /// 6. **Investment Execution**: If `invest` is `true`, generates and executes the investment allocations for the deposited funds.
    ///     - Allocates funds across strategies proportionally to their current state.
    ///     - Executes the investment to transition idle funds into the vault's strategies.
    ///
    /// # Notes
    /// - For the first deposit, if the vault has only one asset, shares are calculated directly based on the deposit amount.
    /// - For multiple assets, the function delegates to `calculate_deposit_amounts_and_shares_to_mint`
    ///   for precise share computation.
    /// - An event is emitted to log the deposit, including the actual deposited amounts and minted shares.
    /// - If `invest` is `false`, deposited funds remain idle, allowing for manual investment at a later time.
    ///
    /// # Errors
    /// - Returns a `ContractError` if any validation or execution step fails.
    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
        invest: bool,
    ) -> Result<(Vec<i128>, i128), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;
        from.require_auth();

        let total_managed_funds = fetch_total_managed_funds(&e);

        let assets = get_assets(&e);

        let (amounts, shares_to_mint) =
            process_deposit(
                &e, 
                &assets, 
                &total_managed_funds,
                &amounts_desired, 
                &amounts_min, 
                &from)?;
        events::emit_deposit_event(&e, from, amounts.clone(), shares_to_mint.clone());

        if invest {
            let asset_investments = generate_investment_allocations(
                &e,
                &assets,
                &total_managed_funds,
                &amounts,
            )?;
            check_and_execute_investments(&e, &assets, &asset_investments)?;
        }
        Ok((amounts, shares_to_mint))

    }

    /// Handles the withdrawal process for a specified number of vault shares.
    ///
    /// This function performs the following steps:
    /// 1. Validates the environment and the inputs:
    ///    - Ensures the contract is initialized.
    ///    - Checks that the withdrawal amount (`withdraw_shares`) is non-negative.
    ///    - Verifies the authorization of the `from` address.
    /// 2. Collects applicable fees.
    /// 3. Calculates the proportionate withdrawal amounts for each asset based on the number of shares.
    /// 4. Burns the specified shares from the user's account.
    /// 5. Processes the withdrawal for each asset:
    ///    - First attempts to cover the withdrawal amount using idle funds.
    ///    - If idle funds are insufficient, unwinds investments from the associated strategies
    ///      to cover the remaining amount, accounting for rounding errors in the last strategy.
    /// 6. Transfers the withdrawn funds to the user's address (`from`).
    /// 7. Emits an event to record the withdrawal details.
    ///
    /// ## Parameters:
    /// - `e`: The contract environment (`Env`).
    /// - `withdraw_shares`: The number of vault shares to withdraw.
    /// - `from`: The address initiating the withdrawal.
    ///
    /// ## Returns:
    /// - A `Result` containing a vector of withdrawn amounts for each asset (`Vec<i128>`),
    ///   or a `ContractError` if the withdrawal fails.
    ///
    /// ## Errors:
    /// - `ContractError::AmountOverTotalSupply`: If the specified shares exceed the total supply.
    /// - `ContractError::ArithmeticError`: If any arithmetic operation fails during calculations.
    /// - `ContractError::WrongAmountsLength`: If there is a mismatch in asset allocation data.
    ///
    /// ## TODOs:
    /// - Implement minimum amounts for withdrawals to ensure compliance with potential restrictions.
    /// - Replace the returned vector with the original `asset_withdrawal_amounts` map for better structure.
    /// - avoid the usage of a Map, choose between using map or vector
    fn withdraw(
        e: Env,
        withdraw_shares: i128,
        from: Address,
    ) -> Result<Vec<i128>, ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;
        check_nonnegative_amount(withdraw_shares)?;
        from.require_auth();
    
        // Calculate the withdrawal amounts for each asset based on the share amounts
        let total_managed_funds = fetch_total_managed_funds(&e);

        let asset_withdrawal_amounts = calculate_asset_amounts_per_vault_shares(
            &e,
            withdraw_shares,
            &total_managed_funds,
        )?;
    
        // Burn the shares after calculating the withdrawal amounts
        // This will panic with error if the user does not have enough balance
        internal_burn(e.clone(), from.clone(), withdraw_shares);
    
        let assets = get_assets(&e); // Use assets for iteration order
        // Loop through each asset to handle the withdrawal
        let mut withdrawn_amounts: Vec<i128> = Vec::new(&e);

        for asset in assets.iter() { // Use assets instead of asset_withdrawal_amounts
            let asset_address = &asset.address;

            if let Some(requested_withdrawal_amount) = asset_withdrawal_amounts.get(asset_address.clone()) {
                let asset_allocation = total_managed_funds
                    .get(asset_address.clone())
                    .unwrap_or_else(|| panic_with_error!(&e, ContractError::WrongAmountsLength));

                let idle_funds = asset_allocation.idle_amount;

                if idle_funds >= requested_withdrawal_amount {
                    TokenClient::new(&e, asset_address).transfer(
                        &e.current_contract_address(),
                        &from,
                        &requested_withdrawal_amount,
                    );
                    withdrawn_amounts.push_back(requested_withdrawal_amount);
                } else {
                    let mut cumulative_amount_for_asset = idle_funds;
                    let remaining_amount_to_unwind = requested_withdrawal_amount
                        .checked_sub(idle_funds)
                        .unwrap();

                    let total_invested_amount = asset_allocation.invested_amount;

                    for (i, strategy_allocation) in asset_allocation.strategy_allocations.iter().enumerate() {
                        let strategy_amount_to_unwind: i128 = if i == (asset_allocation.strategy_allocations.len() as usize) - 1 {
                            requested_withdrawal_amount
                                .checked_sub(cumulative_amount_for_asset)
                                .unwrap()
                        } else {
                            remaining_amount_to_unwind
                                .checked_mul(strategy_allocation.amount)
                                .and_then(|result| result.checked_div(total_invested_amount))
                                .unwrap_or(0)
                        };

                        if strategy_amount_to_unwind > 0 {
                            unwind_from_strategy(&e, &strategy_allocation.strategy_address, &strategy_amount_to_unwind, &e.current_contract_address())?;
                            cumulative_amount_for_asset += strategy_amount_to_unwind;
                        }
                    }

                    TokenClient::new(&e, asset_address).transfer(
                        &e.current_contract_address(),
                        &from,
                        &cumulative_amount_for_asset,
                    );
                    withdrawn_amounts.push_back(cumulative_amount_for_asset);
                }
            } else {
                withdrawn_amounts.push_back(0); // No withdrawal for this asset
            }
        }

        
        // TODO: Add minimuim amounts for withdrawn_amounts
        // TODO: Return the asset_withdrawal_amounts Map instead of a vec
        events::emit_withdraw_event(&e, from, withdraw_shares, withdrawn_amounts.clone());
    
        Ok(withdrawn_amounts)
    }

    /// Executes an emergency withdrawal from a specific strategy.
    ///
    /// This function allows the emergency manager or manager to withdraw all assets from a particular strategy
    /// and store them as idle funds within the vault. It also pauses the strategy to prevent further use until
    /// unpaused.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `strategy_address` - The address of the strategy to withdraw from.
    /// * `caller` - The address initiating the emergency withdrawal (must be the manager or emergency manager).
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn emergency_withdraw(
        e: Env,
        strategy_address: Address,
        caller: Address,
    ) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        // Ensure the caller is the Manager or Emergency Manager
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(
            &[RolesDataKey::EmergencyManager, RolesDataKey::Manager],
            &caller,
        );

        // Find the strategy and its associated asset
        let asset = get_strategy_asset(&e, &strategy_address)?;
        // This ensures that the vault has this strategy in its list of assets
        let strategy = get_strategy_struct(&strategy_address, &asset)?;

        // Withdraw all assets from the strategy
        let strategy_client = get_strategy_client(&e, strategy.address.clone());
        let strategy_balance = strategy_client.balance(&e.current_contract_address());

        if strategy_balance > 0 {
            let mut report = unwind_from_strategy(&e, &strategy_address, &strategy_balance, &e.current_contract_address())?;
            report.reset();
            set_report(&e, &strategy_address, &report);
            //TODO: Should we check if the idle funds are corresponding to the strategy balance withdrawed?
        }

        // Pause the strategy
        pause_strategy(&e, strategy_address.clone())?;

        events::emit_emergency_withdraw_event(&e, caller, strategy_address, strategy_balance);
        Ok(())
    }

    /// Pauses a strategy to prevent it from being used in the vault.
    ///
    /// This function pauses a strategy by setting its `paused` field to `true`. Only the manager or emergency
    /// manager can pause a strategy.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `strategy_address` - The address of the strategy to pause.
    /// * `caller` - The address initiating the pause (must be the manager or emergency manager).
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn pause_strategy(
        e: Env,
        strategy_address: Address,
        caller: Address,
    ) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        // Ensure the caller is the Manager or Emergency Manager
        // TODO: Should check if the strategy has any amount invested on it, and return an error if it has, should we let the manager to pause a strategy with funds invested?
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(
            &[RolesDataKey::EmergencyManager, RolesDataKey::Manager],
            &caller,
        );

        pause_strategy(&e, strategy_address.clone())?;
        events::emit_strategy_paused_event(&e, strategy_address, caller);
        Ok(())
    }

    /// Unpauses a previously paused strategy.
    ///
    /// This function unpauses a strategy by setting its `paused` field to `false`, allowing it to be used
    /// again in the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `strategy_address` - The address of the strategy to unpause.
    /// * `caller` - The address initiating the unpause (must be the manager or emergency manager).
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn unpause_strategy(
        e: Env,
        strategy_address: Address,
        caller: Address,
    ) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        // Ensure the caller is the Manager or Emergency Manager
        let access_control = AccessControl::new(&e);
        access_control.require_any_role(
            &[RolesDataKey::EmergencyManager, RolesDataKey::Manager],
            &caller,
        );

        unpause_strategy(&e, strategy_address.clone())?;
        events::emit_strategy_unpaused_event(&e, strategy_address, caller);
        Ok(())
    }

    /// Retrieves the list of assets managed by the DeFindex Vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Vec<AssetStrategySet>` - A vector of `AssetStrategySet` structs representing the assets managed by the vault.
    fn get_assets(e: Env) -> Vec<AssetStrategySet> {
        extend_instance_ttl(&e);
        get_assets(&e)
    }

    /// Returns the total managed funds of the vault, including both invested and idle funds.
    ///
    /// This function provides a map where the key is the asset address and the value is the total amount
    /// of that asset being managed by the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Map<Address, i128>` - A map of asset addresses to their total managed amounts.
    fn fetch_total_managed_funds(e: &Env) -> Map<Address, CurrentAssetInvestmentAllocation> {
        extend_instance_ttl(&e);
        fetch_total_managed_funds(e)
    }

    /// Returns the current invested funds, representing the total assets allocated to strategies.
    ///
    /// This function provides a map where the key is the asset address and the value is the total amount
    /// of that asset currently invested in various strategies.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Map<Address, i128>` - A map of asset addresses to their total invested amounts.
    fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128> {
        extend_instance_ttl(&e);
        fetch_current_invested_funds(e)
    }

    /// Returns the current idle funds, representing the total assets held directly by the vault (not invested).
    ///
    /// This function provides a map where the key is the asset address and the value is the total amount
    /// of that asset held as idle funds within the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Map<Address, i128>` - A map of asset addresses to their total idle amounts.
    fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128> {
        extend_instance_ttl(&e);
        fetch_current_idle_funds(e)
    }

    // Calculates the corresponding amounts of each asset per a given number of vault shares.
    /// This function extends the contract's time-to-live and calculates how much of each asset corresponds 
    /// per the provided number of vault shares (`vault_shares`). It provides proportional allocations for each asset 
    /// in the vault relative to the specified shares.
    ///
    /// # Arguments
    /// * `e` - The current environment reference.
    /// * `vault_shares` - The number of vault shares for which the corresponding asset amounts are calculated.
    ///
    /// # Returns
    /// * `Map<Address, i128>` - A map containing each asset address and its corresponding proportional amount.
    fn get_asset_amounts_per_shares(e: Env, vault_shares: i128) -> Result<Map<Address, i128>, ContractError> {
        extend_instance_ttl(&e);

        let total_managed_funds = fetch_total_managed_funds(&e);
        Ok(calculate_asset_amounts_per_vault_shares(&e, vault_shares, &total_managed_funds)?)
    }

    /// Retrieves the current fee rates for the vault and the DeFindex protocol.
    ///
    /// This function returns the fee rates for both the vault and the DeFindex protocol.
    ///
    /// # Arguments
    /// * `e` - The environment.
    ///
    /// # Returns
    /// * `(u32, u32)` - A tuple containing:
    ///     - The vault fee rate as a percentage in basis points.
    ///     - The DeFindex protocol fee rate as a percentage in basis points.
    ///
    fn get_fees(e: Env) -> (u32, u32) {
        extend_instance_ttl(&e);
        let defindex_protocol_fee = get_defindex_protocol_fee_rate(&e);
        let vault_fee = get_vault_fee(&e);
        (vault_fee, defindex_protocol_fee)
    }

    fn report(e: Env) -> Result<Vec<Report>, ContractError> {
        extend_instance_ttl(&e);

        // Get all assets and their strategies
        let assets = get_assets(&e);
        let mut reports: Vec<Report> = Vec::new(&e);

        // Loop through each asset and its strategies to report the balances
        for asset in assets.iter() {
            for strategy in asset.strategies.iter() {
                let strategy_client = get_strategy_client(&e, strategy.address.clone());
                let strategy_invested_funds = strategy_client.balance(&e.current_contract_address());

                let report_result = report::report(&e, &strategy.address, &strategy_invested_funds);
                reports.push_back(report_result);
            }
        }

        Ok(reports)
    }
}

#[contractimpl]
impl AdminInterfaceTrait for DeFindexVault {
    /// Sets the fee receiver for the vault.
    ///
    /// This function allows the manager or emergency manager to set a new fee receiver address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `caller` - The address initiating the change (must be the manager or emergency manager).
    /// * `vault_fee_receiver` - The new fee receiver address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_fee_receiver(e: Env, caller: Address, new_fee_receiver: Address) {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.set_fee_receiver(&caller, &new_fee_receiver);

        events::emit_fee_receiver_changed_event(&e, new_fee_receiver, caller);
    }

    /// Retrieves the current fee receiver address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The fee receiver address if successful, otherwise returns a ContractError.
    fn get_fee_receiver(e: Env) -> Result<Address, ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.get_fee_receiver()
    }

    /// Sets the manager for the vault.
    ///
    /// This function allows the current manager or emergency manager to set a new manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `manager` - The new manager address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_manager(e: Env, manager: Address) {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.set_manager(&manager);

        events::emit_manager_changed_event(&e, manager);
    }

    /// Retrieves the current manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The manager address if successful, otherwise returns a ContractError.
    fn get_manager(e: Env) -> Result<Address, ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.get_manager()
    }

    /// Sets the emergency manager for the vault.
    ///
    /// This function allows the current manager or emergency manager to set a new emergency manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `emergency_manager` - The new emergency manager address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_emergency_manager(e: Env, emergency_manager: Address) {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.set_emergency_manager(&emergency_manager);

        events::emit_emergency_manager_changed_event(&e, emergency_manager);
    }

    /// Retrieves the current emergency manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The emergency manager address if successful, otherwise returns a ContractError.
    fn get_emergency_manager(e: Env) -> Result<Address, ContractError> {
        let access_control = AccessControl::new(&e);
        access_control.get_emergency_manager()
    }
}

#[contractimpl]
impl VaultManagementTrait for DeFindexVault {
    /// Executes the investment of the vault's idle funds based on the specified asset allocations.
    /// This function allows partial investments by providing an optional allocation for each asset,
    /// and it ensures proper authorization and validation checks before proceeding with investments.
    ///
    /// # Arguments
    /// * `e` - The current environment reference.
    /// * `asset_investments` - A vector of optional `AssetInvestmentAllocation` structures, where each element
    ///   represents an allocation for a specific asset. The vector must match the number of vault assets in length.
    ///
    /// # Returns
    /// * `Result<(), ContractError>` - Returns `Ok(())` if the investments are successful or a `ContractError`
    ///   if any issue occurs during validation or execution.
    ///
    /// # Function Flow
    /// 1. **Extend Instance TTL**: Extends the contract instance's time-to-live to keep the instance active.
    /// 2. **Check Initialization**: Verifies that the vault is properly initialized before proceeding.
    /// 3. **Access Control**: Ensures the caller has the `Manager` role required to initiate investments.
    /// 4. **Asset Count Validation**: Verifies that the length of the `asset_investments` vector matches
    ///    the number of assets managed by the vault. If they don't match, a `WrongInvestmentLength` error is returned.
    /// 5. **Investment Execution**: Calls the `check_and_execute_investments` function to perform the investment
    ///    after validating the inputs and ensuring correct execution flows for each asset allocation.
    ///
    /// # Errors
    /// * Returns `ContractError::WrongInvestmentLength` if the length of `asset_investments` does not match the vault assets.
    /// * Returns `ContractError` if access control validation fails or if investment execution encounters an issue.
    ///
    /// # Security
    /// - Only addresses with the `Manager` role can call this function, ensuring restricted access to managing investments.
    fn invest(
        e: Env,
        asset_investments: Vec<Option<AssetInvestmentAllocation>>,
    ) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        // Access control: ensure caller has the required manager role
        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        let assets = get_assets(&e);

        // Ensure the length of `asset_investments` matches the number of vault assets
        if asset_investments.len() != assets.len() {
            panic_with_error!(&e, ContractError::WrongInvestmentLength);
        }

        // Check and execute investments for each asset allocation
        check_and_execute_investments(
            &e, 
            &assets, 
            &asset_investments)?;

        Ok(())
    }

    /// Rebalances the vault by executing a series of instructions.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `instructions` - A vector of `Instruction` structs representing actions (withdraw, invest, swap, zapper) to be taken.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn rebalance(e: Env, instructions: Vec<Instruction>) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        if instructions.is_empty() {
            panic_with_error!(&e, ContractError::NoInstructions);
        }

        for instruction in instructions.iter() {
            match instruction.action {
                ActionType::Withdraw => match (&instruction.strategy, &instruction.amount) {
                    (Some(strategy_address), Some(amount)) => {
                        unwind_from_strategy(&e, strategy_address, amount, &e.current_contract_address())?;
                    }
                    _ => return Err(ContractError::MissingInstructionData),
                },
                ActionType::Invest => match (&instruction.strategy, &instruction.amount) {
                    (Some(strategy_address), Some(amount)) => {
                        let asset_address = get_strategy_asset(&e, strategy_address)?;
                        invest_in_strategy(&e, &asset_address.address, strategy_address, amount)?;
                    }
                    _ => return Err(ContractError::MissingInstructionData),
                },
                ActionType::SwapExactIn => match &instruction.swap_details_exact_in {
                    OptionalSwapDetailsExactIn::Some(swap_details) => {
                        internal_swap_exact_tokens_for_tokens(
                            &e,
                            &swap_details.token_in,
                            &swap_details.token_out,
                            &swap_details.amount_in,
                            &swap_details.amount_out_min,
                            &swap_details.distribution,
                            &swap_details.deadline,
                        )?;
                    }
                    _ => return Err(ContractError::MissingInstructionData),
                },
                ActionType::SwapExactOut => match &instruction.swap_details_exact_out {
                    OptionalSwapDetailsExactOut::Some(swap_details) => {
                        internal_swap_tokens_for_exact_tokens(
                            &e,
                            &swap_details.token_in,
                            &swap_details.token_out,
                            &swap_details.amount_out,
                            &swap_details.amount_in_max,
                            &swap_details.distribution,
                            &swap_details.deadline,
                        )?;
                    }
                    _ => return Err(ContractError::MissingInstructionData),
                },
                ActionType::Zapper => {
                    // TODO: Implement Zapper instructions
                }
            }
        }

        Ok(())
    }

    /// Locks fees for all assets and their strategies.
    ///
    /// Iterates through each asset and its strategies, locking fees based on `new_fee_bps` or the default vault fee.
    ///
    /// # Arguments
    /// * `e` - The environment reference.
    /// * `new_fee_bps` - Optional fee basis points to override the default.
    ///
    /// # Returns
    /// * `Result<Vec<(Address, i128)>, ContractError>` - A vector of tuples with strategy addresses and locked fee amounts in their underlying_asset.
    fn lock_fees(e: Env, new_fee_bps: Option<u32>) -> Result<Vec<Report>, ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        // If a new fee basis points is provided, set the new vault fee
        if let Some(fee_bps) = new_fee_bps {
            set_vault_fee(&e, &fee_bps);
        }

        // Get the current vault fee
        let current_vault_fee = get_vault_fee(&e);

        // Get all assets and their strategies
        let assets = get_assets(&e);
        let mut reports: Vec<Report> = Vec::new(&e);

        // Loop through each asset and its strategies to lock the fees
        for asset in assets.iter() {
            for strategy in asset.strategies.iter() {
                let mut report = get_report(&e, &strategy.address);
                if report.gains_or_losses > 0 {
                    report.lock_fee(current_vault_fee);
                    set_report(&e, &strategy.address, &report);
                    reports.push_back(report);
                }
            }
        };

        Ok(reports)
    }

    /// Releases locked fees for a specific strategy.
    ///
    /// # Arguments
    /// * `e` - The environment reference.
    /// * `strategy` - The address of the strategy for which to release fees.
    /// * `amount` - The amount of fees to release.
    ///
    /// # Returns
    /// * `Result<Report, ContractError>` - A report of the released fees or a `ContractError` if the operation fails.
    fn release_fees(e: Env, strategy: Address, amount: i128) -> Result<Report, ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        let mut report = get_report(&e, &strategy);

        report.release_fee(&e, amount);
        set_report(&e, &strategy, &report);
        Ok(report)
    }

    /// Distributes the locked fees for all assets and their strategies.
    ///
    /// This function iterates through each asset and its strategies, calculating the fees to be distributed
    /// to the vault fee receiver and the DeFindex protocol fee receiver based on their respective fee rates.
    /// It ensures proper authorization and validation checks before proceeding with the distribution.
    ///
    /// # Arguments
    /// * `e` - The environment reference.
    ///
    /// # Returns
    /// * `Result<Vec<(Address, i128)>, ContractError>` - A vector of tuples with asset addresses and the total distributed fee amounts.
    fn distribute_fees(e: Env) -> Result<Vec<(Address, i128)>, ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        // Get all assets and their strategies
        let assets = get_assets(&e);

        let vault_fee_receiver = access_control.get_fee_receiver()?;
        let defindex_protocol_receiver = get_defindex_protocol_fee_receiver(&e);
        let defindex_fee = get_defindex_protocol_fee_rate(&e);

        let mut distributed_fees: Vec<(Address, i128)> = Vec::new(&e);

        // Loop through each asset and its strategies to lock the fees
        for asset in assets.iter() {
            let mut total_fees_distributed: i128 = 0;

            for strategy in asset.strategies.iter() {
                let mut report = get_report(&e, &strategy.address);

                if report.locked_fee > 0 {
                    // Calculate shares for each receiver based on their fee proportion
                    let numerator = report.locked_fee
                        .checked_mul(defindex_fee as i128)
                        .unwrap();
                    let defindex_fee_amount = numerator.checked_div(MAX_BPS).unwrap();

                    let vault_fee_amount = report.locked_fee - defindex_fee_amount;

                    report.prev_balance = report.prev_balance - report.locked_fee;

                    unwind_from_strategy(&e, &strategy.address, &defindex_fee_amount, &defindex_protocol_receiver)?;
                    unwind_from_strategy(&e, &strategy.address, &vault_fee_amount, &vault_fee_receiver)?;
                    total_fees_distributed += report.locked_fee;
                    report.locked_fee = 0;
                    set_report(&e, &strategy.address, &report);
                }
            }

            if total_fees_distributed > 0 {
                distributed_fees.push_back((asset.address.clone(), total_fees_distributed));
            }
        };

        events::emit_fees_distributed_event(&e, distributed_fees.clone());

        Ok(distributed_fees)
    }
}
