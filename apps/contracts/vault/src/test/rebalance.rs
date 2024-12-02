use soroban_sdk::{vec as sorobanvec, String, Vec};

use crate::test::{
    create_strategy_params_token0,
    defindex_vault::{
        ActionType, 
        AssetStrategySet, 
        Instruction, 
        AssetInvestmentAllocation,  
        StrategyInvestment,
        OptionalSwapDetailsExactIn,
        OptionalSwapDetailsExactOut,
    },
    DeFindexVaultTest,
};

#[test]
fn rebalance() {
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

    test.defindex_contract.initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.vault_fee_receiver,
        &2000u32,
        &test.defindex_protocol_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );
    let amount = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token0.address.clone(),
            strategy_investments: sorobanvec![
                &test.env,
                Some(StrategyInvestment {
                    strategy: test.strategy_client_token0.address.clone(),
                    amount: amount,
                }),
            ],
        }),
    ];

    test.defindex_contract.invest(&investments);

    let vault_balance = test.token0.balance(&test.defindex_contract.address);
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

    test.defindex_contract.rebalance(&instructions);

    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, instruction_amount_1);
}
