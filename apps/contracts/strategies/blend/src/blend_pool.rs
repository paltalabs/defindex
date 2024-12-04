use defindex_strategy_core::StrategyError;
use soroban_sdk::{auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, panic_with_error, token::TokenClient, vec, Address, Env, IntoVal, Symbol, Vec};

use crate::{constants::REWARD_THRESHOLD, reserves, soroswap::internal_swap_exact_tokens_for_tokens, storage::{self, Config}};

soroban_sdk::contractimport!(
    file = "../external_wasms/blend/blend_pool.wasm"
  );
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

pub fn supply(e: &Env, from: &Address, amount: &i128, config: &Config) -> i128 {
    let pool_client = BlendPoolClient::new(e, &config.pool);

    // Get deposit amount pre-supply
    let pre_supply = pool_client
        .get_positions(&e.current_contract_address())
        .supply
        .get(config.reserve_id)
        .unwrap_or(0);

    let requests: Vec<Request> = vec![&e, Request {
        address: config.asset.clone(),
        amount: amount.clone(),
        request_type: RequestType::Supply.to_u32(),
    }];

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: config.asset.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    config.pool.clone(),
                    amount.clone()).into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);
    
    let new_positions = pool_client.submit(
        &e.current_contract_address(),
        &e.current_contract_address(),
        &from,
        &requests
    );

    // Calculate the amount of bTokens received
    let b_tokens_amount = new_positions.supply.get_unchecked(config.reserve_id) - pre_supply;
    b_tokens_amount
}

pub fn withdraw(e: &Env, from: &Address, amount: &i128, config: &Config) -> (i128, i128) {
    let pool_client = BlendPoolClient::new(e, &config.pool);

    let pre_supply = pool_client
        .get_positions(&e.current_contract_address())
        .supply
        .get(config.reserve_id)
        .unwrap_or_else(|| panic_with_error!(e, StrategyError::InsufficientBalance));

    // Get balance pre-withdraw, as the pool can modify the withdrawal amount
    let pre_withdrawal_balance = TokenClient::new(&e, &config.asset).balance(&from);

    let requests: Vec<Request> = vec![&e, Request {
        address: config.asset.clone(),
        amount: amount.clone(),
        request_type: RequestType::Withdraw.to_u32(),
    }];

    // Execute the withdrawal - the tokens are transferred from the pool to the vault
    let new_positions = pool_client.submit(
        &e.current_contract_address(),
        &e.current_contract_address(),
        &from,
        &requests
    );

    // Calculate the amount of tokens withdrawn and bTokens burnt
    let post_withdrawal_balance = TokenClient::new(&e, &config.asset).balance(&from);
    let real_amount = post_withdrawal_balance - pre_withdrawal_balance;
    
    // position entry is deleted if the position is cleared
    let b_tokens_amount = pre_supply - new_positions.supply.get(config.reserve_id).unwrap_or(0);
    (real_amount, b_tokens_amount)
}

pub fn claim(e: &Env, from: &Address, config: &Config) -> i128 {
    let pool_client = BlendPoolClient::new(e, &config.pool);

    // TODO: Check reserve_token_ids and how to get the correct one
    pool_client.claim(from, &vec![&e, config.reserve_id], from)
}

pub fn perform_reinvest(e: &Env, config: &Config) -> Result<bool, StrategyError>{
    // Check the current BLND balance
    let blnd_balance = TokenClient::new(e, &config.blend_token).balance(&e.current_contract_address());

    // If balance does not exceed threshold, skip harvest
    if blnd_balance < REWARD_THRESHOLD {
        return Ok(false);
    }
    
    // Swap BLND to the underlying asset
    let mut swap_path: Vec<Address> = vec![&e];
    swap_path.push_back(config.blend_token.clone());
    swap_path.push_back(config.asset.clone());

    let deadline = e.ledger().timestamp() + 600;

    // Swapping BLND tokens to Underlying Asset
    let swapped_amounts = internal_swap_exact_tokens_for_tokens(
        e,
        &blnd_balance,
        &0i128,
        swap_path,
        &e.current_contract_address(),
        &deadline,
        config,
    )?;
    let amount_out: i128 = swapped_amounts
        .get(1)
        .ok_or(StrategyError::InvalidArgument)?
        .into_val(e);
    
    // Supplying underlying asset into blend pool
    let b_tokens_minted = supply(&e, &e.current_contract_address(), &amount_out, &config);

    let reserves = storage::get_strategy_reserves(&e);
    reserves::harvest(&e, reserves, amount_out, b_tokens_minted);

    Ok(true)
}