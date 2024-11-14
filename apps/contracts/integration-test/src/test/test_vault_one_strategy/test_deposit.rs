use crate::{setup::create_vault_one_asset_hodl_strategy, test::IntegrationTest, vault::MINIMUM_LIQUIDITY};
use soroban_sdk::{testutils::{MockAuth, MockAuthInvoke}, vec as svec, IntoVal, Vec};

#[test]
fn test_deposit_success() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, deposit_amount);

    let user_balance_after_deposit = enviroment.token.balance(user);
    assert_eq!(user_balance_after_deposit, user_starting_balance - deposit_amount);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let total_supply = enviroment.vault_contract.total_supply();
    assert_eq!(total_supply, deposit_amount);
}

// #[test]
// fn test_consecutive_deposits_and_partial_withdrawal() {
//   let test = DeFindexFactoryTest::setup();
//   test.env.mock_all_auths();

//   test.factory_contract.initialize(&test.admin, &test.defindex_receiver, &50u32, &test.defindex_wasm_hash);

//   let asset_params = create_asset_params(&test);

//   let salt = BytesN::from_array(&test.env, &[0; 32]);

//   // Create a new DeFindex vault
//   test.factory_contract.create_defindex_vault(
//     &test.emergency_manager, 
//     &test.fee_receiver,
//     &VAULT_FEE,
//     &String::from_str(&test.env, "dfToken"),
//     &String::from_str(&test.env, "DFT"),
//     &test.manager,
//     &asset_params,
//     &salt
//   );

//   // Verify that the vault was created
//   let deployed_defindexes = test.factory_contract.deployed_defindexes();
//   assert_eq!(deployed_defindexes.len(), 1);

//   // Get the address of the created vault and create a client for it
//   let defindex_address = deployed_defindexes.get(0).unwrap();
//   let defindex_contract = defindex_vault_contract::Client::new(&test.env, &defindex_address);

//   // Define the amounts to be deposited
//   let amount_token0_user1 = 1_000i128;
//   let amount_token1_user1 = 12_000i128;
//   let amount_token0_user2 = 500i128;
//   let amount_token1_user2 = 6_000i128;

//   // Generate random users for the test
//   let users = DeFindexFactoryTest::generate_random_users(&test.env, 2);
    
//   // Mint Token 0 and Token 1 to user 1 and verify the balance
//   test.token0_admin_client.mint(&users[0], &amount_token0_user1);
//   test.token1_admin_client.mint(&users[0], &amount_token1_user1);
//   assert_eq!(test.token0.balance(&users[0]), amount_token0_user1);
//   assert_eq!(test.token1.balance(&users[0]), amount_token1_user1);

//   // Mint Token 0 and Token 1 to user 2 and verify the balance
//   test.token0_admin_client.mint(&users[1], &amount_token0_user2);
//   test.token1_admin_client.mint(&users[1], &amount_token1_user2);
//   assert_eq!(test.token0.balance(&users[1]), amount_token0_user2);
//   assert_eq!(test.token1.balance(&users[1]), amount_token1_user2);

//   // Verify the initial balance of dfTokens for both users
//   assert_eq!(defindex_contract.balance(&users[0]), 0i128);
//   assert_eq!(defindex_contract.balance(&users[1]), 0i128);

//   // User 1 deposits Token 0 and Token 1 into the vault
//   defindex_contract.deposit(&vec![&test.env, amount_token0_user1, amount_token1_user1], &vec![&test.env, 0, 0], &users[0]);
//   assert_eq!(defindex_contract.balance(&users[0]), amount_token0_user1 + amount_token1_user1);

//   // User 1 should have deposited all their tokens into the vault
//   assert_eq!(test.token0.balance(&users[0]), 0);
//   assert_eq!(test.token1.balance(&users[0]), 0);
//   assert_eq!(test.token0.balance(&defindex_contract.address), amount_token0_user1);
//   assert_eq!(test.token1.balance(&defindex_contract.address), amount_token1_user1);

//   // Since this is the first deposit, no fees should be minted
//   let total_supply = defindex_contract.total_supply();
//   assert_eq!(total_supply, amount_token0_user1 + amount_token1_user1);

//   let mut ledger_info = test.env.ledger().get();
//   ledger_info.timestamp += 31_536_000;
//   test.env.ledger().set(ledger_info);

//   // User 2 deposits Token 0 and Token 1 into the vault
//   // total_fees = (fee_rate as i128 * total_supply * time_elapsed) / ((SECONDS_PER_YEAR * MAX_BPS) - (fee_rate as i128 * time_elapsed));
//   let fee_rate = DEFINDEX_FEE + VAULT_FEE;
//   let expected_minted_fee: i128 = (fee_rate as i128).checked_mul(total_supply).unwrap().checked_mul(31_536_000i128).unwrap().checked_div(31_536_000i128.checked_mul(MAX_BPS as i128).unwrap().checked_sub((fee_rate as i128).checked_mul(31_536_000i128).unwrap()).unwrap()).unwrap();
  
//   defindex_contract.deposit(&vec![&test.env, amount_token0_user2, amount_token1_user2], &vec![&test.env, 0, 0], &users[1]);
//   // tvl = 13000 = 13197 dfTokens
//   // new_tvl = 13000 + 6500 = 19500 dfTokens

//   assert_eq!(defindex_contract.balance(&users[1]), 6598i128);
//   // User 2 should have deposited all their tokens into the vault
//   assert_eq!(test.token0.balance(&users[1]), 0);
//   // TODO: There is an error with the deposit, since depositing only 500 of the token1 when the user is trying to deposit 6000 
//   assert_eq!(test.token1.balance(&users[1]), 0);
  
//   assert_eq!(test.token0.balance(&defindex_contract.address), amount_token0_user1 + amount_token0_user2);
//   assert_eq!(test.token1.balance(&defindex_contract.address), amount_token1_user1 + amount_token1_user2);

//   let total_supply = defindex_contract.total_supply();
//   assert_eq!(total_supply, (amount_token0_user1 + amount_token1_user1) + ((amount_token0_user1 + amount_token1_user1) * 150) / 10000);

//   // // Create investment strategies for the deposited tokens
//   // let investments = vec![
//   //   &test.env, 
//   //   Investment {
//   //     amount: amount_token0_user1 + amount_token0_user2, 
//   //     strategy: test.strategy_contract_token0.address.clone()
//   //   }, 
//   //   Investment {
//   //     amount: amount_token1_user1 + amount_token1_user2, 
//   //     strategy: test.strategy_contract_token1.address.clone()
//   //   }];

//   // // Invest the tokens into the strategies
//   // defindex_contract.invest(&investments);

//   // // Verify the vault's balance of Token 0 and Token 1 after investment
//   // assert_eq!(test.token0.balance(&defindex_contract.address), 0i128);
//   // assert_eq!(test.token1.balance(&defindex_contract.address), 0i128);

//   // // Verify the strategy's balance of Token 0 and Token 1 after investment
//   // assert_eq!(test.token0.balance(&test.strategy_contract_token0.address), amount_token0_user1 + amount_token0_user2);
//   // assert_eq!(test.token1.balance(&test.strategy_contract_token1.address), amount_token1_user1 + amount_token1_user2);

//   // // User 1 withdraws a part of their dfTokens
//   // let partial_withdraw_amount = 5_000i128;
//   // let withdraw_result = defindex_contract.withdraw(&partial_withdraw_amount, &users[0]);
//   // assert_eq!(withdraw_result, vec![&test.env, 5000i128, 0i128]);

//   // // Verify the balance of dfTokens after partial withdrawal
//   // assert_eq!(defindex_contract.balance(&users[0]), (amount_token0_user1 + amount_token1_user1) - partial_withdraw_amount);
//   // assert_eq!(defindex_contract.balance(&users[1]), amount_token0_user2 + amount_token1_user2);

//   // // Verify the user's balance of Token 0 and Token 1 after partial withdrawal
//   // assert_eq!(test.token0.balance(&users[0]), partial_withdraw_amount);
//   // assert_eq!(test.token1.balance(&users[0]), 0i128);

//   // // Verify the vault's balance of Token 0 and Token 1 after partial withdrawal
//   // assert_eq!(test.token0.balance(&defindex_contract.address), 0i128);
//   // assert_eq!(test.token1.balance(&defindex_contract.address), 0i128);

//   // // Verify the strategy's balance of Token 0 and Token 1 after partial withdrawal
//   // assert_eq!(test.token0.balance(&test.strategy_contract_token0.address), (amount_token0_user1 + amount_token0_user2) - partial_withdraw_amount);
//   // assert_eq!(test.token1.balance(&test.strategy_contract_token1.address), amount_token1_user1 + amount_token1_user2);
// }