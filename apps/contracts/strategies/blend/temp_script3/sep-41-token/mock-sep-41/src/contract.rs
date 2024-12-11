use crate::allowance::{create_allowance, spend_allowance};
use crate::balance::{receive_balance, spend_balance};
use crate::error::TokenError;
use crate::storage::{self, TokenMetadata};
use sep_41_token::{Token, TokenEvents};
use soroban_sdk::{contract, contractimpl, panic_with_error, Address, Env, String};

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("negative amount is not allowed: {}", amount)
    }
}

#[contract]
pub struct MockToken;

#[contractimpl]
impl MockToken {
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if storage::has_admin(&e) {
            panic_with_error!(e, TokenError::AlreadyInitializedError);
        }
        if decimal > 27 {
            panic_with_error!(e, TokenError::OperationNotSupportedError);
        }
        storage::set_admin(&e, &admin);
        let metadata = TokenMetadata {
            decimal,
            name,
            symbol,
        };
        storage::set_metadata(&e, &metadata);
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = storage::get_admin(&e);
        admin.require_auth();
        storage::extend_instance(&e);

        receive_balance(&e, &to, amount);

        TokenEvents::mint(&e, admin, to, amount);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = storage::get_admin(&e);
        admin.require_auth();
        storage::extend_instance(&e);

        storage::set_admin(&e, &new_admin);
    }
}

#[contractimpl]
impl Token for MockToken {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        let result = storage::get_allowance(&e, &from, &spender);
        result.amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();
        check_nonnegative_amount(amount);
        storage::extend_instance(&e);

        create_allowance(&e, &from, &spender, amount, expiration_ledger);

        TokenEvents::approve(&e, from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        storage::extend_instance(&e);
        storage::get_balance(&e, &id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        check_nonnegative_amount(amount);
        storage::extend_instance(&e);

        spend_balance(&e, &from, amount);
        receive_balance(&e, &to, amount);

        TokenEvents::transfer(&e, from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();
        check_nonnegative_amount(amount);
        storage::extend_instance(&e);

        spend_allowance(&e, &from, &spender, amount);
        spend_balance(&e, &from, amount);
        receive_balance(&e, &to, amount);

        TokenEvents::transfer(&e, from, to, amount);
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();
        check_nonnegative_amount(amount);
        storage::extend_instance(&e);

        spend_balance(&e, &from, amount);

        TokenEvents::burn(&e, from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();
        check_nonnegative_amount(amount);
        storage::extend_instance(&e);

        spend_allowance(&e, &from, &spender, amount);
        spend_balance(&e, &from, amount);

        TokenEvents::burn(&e, from, amount);
    }

    fn decimals(e: Env) -> u32 {
        storage::get_metadata(&e).decimal
    }

    fn name(e: Env) -> String {
        storage::get_metadata(&e).name
    }

    fn symbol(e: Env) -> String {
        storage::get_metadata(&e).symbol
    }
}
