use soroban_sdk::{vec as sorobanvec, Address, InvokeError, Map, String, Vec};

use crate::test::defindex_vault::{ContractError, RolesDataKey};
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{
        AssetInvestmentAllocation, AssetStrategySet, CurrentAssetInvestmentAllocation, Instruction,
        StrategyAllocation,
    },
    DeFindexVaultTest,
};

#[test]
fn multi_instructions() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
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
    
    let amount = 987654321i128;

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

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);

    // REBALANCE

    let instruction_amount_0 = 200i128;
    let instruction_amount_1 = 100i128;

    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            instruction_amount_0
        ),
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            instruction_amount_1
        ),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_1);
}

#[test]
fn one_instruction() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
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
    
    let amount = 987654321i128;

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

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);

    // REBALANCE

    let instruction_amount_0 = 200i128;

    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            instruction_amount_0
        ),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_0);
}

#[test]
fn empty_instructions() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
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
    
    let amount: i128 = 987654321;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token_0_admin_client.mint(&users[0], &amount);
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);
}

#[test]
fn no_instructions() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
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
    
    let amount: i128 = 987654321;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token_0_admin_client.mint(&users[0], &amount);
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let rebalance = defindex_contract.try_rebalance(&test.rebalance_manager, &sorobanvec![&test.env]);
    assert_eq!(rebalance, Err(Ok(ContractError::NoInstructions)));
}

#[test]
fn insufficient_balance() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
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
    
    let amount: i128 = 987654321;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token_0_admin_client.mint(&users[0], &amount);

    //Balance should be 0
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    //Withdraw with no funds
    let withdraw_no_funds_instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(test.strategy_client_token_0.address.clone(), amount + 1),
    ];

    let withdraw_no_funds = defindex_contract.try_rebalance(&test.rebalance_manager, &withdraw_no_funds_instructions);
    assert_eq!(
        withdraw_no_funds,
        Err(Ok(ContractError::StrategyWithdrawError))
    ); //Contract should respond 'Insuficient balance'?

    let invest_no_funds_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 1),
    ];

    let invest_no_funds = defindex_contract.try_rebalance(&test.rebalance_manager, &invest_no_funds_instructions);

    //Contract should fail with error #10 no balance or panic the test
    if invest_no_funds != Err(Err(InvokeError::Contract(10))) {
        panic!("Expected error not returned");
    }

    //Deposit 987654321 stroops
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );
    let df_balance: i128 = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    //Withdraw more than available
    let withdraw_instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(test.strategy_client_token_0.address.clone(), amount + 1),
    ];

    let rebalance = defindex_contract.try_rebalance(&test.rebalance_manager, &withdraw_instructions);
    assert_eq!(rebalance, Err(Ok(ContractError::StrategyWithdrawError)));

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount + 1),
    ];

    //Contract should fail with error #10 no balance
    let rebalance = defindex_contract.try_rebalance(&test.rebalance_manager, &invest_instructions);
    if rebalance == Err(Err(InvokeError::Contract(10))) {
        return;
    } else {
        panic!("Expected error not returned");
    }
}

#[test]
fn swap_exact_in() {
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
        test.defindex_factory.clone(),
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

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.set(
        test.token_1.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1,
            idle_amount: amount1,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    let amount_in = 1_000_000;
    //(1000000×997×4000000000000000000)÷(1000000000000000000×1000+997×1000000) = 3987999,9
    let expected_amount_out = 3987999;

    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::SwapExactIn(
            test.token_0.address.clone(),
            test.token_1.address.clone(),
            amount_in,
            0, // amount_out_min
            test.env.ledger().timestamp() + 3600u64
        ),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0 - amount_in,
            idle_amount: amount0 - amount_in,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.set(
        test.token_1.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1 + expected_amount_out,
            idle_amount: amount1 + expected_amount_out,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
}

#[test]
fn swap_exact_out() {
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
        test.defindex_factory.clone(),
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

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.set(
        test.token_1.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1,
            idle_amount: amount1,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    let expected_amount_out = 5_000_000;
    // (r_in*amount_out)*1000 / (r_out - amount_out)*997
    // (1000000000000000000*5000000)*1000 / ((4000000000000000000 - 5000000)*997) + 1 = 1253762,2
    // because cealing div = 1253763
    let amount_in_should = 1253763;

    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token_0.address.clone());
    path.push_back(test.token_1.address.clone());

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::SwapExactOut(
            test.token_0.address.clone(),
            test.token_1.address.clone(),
            expected_amount_out, //amount_out
            amount_in_should,    // amount_in_max
            test.env.ledger().timestamp() + 3600u64
        ),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    // check total managed funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0 - amount_in_should,
            idle_amount: amount0 - amount_in_should,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.set(
        test.token_1.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: amount1 + expected_amount_out,
            idle_amount: amount1 + expected_amount_out,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_1,
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
}
