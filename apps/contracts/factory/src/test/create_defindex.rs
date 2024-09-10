use soroban_sdk::{vec, Address, BytesN, Vec};

use crate::test::{create_strategy_params, DeFindexFactoryTest};

#[test]
fn test_create_defindex_success() {
  let test = DeFindexFactoryTest::setup();

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &test.defindex_wasm_hash);

  let tokens: Vec<Address> = vec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
  let ratios: Vec<u32> = vec![&test.env, 1, 1];

  let strategy_params = create_strategy_params(&test);

  let salt = BytesN::from_array(&test.env, &[0; 32]);

  test.factory_contract.create_defindex_vault(
    &test.emergency_manager, 
    &test.fee_receiver,
    &test.manager,
    &tokens,
    &ratios,
    &strategy_params,
    &salt
  );
}