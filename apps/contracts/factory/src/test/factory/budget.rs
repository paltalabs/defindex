extern crate std;

use crate::test::{create_asset_params, DeFindexFactoryTest};
use soroban_sdk::{vec as sorobanvec, BytesN, String, Vec};

#[test]
fn budget() {
    
  let test = DeFindexFactoryTest::setup();
  
  test.env.budget().reset_unlimited();

  // initialize factory contract

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &100u32, &test.defindex_wasm_hash);

  let mem = test.env.budget().memory_bytes_cost();
  let cpu = test.env.budget().cpu_instruction_cost();

  std::println!("initialize()                                              | cpu: {},      mem: {}", cpu, mem);

  test.env.budget().reset_unlimited();

  // create defindex vault

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
  
  let mem = test.env.budget().memory_bytes_cost();
  let cpu = test.env.budget().cpu_instruction_cost();
  std::println!("create_defindex_vault()                                   | cpu: {},      mem: {}", cpu, mem);
  
  test.env.budget().reset_unlimited();
  let users = DeFindexFactoryTest::generate_random_users(&test.env, 2);
  // create defindex vault deposit
  let asset_params = create_asset_params(&test);
  let salt = BytesN::from_array(&test.env, &[0; 32]);

  let amount_0 = 100i128;
  let amount_1 = 200i128;

  let amounts: Vec<i128> = sorobanvec![&test.env, amount_0.clone(), amount_1.clone()];

  test.factory_contract.try_create_defindex_vault_deposit(
    &users[0],
    &test.emergency_manager, 
    &test.fee_receiver,
    &10u32,
    &String::from_str(&test.env, "DFTa"),
    &String::from_str(&test.env, "AAA"),
    &test.manager,
    &asset_params,
    &amounts,
    &salt
  );

  let mem = test.env.budget().memory_bytes_cost();
  let cpu = test.env.budget().cpu_instruction_cost();
  std::println!("create_defindex_vault_deposit()                                              | cpu: {},      mem: {}", cpu, mem);


}