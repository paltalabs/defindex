// External crates
use soroban_sdk::{
    symbol_short,
    Bytes,
    String,
    FromVal,
    IntoVal,
    testutils::Events,
};
extern crate std;

// Internal crates
use defindex_strategy_core::{
    StrategyError,
    event::HarvestEvent,
};

// Local imports
use super::utils::{create_generic_strategy, mint_and_deposit_to_strategy};
use crate::{
    constants::SCALAR_12, reserves, test::EnvTestUtils, BlendStrategyClient, STRATEGY_NAME
};

#[test]
fn set_validated_vault_shares() {
  let strategy = create_generic_strategy();
  let e = strategy.e;

  let error_result = e.as_contract(&strategy.address, || reserves::set_validated_vault_shares(&e, &strategy.address, -100));
  assert_eq!(
    error_result,
    Err(StrategyError::OnlyPositiveAmountAllowed));
  
  let success_result = e.as_contract(&strategy.address, || reserves::set_validated_vault_shares(&e, &strategy.address, 100));
  assert_eq!(success_result, Ok(100i128));
}

#[test]
fn harvest_emit_event() {
  let strategy = create_generic_strategy();
  let e = strategy.e.clone();
  let keeper = strategy.keeper.clone();

  e.mock_all_auths();

  let amount: i128 = 10 * 10_000_000;
  mint_and_deposit_to_strategy(&strategy, &strategy.pool_admin, amount);

  // Call harvest as keeper
  let strategy_client = BlendStrategyClient::new(&e, &strategy.address);
  strategy_client.harvest(&keeper, &None::<Bytes>);

  // Fetch the last event
  let events = e.events().all();
  let last_event = events.last().unwrap();

  let expected_topic = (String::from_str(&e, STRATEGY_NAME), symbol_short!("harvest")).into_val(&e);
  assert_eq!(last_event.1, expected_topic);

  let event_data: HarvestEvent = FromVal::from_val(&e, &last_event.2);
  assert_eq!(event_data.from, keeper);
  assert!(event_data.amount >= 0);
  // it should be one since it hasnt passed time yet
  assert!(event_data.price_per_share == SCALAR_12);

  // let's pass time
  e.jump(1000u32);

  // call harvest again
  strategy_client.harvest(&keeper, &None::<Bytes>);

  // Fetch the last event
  let events = e.events().all();
  let last_event = events.last().unwrap();

  let expected_topic = (String::from_str(&e, STRATEGY_NAME), symbol_short!("harvest")).into_val(&e);
  assert_eq!(last_event.1, expected_topic);

  let event_data: HarvestEvent = FromVal::from_val(&e, &last_event.2);
  assert_eq!(event_data.from, keeper);
  assert!(event_data.amount >= 0);
  std::println!("event_data.price_per_share: {}", event_data.price_per_share);
  // it should be greater than 1 since it has passed time
  assert!(event_data.price_per_share > SCALAR_12);
}