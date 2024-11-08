#![no_std]
use soroban_sdk::{
    contract, contractimpl, panic_with_error,
    token::{TokenClient, TokenInterface},
    Address, Env, Map, String, Vec,
};
use soroban_token_sdk::metadata::TokenMetadata;

mod access;
mod aggregator;
mod constants;
mod error;
mod events;
mod fee;
mod funds;
mod interface;
mod investment;
mod models;
mod storage;
mod strategies;
mod test;
mod token;
mod utils;

use access::{AccessControl, AccessControlTrait, RolesDataKey};
use aggregator::{internal_swap_exact_tokens_for_tokens, internal_swap_tokens_for_exact_tokens};
use fee::collect_fees;
use funds::{fetch_current_idle_funds, fetch_current_invested_funds, fetch_total_managed_funds};
use interface::{AdminInterfaceTrait, VaultManagementTrait, VaultTrait};
use investment::{execute_investment, prepare_investment};
use models::{
    ActionType, AssetAllocation, Instruction, Investment, OptionalSwapDetailsExactIn,
    OptionalSwapDetailsExactOut,
};
use storage::{
    get_assets, set_asset, set_defindex_protocol_fee_receiver, set_factory, set_last_fee_assesment,
    set_total_assets, set_vault_fee, extend_instance_ttl
};
use strategies::{
    get_asset_allocation_from_address, get_strategy_asset, get_strategy_client,
    get_strategy_struct, invest_in_strategy, pause_strategy, unpause_strategy,
    withdraw_from_strategy,
};
use token::{internal_burn, internal_mint, write_metadata, VaultToken};
use utils::{
    calculate_asset_amounts_for_dftokens, calculate_deposit_amounts_and_shares_to_mint,
    calculate_withdrawal_amounts, check_initialized, check_nonnegative_amount,
};

use defindex_strategy_core::DeFindexStrategyClient;

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
    fn initialize(
        e: Env,
        assets: Vec<AssetAllocation>,
        manager: Address,
        emergency_manager: Address,
        vault_fee_receiver: Address,
        vault_fee: u32,
        defindex_protocol_receiver: Address,
        factory: Address,
        vault_name: String,
        vault_symbol: String,
    ) -> Result<(), ContractError> {
        extend_instance_ttl(&e);

        let access_control = AccessControl::new(&e);
        if access_control.has_role(&RolesDataKey::Manager) {
            panic_with_error!(&e, ContractError::AlreadyInitialized);
        }

        access_control.set_role(&RolesDataKey::EmergencyManager, &emergency_manager);
        access_control.set_role(&RolesDataKey::VaultFeeReceiver, &vault_fee_receiver);
        access_control.set_role(&RolesDataKey::Manager, &manager);

        // Set Vault Fee (in basis points)
        set_vault_fee(&e, &vault_fee);

        // Set Paltalabs Fee Receiver
        set_defindex_protocol_fee_receiver(&e, &defindex_protocol_receiver);

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

        events::emit_initialized_vault(
            &e,
            emergency_manager,
            vault_fee_receiver,
            manager,
            defindex_protocol_receiver,
            assets,
        );

        Ok(())
    }

    /// Handles user deposits into the DeFindex Vault.
    ///
    /// This function processes a deposit by transferring each specified asset amount from the user's address to
    /// the vault, allocating assets according to the vault's defined strategy ratios, and minting dfTokens that 
    /// represent the user's proportional share in the vault. The `amounts_desired` and `amounts_min` vectors should 
    /// align with the vault's asset order to ensure correct allocation.
    ///
    /// # Parameters
    /// * `e` - The current environment reference (`Env`), for access to the contract state and utilities.
    /// * `amounts_desired` - A vector specifying the user's intended deposit amounts for each asset.
    /// * `amounts_min` - A vector of minimum deposit amounts required for the transaction to proceed.
    /// * `from` - The address of the user making the deposit.
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
    /// 5. **Vault shares Minting**: Mints vault shares for the user to represent their ownership in the vault.
    ///
    /// # Notes
    /// - For the first deposit, if the vault has only one asset, shares are calculated directly based on the deposit amount.
    /// - For multiple assets, the function delegates to `calculate_deposit_amounts_and_shares_to_mint`
    ///   for precise share computation.
    /// - An event is emitted to log the deposit, including the actual deposited amounts and minted shares.
    ///
    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
    ) -> Result<(Vec<i128>, i128), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;
        from.require_auth();

        // Collect Fees
        // If this was not done before, last_fee_assesment will set to be current timestamp and this will return without action
        collect_fees(&e)?; 

        // fees assesment
        collect_fees(&e)?;

        // get assets
        let assets = get_assets(&e);
        let assets_length = assets.len();

        // assets lenght should be equal to amounts_desired and amounts_min length
        if assets_length != amounts_desired.len() || assets_length != amounts_min.len() {
            panic_with_error!(&e, ContractError::WrongAmountsLength);
        }

        // for every amount desired, check non negative
        for amount in amounts_desired.iter() {
            check_nonnegative_amount(amount)?;
        }
        // for amount min is not necesary to check if it is negative

        let total_supply = VaultToken::total_supply(e.clone());
        let (amounts, shares_to_mint) = if assets_length == 1 {
            let shares = if total_supply == 0 {
                // If we have only one asset, and this is the first deposit, we will mint a share proportional to the amount desired
                // TODO In this case we might also want to mint a MINIMUM LIQUIDITY to be locked forever in the contract
                // this might be for security and practical reasons as well
                // shares will be equal to the amount desired to deposit, just for simplicity
                amounts_desired.get(0).unwrap() // here we have already check same lenght
            } else {
                // If we have only one asset, but we already have some shares minted
                // we will mint a share proportional to the total managed funds 
                // read whitepaper!
                let total_managed_funds = fetch_total_managed_funds(&e);
                // if checked mul gives error, return ArithmeticError
                VaultToken::total_supply(e.clone()).checked_mul(amounts_desired.get(0)
                .unwrap()).unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError))
                .checked_div(total_managed_funds.get(assets.get(0).unwrap().address.clone())
                .unwrap()).unwrap_or_else(|| panic_with_error!(&e, ContractError::ArithmeticError))
            };
            // TODO check that min amount is ok
            (amounts_desired, shares)
        } else {
            if total_supply == 0 {
                // for ths first supply, we will consider the amounts desired, and the shares to mint will just be the sum
                // of the amounts desired
                (amounts_desired.clone(), amounts_desired.iter().sum())
            }
            else {
                // If Total Assets > 1
                calculate_deposit_amounts_and_shares_to_mint(
                    &e,
                    &assets,
                    &amounts_desired,
                    &amounts_min,
                )?
            }
        };

        // for every asset
        for (i, amount) in amounts.iter().enumerate() {
            // if amount is less than minimum, return error InsufficientAmount
            if amount < amounts_min.get(i as u32).unwrap() {
                panic_with_error!(&e, ContractError::InsufficientAmount);
            }
            // its possible that some amounts are 0.
            if amount > 0 {
                let asset = assets.get(i as u32).unwrap();
                let asset_client = TokenClient::new(&e, &asset.address);
                // send the current amount to this contract. This will be held as idle funds.
                asset_client.transfer(&from, &e.current_contract_address(), &amount);
            }
        }

        // now we mint the corresponding dfToken
        // TODO. If total_sypply==0, mint minimum liquidity to be locked forever in the contract
        internal_mint(e.clone(), from.clone(), shares_to_mint);

        events::emit_deposit_event(&e, from, amounts.clone(), shares_to_mint.clone());

        Ok((amounts, shares_to_mint))
    }

    /// Withdraws assets from the DeFindex Vault by burning dfTokens.
    ///
    /// This function calculates the amount of assets to withdraw based on the number of dfTokens being burned,
    /// then transfers the appropriate assets back to the user, pulling from both idle funds and strategies
    /// as needed.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `df_amount` - The amount of dfTokens to burn for the withdrawal.
    /// * `from` - The address of the user requesting the withdrawal.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<Vec<i128>, ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;
        check_nonnegative_amount(df_amount)?;
        from.require_auth();

        // fees assesment
        collect_fees(&e)?;
    
        // Check if the user has enough dfTokens
        let df_user_balance = VaultToken::balance(e.clone(), from.clone());
        if df_user_balance < df_amount {
            return Err(ContractError::InsufficientBalance);
        }

        // Calculate the withdrawal amounts for each asset based on the dfToken amount
        let asset_amounts = calculate_asset_amounts_for_dftokens(&e, df_amount);

        // Burn the dfTokens after calculating the withdrawal amounts (so total supply is correct)
        internal_burn(e.clone(), from.clone(), df_amount);

        // Create a map to store the total amounts to transfer for each asset address
        let mut total_amounts_to_transfer: Map<Address, i128> = Map::new(&e);

        // Get idle funds for each asset (Map<Address, i128>)
        let idle_funds = fetch_current_idle_funds(&e);

        // Loop through each asset and handle the withdrawal
        for (asset_address, required_amount) in asset_amounts.iter() {
            // Check idle funds for this asset
            let idle_balance = idle_funds.get(asset_address.clone()).unwrap_or(0);

            let mut remaining_amount = required_amount;

            // Withdraw as much as possible from idle funds first
            if idle_balance > 0 {
                if idle_balance >= required_amount {
                    // Idle funds cover the full amount
                    total_amounts_to_transfer.set(asset_address.clone(), required_amount);
                    continue; // No need to withdraw from the strategy
                } else {
                    // Partial withdrawal from idle funds
                    total_amounts_to_transfer.set(asset_address.clone(), idle_balance);
                    remaining_amount = required_amount - idle_balance; // Update remaining amount
                }
            }

            // Find the corresponding asset address for this strategy
            let asset_allocation = get_asset_allocation_from_address(&e, asset_address.clone())?;
            let withdrawal_amounts =
                calculate_withdrawal_amounts(&e, remaining_amount, asset_allocation);

            for (strategy_address, amount) in withdrawal_amounts.iter() {
                // TODO: What if the withdraw method exceeds the instructions limit? since im trying to ithdraw from all strategies of all assets...
                withdraw_from_strategy(&e, &strategy_address, &amount)?;

                // Update the total amounts to transfer map
                let current_amount = total_amounts_to_transfer
                    .get(strategy_address.clone())
                    .unwrap_or(0);
                total_amounts_to_transfer.set(asset_address.clone(), current_amount + amount);
            }
        }

        // Perform the transfers for the total amounts
        let mut amounts_withdrawn: Vec<i128> = Vec::new(&e);
        for (asset_address, total_amount) in total_amounts_to_transfer.iter() {
            TokenClient::new(&e, &asset_address).transfer(
                &e.current_contract_address(),
                &from,
                &total_amount,
            );
            amounts_withdrawn.push_back(total_amount);
        }

        events::emit_withdraw_event(&e, from, df_amount, amounts_withdrawn.clone());
    
        Ok(amounts_withdrawn)
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
            strategy_client.withdraw(&strategy_balance, &e.current_contract_address());

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
    /// * `Vec<AssetAllocation>` - A vector of `AssetAllocation` structs representing the assets managed by the vault.
    fn get_assets(e: Env) -> Vec<AssetAllocation> {
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
    fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128> {
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

    // TODO: DELETE THIS, USED FOR TESTING
    /// Temporary method for testing purposes.
    fn get_asset_amounts_for_dftokens(e: Env, df_tokens: i128) -> Map<Address, i128> {
        extend_instance_ttl(&e);
        calculate_asset_amounts_for_dftokens(&e, df_tokens)
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
    /// Invests the vault's idle funds into the specified strategies.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `investment` - A vector of `Investment` structs representing the amount to invest in each strategy.
    /// * `caller` - The address of the caller.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn invest(e: Env, investments: Vec<Investment>) -> Result<(), ContractError> {
        extend_instance_ttl(&e);
        check_initialized(&e)?;

        let access_control = AccessControl::new(&e);
        access_control.require_role(&RolesDataKey::Manager);
        e.current_contract_address().require_auth();

        // Get the current idle funds for all assets
        let idle_funds = fetch_current_idle_funds(&e);

        // Prepare investments based on current idle funds
        // This checks if the total investment exceeds the idle funds
        prepare_investment(&e, investments.clone(), idle_funds)?;

        // Now proceed with the actual investments if all checks passed
        execute_investment(&e, investments)?;

        // auto invest mockup
        // if auto_invest {
        //     let idle_funds = fetch_current_idle_funds(&e);

        //     // Prepare investments based on current ratios of invested funds
        //     let investments = calculate_investments_based_on_ratios(&e);
        //     prepare_investment(&e, investments.clone(), idle_funds)?;
        //     execute_investment(&e, investments)?;
        // }
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
        e.current_contract_address().require_auth();

        for instruction in instructions.iter() {
            match instruction.action {
                ActionType::Withdraw => match (&instruction.strategy, &instruction.amount) {
                    (Some(strategy_address), Some(amount)) => {
                        withdraw_from_strategy(&e, strategy_address, amount)?;
                    }
                    _ => return Err(ContractError::MissingInstructionData),
                },
                ActionType::Invest => match (&instruction.strategy, &instruction.amount) {
                    (Some(strategy_address), Some(amount)) => {
                        invest_in_strategy(&e, strategy_address, amount)?;
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
}
