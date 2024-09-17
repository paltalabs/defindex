use crate::storage::get_strategy;
use defindex_strategy_core::DeFindexStrategyClient;
use soroban_sdk::Env;

pub fn get_strategy_client(e: &Env, index: u32) -> DeFindexStrategyClient {
    let strategy = get_strategy(&e, index.clone());
    let strategy_address = strategy.address;
    // if adapter.paused {
    //     return Err(AggregatorError::ProtocolPaused);
    // }
    // Ok(AdapterClient::new(&e, &adapter.address))
    DeFindexStrategyClient::new(&e, &strategy_address)
}
