#![no_std]
mod blend_pool;
mod constants;
mod reserves;
mod soroswap;
mod storage;
mod utils;

pub use defindex_strategy_core::{event, DeFindexStrategyTrait, StrategyError};
use soroban_fixed_point_math::i128;
use soroban_sdk::{
    token::TokenClient,
    Address, Bytes, contract, contractimpl, Env, IntoVal, String, symbol_short, Val, Vec, vec
};
use storage::{extend_instance_ttl, Config};
use utils::{calculate_optimal_deposit_amount, calculate_optimal_withdraw_amount, shares_to_underlying};

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
    /// 5. `keeper: Address` - The address of the account that will be allowed to do harvest.
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

        //Validate that the reward treshold is positive
        check_positive_amount(reward_threshold).expect("Reward threshold must be positive");

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
    /// deposited amount.
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `amount: i128` - The amount of the underlying asset to deposit.
    /// * `from: Address` - The address from which the asset is being transferred.
    ///
    /// # Returns
    ///
    /// * `Result<i128, StrategyError>` - The underlying balance of the vault (caller) after the deposit or an error.
    fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        extend_instance_ttl(&e);

        check_positive_amount(amount)?;
        from.require_auth();

        let config = storage::get_config(&e)?;
        let reserves = reserves::get_strategy_reserve_updated(&e, &config);
        let (optimal_deposit_amount, b_tokens_minted) = calculate_optimal_deposit_amount(amount, &reserves)?;

        // transfer tokens from the vault to this (strategy) contract
        let token_client = TokenClient::new(&e, &config.asset);
        token_client.transfer(&from, &e.current_contract_address(), &amount);
        token_client.transfer(&e.current_contract_address(), &from, &(amount - optimal_deposit_amount));
        
        // supplies the asset to the Blend pool and mints bTokens
        blend_pool::supply(&e, &from, &optimal_deposit_amount, &config)?;

        // Keeping track of the total deposited amount and the total bTokens owned by the caller (vault)
        let (vault_shares, reserves) =
            reserves::deposit(
                &e, 
                &from, 
                b_tokens_minted,
                &reserves
            )?;
        
        // Calculates the new amount of underlying assets invested in the Blend Vault, owned by the caller (vault)
        let underlying_balance = shares_to_underlying(vault_shares, reserves)?;

        event::emit_deposit(&e, String::from_str(&e, STRATEGY_NAME), amount, from);
        Ok(underlying_balance)
    }

    /// Harvests rewards from the Blend pool and reinvests them into the strategy.
    ///
    /// This function claims rewards from the Blend pool and reinvests them if the balance  
    /// exceeds the reward threshold. It also emits a harvest event upon completion.  
    ///
    /// To comply with the Strategy Crate, this function requires a `from` argument,  
    /// which is not strictly necessary in this context. However, the function enforces  
    /// that the caller (keeper) provides their own address for authorization.  
    ///
    /// # Arguments
    ///
    /// * `e: Env` - The execution environment.
    /// * `from: Address` - The address initiating the harvest (must be the keeper).
    ///
    /// # Returns
    ///
    /// * `Result<(), StrategyError>` - Returns `Ok(())` on success or a `StrategyError` on failure.
    fn harvest(e: Env, from: Address, data: Option<Bytes>) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);
        
        let keeper = storage::get_keeper(&e)?;
        keeper.require_auth();

        if from != keeper {
            return Err(StrategyError::NotAuthorized);
        }

        let config = storage::get_config(&e)?;

        let harvested_blend = blend_pool::claim(&e, &e.current_contract_address(), &config);
        
        // Convert Bytes to i128
        let amount_out_min: i128 = match &data {
            Some(bytes) if !bytes.is_empty() => {
                let mut slice = [0u8; 16];
                bytes.copy_into_slice(&mut slice);
                i128::from_be_bytes(slice)
            },
            _ => 0, // Default to 0 if no data is provided or empty bytes
        };
        
        blend_pool::perform_reinvest(&e, &config, amount_out_min)?;

        event::emit_harvest(
            &e,
            String::from_str(&e, STRATEGY_NAME),
            harvested_blend,
            keeper,
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
    /// * `Result<i128, StrategyError>` - The remaining balance of the vault (caller) after the withdrawal or an error.
    fn withdraw(e: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError> {
        extend_instance_ttl(&e);

        check_positive_amount(amount)?; 
        from.require_auth();

        let config = storage::get_config(&e)?;
        let reserves = reserves::get_strategy_reserve_updated(&e, &config);
        let (optimal_withdraw_amount, b_tokens_burnt) = calculate_optimal_withdraw_amount(amount, &reserves)?;

        blend_pool::withdraw(&e, &to, &optimal_withdraw_amount, &config)?;

        let (vault_shares, reserves) = reserves::withdraw(
            &e,
            &from,
            b_tokens_burnt,
            &reserves
        )?;
        let underlying_balance = shares_to_underlying(vault_shares, reserves)?;

        event::emit_withdraw(&e, String::from_str(&e, STRATEGY_NAME), optimal_withdraw_amount, from);
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
    /// * `new_keeper: Address` - The new keeper address to set.
    ///
    /// # Returns
    ///
    /// * `Result<(), StrategyError>` - An empty result or an error.
    pub fn set_keeper(e: Env, new_keeper: Address) -> Result<(), StrategyError> {
        extend_instance_ttl(&e);
        
        let old_keeper = storage::get_keeper(&e)?;
        old_keeper.require_auth();
            
        storage::set_keeper(&e, &new_keeper);
        e.events().publish(
            (String::from_str(&e, STRATEGY_NAME), symbol_short!("setkeeper")),
            (old_keeper, new_keeper),
        );
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

mod test;
