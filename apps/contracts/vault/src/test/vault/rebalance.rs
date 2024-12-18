// use soroban_sdk::{
//     Address, vec as sorobanvec, InvokeError, String, Vec, Map};

// use crate::test::{
//     create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
//     defindex_vault::{
//         ActionType, AssetInvestmentAllocation, AssetStrategySet, Instruction, OptionalSwapDetailsExactIn, 
//         OptionalSwapDetailsExactOut, StrategyAllocation, DexDistribution, SwapDetailsExactIn, CurrentAssetInvestmentAllocation}, 
//     DeFindexVaultTest
// };
// use crate::test::defindex_vault::ContractError;

// #[test]
// fn multi_instructions() {
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();
//     let strategy_params_token_0 = create_strategy_params_token_0(&test);
//     let assets: Vec<AssetStrategySet> = sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token_0.address.clone(),
//             strategies: strategy_params_token_0.clone()
//         }
//     ];

//     let defindex_contract = create_defindex_vault(
//         &test.env,
//         assets,
//         test.manager.clone(),
//         test.emergency_manager.clone(),
//         test.vault_fee_receiver.clone(),
//         2000u32,
//         test.defindex_protocol_receiver.clone(),
//         test.defindex_factory.clone(),
//         String::from_str(&test.env, "dfToken"),
//         String::from_str(&test.env, "DFT"),
//     );
//     let amount = 987654321i128;

//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

//     test.token_0_admin_client.mint(&users[0], &amount);
//     let user_balance = test.token_0.balance(&users[0]);
//     assert_eq!(user_balance, amount);

//     let df_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, 0i128);

//     defindex_contract.deposit(
//         &sorobanvec![&test.env, amount],
//         &sorobanvec![&test.env, amount],
//         &users[0],
//         &false
//     );

//     let df_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, amount - 1000);

//     let investments = sorobanvec![
//         &test.env,
//         Some(AssetInvestmentAllocation {
//             asset: test.token_0.address.clone(),
//             strategy_allocations: sorobanvec![
//                 &test.env,
//                 Some(StrategyAllocation {
//                     strategy_address: test.strategy_client_token_0.address.clone(),
//                     amount: amount,
//                 }),
//             ],
//         }),
//     ];

//     defindex_contract.invest(&investments);

//     let vault_balance = test.token_0.balance(&defindex_contract.address);
//     assert_eq!(vault_balance, 0);

//     // REBALANCE

//     let instruction_amount_0 = 200i128;
//     let instruction_amount_1 = 100i128;

//     let instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(instruction_amount_0),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//         Instruction {
//             action: ActionType::Invest,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(instruction_amount_1),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         }
//     ];

//     defindex_contract.rebalance(&instructions);

//     let vault_balance = test.token_0.balance(&defindex_contract.address);
//     assert_eq!(vault_balance, instruction_amount_1);
// }

// #[test]
// fn one_instruction() {
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();
//     let strategy_params_token_0 = create_strategy_params_token_0(&test);
//     let assets: Vec<AssetStrategySet> = sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token_0.address.clone(),
//             strategies: strategy_params_token_0.clone()
//         }
//     ];

//     let defindex_contract = create_defindex_vault(
//         &test.env,
//         assets,
//         test.manager.clone(),
//         test.emergency_manager.clone(),
//         test.vault_fee_receiver.clone(),
//         2000u32,
//         test.defindex_protocol_receiver.clone(),
//         test.defindex_factory.clone(),
//         String::from_str(&test.env, "dfToken"),
//         String::from_str(&test.env, "DFT"),
//     );
//     let amount = 987654321i128;

//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

//     test.token_0_admin_client.mint(&users[0], &amount);
//     let user_balance = test.token_0.balance(&users[0]);
//     assert_eq!(user_balance, amount);

//     let df_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, 0i128);

//     defindex_contract.deposit(
//         &sorobanvec![&test.env, amount],
//         &sorobanvec![&test.env, amount],
//         &users[0],
//         &false
//     );

//     let df_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, amount - 1000);

//     let investments = sorobanvec![
//         &test.env,
//         Some(AssetInvestmentAllocation {
//             asset: test.token_0.address.clone(),
//             strategy_allocations: sorobanvec![
//                 &test.env,
//                 Some(StrategyAllocation {
//                     strategy_address: test.strategy_client_token_0.address.clone(),
//                     amount: amount,
//                 }),
//             ],
//         }),
//     ];

//     defindex_contract.invest(&investments);

//     let vault_balance = test.token_0.balance(&defindex_contract.address);
//     assert_eq!(vault_balance, 0);

//     // REBALANCE

//     let instruction_amount_0 = 200i128;

//     let instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(instruction_amount_0),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];

//     defindex_contract.rebalance(&instructions);

//     let vault_balance = test.token_0.balance(&defindex_contract.address);
//     assert_eq!(vault_balance, instruction_amount_0);
// }

// #[test]
// fn empty_instructions(){
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();

//     let strategy_params_token_0 = create_strategy_params_token_0(&test);
//     let assets: Vec<AssetStrategySet> = sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token_0.address.clone(),
//             strategies: strategy_params_token_0.clone()
//         }
//     ];
//     let defindex_contract = create_defindex_vault(
//         &test.env,
//         assets,
//         test.manager.clone(),
//         test.emergency_manager.clone(),
//         test.vault_fee_receiver.clone(),
//         2000u32,
//         test.defindex_protocol_receiver.clone(),
//         test.defindex_factory.clone(),
//         String::from_str(&test.env, "dfToken"),
//         String::from_str(&test.env, "DFT"),
//     );
//     let amount: i128 = 987654321;
//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
//     test.token_0_admin_client.mint(&users[0], &amount);
//     let vault_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(vault_balance, 0i128);

//     defindex_contract.deposit(
//         &sorobanvec![&test.env, amount],
//         &sorobanvec![&test.env, amount],
//         &users[0],
//         &false
//     );
//     let df_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, amount - 1000);

//     let instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: None,
//             amount: None,
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];
//     let rebalance = defindex_contract.try_rebalance(&instructions);
//     assert_eq!(rebalance, Err(Ok(ContractError::MissingInstructionData)));

//     let no_strategy_instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: None,
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];
//     let rebalance = defindex_contract.try_rebalance(&no_strategy_instructions);
//     assert_eq!(rebalance, Err(Ok(ContractError::MissingInstructionData)));

//     let no_amount_instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: None,
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];
//     let rebalance = defindex_contract.try_rebalance(&no_amount_instructions);
//     assert_eq!(rebalance, Err(Ok(ContractError::MissingInstructionData)));
// }

// #[test]
// fn no_instructions(){
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();

//     let strategy_params_token_0 = create_strategy_params_token_0(&test);
//     let assets: Vec<AssetStrategySet> = sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token_0.address.clone(),
//             strategies: strategy_params_token_0.clone()
//         }
//     ];
//     let defindex_contract = create_defindex_vault(
//         &test.env,
//         assets,
//         test.manager.clone(),
//         test.emergency_manager.clone(),
//         test.vault_fee_receiver.clone(),
//         2000u32,
//         test.defindex_protocol_receiver.clone(),
//         test.defindex_factory.clone(),
//         String::from_str(&test.env, "dfToken"),
//         String::from_str(&test.env, "DFT"),
//     );
//     let amount: i128 = 987654321;
//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
//     test.token_0_admin_client.mint(&users[0], &amount);
//     let vault_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(vault_balance, 0i128);

//     defindex_contract.deposit(
//         &sorobanvec![&test.env, amount],
//         &sorobanvec![&test.env, amount],
//         &users[0],
//         &false
//     );
//     let df_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, amount - 1000);

//     let rebalance = defindex_contract.try_rebalance(&sorobanvec![&test.env]);
//     assert_eq!(rebalance, Err(Ok(ContractError::NoInstructions)));
// }

// #[test]
// fn insufficient_balance(){
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();

//     let strategy_params_token_0 = create_strategy_params_token_0(&test);
//     let assets: Vec<AssetStrategySet> = sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token_0.address.clone(),
//             strategies: strategy_params_token_0.clone()
//         }
//     ];
//     let defindex_contract = create_defindex_vault(
//         &test.env,
//         assets,
//         test.manager.clone(),
//         test.emergency_manager.clone(),
//         test.vault_fee_receiver.clone(),
//         2000u32,
//         test.defindex_protocol_receiver.clone(),
//         test.defindex_factory.clone(),
//         String::from_str(&test.env, "dfToken"),
//         String::from_str(&test.env, "DFT"),
//     );
//     let amount: i128 = 987654321;
//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
//     test.token_0_admin_client.mint(&users[0], &amount);
    
//     //Balance should be 0
//     let vault_balance = defindex_contract.balance(&users[0]);
//     assert_eq!(vault_balance, 0i128);

//     //Withdraw with no funds
//     let withdraw_no_funds_instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(amount + 1),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];

//     let withdraw_no_funds = defindex_contract.try_rebalance(&withdraw_no_funds_instructions);
//     assert_eq!(withdraw_no_funds, Err(Ok(ContractError::StrategyWithdrawError))); //Contract should respond 'Insuficient balance'?

//     //Invest with no funds
//     let invest_no_funds_instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Invest,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(1),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];
//     let invest_no_funds = defindex_contract.try_rebalance(&invest_no_funds_instructions);

//     //Contract should fail with error #10 no balance or panic the test
//     if invest_no_funds != Err(Err(InvokeError::Contract(10))) {
//         panic!("Expected error not returned");
//     }

//     //Deposit 987654321 stroops
//     defindex_contract.deposit(
//         &sorobanvec![&test.env, amount],
//         &sorobanvec![&test.env, amount],
//         &users[0],
//         &false
//     );
//     let df_balance: i128 = defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, amount - 1000);

//     //Withdraw more than available
//     let withdraw_instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::Withdraw,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(amount + 1),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     ];
//     let rebalance = defindex_contract.try_rebalance(&withdraw_instructions);
//     assert_eq!(rebalance, Err(Ok(ContractError::StrategyWithdrawError)));

//     let invest_instructions = sorobanvec!(
//         &test.env,
//         Instruction {
//             action: ActionType::Invest,
//             strategy: Some(test.strategy_client_token_0.address.clone()),
//             amount: Some(amount + 1),
//             swap_details_exact_in: OptionalSwapDetailsExactIn::None,
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         },
//     );

//     //Contract should fail with error #10 no balance
//     let rebalance = defindex_contract.try_rebalance(&invest_instructions);
//     if rebalance == Err(Err(InvokeError::Contract(10))) {
//         return;
//     } else {
//         panic!("Expected error not returned");
//     }
// }

// #[test]
// fn swap_exact_in() {
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();
//     let strategy_params_token_0 = create_strategy_params_token_0(&test);
//     let strategy_params_token_1 = create_strategy_params_token_1(&test);

//     // initialize with 2 assets
//     let assets: Vec<AssetStrategySet> = sorobanvec![
//         &test.env,
//         AssetStrategySet {
//             address: test.token_0.address.clone(),
//             strategies: strategy_params_token_0.clone()
//         },
//         AssetStrategySet {
//             address: test.token_1.address.clone(),
//             strategies: strategy_params_token_1.clone()
//         }
//     ];

//     let defindex_contract = create_defindex_vault(
//         &test.env,
//         assets,
//         test.manager.clone(),
//         test.emergency_manager.clone(),
//         test.vault_fee_receiver.clone(),
//         2000u32,
//         test.defindex_protocol_receiver.clone(),
//         test.defindex_factory.clone(),
//         String::from_str(&test.env, "dfToken"),
//         String::from_str(&test.env, "DFT"),
//     );
//     let amount0 = 123456789i128;
//     let amount1 = 987654321i128;

//     let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

//     test.token_0_admin_client.mint(&users[0], &amount0);
//     test.token_1_admin_client.mint(&users[0], &amount1);

//     let deposit_result=defindex_contract.deposit(
//         &sorobanvec![&test.env, amount0, amount1],
//         &sorobanvec![&test.env, amount0, amount1],
//         &users[0],
//         &false,
//     );

//     // check total managed funds
//     let mut total_managed_funds_expected = Map::new(&test.env);
//     let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
//         strategy_address: test.strategy_client_token_0.address.clone(),
//         amount: 0, // funds have not been invested yet!
//     }];
//     let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
//         strategy_address: test.strategy_client_token_1.address.clone(),
//         amount: 0, // funds have not been invested yet!
//     }];
//     total_managed_funds_expected.set(test.token_0.address.clone(), 
//         CurrentAssetInvestmentAllocation {
//             asset: test.token_0.address.clone(),
//             total_amount: amount0,
//             idle_amount: amount0,
//             invested_amount: 0i128,
//             strategy_allocations: strategy_investments_expected_token_0,
//         }
//     );
//     total_managed_funds_expected.set(test.token_1.address.clone(), 
//         CurrentAssetInvestmentAllocation {
//             asset: test.token_1.address.clone(),
//             total_amount: amount1,
//             idle_amount: amount1,
//             invested_amount: 0i128,
//             strategy_allocations: strategy_investments_expected_token_1,
//         }
//     );
//     let total_managed_funds = defindex_contract.fetch_total_managed_funds();
//     assert_eq!(total_managed_funds, total_managed_funds_expected);
    
//     let amount_in = 1_000_000;
//     //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
//     let expected_amount_out = 3987999;

//     let mut distribution_vec = Vec::new(&test.env);
//     // add one with part 1 and other with part 0
//     let mut path: Vec<Address> = Vec::new(&test.env);
//     path.push_back(test.token_0.address.clone());
//     path.push_back(test.token_1.address.clone());

//     let distribution_0 = DexDistribution {
//         protocol_id: String::from_str(&test.env, "soroswap"),
//         path,
//         parts: 1,
//     };
//     distribution_vec.push_back(distribution_0);

//     // Rebalance from here on
//     let instructions = sorobanvec![
//         &test.env,
//         Instruction {
//             action: ActionType::SwapExactIn,
//             strategy: None,
//             amount: None,
//             swap_details_exact_in: OptionalSwapDetailsExactIn::Some(SwapDetailsExactIn {
//                 token_in: test.token_0.address.clone(),
//                 token_out: test.token_1.address.clone(),
//                 amount_in: amount_in,
//                 amount_out_min: 0,
//                 distribution: distribution_vec,
//                 deadline: test.env.ledger().timestamp() + 3600u64,
//                 // router: test.soroswap_router.address.clone(),
//                 // pair: test.soroswap_pair.clone(),
//             }),
//             swap_details_exact_out: OptionalSwapDetailsExactOut::None,
//         }
//     ];

//     defindex_contract.rebalance(&instructions);

//     // check total managed funds
//     let mut total_managed_funds_expected = Map::new(&test.env);
//     let strategy_investments_expected_token_0 = sorobanvec![&test.env, StrategyAllocation {
//         strategy_address: test.strategy_client_token_0.address.clone(),
//         amount: 0, // funds have not been invested yet!
//     }];
//     let strategy_investments_expected_token_1 = sorobanvec![&test.env, StrategyAllocation {
//         strategy_address: test.strategy_client_token_1.address.clone(),
//         amount: 0, // funds have not been invested yet!
//     }];
//     total_managed_funds_expected.set(test.token_0.address.clone(), 
//         CurrentAssetInvestmentAllocation {
//             asset: test.token_0.address.clone(),
//             total_amount: amount0 - amount_in,
//             idle_amount: amount0 - amount_in,
//             invested_amount: 0i128,
//             strategy_allocations: strategy_investments_expected_token_0,
//         }
//     );
//     total_managed_funds_expected.set(test.token_1.address.clone(), 
//         CurrentAssetInvestmentAllocation {
//             asset: test.token_1.address.clone(),
//             total_amount: amount1 + expected_amount_out,
//             idle_amount: amount1 + expected_amount_out,
//             invested_amount: 0i128,
//             strategy_allocations: strategy_investments_expected_token_1,
//         }
//     );
//     let total_managed_funds = defindex_contract.fetch_total_managed_funds();
//     assert_eq!(total_managed_funds, total_managed_funds_expected);
    
// }

