use soroban_sdk::{Env, Map, Address};
use soroban_sdk::token::{TokenClient};

use crate::storage::{get_tokens};

fn get_idle_funds_of_token(e: &Env, token_address: &Address) -> i128 {
    TokenClient::new(e, token_address).balance(&e.current_contract_address())
}

pub fn get_current_idle_funds(e: &Env) -> Map<Address, i128> {
    let tokens= get_tokens(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for token in tokens {
        map.set(token.clone(), get_idle_funds_of_token(e, &token));
        // TODO get local idle balance of tokens
    }
    map
}

pub fn get_total_managed_funds(e: &Env) -> Map<Address, i128> {
    let tokens= get_tokens(e);
    let mut manage_funds: Map<Address, i128> = Map::new(e);
    for token in tokens {
        // TODO get local idle balance of tokens
        // TODO get balance of token for each strategy
    }
    // return dummy map
    let dummy_address = e.current_contract_address();
    manage_funds.set(dummy_address, 0);
    manage_funds

}

pub fn get_current_invested_funds(e: &Env) -> Map<Address, i128> {
    // return dummy map
    let mut map: Map<Address, i128> = Map::new(e);
    let dummy_address = e.current_contract_address();
    map.set(dummy_address, 0);
    map

}

