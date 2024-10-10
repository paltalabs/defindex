use soroban_sdk::{Address, Env, Map, Vec};

use crate::{
    models::{AssetAllocation, Investment},
    ContractError,
};

pub trait VaultTrait {

    /// Initializes the DeFindex Vault contract with the required parameters.
    ///
    /// This function sets the roles for emergency manager, fee receiver, and manager.
    /// It also stores the list of assets to be managed by the vault, including strategies for each asset.
    /// 
    /// # Arguments:
    /// * `e` - The environment.
    /// * `assets` - A vector of `AssetAllocation` structs representing the assets and their associated strategies.
    /// * `manager` - The address responsible for managing the vault.
    /// * `emergency_manager` - The address with emergency control over the vault.
    /// * `fee_receiver` - The address that will receive fees from the vault.
    /// * `vault_share` - The percentage of the vault's fees that will be sent to the DeFindex receiver. in BPS.
    /// * `defindex_receiver` - The address that will receive fees for DeFindex from the vault.
    /// * `factory` - The address of the factory that deployed the vault.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn initialize(
        e: Env,
        assets: Vec<AssetAllocation>,
        manager: Address,
        emergency_manager: Address,
        fee_receiver: Address,
        vault_share: u32,
        defindex_receiver: Address,
        factory: Address,
    ) -> Result<(), ContractError>;

    /// Handles deposits into the DeFindex Vault.
    ///
    /// This function transfers the desired amounts of each asset into the vault, distributes the assets
    /// across the strategies according to the vault's ratios, and mints dfTokens representing the user's
    /// share in the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `amounts_desired` - A vector of the amounts the user wishes to deposit for each asset.
    /// * `amounts_min` - A vector of minimum amounts required for the deposit to proceed.
    /// * `from` - The address of the user making the deposit.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn deposit(
        e: Env,
        amounts_desired: Vec<i128>,
        amounts_min: Vec<i128>,
        from: Address,
    ) -> Result<(), ContractError>;

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
    fn emergency_withdraw(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError>;

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
    fn pause_strategy(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError>;

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
    fn unpause_strategy(e: Env, strategy_address: Address, caller: Address) -> Result<(), ContractError>;

    /// Retrieves the list of assets managed by the DeFindex Vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Vec<AssetAllocation>` - A vector of `AssetAllocation` structs representing the assets managed by the vault.
    fn get_assets(e: Env) -> Vec<AssetAllocation>;

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
    /// * `fee_receiver` - The new fee receiver address.
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
    /// * `investment` - A vector of `Investment` structs representing the amount to invest in each strategy.
    /// * `caller` - The address of the caller.
    /// 
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn invest(e: Env, investment: Vec<Investment>) -> Result<(), ContractError>;
}