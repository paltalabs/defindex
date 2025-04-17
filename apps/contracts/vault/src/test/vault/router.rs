use soroban_sdk::{vec as sorobanvec, Address, Map, String, Vec};
use crate::router;
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
    DeFindexVaultTest,
    defindex_vault::{
      AssetStrategySet, 
      RolesDataKey,
    },
};

#[test]
#[should_panic(expected = "HostError: Error(Contract, #201)")] // SwapExactInError
fn swap_exact_in_error(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_strategy_params_token_0(&test);
  let strategy_params_token_1 = create_strategy_params_token_1(&test);

  // initialize with 2 assets
  let assets: Vec<AssetStrategySet> = sorobanvec![
      &test.env,
      AssetStrategySet {
          address: test.token_0.address.clone(),
          strategies: strategy_params_token_0.clone()
      },
      AssetStrategySet {
          address: test.token_1.address.clone(),
          strategies: strategy_params_token_1.clone()
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
      test.soroswap_router.address.clone(),
      name_symbol,
      true
  );
  
  let amount0 = 123456789i128;
  let amount1 = 987654321i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

  test.token_0_admin_client.mint(&users[0], &amount0);
  test.token_1_admin_client.mint(&users[0], &amount1);

  defindex_contract.deposit(
    &sorobanvec![&test.env, amount0, amount1],
    &sorobanvec![&test.env, amount0, amount1],
    &users[0],
    &false,
);

  let amount_in = -1_000_000i128;
  let amount_out_min = 1_000i128;
  let deadline = test.env.ledger().timestamp() + 1_000_000u64;

  // add one with part 1 and other with part 0
  let mut path: Vec<Address> = Vec::new(&test.env);
  path.push_back(test.token_0.address.clone());
  path.push_back(test.token_1.address.clone());
  let _result = test.env.as_contract(&defindex_contract.address, || router::internal_swap_exact_tokens_for_tokens(&test.env, &test.token_0.address.clone(), &test.token_1.address.clone(), &amount_in, &amount_out_min, &deadline));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #202)")] // SwapExactOutError
fn swap_exact_out_error(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_strategy_params_token_0(&test);
  let strategy_params_token_1 = create_strategy_params_token_1(&test);

  // initialize with 2 assets
  let assets: Vec<AssetStrategySet> = sorobanvec![
      &test.env,
      AssetStrategySet {
          address: test.token_0.address.clone(),
          strategies: strategy_params_token_0.clone()
      },
      AssetStrategySet {
          address: test.token_1.address.clone(),
          strategies: strategy_params_token_1.clone()
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
      test.soroswap_router.address.clone(),
      name_symbol,
      true
  );
  
  let amount0 = 123456789i128;
  let amount1 = 987654321i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

  test.token_0_admin_client.mint(&users[0], &amount0);
  test.token_1_admin_client.mint(&users[0], &amount1);

  defindex_contract.deposit(
    &sorobanvec![&test.env, amount0, amount1],
    &sorobanvec![&test.env, amount0, amount1],
    &users[0],
    &false,
);

  let amount_in_max = 10_0_000_000i128;
  let amount_out = 1_0_000_000i128;
  let deadline = test.env.ledger().timestamp() - 1_000_000u64;

  // add one with part 1 and other with part 0
  let mut path: Vec<Address> = Vec::new(&test.env);
  path.push_back(test.token_0.address.clone());
  path.push_back(test.token_1.address.clone());
  let _result = test.env.as_contract(&defindex_contract.address, || router::internal_swap_tokens_for_exact_tokens(&test.env, &test.token_0.address.clone(), &test.token_1.address.clone(), &amount_out, &amount_in_max, &deadline));
}
#[test]
#[should_panic(expected = "HostError: Error(Contract, #200)")]
fn same_asset_swap(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_strategy_params_token_0(&test);
  let strategy_params_token_1 = create_strategy_params_token_1(&test);

  // initialize with 2 assets
  let assets: Vec<AssetStrategySet> = sorobanvec![
      &test.env,
      AssetStrategySet {
          address: test.token_0.address.clone(),
          strategies: strategy_params_token_0.clone()
      },
      AssetStrategySet {
          address: test.token_1.address.clone(),
          strategies: strategy_params_token_1.clone()
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
      test.soroswap_router.address.clone(),
      name_symbol,
      true
  );
  
  let amount0 = 123456789i128;
  let amount1 = 987654321i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

  test.token_0_admin_client.mint(&users[0], &amount0);
  test.token_1_admin_client.mint(&users[0], &amount1);

  defindex_contract.deposit(
    &sorobanvec![&test.env, amount0, amount1],
    &sorobanvec![&test.env, amount0, amount1],
    &users[0],
    &false,
);

  let amount_in_max = 10_0_000_000i128;
  let amount_out = 1_0_000_000i128;
  let deadline = test.env.ledger().timestamp() - 1_000_000u64;

  // add one with part 1 and other with part 0
  let mut path: Vec<Address> = Vec::new(&test.env);
  path.push_back(test.token_0.address.clone());
  path.push_back(test.token_1.address.clone());
  let _result = test.env.as_contract(&defindex_contract.address, || router::internal_swap_tokens_for_exact_tokens(&test.env, &test.token_0.address.clone(), &test.token_0.address.clone(), &amount_out, &amount_in_max, &deadline));
}