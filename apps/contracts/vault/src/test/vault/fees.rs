use soroban_sdk::{ vec as sorobanvec, Address, Map, String, Vec};

use crate::{
  constants::ONE_DAY_IN_SECONDS, 
  test::{create_defindex_vault, create_fixed_strategy_params_token_0, 
    defindex_vault::{ AssetStrategySet, Instruction, RolesDataKey,}, 
  DeFindexVaultTest, EnvTestUtils}};


#[test]
fn rebalance_invest(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_fixed_strategy_params_token_0(&test);
  let assets: Vec<AssetStrategySet> = sorobanvec![
      &test.env,
      AssetStrategySet {
          address: test.token_0.address.clone(),
          strategies: strategy_params_token_0.clone()
      }
  ];

  let mut roles: Map<u32, Address> = Map::new(&test.env);
  roles.set(RolesDataKey::Manager as u32, test.manager.clone());
  roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
  roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
  roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

  let mut name_symbol: Map<String, String> = Map::new(&test.env);
  name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
  name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

  let defindex_contract = create_defindex_vault(
      &test.env,
      assets,
      roles,
      2000u32,
      test.defindex_protocol_receiver.clone(),
      2500u32,
      test.defindex_factory.clone(),
      test.soroswap_router.address.clone(),
      name_symbol,
      true
  );
  
  let amount = 1000_0_000_000i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

  test.token_0_admin_client.mint(&users[0], &amount);
  let user_balance = test.token_0.balance(&users[0]);
  assert_eq!(user_balance, amount);

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  defindex_contract.deposit(
      &sorobanvec![&test.env, amount],
      &sorobanvec![&test.env, amount],
      &users[0],
      &false,
  );

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, amount - 1000);


  // REBALANCE

  let instruction_amount_0 = 500_0_000_000i128;

  let instructions = sorobanvec![
      &test.env,
      Instruction::Invest(
          test.fixed_strategy_client_token_0.address.clone(),
          instruction_amount_0
      ),
  ];

  defindex_contract.rebalance(&test.rebalance_manager, &instructions);

  let vault_balance = test.token_0.balance(&defindex_contract.address);
  assert_eq!(vault_balance, instruction_amount_0); 

  defindex_contract.report();

  test.env.jump_time(ONE_DAY_IN_SECONDS*365);

  test.fixed_strategy_client_token_0.harvest(&defindex_contract.address);

  let instruction_amount_1 = 500_0_000_000i128;

  let instructions = sorobanvec![
      &test.env,
      Instruction::Invest(
          test.fixed_strategy_client_token_0.address.clone(),
          instruction_amount_1
      ),
  ];

  defindex_contract.rebalance(&test.rebalance_manager, &instructions);
  let report = defindex_contract.report().get(0).unwrap().gains_or_losses;

  let expected_reward = instruction_amount_0 / 10;

  assert_eq!(report, expected_reward);
}

#[test]
fn rebalance_unwind(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_fixed_strategy_params_token_0(&test);
  let assets: Vec<AssetStrategySet> = sorobanvec![
      &test.env,
      AssetStrategySet {
          address: test.token_0.address.clone(),
          strategies: strategy_params_token_0.clone()
      }
  ];

  let mut roles: Map<u32, Address> = Map::new(&test.env);
  roles.set(RolesDataKey::Manager as u32, test.manager.clone());
  roles.set(RolesDataKey::EmergencyManager as u32, test.emergency_manager.clone());
  roles.set(RolesDataKey::VaultFeeReceiver as u32, test.vault_fee_receiver.clone());
  roles.set(RolesDataKey::RebalanceManager as u32, test.rebalance_manager.clone());

  let mut name_symbol: Map<String, String> = Map::new(&test.env);
  name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
  name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

  let defindex_contract = create_defindex_vault(
      &test.env,
      assets,
      roles,
      2000u32,
      test.defindex_protocol_receiver.clone(),
      2500u32,
      test.defindex_factory.clone(),
      test.soroswap_router.address.clone(),
      name_symbol,
      true
  );
  
  let amount = 1000_0_000_000i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

  test.token_0_admin_client.mint(&users[0], &amount);
  let user_balance = test.token_0.balance(&users[0]);
  assert_eq!(user_balance, amount);

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  defindex_contract.deposit(
      &sorobanvec![&test.env, amount],
      &sorobanvec![&test.env, amount],
      &users[0],
      &false,
  );

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, amount - 1000);


  // REBALANCE

  let instruction_amount_0 = 500_0_000_000i128;

  let instructions = sorobanvec![
      &test.env,
      Instruction::Invest(
          test.fixed_strategy_client_token_0.address.clone(),
          instruction_amount_0
      ),
  ];

  defindex_contract.rebalance(&test.rebalance_manager, &instructions);

  let vault_balance = test.token_0.balance(&defindex_contract.address);
  assert_eq!(vault_balance, instruction_amount_0); 

  defindex_contract.report();

  test.env.jump_time(ONE_DAY_IN_SECONDS*365);

  test.fixed_strategy_client_token_0.harvest(&defindex_contract.address);

  let instruction_amount_1 = 300_0_000_000i128;

  let instructions = sorobanvec![
      &test.env,
      Instruction::Unwind(
          test.fixed_strategy_client_token_0.address.clone(),
          instruction_amount_1
      ),
  ];

  defindex_contract.rebalance(&test.rebalance_manager, &instructions);
  let report = defindex_contract.report().get(0).unwrap().gains_or_losses;

  let expected_reward = instruction_amount_0 / 10;

  assert_eq!(report, expected_reward);
}