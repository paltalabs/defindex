// use soroban_sdk::{testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, Address, IntoVal, Symbol, Vec, vec as sorobanvec};

// use crate::test::{DeFindexVaultTest, create_strategy_params};

// extern crate alloc;
// use alloc::vec;

// #[test]
// fn test_set_new_fee_receiver_by_fee_receiver() {
//     let test = DeFindexVaultTest::setup();
//     let strategy_params = create_strategy_params(&test);
//     let tokens: Vec<Address> = sorobanvec![&test.env, test.token0.address.clone(), test.token1.address.clone()];
//     let ratios: Vec<u32> = sorobanvec![&test.env, 1, 1];

//     test.defindex_contract.initialize(&test.emergency_manager, &test.fee_receiver, &test.manager, &tokens, &ratios, &strategy_params);

//     let fee_receiver_role = test.defindex_contract.get_palta_receiver();
//     assert_eq!(fee_receiver_role, test.fee_receiver);

//     let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
//     // Fee Receiver is setting the new fee receiver
//     test.defindex_contract.set_fee_receiver(&test.fee_receiver, &users[0]);

//     let new_fee_receiver_role = test.defindex_contract.get_palta_receiver();
//     assert_eq!(new_fee_receiver_role, users[0]);
// }
