#![no_std]
use constants::{MIN_DUST};
use reserves::StrategyReserves;
use soroban_sdk::{
    contract, contractimpl, Address, Env, IntoVal, String, Val, Vec,
};

mod blend_pool;
mod constants;
mod reserves;
mod soroswap;
mod storage;

use storage::{extend_instance_ttl, Config};

pub use defindex_strategy_core::{event, DeFindexStrategyTrait, StrategyError};

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
    if amount < 0 {
        Err(StrategyError::NegativeNotAllowed)
    } else {
        Ok(())
    }
}

const STRATEGY_NAME: &str = "BlendStrategy";

#[contract]
pub struct BlendStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for BlendStrategy {
    /// Constructor function to initialize the contract's configuration with the necessary parameters.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment (provided automatically when the contract is invoked).
    /// * `asset: Address` - The address of the asset being managed by the contract.
    /// * `init_args: Vec<Val>` - A vector of initialization arguments required for configuring the contract.
    ///
    /// # Process
    ///
    /// This constructor function takes in the following arguments:
    ///
    /// 1. **blend_pool_address** (`Address`) - The address of the Blend pool where assets will be deposited.
    /// 2. **reserve_id** (`u32`) - The identifier for the specific reserve within the Blend pool.
    /// 3. **blend_token** (`Address`) - The address of the reward token (e.g., BLND) given by the Blend pool.
    /// 4. **soroswap_router** (`Address`) - The address of the Soroswap AMM router, enabling asset swaps.
    /// 5. **claim_ids** (`Vec<u32>`) - A list of IDs of the claimable tokens within the pool.
    /// 5. **reward_threshold** (`i128`) - The threshold amount of rewards that will trigger a reinvestment.
    ///
    /// # IDs and Their Use
    ///
    /// - **reserve_id**: A unique identifier for the reserve within the Blend pool. This ID allows the contract to interact with a specific reserve and differentiate it from other reserves in the pool.
    ///
    /// - **claim_ids**: These are the identifiers for tokens that can be claimed from the pool. Each token that is eligible for claim has a unique ID, allowing for precise tracking and claiming of rewards.
    ///
    /// The **reserve_token_id** is calculated based on the **reserve_index**:
    /// - `d_token_id = reserve_index * 2`
    /// - `b_token_id = reserve_index * 2 + 1`
    ///
    /// This ensures each reserve has distinct token IDs for its d_token and b_token.
    /// You can retrieve the **reserve_index** from a **reserve_token_id** by using the formula:
    /// `reserve_index = floor(reserve_token_id / 2)`
    ///
    /// A bToken represents a supply made to a Blend pool. Each reserve has a unique bToken per pool, and they are non-transferable.
    /// We are interested in the b_tokens as we are supplying into the pool
    ///
    /// The function sets these parameters into a configuration struct (`Config`) and stores it in the contract's storage.
    ///
    /// # Example
    ///
    /// ```rust
    /// let e: Env = ...;
    /// let asset: Address = ...;
    /// let init_args: Vec<Val> = ...;
    /// __constructor(e, asset, init_args);
    /// ```
    fn __constructor(e: Env, asset: Address, init_args: Vec<Val>) {
        let blend_pool_address: Address = init_args
            .get(0)
            .expect("Invalid argument: blend_pool_address")
            .into_val(&e);
        let reserve_id: u32 = init_args
            .get(1)
            .expect("Invalid argument: reserve_id")
            .into_val(&e);
        let blend_token: Address = init_args
            .get(2)
            .expect("Invalid argument: blend_token")
            .into_val(&e);
        let soroswap_router: Address = init_args
            .get(3)
            .expect("Invalid argument: soroswap_router")
            .into_val(&e);
        let claim_ids: Vec<u32> = init_args
            .get(4)
            .expect("Invalid argument: claim_ids")
            .into_val(&e);
        let reward_threshold: i128 = init_args
            .get(5)
            .expect("Invalid argument: reward_threshold")
            .into_val(&e);

        let config = Config {
            asset: asset.clone(),
            pool: blend_pool_address,
            reserve_id,
            blend_token,
            router: soroswap_router,
            claim_ids,
            reward_threshold
        };

        storage::set_config(&e, config);
    }

    /// Retrieves the asset address from the contract's stored configuration.
    ///
    /// This function extends the contract's TTL and returns the asset address stored in the configuration.
    /// It returns a `Result<Address, StrategyError>` where `Ok` contains the asset address and `Err` indicates an error.
    ///
    /// # Arguments
    /// * `e: Env` - The execution environment.
    ///
    /// # Returns
    /// * `Result<Address, StrategyError>` - The asset address or an error.
    fn asset(e: Env) -> Result<Address, StrategyError> {
        extend_instance_ttl(&e);
        Ok(storage::get_config(&e)?.asset)
    }

    /// Deposits a specified amount of the underlying asset into the strategy.
    ///
    /// This function transfers the specified amount of the underlying asset from the `from` address
    /// to the strategy contract, supplies it to the Blend pool, and mints shares representing the
    /// deposited amount. It also handles reinvestment of any rewards if the balance exceeds the
    /// reward threshold.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `amount: i128` - The amount of the underlying asset to deposit.
    /// * `from: Address` - The address from which the asset is being transferred.
    ///
    /// # Returns
    ///
    /// * `Result<i128, StrategyError>` - The underlying balance after the deposit or an error.
    fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        // protect against rouding of reserve_vault::update_rate, as small amounts
        // can cause incorrect b_rate calculations due to the pool rounding
        if amount < MIN_DUST {
            return Err(StrategyError::AmountBelowMinDust);
        }

        let config = storage::get_config(&e)?;
        blend_pool::claim(&e, &e.current_contract_address(), &config);

        // will reinvest only if blnd_balance > REWARD_THRESHOLD
        blend_pool::perform_reinvest(&e, &config)?;

        let b_tokens_minted = blend_pool::supply(&e, &from, &amount, &config)?;

        // Keeping track of the total deposited amount and the total bTokens owned by the strategy depositors
        let (vault_shares, reserves) =
            reserves::deposit(
                &e, 
                &from, 
                b_tokens_minted,
                &config
            )?;

        let underlying_balance = shares_to_underlying(vault_shares, reserves)?;

        event::emit_deposit(&e, String::from_str(&e, STRATEGY_NAME), amount, from);
        Ok(underlying_balance)
    }

    /// Harvests the rewards from the Blend pool and reinvests them into the strategy.
    ///
    /// This function claims the rewards from the Blend pool, reinvests them if the balance exceeds
    /// the reward threshold, and emits a harvest event.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `from: Address` - The address initiating the harvest.
    ///
    /// # Returns
    ///
    /// * `Result<(), StrategyError>` - An empty result or an error.
    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);
        from.require_auth();
        let config = storage::get_config(&e)?;
        let harvested_blend = blend_pool::claim(&e, &e.current_contract_address(), &config);

        blend_pool::perform_reinvest(&e, &config)?;

        event::emit_harvest(
            &e,
            String::from_str(&e, STRATEGY_NAME),
            harvested_blend,
            from,
        );
        Ok(())
    }

    /// Withdraws a specified amount of the underlying asset from the strategy.
    ///
    /// This function transfers the specified amount of the underlying asset from the strategy contract
    /// to the `to` address, burns the corresponding bTokens, and updates the reserves.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `amount: i128` - The amount of the underlying asset to withdraw.
    /// * `from: Address` - The address from which the asset is being withdrawn.
    /// * `to: Address` - The address to which the asset is being transferred.
    ///
    /// # Returns
    ///
    /// * `Result<i128, StrategyError>` - The remaining balance of the vault after the withdrawal or an error.
    fn withdraw(e: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError> {
        check_nonnegative_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        // protect against rouding of reserve_vault::update_rate, as small amounts
        // can cause incorrect b_rate calculations due to the pool rounding
        if amount < MIN_DUST {
            return Err(StrategyError::AmountBelowMinDust);
        }

        let config = storage::get_config(&e)?;


        blend_pool::claim(&e, &e.current_contract_address(), &config);
        blend_pool::perform_reinvest(&e, &config)?;

        let (_tokens_withdrawn, b_tokens_burnt) = blend_pool::withdraw(&e, &to, &amount, &config)?;

        let (vault_shares, reserves) = reserves::withdraw(
            &e,
            &from,
            b_tokens_burnt,
            &config
        )?;
        let underlying_balance = shares_to_underlying(vault_shares, reserves)?;

        event::emit_withdraw(&e, String::from_str(&e, STRATEGY_NAME), amount, from);

        Ok(underlying_balance)
    }

    /// Returns the balance of the underlying asset for a given address.
    ///
    /// This function calculates the balance of the underlying asset for the specified 'from' address
    /// by converting the 'from' shares to the underlying asset amount.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `from: Address` - The address for which the balance is being queried.
    ///
    /// # Returns
    ///
    /// * `Result<i128, StrategyError>` - The balance of the underlying asset or an error.
    ///
    fn balance(e: Env, from: Address) -> Result<i128, StrategyError> {
        extend_instance_ttl(&e);

        let vault_shares = storage::get_vault_shares(&e, &from);
        if vault_shares > 0 {
            // Get the  updated strategy's total shares and bTokens
            let config = storage::get_config(&e)?;
            let reserves = reserves::get_strategy_reserve_updated(&e, &config);
            let underlying_balance = shares_to_underlying(vault_shares, reserves)?;

            Ok(underlying_balance)
        } else {
            Ok(0)
        }

        
    }
}

/// Converts a given amount of shares to the corresponding amount of underlying assets.
///
/// This function first converts the shares to bTokens using the `shares_to_b_tokens_down` method
/// from the `reserves`. It then uses the `b_rate` to convert the bTokens to the underlying assets.
///
/// # Arguments
///
/// * `shares` - The amount of shares to be converted.
/// * `reserves` - The strategy reserves containing the total shares, total bTokens, and b_rate.
///
/// # Returns
///
/// * `Result<i128, StrategyError>` - The amount of underlying assets corresponding to the given shares,
///   or an error if the conversion fails.
///
fn shares_to_underlying(shares: i128, reserves: StrategyReserves) -> Result<i128, StrategyError> {
    let total_shares = reserves.total_shares;
    let total_b_tokens = reserves.total_b_tokens;

    if total_shares == 0 || total_b_tokens == 0 {
        // No shares or bTokens in the strategy
        return Ok(0i128);
    }
    // Calculate the bTokens corresponding to the vault's shares
    let vault_b_tokens = reserves.shares_to_b_tokens_down(shares)?;
    // Use the b_rate to convert bTokens to underlying assets
    reserves.b_tokens_to_underlying_down(vault_b_tokens)
}
mod test;
