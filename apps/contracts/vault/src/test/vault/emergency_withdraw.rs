use soroban_sdk::{vec as sorobanvec, String, Vec};

use crate::test::{
    create_defindex_vault, create_strategy_params_token0, defindex_vault::{AssetInvestmentAllocation, AssetStrategySet, StrategyAllocation
    }, DeFindexVaultTest
};

#[test]
fn withdraw_success() {
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

    // Deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
        &false
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount - 1000);   

    // Balance of the token0 on the vault should be `amount` since it is deposited into the vault first
    let vault_balance_of_token = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);
    
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

    // Balance of the token0 on the vault should be 0
    let vault_balance_of_token = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, 0);

    // Balance of the vault on the strategy contract should be `amount`
    let strategy_balance = test
        .strategy_client_token0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, amount);

    defindex_contract.emergency_withdraw(
        &strategy_params_token0.first().unwrap().address,
        &test.emergency_manager,
    );

    // Balance of the vault on the strategy should be 0
    let strategy_balance = test
        .strategy_client_token0
        .balance(&defindex_contract.address);
    assert_eq!(strategy_balance, 0);

    // Balance of the token0 on the vault should be `amount`
    let vault_balance_of_token = test.token0.balance(&defindex_contract.address);
    assert_eq!(vault_balance_of_token, amount);

    // check if strategy is paused
    let asset = defindex_contract.get_assets().first().unwrap();
    assert_eq!(asset.strategies.first().unwrap().paused, true);
}
