use soroban_sdk::{Address, Env, Map, String, Vec};

use crate::{
    models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, Instruction}, report::Report, ContractError
};
use common::models::AssetStrategySet;

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
    /// - `vault_fee`: Vault-specific fee percentage in basis points.
    /// - `defindex_protocol_receiver`: Address receiving DeFindex’s protocol-wide fee in basis points.
    /// - `defindex_protocol_rate`: DeFindex’s protocol fee percentage in basis points.
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
    );

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
        invest: bool,
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
    fn fetch_total_managed_funds(e: &Env) -> Map<Address, CurrentAssetInvestmentAllocation>;

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
    fn get_asset_amounts_per_shares(e: Env, vault_shares: i128) -> Result<Map<Address, i128>, ContractError>;
    
    fn get_fees(e: Env) -> (u32, u32);

    /// Reports the gains or losses for all strategies in the vault based on their current balances.
    ///
    /// This function iterates through all the strategies managed by the vault and calculates the gains or losses
    /// for each strategy based on their current balances. It updates the vault's records accordingly.
    ///
    /// # Arguments
    /// * `e` - A reference to the environment.
    ///
    /// # Returns 
    /// * `Result<Vec<(Address, (i128, i128))>, ContractError>` - A vector of tuples containing the strategy address, current balance, and the gain or loss.
    fn report(e: Env) -> Result<Vec<Report>, ContractError>;

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
    fn invest(e: Env, 
        asset_investments: Vec<Option<AssetInvestmentAllocation>>
    ) -> Result<(), ContractError>;

    /// Rebalances the vault by executing a series of instructions.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `instructions` - A vector of `Instruction` structs representing actions (withdraw, invest, swap, zapper) to be taken.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn rebalance(e: Env, instructions: Vec<Instruction>) -> Result<(), ContractError>;

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
    fn lock_fees(e: Env, new_fee_bps: Option<u32>) -> Result<Vec<Report>, ContractError>;

    /// Releases locked fees for a specific strategy.
    ///
    /// # Arguments
    /// * `e` - The environment reference.
    /// * `strategy` - The address of the strategy for which to release fees.
    /// * `amount` - The amount of fees to release.
    ///
    /// # Returns
    /// * `Result<Report, ContractError>` - A report of the released fees or a `ContractError` if the operation fails.
    fn release_fees(e: Env, strategy: Address, amount: i128) -> Result<Report, ContractError>;

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
    fn distribute_fees(e: Env) -> Result<Vec<(Address, i128)>, ContractError>;
}
