use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use soroban_sdk::{vec as sorobanvec, Address, InvokeError, Map, String, Vec, IntoVal};

use crate::storage;
use crate::test::defindex_vault::{ContractError, RolesDataKey, Strategy};
use crate::test::{
    std,
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{
        AssetStrategySet, CurrentAssetInvestmentAllocation, Instruction,
        StrategyAllocation,
    },
    DeFindexVaultTest,
};

#[test]
fn multi_instructions() {
    let test = DeFindexVaultTest::setup();

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

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount
        ),
    ];

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                invest_instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &invest_instructions);

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

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_1);
}

#[test]
fn one_instruction() {
    let test = DeFindexVaultTest::setup();

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

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount
        ),
    ];

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                invest_instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &invest_instructions);

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

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_0);
}

#[test]
fn no_instructions() {
    let test = DeFindexVaultTest::setup();

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
    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount);
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false,
    );
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let empty_intructions = sorobanvec![&test.env];

    let rebalance = defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                empty_intructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .try_rebalance(&test.rebalance_manager, &empty_intructions);
    assert_eq!(rebalance, Err(Ok(ContractError::NoInstructions)));
}

#[test]
fn insufficient_balance() {
    let test = DeFindexVaultTest::setup();

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
    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount);

    //Balance should be 0
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    //Withdraw with no funds
    let withdraw_no_funds_instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(test.strategy_client_token_0.address.clone(), amount + 1),
    ];

    let withdraw_no_funds = defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                withdraw_no_funds_instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .try_rebalance(&test.rebalance_manager, &withdraw_no_funds_instructions);
    assert_eq!(
        withdraw_no_funds,
        Err(Ok(ContractError::StrategyWithdrawError))
    ); //Contract should respond 'Insuficient balance'?

    let invest_no_funds_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 1),
    ];

    let invest_no_funds = defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                invest_no_funds_instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .try_rebalance(&test.rebalance_manager, &invest_no_funds_instructions);

    //Contract should fail with error #10 no balance or panic the test
    if invest_no_funds != Err(Err(InvokeError::Contract(10))) {
        panic!("Expected error not returned");
    }

    //Deposit 987654321 stroops
    defindex_contract.mock_all_auths().deposit(
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

    let rebalance = defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                withdraw_instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .try_rebalance(&test.rebalance_manager, &withdraw_instructions);
    assert_eq!(rebalance, Err(Ok(ContractError::StrategyWithdrawError)));

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), amount + 1),
    ];

    //Contract should fail with error #10 no balance
    let rebalance = defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                invest_instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .try_rebalance(&test.rebalance_manager, &invest_instructions);
    if rebalance == Err(Err(InvokeError::Contract(10))) {
        return;
    } else {
        panic!("Expected error not returned");
    }
}

#[test]
fn swap_exact_in() {
    let test = DeFindexVaultTest::setup();

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

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount0);
    test.token_1_admin_client.mock_all_auths().mint(&users[0], &amount1);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
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

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0 - amount_in,
            idle_amount: amount0 - amount_in,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
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

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount0);
    test.token_1_admin_client.mock_all_auths().mint(&users[0], &amount1);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0,
            idle_amount: amount0,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
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

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &test.rebalance_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);

    // check total managed funds
    let mut total_managed_funds_expected = Vec::new(&test.env);
    let strategy_investments_expected_token_0 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    let strategy_investments_expected_token_1 = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: 0, // funds have not been invested yet!
            paused: false,
        }
    ];
    total_managed_funds_expected.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount0 - amount_in_should,
            idle_amount: amount0 - amount_in_should,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected_token_0,
        },
    );
    total_managed_funds_expected.push_back(
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
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn swap_from_unauthorized(){
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount0);
    test.token_1_admin_client.mock_all_auths().mint(&users[0], &amount1);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_in = 1_000_000;

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

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &users[2].clone(), //use unauthorized user
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);
}
#[test]
fn swap_wrong_asset_in(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        },
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
    let amount2 = 987654321i128;

    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    test.token_2_admin_client.mint(&users[0], &amount2);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_in = 1_000_000;

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::SwapExactIn(
            test.token_2.address.clone(),
            test.token_1.address.clone(),
            amount_in,
            0, // amount_out_min
            test.env.ledger().timestamp() + 3600u64
        ),
    ];

    let result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    assert_eq!(result, Err(Ok(ContractError::UnsupportedAsset)));

}
#[test]
fn swap_wrong_asset_out(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        },
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
    let amount2 = 987654321i128;

    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    test.token_2_admin_client.mint(&users[0], &amount2);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_in = 1_000_000;

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::SwapExactOut(
            test.token_0.address.clone(),
            test.token_2.address.clone(),
            amount_in,
            0, // amount_out_min
            test.env.ledger().timestamp() + 3600u64
        ),
    ];

    let result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    assert_eq!(result, Err(Ok(ContractError::UnsupportedAsset)));
}
#[test]
#[should_panic(expected = "HostError: Error(Contract, #410)")]
fn invest_negative_amount(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        },
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
    let amount2 = 987654321i128;

    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    test.token_2_admin_client.mint(&users[0], &amount2);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_in = -(1_000_000i128);

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_in,
        ),
    ];

    defindex_contract.rebalance(&test.rebalance_manager, &instructions);
}
#[test]
fn invest_wrong_address(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        },
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
    let amount2 = 987654321i128;

    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    test.token_2_admin_client.mint(&users[0], &amount2);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_to_invest = 1_000_000i128;

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_2.address.clone(),
            amount_to_invest,
        ),
    ];

    let result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    assert_eq!(result, Err(Ok(ContractError::StrategyNotFound)));
}
#[test]
fn invest_paused_strategy(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 123456789i128;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    let pause_strategy_0 = defindex_contract.try_pause_strategy(&test.strategy_client_token_0.address, &test.manager);
    assert_eq!(pause_strategy_0, Ok(Ok(())));

    let assets = defindex_contract.get_assets();


    // Check if strategies are paused
    let expected_strategy_0:Vec<Strategy> = sorobanvec![&test.env, 
        Strategy{
            address: test.strategy_client_token_0.address.clone(),
            name: String::from_str(&test.env, "Strategy 1"),
            paused: true,
        },
    ];

    let expected_assets:Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: expected_strategy_0,
        }
    ];
    assert_eq!(assets, expected_assets);

    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);

    // Rebalance from here on
    let amount_to_invest = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest,
        ),
    ];
    let result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    
    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);


    assert_eq!(result, Err(Ok(ContractError::StrategyPaused)));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")]
fn invest_more_than_idle_funds(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 1_0_000_000;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    // Check if invested funds are 0
    let managed_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap();
    assert_eq!(managed_funds.invested_amount, 0i128);

    let amount_to_invest = managed_funds.idle_amount + 1i128;
    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn invest_unauthorized(){
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount0);
    test.token_1_admin_client.mock_all_auths().mint(&users[0], &amount1);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_in = 1_000_000;

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_in,
        ),
    ];

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &users[2].clone(), //use unauthorized user
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);
}

#[test]
fn unwind_paused_strategy(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 123456789i128;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);

    let amount_to_invest = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount_to_invest);

    let pause_strategy_0 = defindex_contract.try_pause_strategy(&test.strategy_client_token_0.address, &test.manager);
    assert_eq!(pause_strategy_0, Ok(Ok(())));

    let assets = defindex_contract.get_assets();

    // Check if strategies are paused
    let expected_strategy_0:Vec<Strategy> = sorobanvec![&test.env, 
        Strategy{
            address: test.strategy_client_token_0.address.clone(),
            name: String::from_str(&test.env, "Strategy 1"),
            paused: true,
        },
    ];

    let expected_assets:Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: expected_strategy_0,
        }
    ];
    assert_eq!(assets, expected_assets);

    // Rebalance from here on
    let amount_to_unwind = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            amount_to_unwind,
        ),
    ];
    let unwind_result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    
    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount_to_invest-amount_to_unwind);
    assert_eq!(unwind_result, Ok(Ok(())));
}

#[test]
fn unwind_wrong_address(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 123456789i128;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);

    let amount_to_invest = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount_to_invest);

    let pause_strategy_0 = defindex_contract.try_pause_strategy(&test.strategy_client_token_0.address, &test.manager);
    assert_eq!(pause_strategy_0, Ok(Ok(())));

    let assets = defindex_contract.get_assets();

    // Check if strategies are paused
    let expected_strategy_0:Vec<Strategy> = sorobanvec![&test.env, 
        Strategy{
            address: test.strategy_client_token_0.address.clone(),
            name: String::from_str(&test.env, "Strategy 1"),
            paused: true,
        },
    ];

    let expected_assets:Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: expected_strategy_0,
        }
    ];
    assert_eq!(assets, expected_assets);

    // Rebalance from here on
    let amount_to_unwind = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            users[0].clone(),
            amount_to_unwind,
        ),
    ];
    let unwind_result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    
    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount_to_invest);
    assert_eq!(unwind_result, Err(Ok(ContractError::StrategyWithdrawError)));
}

#[test]
fn unwind_negative_amount(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 123456789i128;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);

    let amount_to_invest = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount_to_invest,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount_to_invest);

    let pause_strategy_0 = defindex_contract.try_pause_strategy(&test.strategy_client_token_0.address, &test.manager);
    assert_eq!(pause_strategy_0, Ok(Ok(())));

    let assets = defindex_contract.get_assets();

    // Check if strategies are paused
    let expected_strategy_0:Vec<Strategy> = sorobanvec![&test.env, 
        Strategy{
            address: test.strategy_client_token_0.address.clone(),
            name: String::from_str(&test.env, "Strategy 1"),
            paused: true,
        },
    ];

    let expected_assets:Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: expected_strategy_0,
        }
    ];
    assert_eq!(assets, expected_assets);

    // Rebalance from here on
    let amount_to_unwind = 1_000_000i128;
    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            -amount_to_unwind,
        ),
    ];
    let unwind_result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);
    
    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount_to_invest);
    assert_eq!(unwind_result, Err(Ok(ContractError::StrategyWithdrawError)));
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn unwind_unauthorized(){
    let test = DeFindexVaultTest::setup();

    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

    test.token_0_admin_client.mock_all_auths().mint(&users[0], &amount0);
    test.token_1_admin_client.mock_all_auths().mint(&users[0], &amount1);

    defindex_contract.mock_all_auths().deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let amount_in = 1_000_000;

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            amount_in,
        ),
    ];

    defindex_contract
    .mock_auths(&[MockAuth {
        address: &users[2].clone(), //use unauthorized user
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "rebalance",
            args: (
                test.rebalance_manager.clone(),
                instructions.clone(),
            )
                .into_val(&test.env),
            sub_invokes: &[],
        },
    }])
    .rebalance(&test.rebalance_manager, &instructions);
}

#[test]
fn unwind_over_max(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 1_0_000_000i128;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);

    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount0,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount0);

    let withdraw_amount = amount0 + 1;

    // Try to unwind more than invested (should fail)
    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            withdraw_amount,
        ),
    ];
    let unwind_result = defindex_contract.try_rebalance(&test.rebalance_manager, &instructions);

    // Check that the unwind has no effects
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount0);
    assert_eq!(unwind_result, Err(Ok(ContractError::StrategyWithdrawError)));
}

#[test]
fn should_report(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);

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
    
    let amount0 = 1_0_000_000i128;

    test.token_0_admin_client.mint(&users[0], &amount0);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    // Check if invested funds are 0
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, 0i128);

    // Get the initial report
    let initial_report = test.env.as_contract(&defindex_contract.address, || storage::get_report(&test.env, &test.strategy_client_token_0.address.clone()));
    std::println!("Initial report: {:?}", initial_report);
    //Invest
    let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            amount0,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);

    // Check investment effects
    let invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    assert_eq!(invested_funds, amount0);

    // Get the report after investment
    let report_after_investment = test.env.as_contract(&defindex_contract.address, || storage::get_report(&test.env, &test.strategy_client_token_0.address.clone()));
    std::println!("Report after investment: {:?}", report_after_investment);
    // Compare reports
    assert_ne!(initial_report, report_after_investment);

    // Unwind
    let withdraw_amount = amount0;

    let instructions = sorobanvec![
        &test.env,
        Instruction::Unwind(
            test.strategy_client_token_0.address.clone(),
            withdraw_amount,
        ),
    ];
    defindex_contract.rebalance(&test.rebalance_manager, &instructions);
    // Get report after unwind
    let report_after_unwind = test.env.as_contract(&defindex_contract.address, || storage::get_report(&test.env, &test.strategy_client_token_0.address.clone()));

    assert_ne!(report_after_investment, report_after_unwind);
    std::println!("Report after unwind: {:?}", report_after_unwind);
}