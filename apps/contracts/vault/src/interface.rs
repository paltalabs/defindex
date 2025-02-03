use soroban_sdk::{Address, BytesN, Env, Map, String, Vec};

use crate::{
    models::{AssetInvestmentAllocation, CurrentAssetInvestmentAllocation, Instruction}, report::Report, ContractError
};
use common::models::AssetStrategySet;

pub trait VaultTrait {
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
    /// * `vault_fee` - Vault-specific fee in basis points (0-2000 for 0-20%)
    /// * `defindex_protocol_receiver` - Address receiving protocol fees
    /// * `defindex_protocol_rate` - Protocol fee rate in basis points (0-9000 for 0-90%)
    /// * `factory` - Factory contract address
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
        factory: Address,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
        upgradable: bool,
    );

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
    ) -> Result<(Vec<i128>, i128, Option<Vec<Option<AssetInvestmentAllocation>>>), ContractError>;

    /// Handles user withdrawals from the DeFindex Vault by burning shares and returning assets.
    ///
    /// This function processes a withdrawal request by burning the specified amount of vault shares
    /// and returning a proportional amount of the vault's assets to the user. It can unwind positions
    /// from strategies if necessary to fulfill the withdrawal.
    ///
    /// ## Parameters:
    /// - `e`: The contract environment (`Env`).
    /// - `withdraw_shares`: The number of vault shares to withdraw.
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
    fn withdraw(e: Env, df_amount: i128, from: Address) -> Result<Vec<i128>, ContractError>;

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
    /// # Returns
    /// * `Result<(), ContractError>` - Success (()) or ContractError if the pause operation fails
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
    fn get_assets(e: Env) -> Result<Vec<AssetStrategySet>, ContractError>;

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
    fn fetch_total_managed_funds(e: &Env) -> Result<Vec<CurrentAssetInvestmentAllocation>, ContractError>;

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
    ) -> Result<Vec<i128>, ContractError>;

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

    fn get_fees(e: Env) -> (u32, u32);

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
    /// This function allows the current manager to set a new manager for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `new_manager` - The new manager address.
    ///
    /// # Returns
    /// * `Result<(), ContractError>` - Success (()) or ContractError if the manager change fails
    fn set_manager(e: Env, new_manager: Address) -> Result<(), ContractError>;

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
    fn set_rebalance_manager(e: Env, new_rebalance_manager: Address);

    /// Retrieves the current rebalance manager address for the vault.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    ///
    /// # Returns:
    /// * `Result<Address, ContractError>` - The rebalance manager address if successful, otherwise returns a ContractError.
    fn get_rebalance_manager(e: Env) -> Result<Address, ContractError>;

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
    fn upgrade(e: Env, new_wasm_hash: BytesN<32>) -> Result<(), ContractError>;
}

pub trait VaultManagementTrait {
    /// Rebalances the vault by executing a series of instructions.
    ///
    /// # Arguments:
    /// * `e` - The environment.
    /// * `instructions` - A vector of `Instruction` structs representing actions (withdraw, invest, swap, zapper) to be taken.
    ///
    /// # Returns:
    /// * `Result<(), ContractError>` - Ok if successful, otherwise returns a ContractError.
    fn rebalance(e: Env, caller: Address, instructions: Vec<Instruction>) -> Result<(), ContractError>;

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
    /// * `caller` - The address initiating the fee distribution.
    ///
    /// # Returns
    /// * `Result<Vec<(Address, i128)>, ContractError>` - A vector of tuples with asset addresses and the total distributed fee amounts.
    fn distribute_fees(e: Env, caller: Address) -> Result<Vec<(Address, i128)>, ContractError>;
}
