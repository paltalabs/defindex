use defindex_strategy_core::StrategyError;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    token::TokenClient,
    vec, Address, Env, IntoVal, Symbol, Vec,
};

use crate::{
    reserves,
    soroswap::internal_swap_exact_tokens_for_tokens,
    storage::{Config},
};

soroban_sdk::contractimport!(file = "../external_wasms/blend/pool.wasm");
pub type BlendPoolClient<'a> = Client<'a>;

// Define the RequestType enum with explicit u32 values
#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum RequestType {
    Supply = 0,
    Withdraw = 1,
    // SupplyCollateral = 2,
    // WithdrawCollateral = 3,
    // Borrow = 4,
    // Repay = 5,
    // FillUserLiquidationAuction = 6,
    // FillBadDebtAuction = 7,
    // FillInterestAuction = 8,
    // DeleteLiquidationAuction = 9,
}

// Implement a method to convert RequestType to u32
impl RequestType {
    fn to_u32(self) -> u32 {
        self as u32
    }
}

/// Supplies the underlying asset to the Blend pool as defined in the contract's configuration.
///
/// This function transfers the specified `amount` of the underlying asset from the strategy contract
/// to the Blend pool and determines the exact number of `bTokens` minted in return.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `from` - The address initiating the supply transaction.
/// * `amount` - The amount of the underlying asset to be supplied.
/// * `config` - The contract configuration containing pool and asset details.
///
/// # Returns
/// * `Ok(i128)` - The number of `bTokens` minted to the strategy if reinvest is true, otherwise the supplied amount is returned.
/// * `Err(StrategyError)` - If an underflow or overflow occurs.
pub fn supply(
    e: &Env,
    from: &Address,
    amount: &i128,
    config: &Config,
    is_reinvest: bool,
) -> Result<i128, StrategyError> {
    let pool_client = BlendPoolClient::new(e, &config.pool);
    
    // Get deposit amount pre-supply only if we need it for reinvest
    let pre_supply_amount = if is_reinvest {
        pool_client
            .get_positions(&e.current_contract_address())
            .supply
            .try_get(config.reserve_id)
            .unwrap_or(Some(0))
            .unwrap_or(0)
    } else {
        0
    };

    let requests: Vec<Request> = vec![
        &e,
        Request {
            address: config.asset.clone(),
            amount: amount.clone(),
            request_type: RequestType::Supply.to_u32(),
        },
    ];

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: config.asset.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    config.pool.clone(),
                    amount.clone(),
                )
                    .into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);

    if is_reinvest {
        let new_positions = pool_client.submit(
            &e.current_contract_address(),
            &e.current_contract_address(),
            &from,
            &requests,
        );
        // Calculate the amount of bTokens received
        let new_supply_amount = new_positions
            .supply
            .try_get(config.reserve_id)
            .unwrap_or(Some(0))
            .unwrap_or(0);
    
        // Calculate the amount of bTokens received
        let b_tokens_amount = new_supply_amount
            .checked_sub(pre_supply_amount)
            .ok_or_else(|| StrategyError::UnderflowOverflow)?;
        
        Ok(b_tokens_amount)
    } else {
        pool_client.submit(
            &e.current_contract_address(),
            &e.current_contract_address(),
            &from,
            &requests,
        );
        Ok(*amount)
    }
}

/// Executes a user withdrawal of the underlying asset from the Blend pool on behalf of the strategy.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `to` - The recipient address of the withdrawn underlying asset.
/// * `amount` - The requested withdrawal amount.
/// * `config` - The contract configuration containing pool and asset details.
///
/// # Returns
/// * `Ok(i128)` - The amount of `bTokens` burned in the process.
/// * `Err(StrategyError)` - If an error occurs.
pub fn withdraw(
    e: &Env,
    to: &Address,
    amount: &i128,
    config: &Config,
) -> Result<i128, StrategyError> {
    let pool_client = BlendPoolClient::new(e, &config.pool);

    let requests: Vec<Request> = vec![
        &e,
        Request {
            address: config.asset.clone(),
            amount: amount.clone(),
            request_type: RequestType::Withdraw.to_u32(),
        },
    ];

    // Execute the withdrawal - the tokens are transferred from the pool to the vault
    pool_client.submit(
        &e.current_contract_address(),
        &e.current_contract_address(),
        &to,
        &requests,
    );

    // Use the b_tokens_amount from calculate_optimal_withdraw_amount instead of calculating here
    // This is now handled by the caller
    Ok(*amount)
}

/// Claims rewards for the given address from the Blend pool.
///
/// This function interacts with the Blend pool to claim any available rewards associated
/// with the provided address and the configured claim IDs.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `from` - The address for which rewards should be claimed.
/// * `config` - The contract configuration containing pool and claim ID details.
///
/// # Returns
/// * `i128` - The amount of rewards claimed.
pub fn claim(e: &Env, from: &Address, config: &Config) -> i128 {
    let pool_client = BlendPoolClient::new(e, &config.pool);
    pool_client.claim(from, &config.claim_ids, from)
}

/// Reinvests BLND rewards back into the pool.
///
/// This function swaps BLND rewards for the underlying asset using a direct path
/// through the Soroswap router. The swapped assets are then supplied to the Blend
/// pool, and the strategy's reserves are updated accordingly.
///
/// # Process
/// 1. Check the BLND balance of the contract.
/// 2. If the balance is below the reward threshold, exit early.
/// 3. Swap BLND tokens for the underlying asset via Soroswap.
/// 4. Supply the swapped asset to the Blend pool.
/// 5. Update the strategy reserves to reflect the reinvested amount.
///
/// # Arguments
/// * `e` - The execution environment.
/// * `config` - The contract configuration containing asset and pool details.
///
/// # Returns
/// * `Result<bool, StrategyError>` - Returns `true` if reinvestment was successful,
///   `false` if skipped due to low BLND balance, or an error if any step fails.
pub fn perform_reinvest(e: &Env, config: &Config, amount_out_min: i128) -> Result<bool, StrategyError> {
    // Check the current BLND balance
    let blnd_balance =
        TokenClient::new(e, &config.blend_token).balance(&e.current_contract_address());

    // If balance does not exceed threshold, skip harvest
    if blnd_balance < config.reward_threshold {
        return Ok(false);
    }

    let swap_path = vec![e, config.blend_token.clone(), config.asset.clone()];

    let deadline = e
        .ledger()
        .timestamp()
        .checked_add(1)
        .ok_or( StrategyError::UnderflowOverflow)?;

    // Swapping BLND tokens to Underlying Asset
    let swapped_amounts = internal_swap_exact_tokens_for_tokens(
        e,
        &blnd_balance,
        &amount_out_min,
        swap_path,
        &e.current_contract_address(),
        &deadline,
        config,
    )?;
    let amount_out: i128 = swapped_amounts
        .get(1)
        .ok_or(StrategyError::InternalSwapError)?
        .into_val(e);

    // Supplying underlying asset into blend pool
    let b_tokens_minted = supply(&e, &e.current_contract_address(), &amount_out, &config, true)?;

    reserves::harvest(&e, b_tokens_minted, &config)?;

    Ok(true)
}


/// Fetches the asset's b_rate from the pool
///
/// ### Arguments
/// * `e` - The execution environment
/// * `config` - The configuration parameters for the strategy
///
/// ### Returns
/// * `i128` - The b_rate of the asset
pub fn reserve_b_rate(
    e: &Env, 
    config: &Config
) -> i128 {
    let pool_client = BlendPoolClient::new(e, &config.pool);
    pool_client.get_reserve(&config.asset).data.b_rate
}
