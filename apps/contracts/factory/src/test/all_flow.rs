use soroban_sdk::{testutils::Ledger, vec, BytesN, Map, String, Vec};

use crate::test::{create_asset_params, defindex_vault_contract::{self, Investment}, DeFindexFactoryTest};

pub(crate) const DEFINDEX_FEE: u32 = 50u32;
pub(crate) const VAULT_FEE: u32 = 100u32;
pub(crate) const MAX_BPS: u32 = 10_000u32;

#[test]
fn test_deposit_success() {
  let test = DeFindexFactoryTest::setup();
  test.env.mock_all_auths();

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &DEFINDEX_FEE, &test.defindex_wasm_hash);

  let asset_params = create_asset_params(&test);

  let salt = BytesN::from_array(&test.env, &[0; 32]);

  test.factory_contract.create_defindex_vault(
    &test.emergency_manager, 
    &test.fee_receiver,
    &VAULT_FEE,
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

  // Minting Token 0 to user  
  test.token0_admin_client.mint(&users[0], &amount_token0);
  let user_balance = test.token0.balance(&users[0]);
  assert_eq!(user_balance, amount_token0);

  // Minting Token 1 to user
  test.token1_admin_client.mint(&users[0], &amount_token1);
  let user_balance = test.token1.balance(&users[0]);
  assert_eq!(user_balance, amount_token1);

  // Checking user balance of dfTokens
  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  // Depositing Token 0 and Token 1 to defindex
  defindex_contract.deposit(&vec![&test.env, amount_token0, amount_token1], &vec![&test.env, 0, 0], &users[0]);

  // Checking user balance of dfTokens
  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, amount_token0 + amount_token1);

  // Since this is the first deposit, no fees should be minted
  let total_supply = defindex_contract.total_supply();
  assert_eq!(total_supply, amount_token0 + amount_token1);
}

#[test]
fn test_withdraw_success() {
  let test = DeFindexFactoryTest::setup();
  test.env.mock_all_auths();

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &DEFINDEX_FEE, &test.defindex_wasm_hash);

  let asset_params = create_asset_params(&test);

  let salt = BytesN::from_array(&test.env, &[0; 32]);

  // Create a new DeFindex vault
  test.factory_contract.create_defindex_vault(
    &test.emergency_manager, 
    &test.fee_receiver,
    &VAULT_FEE,
    &String::from_str(&test.env, "dfToken"),
    &String::from_str(&test.env, "DFT"),
    &test.manager,
    &asset_params,
    &salt
  );

  // Verify that the vault was created
  let deployed_defindexes = test.factory_contract.deployed_defindexes();
  assert_eq!(deployed_defindexes.len(), 1);

  // Get the address of the created vault and create a client for it
  let defindex_address = deployed_defindexes.get(0).unwrap();
  let defindex_contract = defindex_vault_contract::Client::new(&test.env, &defindex_address);

  // Define the amounts to be deposited
  let amount_token0 = 1_000i128;
  let amount_token1 = 12_000i128;

  // Generate random users for the test
  let users = DeFindexFactoryTest::generate_random_users(&test.env, 1);
    
  // Mint Token 0 to the user and verify the balance
  test.token0_admin_client.mint(&users[0], &amount_token0);
  let user_balance = test.token0.balance(&users[0]);
  assert_eq!(user_balance, amount_token0);

  // Mint Token 1 to the user and verify the balance
  test.token1_admin_client.mint(&users[0], &amount_token1);
  let user_balance = test.token1.balance(&users[0]);
  assert_eq!(user_balance, amount_token1);

  // Verify the initial balance of dfTokens for the user
  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  // Deposit Token 0 and Token 1 into the vault
  defindex_contract.deposit(&vec![&test.env, amount_token0, amount_token1], &vec![&test.env, 0, 0], &users[0]);

  // Verify the balance of dfTokens after deposit
  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance.clone(), amount_token0 + amount_token1);

  // Verify the vault's balance of Token 0 and Token 1 after deposit
  let vault_token0_balance = test.token0.balance(&defindex_contract.address);
  assert_eq!(vault_token0_balance, amount_token0);

  let vault_token1_balance = test.token1.balance(&defindex_contract.address);
  assert_eq!(vault_token1_balance, amount_token1);

  // Create investment strategies for the deposited tokens
  let investments = vec![
    &test.env, 
    Investment {
      amount: amount_token0, 
      strategy: test.strategy_contract_token0.address.clone()
    }, 
    Investment {
      amount: amount_token1, 
      strategy: test.strategy_contract_token1.address.clone()
    }];

  // Invest the tokens into the strategies
  defindex_contract.invest(&investments);

  // Verify the vault's balance of Token 0 and Token 1 after investment
  let vault_token0_balance = test.token0.balance(&defindex_contract.address);
  assert_eq!(vault_token0_balance, 0i128);

  let vault_token1_balance = test.token1.balance(&defindex_contract.address);
  assert_eq!(vault_token1_balance, 0i128);

  // Verify the strategy's balance of Token 0 and Token 1 after investment
  let strategy_token0_balance = test.token0.balance(&test.strategy_contract_token0.address);
  assert_eq!(strategy_token0_balance, amount_token0);

  let strategy_token1_balance = test.token1.balance(&test.strategy_contract_token1.address);
  assert_eq!(strategy_token1_balance, amount_token1);

  // Withdraw the dfTokens and verify the result
  let withdraw_result = defindex_contract.withdraw(&df_balance, &users[0]);
  assert_eq!(withdraw_result, vec![&test.env, 12000i128, 1000i128]);
  
  // Verify the balance of dfTokens after withdrawal
  let df_balance = defindex_contract.balance(&users[0]);
  assert_eq!(df_balance, 0i128);

  // Verify the user's balance of Token 0 and Token 1 after withdrawal
  let user_balance = test.token0.balance(&users[0]);
  assert_eq!(user_balance, amount_token0);

  let user_balance = test.token1.balance(&users[0]);
  assert_eq!(user_balance, amount_token1);

  // Verify the vault's balance of Token 0 and Token 1 after withdrawal
  let vault_token0_balance = test.token0.balance(&defindex_contract.address);
  assert_eq!(vault_token0_balance, 0i128);

  let vault_token1_balance = test.token1.balance(&defindex_contract.address);
  assert_eq!(vault_token1_balance, 0i128);

  // Verify the strategy's balance of Token 0 and Token 1 after withdrawal
  let strategy_token0_balance = test.token0.balance(&test.strategy_contract_token0.address);
  assert_eq!(strategy_token0_balance, 0i128);

  let strategy_token1_balance = test.token1.balance(&test.strategy_contract_token1.address);
  assert_eq!(strategy_token1_balance, 0i128);
}

#[test]
fn test_consecutive_deposits_and_partial_withdrawal() {
  let test = DeFindexFactoryTest::setup();
  test.env.mock_all_auths();

  test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &50u32, &test.defindex_wasm_hash);

  let asset_params = create_asset_params(&test);

  let salt = BytesN::from_array(&test.env, &[0; 32]);

  // Create a new DeFindex vault
  test.factory_contract.create_defindex_vault(
    &test.emergency_manager, 
    &test.fee_receiver,
    &VAULT_FEE,
    &String::from_str(&test.env, "dfToken"),
    &String::from_str(&test.env, "DFT"),
    &test.manager,
    &asset_params,
    &salt
  );

  // Verify that the vault was created
  let deployed_defindexes = test.factory_contract.deployed_defindexes();
  assert_eq!(deployed_defindexes.len(), 1);

  // Get the address of the created vault and create a client for it
  let defindex_address = deployed_defindexes.get(0).unwrap();
  let defindex_contract = defindex_vault_contract::Client::new(&test.env, &defindex_address);

  // Define the amounts to be deposited
  let amount_token0_user1 = 1_000i128;
  let amount_token1_user1 = 12_000i128;
  let amount_token0_user2 = 500i128;
  let amount_token1_user2 = 6_000i128;

  // Generate random users for the test
  let users = DeFindexFactoryTest::generate_random_users(&test.env, 2);
    
  // Mint Token 0 and Token 1 to user 1 and verify the balance
  test.token0_admin_client.mint(&users[0], &amount_token0_user1);
  test.token1_admin_client.mint(&users[0], &amount_token1_user1);
  assert_eq!(test.token0.balance(&users[0]), amount_token0_user1);
  assert_eq!(test.token1.balance(&users[0]), amount_token1_user1);

  // Mint Token 0 and Token 1 to user 2 and verify the balance
  test.token0_admin_client.mint(&users[1], &amount_token0_user2);
  test.token1_admin_client.mint(&users[1], &amount_token1_user2);
  assert_eq!(test.token0.balance(&users[1]), amount_token0_user2);
  assert_eq!(test.token1.balance(&users[1]), amount_token1_user2);

  // Verify the initial balance of dfTokens for both users
  assert_eq!(defindex_contract.balance(&users[0]), 0i128);
  assert_eq!(defindex_contract.balance(&users[1]), 0i128);

  // User 1 deposits Token 0 and Token 1 into the vault
  defindex_contract.deposit(&vec![&test.env, amount_token0_user1, amount_token1_user1], &vec![&test.env, 0, 0], &users[0]);
  assert_eq!(defindex_contract.balance(&users[0]), amount_token0_user1 + amount_token1_user1);

  // User 1 should have deposited all their tokens into the vault
  assert_eq!(test.token0.balance(&users[0]), 0);
  assert_eq!(test.token1.balance(&users[0]), 0);
  assert_eq!(test.token0.balance(&defindex_contract.address), amount_token0_user1);
  assert_eq!(test.token1.balance(&defindex_contract.address), amount_token1_user1);

  // Since this is the first deposit, no fees should be minted
  let total_supply = defindex_contract.total_supply();
  assert_eq!(total_supply, amount_token0_user1 + amount_token1_user1);

  let mut ledger_info = test.env.ledger().get();
  ledger_info.timestamp += 31_536_000;
  test.env.ledger().set(ledger_info);

  // User 2 deposits Token 0 and Token 1 into the vault
  // total_fees = (fee_rate as i128 * total_supply * time_elapsed) / ((SECONDS_PER_YEAR * MAX_BPS) - (fee_rate as i128 * time_elapsed));
  let fee_rate = DEFINDEX_FEE + VAULT_FEE;
  let expected_minted_fee: i128 = (fee_rate as i128).checked_mul(total_supply).unwrap().checked_mul(31_536_000i128).unwrap().checked_div(31_536_000i128.checked_mul(MAX_BPS as i128).unwrap().checked_sub((fee_rate as i128).checked_mul(31_536_000i128).unwrap()).unwrap()).unwrap();
  
  defindex_contract.deposit(&vec![&test.env, amount_token0_user2, amount_token1_user2], &vec![&test.env, 0, 0], &users[1]);
  // tvl = 13000 = 13197 dfTokens
  // new_tvl = 13000 + 6500 = 19500 dfTokens

  assert_eq!(defindex_contract.balance(&users[1]), 6598i128);
  // User 2 should have deposited all their tokens into the vault
  assert_eq!(test.token0.balance(&users[1]), 0);
  // TODO: There is an error with the deposit, since depositing only 500 of the token1 when the user is trying to deposit 6000 
  assert_eq!(test.token1.balance(&users[1]), 0);
  
  assert_eq!(test.token0.balance(&defindex_contract.address), amount_token0_user1 + amount_token0_user2);
  assert_eq!(test.token1.balance(&defindex_contract.address), amount_token1_user1 + amount_token1_user2);

  let total_supply = defindex_contract.total_supply();
  assert_eq!(total_supply, (amount_token0_user1 + amount_token1_user1) + ((amount_token0_user1 + amount_token1_user1) * 150) / 10000);

  // // Create investment strategies for the deposited tokens
  // let investments = vec![
  //   &test.env, 
  //   Investment {
  //     amount: amount_token0_user1 + amount_token0_user2, 
  //     strategy: test.strategy_contract_token0.address.clone()
  //   }, 
  //   Investment {
  //     amount: amount_token1_user1 + amount_token1_user2, 
  //     strategy: test.strategy_contract_token1.address.clone()
  //   }];

  // // Invest the tokens into the strategies
  // defindex_contract.invest(&investments);

  // // Verify the vault's balance of Token 0 and Token 1 after investment
  // assert_eq!(test.token0.balance(&defindex_contract.address), 0i128);
  // assert_eq!(test.token1.balance(&defindex_contract.address), 0i128);

  // // Verify the strategy's balance of Token 0 and Token 1 after investment
  // assert_eq!(test.token0.balance(&test.strategy_contract_token0.address), amount_token0_user1 + amount_token0_user2);
  // assert_eq!(test.token1.balance(&test.strategy_contract_token1.address), amount_token1_user1 + amount_token1_user2);

  // // User 1 withdraws a part of their dfTokens
  // let partial_withdraw_amount = 5_000i128;
  // let withdraw_result = defindex_contract.withdraw(&partial_withdraw_amount, &users[0]);
  // assert_eq!(withdraw_result, vec![&test.env, 5000i128, 0i128]);

  // // Verify the balance of dfTokens after partial withdrawal
  // assert_eq!(defindex_contract.balance(&users[0]), (amount_token0_user1 + amount_token1_user1) - partial_withdraw_amount);
  // assert_eq!(defindex_contract.balance(&users[1]), amount_token0_user2 + amount_token1_user2);

  // // Verify the user's balance of Token 0 and Token 1 after partial withdrawal
  // assert_eq!(test.token0.balance(&users[0]), partial_withdraw_amount);
  // assert_eq!(test.token1.balance(&users[0]), 0i128);

  // // Verify the vault's balance of Token 0 and Token 1 after partial withdrawal
  // assert_eq!(test.token0.balance(&defindex_contract.address), 0i128);
  // assert_eq!(test.token1.balance(&defindex_contract.address), 0i128);

  // // Verify the strategy's balance of Token 0 and Token 1 after partial withdrawal
  // assert_eq!(test.token0.balance(&test.strategy_contract_token0.address), (amount_token0_user1 + amount_token0_user2) - partial_withdraw_amount);
  // assert_eq!(test.token1.balance(&test.strategy_contract_token1.address), amount_token1_user1 + amount_token1_user2);
}