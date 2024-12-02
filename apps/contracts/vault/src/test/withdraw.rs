use soroban_sdk::{vec as sorobanvec, String, Vec};

// use super::hodl_strategy::StrategyError;
use crate::test::{
    create_strategy_params_token0,
    defindex_vault::{
        AssetStrategySet,
        AssetInvestmentAllocation,  
        StrategyInvestment, 
        ContractError
    },
    DeFindexVaultTest,
};


#[test]
fn test_withdraw_not_yet_initialized() {
    let test = DeFindexVaultTest::setup();
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::NotInitialized)));
}

// check that withdraw with negative amount after initialized returns error
#[test]
fn test_withdraw_negative_amount() {
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&-100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::NegativeNotAllowed)));
}


// check that withdraw without balance after initialized returns error InsufficientBalance
#[test]
fn test_withdraw_insufficient_balance() {
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = test.defindex_contract.try_withdraw(&100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::InsufficientBalance)));
}

#[test]
fn test_withdraw_from_idle_success() { 
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
        &false
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
    assert_eq!(df_balance, amount_to_deposit - 1000 ); // 1000  gets locked in the vault forever

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

    // Df balance of user should be equal to deposited amount - amount_to_withdraw - 1000 
    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit - amount_to_withdraw - 1000);

    // user tries to withdraw more than deposited amount
    let amount_to_withdraw_more = amount_to_deposit + 1;
    let result = test
        .defindex_contract
        .try_withdraw(&amount_to_withdraw_more, &users[0]);
    
    assert_eq!(result, 
        Err(Ok(ContractError::InsufficientBalance)));


    // // withdraw remaining balance
   let result= test.defindex_contract
        .withdraw(&(amount_to_deposit - amount_to_withdraw - 1000), &users[0]);

    assert_eq!(result, sorobanvec![&test.env, amount_to_deposit - amount_to_withdraw - 1000]);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);
}

#[test]
fn test_withdraw_from_strategy_success() {
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

    test.defindex_contract.withdraw(&df_balance, &users[0]);

    let df_balance = test.defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);
}
 

// test withdraw without mock all auths
#[test]
fn test_withdraw_from_strategy_success_no_mock_all_auths() {
    todo!();
}