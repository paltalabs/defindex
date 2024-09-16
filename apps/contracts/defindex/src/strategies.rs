use soroban_sdk::{Env};
use defindex_strategy_core::DeFindexStrategyClient;
use crate::storage::get_strategy;

pub fn get_strategy_client(
    e: &Env,
    index: u32,
) -> DeFindexStrategyClient {
    let strategy_address = get_strategy(&e, index.clone());
    // if adapter.paused {
    //     return Err(AggregatorError::ProtocolPaused);
    // }
    // Ok(AdapterClient::new(&e, &adapter.address))
    DeFindexStrategyClient::new(&e, &strategy_address)

}
