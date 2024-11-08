use soroban_sdk::{Address, Env, Map, String, Vec};

use crate::{
    models::{AssetStrategySet, Instruction, AssetInvestmentAllocation},
    ContractError,
};

pub trait VaultTrait {
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
        assets: Vec<AssetStrategySet>,
        manager: Address,
        emergency_manager: Address,
        vault_fee_receiver: Address,
        vault_fee: u32,
        defindex_protocol_receiver: Address,
        factory: Address,
        vault_name: String,
        vault_symbol: String,
    ) -> Result<(), ContractError>;

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
    /// 5. **dfToken Minting**: Mints new dfTokens for the user to represent their ownership in the vault.
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
    ) -> Result<(Vec<i128>, i128), ContractError>;

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
    fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<Vec<i128>, ContractError>;

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
    ) -> Result<(), ContractError>;

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
    ) -> Result<(), ContractError>;

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
    ) -> Result<(), ContractError>;

    /// Retrieves the list of assets managed by the DeFindex Vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Vec<AssetStrategySet>` - A vector of `AssetStrategySet` structs representing the assets managed by the vault.
    fn get_assets(e: Env) -> Vec<AssetStrategySet>;

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
    fn fetch_total_managed_funds(e: &Env) -> Map<Address, i128>;

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
    fn fetch_current_invested_funds(e: &Env) -> Map<Address, i128>;

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
    fn fetch_current_idle_funds(e: &Env) -> Map<Address, i128>;

    // TODO: DELETE THIS, USED FOR TESTING
    /// Temporary method for testing purposes.
    fn get_asset_amounts_for_dftokens(e: Env, df_token: i128) -> Map<Address, i128>;
}

pub trait AdminInterfaceTrait {
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
    fn set_fee_receiver(e: Env, caller: Address, new_fee_receiver: Address);

    /// Retrieves the current fee receiver address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The fee receiver address if successful, otherwise returns a ContractError.
    fn get_fee_receiver(e: Env) -> Result<Address, ContractError>;

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
    fn set_manager(e: Env, new_manager: Address);

    /// Retrieves the current manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The manager address if successful, otherwise returns a ContractError.
    fn get_manager(e: Env) -> Result<Address, ContractError>;

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
    fn set_emergency_manager(e: Env, new_emergency_manager: Address);

    /// Retrieves the current emergency manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The emergency manager address if successful, otherwise returns a ContractError.
    fn get_emergency_manager(e: Env) -> Result<Address, ContractError>;
}

pub trait VaultManagementTrait {
    /// Invests the vault's idle funds into the specified strategies.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `investment` - A vector of `AssetInvestmentAllocation` structs representing the amount to invest in each strategy.
    /// * `caller` - The address of the caller.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn invest(e: Env, investment: Vec<AssetInvestmentAllocation>) -> Result<(), ContractError>;

    /// Rebalances the vault by executing a series of instructions.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `instructions` - A vector of `Instruction` structs representing actions (withdraw, invest, swap, zapper) to be taken.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn rebalance(e: Env, instructions: Vec<Instruction>) -> Result<(), ContractError>;
}
