//! Interface for SEP-41 Token Standard
//! https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md

#![no_std]

#[cfg(any(test, feature = "testutils"))]
pub mod testutils;

use soroban_sdk::{contractclient, symbol_short, Address, Env, String};

/// SEP-0041 Token Standard Trait
#[contractclient(name = "TokenClient")]
pub trait Token {
    /// Returns the allowance for `spender` to transfer from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens to be drawn from.
    /// - `spender` - The address spending the tokens held by `from`.
    fn allowance(env: Env, from: Address, spender: Address) -> i128;

    /// Set the allowance by `amount` for `spender` to transfer/burn from
    /// `from`. Overrides any existing allowance set between `spender` and `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens to be drawn from.
    /// - `spender` - The address being authorized to spend the tokens held by
    /// `from`.
    /// - `amount` - The tokens to be made available to `spender`.
    /// - `live_until_ledger` - The ledger number where this allowance expires.
    /// Cannot be less than the current ledger number unless the amount is being
    /// set to 0.  An expired entry (where live_until_ledger < the current
    /// ledger number) should be treated as a 0 amount allowance.
    ///
    /// # Events
    ///
    /// Emits an event with topics `["approve", from: Address,
    /// spender: Address], data = [amount: i128, live_until_ledger: u32]`
    ///
    /// Emits an event with:
    /// - topics - `["approve", from: Address, spender: Address]`
    /// - data - `[amount: i128, live_until_ledger: u32]`
    fn approve(env: Env, from: Address, spender: Address, amount: i128, live_until_ledger: u32);

    /// Returns the balance of `id`.
    ///
    /// # Arguments
    ///
    /// - `id` - The address for which a balance is being queried. If the
    /// address has no existing balance, returns 0.
    fn balance(env: Env, id: Address) -> i128;

    /// Transfer `amount` from `from` to `to`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be
    /// withdrawn from.
    /// - `to` - The address which will receive the transferred tokens.
    /// - `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `[amount: i128]`
    fn transfer(env: Env, from: Address, to: Address, amount: i128);

    /// Transfer `amount` from `from` to `to`, consuming the allowance of
    /// `spender`. Authorized by spender (`spender.require_auth()`).
    ///
    /// # Arguments
    ///
    /// - `spender` - The address authorizing the transfer, and having its
    /// allowance consumed during the transfer.
    /// - `from` - The address holding the balance of tokens which will be
    /// withdrawn from.
    /// - `to` - The address which will receive the transferred tokens.
    /// - `amount` - The amount of tokens to be transferred.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `[amount: i128]`
    fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128);

    /// Burn `amount` from `from`.
    ///
    /// # Arguments
    ///
    /// - `from` - The address holding the balance of tokens which will be
    /// burned from.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `[amount: i128]`
    fn burn(env: Env, from: Address, amount: i128);

    /// Burn `amount` from `from`, consuming the allowance of `spender`.
    ///
    /// # Arguments
    ///
    /// - `spender` - The address authorizing the burn, and having its allowance
    /// consumed during the burn.
    /// - `from` - The address holding the balance of tokens which will be
    /// burned from.
    /// - `amount` - The amount of tokens to be burned.
    ///
    /// # Events
    ///
    /// Emits an event with:
    /// - topics - `["burn", from: Address]`
    /// - data - `[amount: i128]`
    fn burn_from(env: Env, spender: Address, from: Address, amount: i128);

    /// Returns the number of decimals used to represent amounts of this token.
    fn decimals(env: Env) -> u32;

    /// Returns the name for this token.
    fn name(env: Env) -> String;

    /// Returns the symbol for this token.
    fn symbol(env: Env) -> String;
}

/// Extension for functions implemented by the SEP-0041 compliant native Stellar Asset
/// Contract: https://github.com/stellar/rs-soroban-env/blob/main/soroban-env-host/src/native_contract/token/contract.rs
#[contractclient(name = "StellarAssetClient")]
pub trait StellarAssetExtension {
    /// Create `amount` of tokens and assigns them to `to`.
    ///
    /// Requires authorization by the admin.
    ///
    /// # Arguments
    ///
    /// - `to` - The address which will receive the created tokens.
    /// - `amount` - The amount of tokens to be created.
    fn mint(env: Env, to: Address, amount: i128);

    /// Set the authorization status of an address to `authorize`.
    ///
    /// Requires authorization by the admin.
    ///
    /// # Arguments
    ///
    /// - `addr` - The address which will have their authorization modified.
    /// - `authorize` - The authorization status to be set.
    fn set_authorized(env: Env, addr: Address, authorize: bool);

    /// Get the authorization status of an address.
    ///
    /// # Arguments
    ///
    /// - `addr` - The address which will have their authorization status queried.
    fn authorized(env: Env, addr: Address) -> bool;

    /// Clawback `amount` of tokens from `from`.
    ///
    /// Requires authorization by the admin.
    ///
    /// # Arguments
    ///
    /// - `from` - The address which will have their tokens clawed back.
    /// - `amount` - The amount of tokens to be clawed back.
    fn clawback(env: Env, from: Address, amount: i128);

    /// Set the admin address to `new_admin`.
    ///
    /// Requires authorization by the admin.
    ///
    /// # Arguments
    ///
    /// - `new_admin` - The address which will be set as the new admin.
    fn set_admin(env: Env, new_admin: Address);

    /// Get the admin address.
    fn admin(env: Env);
}

pub struct TokenEvents {}

impl TokenEvents {
    /// Emitted when an allowance is set
    ///
    /// - topics - `["approve", from: Address, spender: Address]`
    /// - data - `[amount: i128, live_until_ledger: u32]`
    pub fn approve(
        env: &Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) {
        let topics = (symbol_short!("approve"), from, spender);
        env.events().publish(topics, (amount, live_until_ledger));
    }

    /// Emitted when an amount is transferred from one address to another
    ///
    /// - topics - `["transfer", from: Address, to: Address]`
    /// - data - `[amount: i128]`
    pub fn transfer(env: &Env, from: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("transfer"), from, to);
        env.events().publish(topics, amount);
    }

    /// Emitted when an amount of tokens is burnt from one address
    ///
    /// - topics - `["burn", from: Address]`
    /// - data - `[amount: i128]`
    pub fn burn(env: &Env, from: Address, amount: i128) {
        let topics = (symbol_short!("burn"), from);
        env.events().publish(topics, amount);
    }

    /// Emitted when an amount of tokens is created and assigned to an address
    ///
    /// - topics - `["mint", admin: Address, to: Address]`
    /// - data - `[amount: i128]`
    pub fn mint(env: &Env, admin: Address, to: Address, amount: i128) {
        let topics = (symbol_short!("mint"), admin, to);
        env.events().publish(topics, amount);
    }
}
