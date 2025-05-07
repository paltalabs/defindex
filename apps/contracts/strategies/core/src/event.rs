//! This module defines the events used in the strategy.
//! These events must be emitted by the strategy implementation
//! to ensure compliance with the expected event interface.
use soroban_sdk::{contracttype, symbol_short, Address, Env, String};

// DEPOSIT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositEvent {
    pub amount: i128,
    pub from: Address,
}

/// Publishes an `DepositEvent` to the event stream.
///
/// # Arguments
///
/// * `e` - An instance of the `Env` struct.
/// * `strategy_name` - The name of the strategy.
/// * `amount` - The amount of tokens deposited.
/// * `from` - The address of the account/vault that deposited the tokens.
pub fn emit_deposit(e: &Env, strategy_name: String, amount: i128, from: Address) {
    let event = DepositEvent { amount, from };

    e.events()
        .publish((strategy_name, symbol_short!("deposit")), event);
}

// HARVEST EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HarvestEvent {
    pub amount: i128,
    pub from: Address,
    pub price_per_share: i128,
}

/// Publishes an `HarvestEvent` to the event stream.
///
/// # Arguments
///
/// * `e` - An instance of the `Env` struct.
/// * `strategy_name` - The name of the strategy.
/// * `amount` - The amount of tokens harvested.`
/// * `from` - The address of the account/vault that harvested the tokens.
/// * `price_per_share` - The price per share of the strategy or any other metric to help determine the APY. This could be multiplied by SCALAR_12 to get the price per share in 12 decimal places.
pub fn emit_harvest(e: &Env, strategy_name: String, amount: i128, from: Address, price_per_share: i128) {
    let event = HarvestEvent { amount, from, price_per_share };

    e.events()
        .publish((strategy_name, symbol_short!("harvest")), event);
}

// WITHDRAW EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawEvent {
    pub amount: i128,
    pub from: Address,
}

/// Publishes an `WithdrawEvent` to the event stream.
///
/// # Arguments
///
/// * `e` - An instance of the `Env` struct.
/// * `strategy_name` - The name of the strategy.
/// * `amount` - The amount of tokens withdrawn.
/// * `from` - The address of the account/vault that withdrew the tokens.
pub fn emit_withdraw(e: &Env, strategy_name: String, amount: i128, from: Address) {
    let event = WithdrawEvent { amount, from };

    e.events()
        .publish((strategy_name, symbol_short!("withdraw")), event);
}
