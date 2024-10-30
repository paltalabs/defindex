use soroban_sdk::{vec, BytesN, String, Vec};

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
    &2000u32,
    &String::from_str(&test.env, "dfToken"),
    &String::from_str(&test.env, "DFT"),
    &test.manager,
    &asset_params,
    &salt
  );

  let deployed_defindexes = test.factory_contract.deployed_defindexes();
  assert_eq!(deployed_defindexes.len(), 1);
}

#[test]
fn test_create_defindex_deposit_success() {
  let test = DeFindexFactoryTest::setup();
  test.env.mock_all_auths();

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &100u32, &test.defindex_wasm_hash);

  let asset_params = create_asset_params(&test);
  let salt = BytesN::from_array(&test.env, &[0; 32]);

  let amount_0 = 1000i128;
  let amount_1 = 2000i128;

  let amounts: Vec<i128> = vec![&test.env, amount_0.clone(), amount_1.clone()];

  // Mint tokens to manager
  test.token0_admin_client.mint(&test.manager, &amount_0);
  test.token1_admin_client.mint(&test.manager, &amount_1);

  test.factory_contract.create_defindex_vault_deposit(
    &test.manager,
    &test.emergency_manager, 
    &test.fee_receiver,
    &2000u32,
    &String::from_str(&test.env, "dfToken"),
    &String::from_str(&test.env, "DFT"),
    &test.manager,
    &asset_params,
    &amounts,
    &salt
  );

  
  let deployed_defindexes = test.factory_contract.deployed_defindexes();
  assert_eq!(deployed_defindexes.len(), 1);

  let token_0_vault_balance = test.token0.balance(&deployed_defindexes.get(0).unwrap());
  assert_eq!(token_0_vault_balance, amount_0);

  let token_1_vault_balance = test.token1.balance(&deployed_defindexes.get(0).unwrap());
  assert_eq!(token_1_vault_balance, amount_1);

}