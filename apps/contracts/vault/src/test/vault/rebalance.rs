use soroban_sdk::{vec as sorobanvec, InvokeError, String, Vec};

use crate::test::{
    create_defindex_vault, create_strategy_params_token0, defindex_vault::{
        ActionType, AssetInvestmentAllocation, AssetStrategySet, Instruction, OptionalSwapDetailsExactIn, OptionalSwapDetailsExactOut, StrategyAllocation
    }, DeFindexVaultTest
};
use crate::test::defindex_vault::ContractError;

#[test]
fn multi_instructions() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let amount = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    let vault_balance = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);

    // REBALANCE

    let instruction_amount_0 = 200i128;
    let instruction_amount_1 = 100i128;

    let instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(instruction_amount_0),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
        Instruction {
            action: ActionType::Invest,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(instruction_amount_1),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        }
    ];

    defindex_contract.rebalance(&instructions);

    let vault_balance = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_1);
}

#[test]
fn one_instruction() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let amount = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    let vault_balance = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);

    // REBALANCE

    let instruction_amount_0 = 200i128;

    let instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(instruction_amount_0),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];

    defindex_contract.rebalance(&instructions);

    let vault_balance = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_0);
}

#[test]
fn empty_instructions(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        }
    ];
    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let amount: i128 = 987654321;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token0_admin_client.mint(&users[0], &amount);
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: None,
            amount: None,
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];
    let rebalance = defindex_contract.try_rebalance(&instructions);
    assert_eq!(rebalance, Err(Ok(ContractError::MissingInstructionData)));

    let no_strategy_instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: None,
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];
    let rebalance = defindex_contract.try_rebalance(&no_strategy_instructions);
    assert_eq!(rebalance, Err(Ok(ContractError::MissingInstructionData)));

    let no_amount_instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: None,
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];
    let rebalance = defindex_contract.try_rebalance(&no_amount_instructions);
    assert_eq!(rebalance, Err(Ok(ContractError::MissingInstructionData)));
}

#[test]
fn no_instructions(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        }
    ];
    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let amount: i128 = 987654321;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token0_admin_client.mint(&users[0], &amount);
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let rebalance = defindex_contract.try_rebalance(&sorobanvec![&test.env]);
    assert_eq!(rebalance, Err(Ok(ContractError::NoInstructions)));
}

#[test]
fn insufficient_balance(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        }
    ];
    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );
    let amount: i128 = 987654321;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token0_admin_client.mint(&users[0], &amount);
    
    //Balance should be 0
    let vault_balance = defindex_contract.balance(&users[0]);
    assert_eq!(vault_balance, 0i128);

    //Withdraw with no funds
    let withdraw_no_funds_instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(amount + 1),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];

    let withdraw_no_funds = defindex_contract.try_rebalance(&withdraw_no_funds_instructions);
    assert_eq!(withdraw_no_funds, Err(Ok(ContractError::StrategyWithdrawError))); //Contract should respond 'Insuficient balance'?

    //Invest with no funds
    let invest_no_funds_instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Invest,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(1),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];
    let invest_no_funds = defindex_contract.try_rebalance(&invest_no_funds_instructions);

    //Contract should fail with error #10 no balance or panic the test
    if invest_no_funds != Err(Err(InvokeError::Contract(10))) {
        panic!("Expected error not returned");
    }

    //Deposit 987654321 stroops
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );
    let df_balance: i128 = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    //Withdraw more than available
    let withdraw_instructions = sorobanvec![
        &test.env,
        Instruction {
            action: ActionType::Withdraw,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(amount + 1),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    ];
    let rebalance = defindex_contract.try_rebalance(&withdraw_instructions);
    assert_eq!(rebalance, Err(Ok(ContractError::StrategyWithdrawError)));

    let invest_instructions = sorobanvec!(
        &test.env,
        Instruction {
            action: ActionType::Invest,
            strategy: Some(test.strategy_client_token0.address.clone()),
            amount: Some(amount + 1),
            swap_details_exact_in: OptionalSwapDetailsExactIn::None,
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
    );

    //Contract should fail with error #10 no balance
    let rebalance = defindex_contract.try_rebalance(&invest_instructions);
    if rebalance == Err(Err(InvokeError::Contract(10))) {
        return;
    } else {
        panic!("Expected error not returned");
    }
}



