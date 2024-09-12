use soroban_sdk::{Env, Map, Address};
use soroban_sdk::token::{TokenClient};

use crate::storage::{get_tokens, get_total_strategies};
use crate::strategies::{get_strategy_client};

// Helper functions
fn get_idle_funds_for_token(e: &Env, token_address: &Address) -> i128 {
    TokenClient::new(e, token_address).balance(&e.current_contract_address())
}

fn get_invested_funds_for_token(e: &Env, token_address: &Address) -> i128 {
    let total_strategies = get_total_strategies(e);
    let mut invested_funds = 0;
    for i in 0..total_strategies {
        let strategy_client = get_strategy_client(e, i);
        // TODO: Every strategy will work with an specific token!
        invested_funds += strategy_client.balance(&e.current_contract_address());
    }
    invested_funds
}

// Pub functions

pub fn get_current_idle_funds(e: &Env) -> Map<Address, i128> {
    let tokens= get_tokens(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for token in tokens {
        map.set(token.clone(), get_idle_funds_for_token(e, &token));
    }
    map
}

pub fn get_current_invested_funds(e: &Env) -> Map<Address, i128> {
    let tokens= get_tokens(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for token in tokens {
        map.set(token.clone(), get_invested_funds_for_token(e, &token));
    }
    map

}

pub fn get_total_managed_funds(e: &Env) -> Map<Address, i128> {
    let tokens= get_tokens(e);
    let mut map: Map<Address, i128> = Map::new(e);
    for token in tokens {
        let idle_funds = get_idle_funds_for_token(e, &token);
        let invested_funds = get_invested_funds_for_token(e, &token);
        map.set(token.clone(), idle_funds + invested_funds);
    }
    map
}


