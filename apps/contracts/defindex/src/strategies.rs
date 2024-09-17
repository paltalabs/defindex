use crate::storage::get_strategy;
use defindex_strategy_core::DeFindexStrategyClient;
use soroban_sdk::{Env, Address};

pub fn get_strategy_client(e: &Env, address: Address) -> DeFindexStrategyClient {
    DeFindexStrategyClient::new(&e, &address)
}
