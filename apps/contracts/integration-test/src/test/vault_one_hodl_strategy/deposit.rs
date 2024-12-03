use crate::{setup::create_vault_one_asset_hodl_strategy, test::IntegrationTest, vault::{VaultContractError, MINIMUM_LIQUIDITY}};
use soroban_sdk::{testutils::{MockAuth, MockAuthInvoke}, vec as svec, IntoVal, Vec};

#[test]
fn test_deposit_success() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user, &false);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, deposit_amount);

    let user_balance_after_deposit = enviroment.token.balance(user);
    assert_eq!(user_balance_after_deposit, user_starting_balance - deposit_amount);

    let df_balance = enviroment.vault_contract.balance(&user);
    assert_eq!(df_balance, deposit_amount - MINIMUM_LIQUIDITY);

    let total_supply = enviroment.vault_contract.total_supply();
    assert_eq!(total_supply, deposit_amount);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #10)")]
fn test_deposit_insufficient_balance() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 5_000_0_000_000i128; // Less than deposit amount

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user, &false);
}

#[test]
fn test_deposit_multiple_users() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 2);
    let user1 = &users[0];
    let user2 = &users[1];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user1, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user1, &user_starting_balance);
    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user2, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user2, &user_starting_balance);

    let user1_balance = enviroment.token.balance(user1);
    let user2_balance = enviroment.token.balance(user2);
    assert_eq!(user1_balance, user_starting_balance);
    assert_eq!(user2_balance, user_starting_balance);

    let deposit_amount = 10_000_0_000_000i128;
    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user1.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user1.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user1.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user1, &false);

    enviroment.vault_contract
    .mock_auths(&[MockAuth {
        address: &user2.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.vault_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&setup.env,[deposit_amount]),
                Vec::from_array(&setup.env,[deposit_amount]),
                user2.clone()
            ).into_val(&setup.env),
            sub_invokes: &[
                MockAuthInvoke {
                    contract: &enviroment.token.address.clone(),
                    fn_name: "transfer",
                    args: (
                        user2.clone(), 
                        &enviroment.vault_contract.address.clone(),
                        deposit_amount
                     ).into_val(&setup.env),
                    sub_invokes: &[]
                }
            ]
        },
    }])
    .deposit(&svec![&setup.env, deposit_amount], &svec![&setup.env, deposit_amount], &user2, &false);

    let vault_balance = enviroment.token.balance(&enviroment.vault_contract.address);
    assert_eq!(vault_balance, deposit_amount * 2);

    let user1_balance_after_deposit = enviroment.token.balance(user1);
    let user2_balance_after_deposit = enviroment.token.balance(user2);
    assert_eq!(user1_balance_after_deposit, user_starting_balance - deposit_amount);
    assert_eq!(user2_balance_after_deposit, user_starting_balance - deposit_amount);

    let df_balance_user1 = enviroment.vault_contract.balance(&user1);
    let df_balance_user2 = enviroment.vault_contract.balance(&user2);
    assert_eq!(df_balance_user1, deposit_amount - MINIMUM_LIQUIDITY);
    assert_eq!(df_balance_user2, deposit_amount);

    let total_supply = enviroment.vault_contract.total_supply();
    assert_eq!(total_supply, deposit_amount * 2);
}

#[test]
fn test_deposit_zero_amount() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = 0i128;
    let result = enviroment.vault_contract.mock_all_auths().try_deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false
    );

    assert_eq!(result, Err(Ok(VaultContractError::InsufficientAmount)));
}

#[test]
fn test_deposit_negative_amount() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = -10_000_0_000_000i128;
    let result = enviroment.vault_contract.mock_all_auths().try_deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false
    );

    assert_eq!(result, Err(Ok(VaultContractError::NegativeNotAllowed)));
}

#[test]
fn test_deposit_insufficient_minimum_liquidity() {
    let enviroment = create_vault_one_asset_hodl_strategy();
    let setup = enviroment.setup;

    let user_starting_balance = 100_000_0_000_000i128;

    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];

    enviroment.token_admin_client.mock_auths(&[MockAuth {
        address: &enviroment.token_admin.clone(),
        invoke: &MockAuthInvoke {
            contract: &enviroment.token.address.clone(),
            fn_name: "mint",
            args: (user, user_starting_balance,).into_val(&setup.env),
            sub_invokes: &[],
        },
    }]).mint(user, &user_starting_balance);
    let user_balance = enviroment.token.balance(user);
    assert_eq!(user_balance, user_starting_balance);

    let deposit_amount = MINIMUM_LIQUIDITY - 1;
    let result = enviroment.vault_contract.mock_all_auths().try_deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false
    );

    assert_eq!(result, Err(Ok(VaultContractError::InsufficientAmount)));
}