#![no_std]
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
use funds::{fetch_strategy_invested_funds, fetch_total_managed_funds};
use interface::{AdminInterfaceTrait, VaultManagementTrait, VaultTrait};
use investment::generate_investment_allocations;
use models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, Instruction, StrategyAllocation};
use storage::{
    extend_instance_ttl, get_assets, get_defindex_protocol_fee_rate,
    get_report, get_vault_fee, set_asset,
    set_defindex_protocol_fee_rate, set_defindex_protocol_fee_receiver, set_report,
    set_soroswap_router, set_total_assets, set_vault_fee, set_is_upgradable
};
use strategies::{
    get_strategy_asset, get_strategy_client, get_strategy_struct, invest_in_strategy,
    pause_strategy, unpause_strategy, unwind_from_strategy,
};
use token::{internal_burn, write_metadata};
use utils::{
    calculate_asset_amounts_per_vault_shares, validate_amount, validate_assets
};

use common::{models::AssetStrategySet, utils::StringExtensions};
use defindex_strategy_core::DeFindexStrategyClient;

use crate::token::VaultToken;

static MINIMUM_LIQUIDITY: i128 = 1000;

pub use error::ContractError;

#[contract]
pub struct DeFindexVault;

#[contractimpl]
impl VaultTrait for DeFindexVault {

    /// Initializes the DeFindex Vault contract with the required parameters.
    ///
    /// # Arguments
    /// * `e` - The environment reference.
    /// * `assets` - List of asset allocations for the vault, including strategies for each asset.
    /// * `roles` - Map of role IDs to addresses containing:
    ///   - Emergency Manager: For emergency control
    ///   - Vault Fee Receiver: For receiving vault fees
    ///   - Manager: For primary vault control
    ///   - Rebalance Manager: For rebalancing operations
    /// * `vault_fee` - Vault-specific fee in basis points (0_2000 for 0.20%)
    /// * `defindex_protocol_receiver` - Address receiving protocol fees
    /// * `defindex_protocol_rate` - Protocol fee rate in basis points (0-9000 for 0-90%)
    /// * `soroswap_router` - Soroswap router address
    /// * `name_symbol` - Map containing:
    ///   - "name": Vault token name
    ///   - "symbol": Vault token symbol
    /// * `upgradable` - Boolean flag for contract upgradeability
    ///
    /// # Function Flow
    /// 1. **Role Assignment**:
    ///    - Sets Emergency Manager
    ///    - Sets Vault Fee Receiver
    ///    - Sets Manager
    ///    - Sets Rebalance Manager
    ///
    /// 2. **Fee Configuration**:
    ///    - Sets vault fee rate
    ///    - Sets protocol fee receiver
    ///    - Validates and sets protocol fee rate
    ///
    /// 3. **Contract Setup**:
    ///    - Sets factory address
    ///    - Sets upgradeability status
    ///    - Sets Soroswap router
    ///
    /// 4. **Asset Validation & Setup**:
    ///    - Validates asset list is not empty
    ///    - Stores total asset count
    ///    - For each asset:
    ///      - Validates strategy compatibility
    ///      - Stores asset configuration
    ///
    /// 5. **Token Initialization**:
    ///    - Sets token decimals (7)
    ///    - Sets token name and symbol
    ///
    /// # Errors
    /// * `ContractError::RolesIncomplete` - If required roles are missing
    /// * `ContractError::MetadataIncomplete` - If name or symbol is missing
    /// * `ContractError::MaximumFeeExceeded` - If protocol fee > 9000 basis points
    /// * `ContractError::NoAssetAllocation` - If assets vector is empty
    /// * `ContractError::StrategyDoesNotSupportAsset` - If strategy validation fails
    ///
    fn __constructor(
        e: Env,
        assets: Vec<AssetStrategySet>,
        roles: Map<u32, Address>,
        vault_fee: u32,
        defindex_protocol_receiver: Address,
        defindex_protocol_rate: u32,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
    ) {
        let access_control = AccessControl::new(&e);

        access_control.set_role(&RolesDataKey::EmergencyManager, &roles.get(RolesDataKey::EmergencyManager as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        access_control.set_role(&RolesDataKey::VaultFeeReceiver, &roles.get(RolesDataKey::VaultFeeReceiver as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        access_control.set_role(&RolesDataKey::Manager, &roles.get(RolesDataKey::Manager as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));
        access_control.set_role(&RolesDataKey::RebalanceManager, &roles.get(RolesDataKey::RebalanceManager as u32).unwrap_or_else(|| panic_with_error!(&e, ContractError::RolesIncomplete)));

        let prefix = String::from_str(&e, "DeFindex-Vault-");
        let vault_name = name_symbol.get(String::from_str(&e, "name")).unwrap_or_else(|| panic_with_error!(&e, ContractError::MetadataIncomplete));
        let vault_name = prefix.concat(&e, vault_name);
        let vault_symbol = name_symbol.get(String::from_str(&e, "symbol")).unwrap_or_else(|| panic_with_error!(&e, ContractError::MetadataIncomplete));

        set_vault_fee(&e, &vault_fee);

        set_defindex_protocol_fee_receiver(&e, &defindex_protocol_receiver);
        
        if defindex_protocol_rate > 9000 {
            panic_with_error!(&e, ContractError::MaximumFeeExceeded);
        }
        set_defindex_protocol_fee_rate(&e, &defindex_protocol_rate);

        set_is_upgradable(&e, &upgradable);

        set_soroswap_router(&e, &soroswap_router);

        // Validate assets
        validate_assets(&e, &assets);
        let total_assets = assets.len();

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
    /// the vault and mints vault shares that represent the user's proportional share in the vault. Additionally, 
    /// if the `invest` parameter is set to `true`, the function will immediately generate and execute investment 
    /// allocations based on the vault's strategy configuration.
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
    /// * `Result<(Vec<i128>, i128, Option<Vec<Option<AssetInvestmentAllocation>>>), ContractError>` - Returns:
    ///   - A vector of actual deposited amounts
    ///   - The number of shares minted
    ///   - Optional investment allocations if `invest` is true
    ///
    /// # Function Flow
    /// 1. **Validation**:
    ///    - Checks contract initialization
    ///    - Verifies authorization of the depositor
    ///    - Validates input parameters
    /// 2. **Current State Assessment**:
    ///    - Fetches total managed funds to calculate share ratios
    /// 3. **Deposit Processing**:
    ///    - Calculates shares to mint based on deposit amounts and current vault state
    ///    - Transfers assets from user to vault
    ///    - Mints vault shares to represent ownership
    /// 4. **Investment Processing** (if `invest` is true):
    ///    - Generates investment allocations based on current strategy ratios
    ///    - Executes investments by deploying idle funds to strategies
    /// 5. **Event Emission**:
    ///    - Emits deposit event with amounts and minted shares
    ///
    /// # Notes
    /// - The function maintains proportional share minting across multiple assets
    /// - Investment allocations follow existing strategy ratios when `invest` is true
    /// - Deposited funds remain idle if `invest` is false
    ///
    /// # Errors
    /// - Returns a `ContractError` if:
    ///   - Contract is not initialized
    ///   - Input validation fails
    ///   - Asset transfers fail
    ///   - Share calculations encounter arithmetic errors
    ///   - Investment execution fails (when `invest` is true)
    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
        invest: bool,
    ) -> Result<(Vec<i128>, i128, Option<Vec<Option<AssetInvestmentAllocation>>>), ContractError> {
        extend_instance_ttl(&e);
        from.require_auth();

        // Fetches the total managed funds for all assets, including idle and invested funds (net of locked fees).
        // Setting the flag to `true` ensures that strategy reports are updated and new fees are locked during the process.
        let total_managed_funds = fetch_total_managed_funds(&e, true)?;

        let (amounts, shares_to_mint) = process_deposit(
            &e,
            &total_managed_funds,
            &amounts_desired,
            &amounts_min,
            &from,
        )?;
        events::emit_deposit_event(&e, from, amounts.clone(), shares_to_mint.clone());

        let asset_investments = if invest {
            let allocations = generate_investment_allocations(&e, &total_managed_funds, &amounts)?;
            //iterate between allocations to execute investments
            for allocation in allocations.iter() {
                //Validate if alloctation is not empty
                if let Some(allocation) = allocation.clone() {
                    let strategies = allocation.strategy_allocations;
                    for strategy in strategies {
                        if let Some(strategy) = strategy {
                            let strategy_address = strategy.strategy_address;
                            let amount = strategy.amount;
                            invest_in_strategy(&e, &allocation.asset, &strategy_address, &amount)?;
                        }

                    }
                }
            }
            Some(allocations)
        } else {
            None
        };
        Ok((amounts, shares_to_mint, asset_investments))
    }

    /// Handles user withdrawals from the DeFindex Vault by burning shares and returning assets.
    ///
    /// This function processes a withdrawal request by burning the specified amount of vault shares
    /// and returning a proportional amount of the vault's assets to the user. It can unwind positions
    /// from strategies if necessary to fulfill the withdrawal.
    ///
    /// ## Parameters:
    /// - `e`: The contract environment (`Env`).
    /// - `withdraw_shares`: The number of vault shares to withdraw.
    /// - `min_amounts_out`: A vector of minimum amounts required for each asset to be withdrawn.
    /// - `from`: The address initiating the withdrawal.
    ///
    /// ## Returns
    /// * `Result<Vec<i128>, ContractError>` - On success, returns a vector of withdrawn amounts 
    ///   where each index corresponds to the asset index in the vault's asset list.
    ///   Returns ContractError if the withdrawal fails.
    ///
    /// ## Errors:
    /// - `ContractError::AmountOverTotalSupply`: If the specified shares exceed the total supply.
    /// - `ContractError::ArithmeticError`: If any arithmetic operation fails during calculations.
    /// - `ContractError::WrongAmountsLength`: If there is a mismatch in asset allocation data.
    fn withdraw(e: Env, withdraw_shares: i128, min_amounts_out: Vec<i128>, from: Address) -> Result<Vec<i128>, ContractError> {
        extend_instance_ttl(&e);
        from.require_auth();
        
        if withdraw_shares <= 0 {
            return Err(ContractError::AmountNotAllowed);
        }
        // Fetches the total managed funds for all assets, including idle and invested funds (net of locked fees).
        // Setting the flag to `true` ensures that strategy reports are updated and new fees are locked during the process.
        let total_managed_funds = fetch_total_managed_funds(&e, true)?;
        
        //Validate min_amounts_out length
        if min_amounts_out.len() != total_managed_funds.len() {
            panic_with_error!(&e, ContractError::WrongAmountsLength);
        }
        //Validate min_amounts_out values
        for amount in min_amounts_out.iter() {
            if amount < 0 {
                panic_with_error!(&e, ContractError::AmountNotAllowed);
            }
        }
        
        let total_shares_supply = VaultToken::total_supply(e.clone());

        // Check if the requested shares amount exceeds the total supply
        if withdraw_shares > total_shares_supply {
            return Err(ContractError::AmountOverTotalSupply);
        }

        // Burn the shares after calculating the withdrawal amounts
        // This will panic with error if the user does not have enough balance
        internal_burn(e.clone(), from.clone(), withdraw_shares);
        
        let mut withdrawn_amounts: Vec<i128> = Vec::new(&e);
        
        // Loop through each asset to handle the withdrawal
        for (i, asset) in total_managed_funds.iter().enumerate() {
            // Use assets instead of asset_withdrawal_amounts
            let asset_address = &asset.asset;

            // Calculate the requested withdrawal amount
            let requested_withdrawal_amount = asset
                .total_amount
                .checked_mul(withdraw_shares)
                .ok_or(ContractError::ArithmeticError)?
                .checked_div(total_shares_supply)
                .ok_or(ContractError::ArithmeticError)?;

            if requested_withdrawal_amount < min_amounts_out.get(i as u32).unwrap() {
                panic_with_error!(&e, ContractError::InsufficientOutputAmount);
            }
            let idle_funds = asset.idle_amount;

            // If didle funds are enough, we dont unwind from any strategy
            if idle_funds >= requested_withdrawal_amount {
                TokenClient::new(&e, asset_address).transfer(
                    &e.current_contract_address(),
                    &from,
                    &requested_withdrawal_amount,
                );
                withdrawn_amounts.push_back(requested_withdrawal_amount);
            } else {
                TokenClient::new(&e, asset_address).transfer(
                    &e.current_contract_address(),
                    &from,
                    &idle_funds,
                );
                let mut cumulative_amount_for_asset = idle_funds;
                let remaining_amount_to_unwind =
                    requested_withdrawal_amount.checked_sub(idle_funds).unwrap();
                    
                for (i, strategy_allocation) in
                    asset.strategy_allocations.iter().enumerate()
                {
                    let strategy_amount_to_unwind: i128 =
                        if i == asset.strategy_allocations.len().checked_sub(1).unwrap_or(0) as usize {
                            requested_withdrawal_amount
                                .checked_sub(cumulative_amount_for_asset)
                                .unwrap()
                        } else {
                            remaining_amount_to_unwind
                                .checked_mul(strategy_allocation.amount)
                                .and_then(|result| result.checked_div(asset.invested_amount))
                                .unwrap_or(0)
                        };

                    if strategy_amount_to_unwind > 0 {
                        // When doing unwind, the token in being transfered directly to the user
                        unwind_from_strategy(
                            &e,
                            &strategy_allocation.strategy_address,
                            &strategy_amount_to_unwind,
                            &from,
                        )?;
                        cumulative_amount_for_asset = cumulative_amount_for_asset.checked_add(strategy_amount_to_unwind).ok_or(ContractError::Overflow)?;
                    }
                }
                withdrawn_amounts.push_back(cumulative_amount_for_asset);
            }
        }

        events::emit_withdraw_event(&e, from, withdraw_shares, withdrawn_amounts.clone());

        Ok(withdrawn_amounts)
    }

    /// Executes rescue (formerly emergency withdrawal) from a specific strategy.
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
    /// # Returns
    /// * `Result<(), ContractError>` - Success (()) or ContractError if the rescue operation fails
    fn rescue(
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

        // Find the strategy and its associated asset
        let asset = get_strategy_asset(&e, &strategy_address)?;
        // This ensures that the vault has this strategy in its list of assets
        let strategy = get_strategy_struct(&strategy_address, &asset)?;
        let strategy_invested_funds = fetch_strategy_invested_funds(&e, &strategy_address, false)?;
        report::update_report_and_lock_fees(&e, &strategy_address, strategy_invested_funds)?;
        let distribution_result = report::distribute_strategy_fees(&e, &strategy.address, &access_control, &asset.address)?;
        if distribution_result > 0 {
            let mut distributed_fees: Vec<(Address, i128)> = Vec::new(&e);
            distributed_fees.push_back((asset.address.clone(), distribution_result));
            events::emit_fees_distributed_event(&e, distributed_fees.clone());
        }

        // Withdraw all assets from the strategy
        let strategy_client = get_strategy_client(&e, strategy.address.clone());
        let strategy_balance = strategy_client.balance(&e.current_contract_address());

        if strategy_balance > 0 {
            unwind_from_strategy(
                &e,
                &strategy_address,
                &strategy_balance,
                &e.current_contract_address(),
            )?;
            
            // Create a new zeroed report directly instead of getting the existing one
            let report = Report {
                prev_balance: 0,
                gains_or_losses: 0,
                locked_fee: 0,
            };
            set_report(&e, &strategy_address, &report);
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
    /// # Returns
    /// * `Result<(), ContractError>` - Success (()) or ContractError if the pause operation fails
    fn pause_strategy(
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
    fn get_assets(e: Env) -> Result<Vec<AssetStrategySet>, ContractError> {
        extend_instance_ttl(&e);
        get_assets(&e)
    }

    /// Returns the total managed funds of the vault, including both invested and idle funds.
    ///
    /// This function provides a vector of `CurrentAssetInvestmentAllocation` structs containing information
    /// about each asset's current allocation, including both invested amounts in strategies and idle amounts.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Vec<CurrentAssetInvestmentAllocation>, ContractError>` - A vector of asset allocations or error
    fn fetch_total_managed_funds(e: &Env) -> Result<Vec<CurrentAssetInvestmentAllocation>, ContractError> {
        extend_instance_ttl(&e);
        let total_managed_funds = fetch_total_managed_funds(e, false)?;

        Ok(total_managed_funds)
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
    /// * `Result<Vec<i128>, ContractError>` - A vector of asset amounts corresponding to the vault shares, where each index
    ///   matches the asset index in the vault's asset list. Returns ContractError if calculation fails.
    fn get_asset_amounts_per_shares(
        e: Env,
        vault_shares: i128,
    ) -> Result<Vec<i128>, ContractError> {
        extend_instance_ttl(&e);

        // Fetches the total managed funds for all assets, including idle and invested funds (net of locked fees).
        // Setting the flag to `true` ensures that strategy reports are updated and new fees are locked during the process.
        let total_managed_funds = fetch_total_managed_funds(&e, true)?;
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

    /// Generates reports for all strategies in the vault, tracking their performance and fee accrual.
    ///
    /// This function iterates through all assets and their associated strategies to generate
    /// performance reports. It updates each strategy's report with current balances and
    /// calculates gains or losses since the last report.
    ///
    /// # Arguments
    /// * `e` - The environment reference.
    ///
    /// # Function Flow
    /// 1. **Instance Extension**:
    ///    - Extends contract TTL
    ///
    /// 2. **Asset & Strategy Retrieval**:
    ///    - Gets all assets and their strategies
    ///    - Initializes reports vector
    ///
    /// 3. **Report Generation**:
    ///    - For each asset:
    ///      - For each strategy:
    ///        - Gets current strategy balance
    ///        - Updates report with new balance
    ///        - Stores updated report
    ///
    /// # Returns
    /// * `Result<Vec<Report>, ContractError>` - On success, returns a vector of reports 
    ///   where each report contains performance metrics for a strategy. Returns 
    ///   ContractError if report generation fails.
    ///
    /// # Note
    /// Reports track:
    /// - Current strategy balance
    /// - Gains or losses since last report
    /// - Locked fees
    /// - Fee distribution status
    fn report(e: Env) -> Result<Vec<Report>, ContractError> {
        extend_instance_ttl(&e);

        // Get all assets and their strategies
        let assets = get_assets(&e)?;
        let mut reports: Vec<Report> = Vec::new(&e);

        // Loop through each asset and its strategies to report the balances
        for asset in assets.iter() {
            for strategy in asset.strategies.iter() {
                let strategy_client = get_strategy_client(&e, strategy.address.clone());
                let strategy_invested_funds =
                    strategy_client.balance(&e.current_contract_address());

                let mut report = get_report(&e, &strategy.address);
                report.report(strategy_invested_funds)?;
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
    /// This function allows the manager or the vault fee receiver to set a new fee receiver address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `caller` - The address initiating the change (must be the manager or the vault fee receiver).
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
    /// This function allows the current manager to set a new manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `new_manager` - The new manager address.
    ///
    /// # Returns
    /// * `Result<(), ContractError>` - Success (()) or ContractError if the manager change fails
    fn set_manager(e: Env, new_manager: Address) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        
        access_control.set_manager(&new_manager);
        events::emit_manager_changed_event(&e, new_manager);
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
    /// This function allows the current manager to set a new emergency manager for the vault.
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
        extend_instance_ttl(&e);
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
    /// # Returns
    /// * `Result<(), ContractError>` - Returns Ok(()) on success, ContractError if upgrade fails
    ///
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError> {
        if !storage::is_upgradable(&e) {
            return Err(ContractError::NotUpgradable);
        }
        extend_instance_ttl(&e);
        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);
        
        e.deployer().update_current_contract_wasm(new_wasm_hash);
        Ok(())
    }
}

#[contractimpl]
impl VaultManagementTrait for DeFindexVault {

    /// Rebalances the vault by executing a series of instructions.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `instructions` - A vector of `Instruction` structs representing actions (withdraw, invest, swap, zapper) to be taken.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn rebalance(e: Env, caller: Address, instructions: Vec<Instruction>) -> Result<(), ContractError> {
        extend_instance_ttl(&e);

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
                    let asset_address = get_strategy_asset(&e, &strategy_address)?;
                    let strategy_invested_funds = fetch_strategy_invested_funds(&e, &strategy_address, true)?;
                    if amount <= 0 {
                        panic_with_error!(&e, ContractError::AmountNotAllowed);
                    }
                    if amount > strategy_invested_funds {
                        return Err(ContractError::UnwindMoreThanAvailable);
                    } else {
                        report::distribute_strategy_fees(&e, &strategy_address, &access_control, &asset_address.address)?;
                        unwind_from_strategy(
                            &e,
                            &strategy_address,
                            &amount,
                            &e.current_contract_address(),
                        )?;
                        let mut report = get_report(&e, &strategy_address);
                        report.prev_balance = strategy_invested_funds - amount;
                        set_report(&e, &strategy_address, &report);
                        let call_params = vec![&e, (strategy_address.clone(), amount, e.current_contract_address())];
                        events::emit_rebalance_unwind_event(&e, call_params, report);
                    }
                }
                Instruction::Invest(strategy_address, amount) => {
                    let asset_address = get_strategy_asset(&e, &strategy_address)?;
                    
                    // Check if strategy is paused before investing
                    let strategy = get_strategy_struct(&strategy_address, &asset_address)?;
                    if strategy.paused {
                        panic_with_error!(&e, ContractError::StrategyPaused);
                    }
                    if amount <= 0 {
                        panic_with_error!(&e, ContractError::AmountNotAllowed);
                    }
                    let report = invest_in_strategy(&e, &asset_address.address, &strategy_address, &amount)?;
                    let call_params = AssetInvestmentAllocation {
                        asset: asset_address.address.clone(),
                        strategy_allocations: vec![&e, Some(StrategyAllocation {
                            strategy_address: strategy_address.clone(),
                            amount: amount.clone(),
                            paused: strategy.paused
                        })],
                    };
                    report::distribute_strategy_fees(&e, &strategy_address, &access_control, &asset_address.address)?;
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

        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        // If a new fee basis points is provided, set the new vault fee
        if let Some(fee_bps) = new_fee_bps {
            set_vault_fee(&e, &fee_bps);
        }

        // Get the current vault fee
        let current_vault_fee = get_vault_fee(&e);

        // Get all assets and their strategies
        let assets = get_assets(&e)?;
        let mut reports: Vec<Report> = Vec::new(&e);

        // Loop through each asset and its strategies to lock the fees
        for asset in assets.iter() {
            for strategy in asset.strategies.iter() {
                let mut report = get_report(&e, &strategy.address);
                if report.gains_or_losses > 0 {
                    report.lock_fee(current_vault_fee)?;
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
        validate_amount(amount)?;
        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);

        let mut report = get_report(&e, &strategy);

        report.release_fee(&e, amount)?;
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
    /// * `caller` - The address initiating the fee distribution.
    ///
    /// # Returns
    /// * `Result<Vec<(Address, i128)>, ContractError>` - A vector of tuples with asset addresses and the total distributed fee amounts.
    fn distribute_fees(e: Env, caller: Address) -> Result<Vec<(Address, i128)>, ContractError> {
        extend_instance_ttl(&e);

        let access_control = AccessControl::new(&e);
        access_control.require_any_role(
            &[RolesDataKey::Manager, RolesDataKey::VaultFeeReceiver],
            &caller,
        );

        // Get all assets and their strategies
        let assets = get_assets(&e)?;

        let mut distributed_fees: Vec<(Address, i128)> = Vec::new(&e);

        // Loop through each asset and its strategies to lock the fees
        for asset in assets.iter() {
            let mut total_fees_distributed: i128 = 0;

            for strategy in asset.strategies.iter() {
                total_fees_distributed =
                total_fees_distributed.checked_add(
                    report::distribute_strategy_fees(&e, &strategy.address, &access_control, &asset.address)?)
                    .unwrap();
            }

            if total_fees_distributed > 0 {
                distributed_fees.push_back((asset.address.clone(), total_fees_distributed));
            }
        }

        events::emit_fees_distributed_event(&e, distributed_fees.clone());

        Ok(distributed_fees)
    }
}
