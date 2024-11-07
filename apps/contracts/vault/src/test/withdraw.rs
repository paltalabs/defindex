use soroban_sdk::{vec as sorobanvec, String, Vec};

use super::hodl_strategy::StrategyError;
use crate::test::{
    create_strategy_params_token0,
    defindex_vault::{AssetAllocation, Investment},
    DeFindexVaultTest,
};

#[test]
fn test_withdraw_from_idle_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
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
    let amount = 1234567890i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    let amount_to_deposit = 567890i128;
    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[0],
    );

    // Check Balances after deposit

    // Token balance of user
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount - amount_to_deposit);

    // Token balance of vault should be amount_to_deposit
    // Because balances are still in indle, balances are not in strategy, but in idle

    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount_to_deposit);

    // Token balance of hodl strategy should be 0 (all in idle)
    let strategy_balance = test.token0.balance(&test.strategy_client_token0.address);
    assert_eq!(strategy_balance, 0);

    // Df balance of user should be equal to deposited amount
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit);

    // user decides to withdraw a portion of deposited amount
    let amount_to_withdraw = 123456i128;
    test.defindex_contract
        .withdraw(&amount_to_withdraw, &users[0]);

    // Check Balances after withdraw

    // Token balance of user should be amount - amount_to_deposit + amount_to_withdraw
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(
        user_balance,
        amount - amount_to_deposit + amount_to_withdraw
    );

    // Token balance of vault should be amount_to_deposit - amount_to_withdraw
    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, amount_to_deposit - amount_to_withdraw);

    // Token balance of hodl strategy should be 0 (all in idle)
    let strategy_balance = test.token0.balance(&test.strategy_client_token0.address);
    assert_eq!(strategy_balance, 0);

    // Df balance of user should be equal to deposited amount - amount_to_withdraw
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit - amount_to_withdraw);

    // user tries to withdraw more than deposited amount
    let amount_to_withdraw_more = amount_to_deposit + 1;
    let result = test
        .defindex_contract
        .try_withdraw(&amount_to_withdraw_more, &users[0]);
    // just check if is error
    assert_eq!(result.is_err(), true);

    // TODO test corresponding error

    // withdraw remaining balance
    test.defindex_contract
        .withdraw(&(amount_to_deposit - amount_to_withdraw), &users[0]);

    // // result is err

    // assert_eq!(result, Err(Ok(StrategyError::InsufficientBalance)));

    // result should be error from contract

    // let df_balance = test.defindex_contract.balance(&users[0]);
    // assert_eq!(df_balance, 0i128);

    // let user_balance = test.token0.balance(&users[0]);
    // assert_eq!(user_balance, amount);
}

#[test]
fn test_withdraw_from_strategy_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token0 = create_strategy_params_token0(&test);
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    test.defindex_contract.deposit(
        &sorobanvec![&test.env, amount],
        &sorobanvec![&test.env, amount],
        &users[0],
    );

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount);

    let investments = sorobanvec![
        &test.env,
        Investment {
            amount: amount,
            strategy: test.strategy_client_token0.address.clone()
        }
    ];

    test.defindex_contract.invest(&investments);

    let vault_balance = test.token0.balance(&test.defindex_contract.address);
    assert_eq!(vault_balance, 0);

    test.defindex_contract.withdraw(&df_balance, &users[0]);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount);
}