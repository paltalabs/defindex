#![no_std]
use constants::{MIN_WITHDRAW_AMOUNT, ONE_DAY_IN_SECONDS};
use report::Report;
use soroban_sdk::{contract, contractimpl, panic_with_error, token::TokenClient, vec, Address, BytesN, Env, IntoVal, Map, String, Val, Vec
};
use soroban_token_sdk::metadata::TokenMetadata;

mod access;
mod router;
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
use router::{internal_swap_exact_tokens_for_tokens, internal_swap_tokens_for_exact_tokens};
use deposit::process_deposit;
use funds::{fetch_current_idle_funds, fetch_current_invested_funds, fetch_total_managed_funds};
use interface::{AdminInterfaceTrait, VaultManagementTrait, VaultTrait};
use investment::{check_and_execute_investments, generate_investment_allocations};
use models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, Instruction, StrategyAllocation};
use storage::{
    extend_instance_ttl, get_assets, get_defindex_protocol_fee_rate,
    get_report, get_vault_fee, set_asset,
    set_defindex_protocol_fee_rate, set_defindex_protocol_fee_receiver, set_factory, set_report,
    set_soroswap_router, set_total_assets, set_vault_fee, set_is_upgradable
};
use strategies::{
    get_strategy_asset, get_strategy_client, get_strategy_struct, invest_in_strategy,
    pause_strategy, unpause_strategy, unwind_from_strategy,
};
use token::{internal_burn, write_metadata};
use utils::{
    calculate_asset_amounts_per_vault_shares, check_initialized, check_min_amount, check_nonnegative_amount
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
    /// - `soroswap_router`: Address of the Soroswap router
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
        roles: Map<u32, Address>,
        vault_fee: u32,
        defindex_protocol_receiver: Address,
        defindex_protocol_rate: u32,
        factory: Address,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
    ) {
        let access_control = AccessControl::new(&e);

        access_control.set_role(&RolesDataKey::EmergencyManager, &roles.get(RolesDataKey::EmergencyManager as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        access_control.set_role(&RolesDataKey::VaultFeeReceiver, &roles.get(RolesDataKey::VaultFeeReceiver as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        access_control.set_role(&RolesDataKey::Manager, &roles.get(RolesDataKey::Manager as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        access_control.set_role(&RolesDataKey::RebalanceManager, &roles.get(RolesDataKey::RebalanceManager as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        
        let vault_name = name_symbol.get(String::from_str(&e, "name")).unwrap_or_else(|| panic_with_error!(&e, ContractError::MetadataIncomplete));
        let vault_symbol = name_symbol.get(String::from_str(&e, "symbol")).unwrap_or_else(|| panic_with_error!(&e, ContractError::MetadataIncomplete));

        set_vault_fee(&e, &vault_fee);

        set_defindex_protocol_fee_receiver(&e, &defindex_protocol_receiver);
        set_defindex_protocol_fee_rate(&e, &defindex_protocol_rate);

        set_factory(&e, &factory);
        set_is_upgradable(&e, &upgradable);

        set_soroswap_router(&e, &soroswap_router);

        let total_assets = assets.len();

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
    ) -> Result<(Vec<i128>, i128, Option<Vec<Option<AssetInvestmentAllocation>>>), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;
        from.require_auth();

        let total_managed_funds = fetch_total_managed_funds(&e, false);

        let assets = get_assets(&e);

        let (amounts, shares_to_mint) = process_deposit(
            &e,
            &assets,
            &total_managed_funds,
            &amounts_desired,
            &amounts_min,
            &from,
        )?;
        events::emit_deposit_event(&e, from, amounts.clone(), shares_to_mint.clone());

        let asset_investments = if invest {
            let allocations = generate_investment_allocations(&e, &assets, &total_managed_funds, &amounts)?;
            check_and_execute_investments(&e, &assets, &allocations)?;
            Some(allocations)
        } else {
            None
        };
        Ok((amounts, shares_to_mint, asset_investments))
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
    fn withdraw(e: Env, withdraw_shares: i128, from: Address) -> Result<Vec<i128>, ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;
        check_nonnegative_amount(withdraw_shares)?;
        from.require_auth();

        check_min_amount(withdraw_shares, MIN_WITHDRAW_AMOUNT)?;
        // Calculate the withdrawal amounts for each asset based on the share amounts
        let total_managed_funds = fetch_total_managed_funds(&e, true);

        let asset_withdrawal_amounts =
            calculate_asset_amounts_per_vault_shares(&e, withdraw_shares, &total_managed_funds)?;

        // Burn the shares after calculating the withdrawal amounts
        // This will panic with error if the user does not have enough balance
        internal_burn(e.clone(), from.clone(), withdraw_shares);

        let assets = get_assets(&e); // Use assets for iteration order
                                     // Loop through each asset to handle the withdrawal
        let mut withdrawn_amounts: Vec<i128> = Vec::new(&e);

        for asset in assets.iter() {
            // Use assets instead of asset_withdrawal_amounts
            let asset_address = &asset.address;

            if let Some(requested_withdrawal_amount) =
                asset_withdrawal_amounts.get(asset_address.clone())
            {
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
                    let remaining_amount_to_unwind =
                        requested_withdrawal_amount.checked_sub(idle_funds).unwrap();

                    let total_invested_amount = asset_allocation.invested_amount;

                    for (i, strategy_allocation) in
                        asset_allocation.strategy_allocations.iter().enumerate()
                    {
                        let strategy_amount_to_unwind: i128 =
                            if i == (asset_allocation.strategy_allocations.len() as usize) - 1 {
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
                            unwind_from_strategy(
                                &e,
                                &strategy_allocation.strategy_address,
                                &strategy_amount_to_unwind,
                                &e.current_contract_address(),
                            )?;
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
    fn rescue(
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

        let distribution_result = report::distribute_strategy_fees(&e, &strategy.address, &access_control)?;
        if distribution_result > 0 {
            let mut distributed_fees: Vec<(Address, i128)> = Vec::new(&e);
            distributed_fees.push_back((asset.address.clone(), distribution_result));
            events::emit_fees_distributed_event(&e, distributed_fees.clone());
        }

        // Withdraw all assets from the strategy
        let strategy_client = get_strategy_client(&e, strategy.address.clone());
        let strategy_balance = strategy_client.balance(&e.current_contract_address());

        if strategy_balance > 0 {
            let mut report = unwind_from_strategy(
                &e,
                &strategy_address,
                &strategy_balance,
                &e.current_contract_address(),
            )?;
            report.reset();
            set_report(&e, &strategy_address, &report);
            //TODO: Should we check if the idle funds are corresponding to the strategy balance withdrawed?
        }

        // Pause the strategy
        pause_strategy(&e, strategy_address.clone())?;

        events::emit_rescue_event(&e, caller, strategy_address, strategy_balance);
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
        fetch_total_managed_funds(e, false)
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
        fetch_current_invested_funds(e, false)
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
    fn get_asset_amounts_per_shares(
        e: Env,
        vault_shares: i128,
    ) -> Result<Map<Address, i128>, ContractError> {
        extend_instance_ttl(&e);

        let total_managed_funds = fetch_total_managed_funds(&e, true);
        Ok(calculate_asset_amounts_per_vault_shares(
            &e,
            vault_shares,
            &total_managed_funds,
        )?)
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
                let strategy_invested_funds =
                    strategy_client.balance(&e.current_contract_address());

                let mut report = get_report(&e, &strategy.address);
                report.report(strategy_invested_funds);
                set_report(&e, &strategy.address, &report);

                reports.push_back(report);
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

    // Sets the manager queue for the vault config.
    //
    // This function allows the current manager to add to queue a new manager for the vault.
    //
    // # Arguments:
    // * `e` - The environment.
    // * `manager` - The new manager address.
    //
    // # Returns:
    // * `Result<Address, ContractError>` - The manager address if successful, otherwise returns a ContractError.
    fn queue_manager(e: Env, manager: Address) -> Result<Address, ContractError> {
        extend_instance_ttl(&e);
        
        let current_timestamp:u64 = e.ledger().timestamp();
        let mut manager_data: Map<u64, Address> = Map::new(&e);
        manager_data.set(current_timestamp, manager.clone());

        let access_control = AccessControl::new(&e);
        access_control.queue_manager(&manager_data);
        events::emit_queued_manager_event(&e, manager_data);
        Ok(manager)
    }

    // Retrieves the manager queue for the vault config.
    //
    // This function allows the anyone to retrieve the manager queue for the vault.
    //
    // # Arguments:
    // * `e` - The environment.
    //
    // # Returns:
    // * `Result<Address, ContractError>` - The manager address if successful, otherwise returns a ContractError.
    fn get_queued_manager(e: Env) -> Result<Address, ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        let queued_manager = access_control.get_queued_manager().values().first().unwrap();

        Ok(queued_manager)
    }

    // clear the manager queue for the vault config.
    // This function allows the current manager clear the manager queue for the vault.
    //
    // # Arguments:
    // * `e` - The environment.
    //
    // # Returns:
    // * `()` - No return value.
    fn clear_queue(e: Env) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.clear_queued_manager();
        let current_timestamp:u64 = e.ledger().timestamp();
        events::emit_clear_manager_queue_event(&e, current_timestamp);
        Ok(())
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
    fn set_manager(e: Env) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        let current_timestamp = e.ledger().timestamp();
        let manager_address = access_control.get_queued_manager().values().first().unwrap();
        let queued_timestamp = access_control.get_queued_manager().keys().first().unwrap();
        let seven_days: u64 = ONE_DAY_IN_SECONDS * 7u64;
        if (current_timestamp - queued_timestamp) < (seven_days) {
            panic_with_error!(&e, ContractError::SetManagerBeforeTime);
        }
        access_control.set_manager();
        events::emit_manager_changed_event(&e, manager_address);
        Ok(())
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

    /// Sets the rebalance manager for the vault.
    ///
    /// This function allows the current manager to set a new rebalance manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `new_rebalance_manager` - The new rebalance manager address.
    ///
    /// # Returns:
    /// * `()` - No return value.
    fn set_rebalance_manager(e: Env, new_rebalance_manager: Address) {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.set_rebalance_manager(&new_rebalance_manager);

        events::emit_rebalance_manager_changed_event(&e, new_rebalance_manager);
    }

    /// Retrieves the current rebalance manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The rebalance manager address if successful, otherwise returns a ContractError.
    fn get_rebalance_manager(e: Env) -> Result<Address, ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.get_rebalance_manager()
    }

    /// Upgrades the contract with new WebAssembly (WASM) code.
    ///
    /// This function updates the contract with new WASM code provided by the `new_wasm_hash`.
    ///
    /// # Arguments
    ///
    /// * `e` - The runtime environment.
    /// * `new_wasm_hash` - The hash of the new WASM code to upgrade the contract to.
    ///
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        if !storage::is_upgradable(&e) {
            return Err(ContractError::NotUpgradable);
        }
        
        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);
        
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}

#[contractimpl]
impl VaultManagementTrait for DeFindexVault {
    
    fn rebalance(e: Env, caller: Address, instructions: Vec<Instruction>) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        let access_control = AccessControl::new(&e);
        access_control.require_any_role(
            &[RolesDataKey::RebalanceManager, RolesDataKey::Manager],
            &caller,
        );

        if instructions.is_empty() {
            panic_with_error!(&e, ContractError::NoInstructions);
        }

        for instruction in instructions.iter() {
            match instruction {
                Instruction::Unwind(strategy_address, amount) => {
                    let report = unwind_from_strategy(
                        &e,
                        &strategy_address,
                        &amount,
                        &e.current_contract_address(),
                    )?;
                    let call_params = vec![&e, (strategy_address, amount, e.current_contract_address())];
                    events::emit_rebalance_unwind_event(&e, call_params, report);
                }
                Instruction::Invest(strategy_address, amount) => {
                    let asset_address = get_strategy_asset(&e, &strategy_address)?;
                    let report = invest_in_strategy(&e, &asset_address.address, &strategy_address, &amount)?;
                    let call_params = AssetInvestmentAllocation {
                        asset: asset_address.address.clone(),
                        strategy_allocations: vec![&e, Some(StrategyAllocation {
                            strategy_address: strategy_address.clone(),
                            amount: amount.clone(),
                        })],
                    };
                    events::emit_rebalance_invest_event(&e, vec![&e, call_params], report);
                }
                Instruction::SwapExactIn(
                    token_in,
                    token_out,
                    amount_in,
                    amount_out_min,
                    deadline,
                ) => {
                    internal_swap_exact_tokens_for_tokens(
                        &e,
                        &token_in,
                        &token_out,
                        &amount_in,
                        &amount_out_min,
                        &deadline,
                    )?;
                    let swap_args: Vec<Val> = vec![
                        &e,
                        amount_in.into_val(&e),
                        amount_out_min.into_val(&e),
                        vec![&e, token_in.to_val(), token_out.to_val()].into_val(&e), // path
                        e.current_contract_address().to_val(),
                        deadline.into_val(&e),
                    ];
                    events::emit_rebalance_swap_exact_in_event(&e, swap_args);
                }
                Instruction::SwapExactOut(
                    token_in,
                    token_out,
                    amount_out,
                    amount_in_max,
                    deadline,
                ) => {
                    internal_swap_tokens_for_exact_tokens(
                        &e,
                        &token_in,
                        &token_out,
                        &amount_out,
                        &amount_in_max,
                        &deadline,
                    )?;
                    let swap_args: Vec<Val> = vec![
                        &e,
                        amount_out.into_val(&e),
                        amount_in_max.into_val(&e),
                        vec![&e, token_in.to_val(), token_out.to_val()].into_val(&e), // path
                        e.current_contract_address().to_val(),
                        deadline.into_val(&e),
                    ];
                    events::emit_rebalance_swap_exact_out_event(&e, swap_args);
                } // Zapper instruction is omitted for now
                  // Instruction::Zapper(instructions) => {
                  //     // TODO: Implement Zapper instructions
                  // }
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
                }
                reports.push_back(report);
            }
        }

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

        let mut distributed_fees: Vec<(Address, i128)> = Vec::new(&e);

        // Loop through each asset and its strategies to lock the fees
        for asset in assets.iter() {
            let mut total_fees_distributed: i128 = 0;

            for strategy in asset.strategies.iter() {
                total_fees_distributed += report::distribute_strategy_fees(&e, &strategy.address, &access_control)?;
            }

            if total_fees_distributed > 0 {
                distributed_fees.push_back((asset.address.clone(), total_fees_distributed));
            }
        }

        events::emit_fees_distributed_event(&e, distributed_fees.clone());

        Ok(distributed_fees)
    }
}
