use soroban_sdk::BytesN;

use crate::test::{create_asset_params, DeFindexFactoryTest};

#[test]
fn test_create_defindex_success() {
  let test = DeFindexFactoryTest::setup();

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &100u32, &test.defindex_wasm_hash);

  let asset_params = create_asset_params(&test);

  let salt = BytesN::from_array(&test.env, &[0; 32]);

  test.factory_contract.create_defindex_vault(
    &test.emergency_manager, 
    &test.fee_receiver,
    &test.manager,
    &asset_params,
    &salt
  );

  let deployed_defindexes = test.factory_contract.deployed_defindexes();
  assert_eq!(deployed_defindexes.len(), 1);
}