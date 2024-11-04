use soroban_sdk::{vec as sorobanvec, String, Vec};

use crate::test::{create_strategy_params, DeFindexVaultTest};
use crate::test::defindex_vault::{AssetAllocation, ContractError};

#[test]
fn deposit_amounts_desired_wrong_length() {
    
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params = create_strategy_params(&test);
    
    // initialize with 2 assets
    let assets: Vec<AssetAllocation> = sorobanvec![
        &test.env,
        AssetAllocation {
            address: test.token0.address.clone(),
            strategies: strategy_params.clone()
        },
        AssetAllocation {
            address: test.token1.address.clone(),
            strategies: strategy_params.clone()
        }
    ];

    test.defindex_contract.initialize(
        &assets,
        &test.manager,
        &test.emergency_manager,
        &test.fee_receiver,
        &2000u32,
        &test.defindex_receiver,
        &test.defindex_factory,
        &String::from_str(&test.env, "dfToken"),
        &String::from_str(&test.env, "DFT"),
    );
    let amount = 1000i128;
    
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    

    let response = test.defindex_contract.try_deposit(
        &sorobanvec![&test.env, amount], // wrong amount desired
        &sorobanvec![&test.env, amount, amount], 
        &users[0]);

    assert_eq!(response, Err(Ok(ContractError::WrongAmuntsLength)));

}


#[test]
fn deposit_amounts_min_wrong_length() {
    todo!();
}


#[test]
fn deposit_amounts_desired_negative() {
    todo!();
}

#[test]
fn deposit_one_asset() {
    todo!();
}


#[test]
fn deposit_several_assets() {
    todo!();
}

// #[test]
// fn test_deposit_success() {
//     todo!();
// }
// #[test]
// fn test_withdraw_success() {
//     let test = DeFindexVaultTest::setup();
//     test.env.mock_all_auths();
//     let strategy_params = create_strategy_params(&test);
//     let assets: Vec<AssetAllocation> = sorobanvec![
//         &test.env,
//         AssetAllocation {
//             address: test.token0.address.clone(),
//
//             strategies: strategy_params.clone()
//         }
//     ];

//     test.defindex_contract.initialize(
//         &assets,
//         &test.manager,
//         &test.emergency_manager,
//         &test.fee_receiver,
//         &test.defindex_receiver,
//     );
//     let amount = 1000i128;
    
//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    
//     test.token0_admin_client.mint(&users[0], &amount);
//     let user_balance = test.token0.balance(&users[0]);
//     assert_eq!(user_balance, amount);
//     // here youll need to create a client for a token with the same address

//     let df_balance = test.defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, 0i128);

//     test.defindex_contract.deposit(&sorobanvec![&test.env, amount], &sorobanvec![&test.env, amount], &users[0]);

//     let df_balance = test.defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, amount);

//     test.defindex_contract.withdraw(&df_balance, &users[0]);
    
//     let df_balance = test.defindex_contract.balance(&users[0]);
//     assert_eq!(df_balance, 0i128);

//     let user_balance = test.token0.balance(&users[0]);
//     assert_eq!(user_balance, amount);
// }