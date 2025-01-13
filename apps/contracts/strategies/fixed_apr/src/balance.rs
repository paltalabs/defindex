use soroban_sdk::{Address, Env};

use crate::storage::{DataKey, INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use crate::StrategyError;

pub fn read_balance(e: &Env, addr: Address) -> i128 {
    let key = DataKey::Balance(addr);
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        e.storage().persistent().extend_ttl(
            &key,
            INSTANCE_LIFETIME_THRESHOLD,
            INSTANCE_BUMP_AMOUNT,
        );
        balance
    } else {
        0
    }
}

fn write_balance(e: &Env, addr: Address, amount: i128) {
    let key = DataKey::Balance(addr);
    e.storage().persistent().set(&key, &amount);
    e.storage()
        .persistent()
        .extend_ttl(&key, INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn receive_balance(e: &Env, addr: Address, amount: i128) {
    let balance = read_balance(e, addr.clone());

    let new_balance = balance
        .checked_add(amount)
        .expect("Integer overflow occurred while adding balance.");

    write_balance(e, addr, new_balance);
}

pub fn spend_balance(e: &Env, addr: Address, amount: i128) -> Result<(), StrategyError> {
    let balance = read_balance(e, addr.clone());
    if balance < amount {
        return Err(StrategyError::InsufficientBalance);
    }
    write_balance(e, addr, balance - amount);
    Ok(())
}
