use defindex_strategy_core::StrategyError;

use super::utils::create_generic_strategy;
use crate::reserves;
extern crate std;
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