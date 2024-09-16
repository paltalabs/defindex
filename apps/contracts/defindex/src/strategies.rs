use soroban_sdk::{Env};
use defindex_strategy_core::DeFindexStrategyClient;
use crate::storage::get_strategy;

pub fn get_strategy_client(
    e: &Env,
    index: u32,
) -> DeFindexStrategyClient {
    let strategy = get_strategy(&e, index.clone());
    let strategy_address = strategy.address;
    // if adapter.paused {
    //     return Err(AggregatorError::ProtocolPaused);
    // }
    // Ok(AdapterClient::new(&e, &adapter.address))
    DeFindexStrategyClient::new(&e, &strategy_address)

}
