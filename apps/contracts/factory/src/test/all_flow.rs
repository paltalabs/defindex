use soroban_sdk::{vec, BytesN, String};

use crate::test::{
  create_asset_params, 
  defindex_vault_contract::{
    self, 
    AssetInvestmentAllocation,  
    StrategyInvestment, 
  }, DeFindexFactoryTest};

#[test]
fn test_deposit_success() {
  let test = DeFindexFactoryTest::setup();
  test.env.mock_all_auths();

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

  let defindex_address = deployed_defindexes.get(0).unwrap();
  let defindex_contract = defindex_vault_contract::Client::new(&test.env, &defindex_address);

  let amount_token0 = 1_000i128;
  let amount_token1 = 12_000i128;

  let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    
  test.token0_admin_client.mint(&users[0], &amount_token0);
  let user_balance = test.token0.balance(&users[0]);
  assert_eq!(user_balance, amount_token0);

  test.token1_admin_client.mint(&users[0], &amount_token1);
  let user_balance = test.token1.balance(&users[0]);
  assert_eq!(user_balance, amount_token1);

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  defindex_contract.deposit(&vec![&test.env, amount_token0, amount_token1], &vec![&test.env, 0, 0], &users[0]);

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, amount_token0 + amount_token1 - 1000); // TODO: The amount of dfTokens minted is the sum of both asset deposited?


  // defindex_contract.withdraw(&df_balance, &users[0]);
  
  // let df_balance = defindex_contract.user_balance(&users[0]);
  // assert_eq!(df_balance, 0i128);

  // let user_balance = test.token0.balance(&users[0]);
  // assert_eq!(user_balance, amount);

}

#[test]
fn test_withdraw_success() {
  let test = DeFindexFactoryTest::setup();
  test.env.mock_all_auths();

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

  let defindex_address = deployed_defindexes.get(0).unwrap();
  let defindex_contract = defindex_vault_contract::Client::new(&test.env, &defindex_address);

  let amount_token0 = 1_000i128;
  let amount_token1 = 12_000i128;

  let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    
  test.token0_admin_client.mint(&users[0], &amount_token0);
  let user_balance = test.token0.balance(&users[0]);
  assert_eq!(user_balance, amount_token0);

  test.token1_admin_client.mint(&users[0], &amount_token1);
  let user_balance = test.token1.balance(&users[0]);
  assert_eq!(user_balance, amount_token1);

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  defindex_contract.deposit(&vec![&test.env, amount_token0, amount_token1], &vec![&test.env, 0, 0], &users[0]);

  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance.clone(), amount_token0 + amount_token1 - 1000); // TODO: The amount of dfTokens minted is the sum of both asset deposited?

  let vault_token0_balance = test.token0.balance(&defindex_contract.address);
  assert_eq!(vault_token0_balance, amount_token0);

  let vault_token1_balance = test.token1.balance(&defindex_contract.address);
  assert_eq!(vault_token1_balance, amount_token1);

  let investments = vec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            strategy_investments: vec![
                &test.env,
                Some(StrategyInvestment {
                    strategy: test.strategy_contract_token0.address.clone(),
                    amount: amount_token0,
                }),
            ],
        }),
        Some(AssetInvestmentAllocation {
          asset: test.token1.address.clone(),
          strategy_investments: vec![
              &test.env,
              Some(StrategyInvestment {
                  strategy: test.strategy_contract_token1.address.clone(),
                  amount: amount_token1,
              }),
          ],
      })
    ];


  defindex_contract.invest(&investments);

  let vault_token0_balance = test.token0.balance(&defindex_contract.address);
  assert_eq!(vault_token0_balance, 0i128);

  let vault_token1_balance = test.token1.balance(&defindex_contract.address);
  assert_eq!(vault_token1_balance, 0i128);

  let strategy_token0_balance = test.token0.balance(&test.strategy_contract_token0.address);
  assert_eq!(strategy_token0_balance, amount_token0);

  let strategy_token1_balance = test.token1.balance(&test.strategy_contract_token1.address);
  assert_eq!(strategy_token1_balance, amount_token1);

  // let test_fee = defindex_contract.asses_fees();
  // assert_eq!(test_fee, 0i128);

  let withdraw_result = defindex_contract.withdraw(&df_balance, &users[0]);
  assert_eq!(withdraw_result, vec![&test.env, 12000i128, 1000i128]);
  
  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  let user_balance = test.token0.balance(&users[0]);
  assert_eq!(user_balance, amount_token0);

  let user_balance = test.token1.balance(&users[0]);
  assert_eq!(user_balance, amount_token1);

  let vault_token0_balance = test.token0.balance(&defindex_contract.address);
  assert_eq!(vault_token0_balance, 0i128);

  let vault_token1_balance = test.token1.balance(&defindex_contract.address);
  assert_eq!(vault_token1_balance, 0i128);

  let strategy_token0_balance = test.token0.balance(&test.strategy_contract_token0.address);
  assert_eq!(strategy_token0_balance, 0i128);

  let strategy_token1_balance = test.token1.balance(&test.strategy_contract_token1.address);
  assert_eq!(strategy_token1_balance, 0i128);

}