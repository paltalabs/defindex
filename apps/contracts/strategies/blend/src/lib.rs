#![no_std]
use reserves::StrategyReserves;
use soroban_sdk::{
    token::TokenClient,
    contract, contractimpl, Address, Env, IntoVal, String, Val, Vec, vec,
};

mod blend_pool;
mod constants;
mod reserves;
mod soroswap;
mod storage;

use storage::{extend_instance_ttl, Config};

pub use defindex_strategy_core::{event, DeFindexStrategyTrait, StrategyError};

pub fn check_positive_amount(amount: i128) -> Result<(), StrategyError> {
    if amount <= 0 {
        Err(StrategyError::OnlyPositiveAmountAllowed)
    } else {
        Ok(())
    }
}

const STRATEGY_NAME: &str = "BlendStrategy";

#[contract]
pub struct BlendStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for BlendStrategy {
    /// Initializes the contract with the necessary configuration parameters.
    ///
    /// # Arguments
    ///
    /// * `e` - The execution environment, provided automatically when the contract is invoked.
    /// * `asset` - The address of the asset managed by the contract.
    /// * `init_args` - A vector of initialization arguments required for configuration.
    ///
    /// # Parameters in `init_args`
    ///
    /// The `init_args` vector must contain the following, in order:
    ///
    /// 1. `blend_pool_address: Address` - The address of the Blend pool where assets are deposited.
    /// 2. `blend_token: Address` - The address of the reward token (e.g., BLND) issued by the Blend pool.
    /// 3. `soroswap_router: Address` - The address of the Soroswap AMM router for asset swaps.
    /// 4. `reward_threshold: i128` - The minimum reward amount that triggers reinvestment.
    ///
    /// # Behavior
    ///
    /// This function:
    /// - Fetches the `reserve_id` from the Blend pool for the given `asset`.
    /// - Calculates `claim_ids` for the asset bTokens.
    /// - Stores all parameters in a `Config` struct in the contract's storage.
    ///
    /// # Token IDs
    ///
    /// - The `reserve_id` identifies a specific reserve in the Blend pool.
    /// - Token IDs are derived from the `reserve_index`:
    ///   - `d_token_id = reserve_index * 2`
    ///   - `b_token_id = reserve_index * 2 + 1`
    /// - To find the `reserve_index` from a token ID: `reserve_index = floor(reserve_token_id / 2)`.
    /// - Only bTokens are used here, as they represent supplied assets.
    ///
    /// # Example
    ///
    /// ```rust
    /// let e: Env = /* ... */;
    /// let asset: Address = /* ... */;
    /// let init_args: Vec<Val> = /* ... */;
    /// __constructor(e, asset, init_args);
    /// ```
    fn __constructor(e: Env, asset: Address, init_args: Vec<Val>) {
        let blend_pool_address: Address = init_args
            .get(0)
            .expect("Invalid argument: blend_pool_address")
            .into_val(&e);
        let blend_token: Address = init_args
            .get(1)
            .expect("Invalid argument: blend_token")
            .into_val(&e);
        let soroswap_router: Address = init_args
            .get(2)
            .expect("Invalid argument: soroswap_router")
            .into_val(&e);
        let reward_threshold: i128 = init_args
            .get(3)
            .expect("Invalid argument: reward_threshold")
            .into_val(&e);
        let keeper: Address = init_args
            .get(4)
            .expect("Invalid argument: keeper")
            .into_val(&e);
        // reserve_id (u32): A unique identifier for a specific reserve within the Blend pool.
        let blend_pool_client = blend_pool::BlendPoolClient::new(&e, &blend_pool_address);
        let reserve_id = blend_pool_client.get_reserve(&asset).config.index;

        // claim_ids: A list of identifiers for tokens that can be claimed from the pool.
        // Each eligible token has a unique ID, enabling precise tracking and claiming of rewards.
        //
        // Token IDs for a reserve are derived from its reserve_index:
        // - d_token_id = reserve_index * 2
        // - b_token_id = reserve_index * 2 + 1
        // This ensures each reserve has distinct IDs for its d_token and b_token.
        // To reverse this and find the reserve_index from a reserve_token_id:
        //   reserve_index = floor(reserve_token_id / 2)
        //
        // bTokens represent a supply made to a Blend pool. Each reserve has a unique, non-transferable
        // bToken per pool. Since we're supplying to the pool, we focus on b_tokens.
        //
        // Here, we calculate the claim_id for the bToken (since we're only claiming emissions for it):
        let claim_id = reserve_id * 2 + 1;
        let claim_ids: Vec<u32> = vec![&e, claim_id];


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
        storage::set_keeper(&e, &keeper);
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
        check_positive_amount(amount)?;
        extend_instance_ttl(&e);
        from.require_auth();

        let config = storage::get_config(&e)?;

        // transfer tokens from the vault to this (strategy) contract
        TokenClient::new(&e, &config.asset).transfer(&from, &e.current_contract_address(), &amount);

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
        let keeper = storage::get_keeper(&e)?;
        if from != keeper {
            return Err(StrategyError::NotAuthorized);
        }
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
        check_positive_amount(amount)?; 
        extend_instance_ttl(&e);
        from.require_auth();

        let config = storage::get_config(&e)?;

        let b_tokens_burnt = blend_pool::withdraw(&e, &to, &amount, &config)?;

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

#[contractimpl]
impl BlendStrategy {
    /// Sets a new keeper address for the strategy.
    ///
    /// This function updates the keeper address stored in the contract's storage.
    /// Only the current keeper can authorize this change.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `from: Address` - The address initiating the keeper change (must be current keeper).
    /// * `new_keeper: Address` - The new keeper address to set.
    ///
    /// # Returns
    ///
    /// * `Result<(), StrategyError>` - An empty result or an error.
    pub fn set_keeper(e: Env, from: Address, new_keeper: Address) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);
        from.require_auth();
        
        let current_keeper = storage::get_keeper(&e)?;
        if from != current_keeper {
            return Err(StrategyError::NotAuthorized);
        }
        
        storage::set_keeper(&e, &new_keeper);
        Ok(())
    }
    
    /// Returns the current keeper address.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    ///
    /// # Returns
    ///
    /// * `Result<Address, StrategyError>` - The current keeper address or an error.
    pub fn get_keeper(e: Env) -> Result<Address, StrategyError> {
        extend_instance_ttl(&e);
        storage::get_keeper(&e)
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
