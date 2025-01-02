use soroban_sdk::{testutils::{MockAuth, MockAuthInvoke}, vec as sorobanvec, Address, IntoVal, Map, String, Vec};

// use super::hodl_strategy::StrategyError;
use crate::test::{
    create_defindex_vault, create_hodl_strategy, create_strategy_params_token_0, create_strategy_params_token_1, defindex_vault::{
        AssetInvestmentAllocation, AssetStrategySet, ContractError, CurrentAssetInvestmentAllocation, RolesDataKey, Strategy, StrategyAllocation
    }, DeFindexVaultTest
};

// check that withdraw with negative amount after initialized returns error
#[test]
fn negative_amount() {
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = defindex_contract.try_withdraw(&-100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::NegativeNotAllowed)));
}

// check that withdraw without balance after initialized returns error AmountOverTotalSupply
#[test]
fn zero_total_supply() {
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

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    let result = defindex_contract.try_withdraw(&100i128, &users[0]);
    assert_eq!(result, Err(Ok(ContractError::AmountOverTotalSupply)));
}

// check that withdraw with not enough balance returns error InsufficientBalance
#[test]
fn not_enough_balance() {
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

    // We need to generate 2 users, to have more total supply than the amount to withdraw
    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    let amount_to_deposit = 567890i128;
    test.token_0_admin_client
        .mint(&users[0], &amount_to_deposit);
    test.token_0_admin_client
        .mint(&users[1], &amount_to_deposit);

    assert_eq!(test.token_0.balance(&users[0]), amount_to_deposit);
    assert_eq!(test.token_0.balance(&users[1]), amount_to_deposit);

    // first the user deposits
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[0],
        &false,
    );

    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[1],
        &false,
    );

    // check that the every user has vault shares
    assert_eq!(
        defindex_contract.balance(&users[0]),
        amount_to_deposit - 1000
    );
    assert_eq!(defindex_contract.balance(&users[1]), amount_to_deposit);
    // check that total supply of vault shares is indeed amount_to_deposit*2
    assert_eq!(defindex_contract.total_supply(), amount_to_deposit * 2);

    // now user 0 tries to withdraw amount_to_deposit - 1000 +1 (more that it has)

    let result = defindex_contract.try_withdraw(&(amount_to_deposit - 1000 + 1), &users[0]);
    assert_eq!(result, Err(Ok(ContractError::InsufficientBalance)));
}

#[test]
fn from_idle_one_asset_one_strategy_success() {
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
    let amount = 1234567890i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    let amount_to_deposit = 567890i128;
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit],
        &sorobanvec![&test.env, amount_to_deposit],
        &users[0],
        &false,
    );

    // Check Balances after deposit

    // Token balance of user
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount - amount_to_deposit);

    // Token balance of vault should be amount_to_deposit
    // Because balances are still in indle, balances are not in strategy, but in idle

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount_to_deposit);

    // Token balance of hodl strategy should be 0 (all in idle)
    let strategy_balance = test.token_0.balance(&test.strategy_client_token_0.address);
    assert_eq!(strategy_balance, 0);

    // Df balance of user should be equal to deposited amount
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit - 1000); // 1000  gets locked in the vault forever

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, //funds has not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount_to_deposit,
            idle_amount: amount_to_deposit,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // user decides to withdraw a portion of deposited amount
    let amount_to_withdraw = 123456i128;
    defindex_contract.withdraw(&amount_to_withdraw, &users[0]);

    // Check Balances after withdraw

    // Token balance of user should be amount - amount_to_deposit + amount_to_withdraw
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(
        user_balance,
        amount - amount_to_deposit + amount_to_withdraw
    );

    // Token balance of vault should be amount_to_deposit - amount_to_withdraw
    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, amount_to_deposit - amount_to_withdraw);

    // Token balance of hodl strategy should be 0 (all in idle)
    let strategy_balance = test.token_0.balance(&test.strategy_client_token_0.address);
    assert_eq!(strategy_balance, 0);

    // Df balance of user should be equal to deposited amount - amount_to_withdraw - 1000
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, amount_to_deposit - amount_to_withdraw - 1000);

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, //funds has not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: amount_to_deposit - amount_to_withdraw,
            idle_amount: amount_to_deposit - amount_to_withdraw,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // user tries to withdraw more than deposited amount
    let amount_to_withdraw_more = amount_to_deposit + 1;
    let result = defindex_contract.try_withdraw(&amount_to_withdraw_more, &users[0]);

    assert_eq!(result, Err(Ok(ContractError::AmountOverTotalSupply)));

    // // withdraw remaining balance
    let result =
        defindex_contract.withdraw(&(amount_to_deposit - amount_to_withdraw - 1000), &users[0]);

    assert_eq!(
        result,
        sorobanvec![&test.env, amount_to_deposit - amount_to_withdraw - 1000]
    );

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    let strategy_investments_expected = sorobanvec![
        &test.env,
        StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: 0, //funds has not been invested yet!
        }
    ];
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 1000,
            idle_amount: 1000,
            invested_amount: 0i128,
            strategy_allocations: strategy_investments_expected,
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
}

#[test]
fn from_idle_two_assets_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
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
    let amount = 1234567890i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    test.token_1_admin_client.mint(&users[0], &amount);
    assert_eq!(test.token_0.balance(&users[0]), amount);
    assert_eq!(test.token_0.balance(&users[0]), amount);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // Deposit
    let amount_to_deposit_0 = 567890i128;
    let amount_to_deposit_1 = 987654i128;
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
        &sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
        &users[0],
        &false,
    );

    // Check Balances after deposit

    // Token balance of user
    assert_eq!(
        test.token_0.balance(&users[0]),
        amount - amount_to_deposit_0
    );
    assert_eq!(
        test.token_1.balance(&users[0]),
        amount - amount_to_deposit_1
    );

    // Token balance of vault should be amount_to_deposit
    // Because balances are still in indle, balances are not in strategy, but in idle

    assert_eq!(
        test.token_0.balance(&defindex_contract.address),
        amount_to_deposit_0
    );
    assert_eq!(
        test.token_1.balance(&defindex_contract.address),
        amount_to_deposit_1
    );

    // Token balance of hodl strategy should be 0 (all in idle)
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        0
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        0
    );

    // Df balance of user should be equal to amount_to_deposit_0+amount_to_deposit_1 - 1000
    // 567890+987654-1000 = 1554544
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 1554544);

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 567890i128,
            idle_amount: 567890i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: 0, //funds has not been invested yet!
                }
            ],
        },
    );

    total_managed_funds_expected.set(
        test.token_1.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: 987654i128,
            idle_amount: 987654i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: test.strategy_client_token_1.address.clone(),
                    amount: 0, //funds has not been invested yet!
                }
            ],
        },
    );
    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);

    // user decides to withdraw a portion of their vault shares
    // from 1554544 it will withdraw 123456.
    // total shares = 567890+987654 = 1555544
    // asset 0 = withdaw_shares*total_asset_0/total_shares = 123456*567890/1555544 = 45070.681279347 = 45070
    // asset 1 = withdaw_shares*total_asset_1/total_shares = 123456*987654/1555544 = 78385.318720653 = 78385

    let amount_to_withdraw = 123456i128;
    let result = defindex_contract.withdraw(&amount_to_withdraw, &users[0]);

    // expected asset vec Vec<AssetStrategySet>
    // pub struct AssetStrategySet {
    //     pub address: Address,
    //     pub strategies: Vec<Strategy>,
    // }
    // pub struct Strategy {
    //     pub address: Address,
    //     pub name: String,
    //     pub paused: bool,
    // }
    let expected_asset_vec = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: sorobanvec![
                &test.env,
                Strategy {
                    address: test.strategy_client_token_0.address.clone(),
                    name: String::from_str(&test.env, "Strategy 1"),
                    paused: false,
                }
            ],
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: sorobanvec![
                &test.env,
                Strategy {
                    address: test.strategy_client_token_1.address.clone(),
                    name: String::from_str(&test.env, "Strategy 1"),
                    paused: false,
                }
            ],
        }
    ];
    assert_eq!(defindex_contract.get_assets(), expected_asset_vec);
    let expected_result = sorobanvec![&test.env, 45070, 78385];
    assert_eq!(result, expected_result);

    // Token balance of user
    assert_eq!(
        test.token_0.balance(&users[0]),
        amount - amount_to_deposit_0 + 45070
    );
    assert_eq!(
        test.token_1.balance(&users[0]),
        amount - amount_to_deposit_1 + 78385
    );

    // Token balance of vault (still idle)

    assert_eq!(
        test.token_0.balance(&defindex_contract.address),
        amount_to_deposit_0 - 45070
    );
    assert_eq!(
        test.token_1.balance(&defindex_contract.address),
        amount_to_deposit_1 - 78385
    );

    // Token balance of hodl strategy should be 0 (all in idle)
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        0
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        0
    );

    // Df balance of user should be equal to amount_to_deposit_0+amount_to_deposit_1 - 1000 - 123456
    // 567890+987654-1000 -123456 = 1434088
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 1431088);

    // check total manage funds
    let mut total_managed_funds_expected = Map::new(&test.env);
    total_managed_funds_expected.set(
        test.token_0.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 567890i128 - 45070,
            idle_amount: 567890i128 - 45070,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: 0, //funds has not been invested yet!
                }
            ],
        },
    );

    total_managed_funds_expected.set(
        test.token_1.address.clone(),
        CurrentAssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            total_amount: 987654i128 - 78385,
            idle_amount: 987654i128 - 78385,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![
                &test.env,
                StrategyAllocation {
                    strategy_address: test.strategy_client_token_1.address.clone(),
                    amount: 0, //funds has not been invested yet!
                }
            ],
        },
    );

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    assert_eq!(total_managed_funds, total_managed_funds_expected);
}

#[test]
fn from_strategy_one_asset_one_strategy_success() {
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
    let amount = 1000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

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

    defindex_contract.withdraw(&df_balance, &users[0]);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);
}

#[test]
fn from_strategies_one_asset_two_strategies_success() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_client_1 = create_hodl_strategy(&test.env, &test.token_0.address.clone());

    let strategy_params = sorobanvec![
        &test.env, 
        Strategy {
            name: String::from_str(&test.env, "strategy1"),
            address: test.strategy_client_token_0.address.clone(),
            paused: false,
        },
        Strategy {
            name: String::from_str(&test.env, "strategy2"),
            address: strategy_client_1.address.clone(),
            paused: false,
        },
    ];
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params.clone(),
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

    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 1);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.strategies.len(), strategy_params.len());

    let amount = 10_0_000_000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

    test.token_0_admin_client.mint(&users[0], &amount);
    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount);
    // here youll need to create a client for a token with the same address

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


    let amount_to_invest = 5_0_000_000i128;

    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                strategy_address: test.strategy_client_token_0.address.clone(),
                amount: amount_to_invest,
                }),
                Some(StrategyAllocation {
                strategy_address: strategy_client_1.address.clone(),
                amount: amount_to_invest,
                }),
            ],
        })
    ];

    defindex_contract.invest(&investments);

    let vault_balance = test.token_0.balance(&defindex_contract.address);
    assert_eq!(vault_balance, 0);

    defindex_contract.withdraw(&df_balance, &users[0]);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    let user_balance = test.token_0.balance(&users[0]);
    assert_eq!(user_balance, amount - 1000);
}

#[test]
fn from_strategies_two_asset_each_one_strategy_success() {
    // We will have two assets, each asset with one strategy
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
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
    // initialize
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
    // mint
    let amount = 987654321i128;
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);
    test.token_0_admin_client.mint(&users[0], &amount);
    test.token_1_admin_client.mint(&users[0], &amount);
    assert_eq!(test.token_0.balance(&users[0]), amount);
    assert_eq!(test.token_1.balance(&users[0]), amount);

    // deposit
    let amount_to_deposit_0 = 123456789i128;
    let amount_to_deposit_1 = 234567890i128;

    let deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
        &sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
        &users[0],
        &false,
    );

    // check deposit result. Ok((amounts, shares_to_mint))
    // shares to mint = 123456789 + 234567890 = 358024679
    assert_eq!(defindex_contract.total_supply(), 358024679);

    assert_eq!(
        deposit_result,
        (
            sorobanvec![&test.env, amount_to_deposit_0, amount_to_deposit_1],
            358024679,
            None
        )
    );

    // check balances
    assert_eq!(
        test.token_0.balance(&users[0]),
        amount - amount_to_deposit_0
    );
    assert_eq!(
        test.token_1.balance(&users[0]),
        amount - amount_to_deposit_1
    );

    // check vault balances
    assert_eq!(
        test.token_0.balance(&defindex_contract.address),
        amount_to_deposit_0
    );
    assert_eq!(
        test.token_1.balance(&defindex_contract.address),
        amount_to_deposit_1
    );

    // check strategy balances
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        0
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        0
    );

    // invest in strategies. We will invest 100% of the idle funds
    let investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: amount_to_deposit_0,
                }),
            ],
        }),
        Some(AssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_1.address.clone(),
                    amount: amount_to_deposit_1,
                }),
            ],
        }),
    ];

    defindex_contract.invest(&investments);

    // check vault balances
    assert_eq!(test.token_0.balance(&defindex_contract.address), 0);
    assert_eq!(test.token_1.balance(&defindex_contract.address), 0);

    // check strategy balances
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        amount_to_deposit_0
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        amount_to_deposit_1
    );

    //check user vault shares
    let df_balance = defindex_contract.balance(&users[0]);
    // vault shares should be amount_to_deposit_0 + amount_to_deposit_1 - 1000
    // 123456789 + 234567890 - 1000 = 358023679
    // but total vault shares are 358023679 + 1000 = 358024679
    assert_eq!(df_balance, 358023679);

    // User wants to withdraw 35353535 shares
    // from asset 0: total_funds_0 * withdraw_shares / total_shares
    // from asset 1: total_funds_1 * withdraw_shares / total_shares
    // user will get asset 0: 123456789 * 35353535 / 358024679 =  12190874.447789436 = 12190874
    // user will get asset 1: 234567890 * 35353535 / 358024679 = 23162660.552210564 = 23162660

    let amount_to_withdraw = 35353535i128;
    let result = defindex_contract.withdraw(&amount_to_withdraw, &users[0]);

    assert_eq!(defindex_contract.total_supply(), 322671144); //358024679- 35353535

    // check expected result
    let expected_result = sorobanvec![&test.env, 12190874, 23162660];
    assert_eq!(result, expected_result);

    // check user balances
    assert_eq!(
        test.token_0.balance(&users[0]),
        amount - amount_to_deposit_0 + 12190874
    );
    assert_eq!(
        test.token_1.balance(&users[0]),
        amount - amount_to_deposit_1 + 23162660
    );

    // check vault balances
    assert_eq!(test.token_0.balance(&defindex_contract.address), 0);
    assert_eq!(test.token_1.balance(&defindex_contract.address), 0);

    // check strategy balances
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        amount_to_deposit_0 - 12190874
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        amount_to_deposit_1 - 23162660
    );

    // check user vault shares // should be 358023679−35353535 = 322670144
    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 322670144);

    // now we deposit again to have a lot in idle funds
    // because the vault has 123456789 - 12190874 = 111,265,915 of token 0
    // and 234567890 - 23162660 = 211,405,230 of token 1
    // this proportion should be maintained

    // if user wants to deposit again 2,222,222 of token 0, it should invest from token 1:
    // 2222222 * 211405230   / 111265915   =  4222221.630236537 = 4222221

    let amount_to_deposit_0_new = 2222222i128;
    let amount_to_deposit_1_new = 4222221i128;

    let (amounts, shares_minted, asset_allocation) = defindex_contract.deposit(
        &sorobanvec![
            &test.env,
            amount_to_deposit_0_new,
            amount_to_deposit_1_new + 100
        ],
        &sorobanvec![
            &test.env,
            amount_to_deposit_0_new,
            amount_to_deposit_1_new - 100
        ],
        &users[0],
        &false,
    );

    // expected amounts
    let expected_amounts = sorobanvec![&test.env, 2222222, 4222221];
    assert_eq!(amounts, expected_amounts);
    assert_eq!(asset_allocation, None);

    // expected shares minted
    // total supply was 123456789+234567890 = 358024679
    // then we withdaw 35353535
    // total supply is 358024679 - 35353535 = 322671144
    // new shares to mint = total_supplly * amount_desired_target / reserve_target
    // 322671144 * 2222222 / 111265915 = 6444443.610264365 = 6444443
    assert_eq!(shares_minted, 6444443);

    assert_eq!(defindex_contract.total_supply(), 329115587); //358024679- 35353535 + 6444443

    // check user balances
    assert_eq!(
        test.token_0.balance(&users[0]),
        amount - amount_to_deposit_0 + 12190874 - 2222222
    );
    assert_eq!(
        test.token_1.balance(&users[0]),
        amount - amount_to_deposit_1 + 23162660 - 4222221
    );

    // check vault balance
    assert_eq!(test.token_0.balance(&defindex_contract.address), 2222222);
    assert_eq!(test.token_1.balance(&defindex_contract.address), 4222221);

    // check strategies balance
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        amount_to_deposit_0 - 12190874
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        amount_to_deposit_1 - 23162660
    );

    // user withdraws only from idle funds 644444 (10% of what just deposited)
    //  this should only affect idle funds

    let amount_to_withdraw = 644444i128;
    let result = defindex_contract.withdraw(&amount_to_withdraw, &users[0]);

    assert_eq!(defindex_contract.total_supply(), 328471143); //358024679- 35353535 + 6444443 - 644444

    // the new totqal supply was 322671144+6444443 = 329115587
    // the total managed funds for asset 0 was 2222222 (idle) + amount_to_deposit_0 - 12190874
    // = 2222222 + 123456789 - 12190874 = 113488137

    // the total managed funds for asset 1 was 4222221 (idle) + amount_to_deposit_1 - 23162660
    // = 4222221 + 234567890 - 23162660 = 215627451

    // the expected amount to withdraw for asset 0 was total_funds_0 * withdraw_shares / total_shares
    // = 113488137 * 644444 / 329115587 = 222222.075920178 = 222222

    // the expected amount to withdraw for asset 1 was total_funds_1 * withdraw_shares / total_shares
    // = 215627451 * 644444 / 329115587 = 422221.92603793 = 422221

    let expected_result = sorobanvec![&test.env, 222222, 422221];
    assert_eq!(result, expected_result);

    // check balances
    assert_eq!(
        test.token_0.balance(&users[0]),
        amount - amount_to_deposit_0 + 12190874 - 2222222 + 222222
    );
    assert_eq!(
        test.token_1.balance(&users[0]),
        amount - amount_to_deposit_1 + 23162660 - 4222221 + 422221
    );

    // check vault balance
    assert_eq!(
        test.token_0.balance(&defindex_contract.address),
        2222222 - 222222
    );
    assert_eq!(
        test.token_1.balance(&defindex_contract.address),
        4222221 - 422221
    );

    // check strategies balance

    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        amount_to_deposit_0 - 12190874
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        amount_to_deposit_1 - 23162660
    );

    assert_eq!(defindex_contract.total_supply(), 328471143); //358024679- 35353535 + 6444443 - 644444

    // check df tokens balance of user
    assert_eq!(defindex_contract.balance(&users[0]), 328470143);

    // // Now we will wihdraw the total remineder amount of vault shares of the user
    // // 328471143 - 1000 = 328470143
    let result = defindex_contract.withdraw(&328470143, &users[0]);

    // from the total supply 328471143, the user will take 328470143 (almost all)
    // for asset 0 this means
    // 2222222 - 222222 (idle) + amount_to_deposit_0 - 12190874
    // 2000000 + 123456789 - 12190874 = 113265915

    // for asset 1 this means
    // 4222221 - 422221 (idle) + amount_to_deposit_1 - 23162660
    // 3800000 + 234567890 - 23162660 = 215205230

    // amounts to withdraw
    // for asset 0: total_funds_0 * withdraw_shares / total_shares
    // 113265915 * 328470143 / 328471143 = 113265570.17240277 = 113265570

    // for asset 1: total_funds_1 * withdraw_shares / total_shares
    // 215205230 * 328470143 / 328471143 = 215204574.827591141 = 215204574

    let expected_result = sorobanvec![&test.env, 113265570, 215204574];
    assert_eq!(result, expected_result);

    assert_eq!(defindex_contract.balance(&users[0]), 0);
    assert_eq!(defindex_contract.balance(&defindex_contract.address), 1000);

    // CHECK IDLE BALANCES
    // check vault balance
    assert_eq!(test.token_0.balance(&defindex_contract.address), 0);
    assert_eq!(test.token_1.balance(&defindex_contract.address), 0);

    // check strategies balance, they will hold the rest
    // for asset 0: total_funds_0 * 1000 / total_shares
    // 113265915 - 113265570 = 345

    // for asset 1: total_funds_1 * withdraw_shares / total_shares
    // 215205230- 215204574 = 656
    assert_eq!(
        test.token_0.balance(&test.strategy_client_token_0.address),
        345
    );
    assert_eq!(
        test.token_1.balance(&test.strategy_client_token_1.address),
        656
    );
}

// test withdraw without mock all auths
#[test]
fn from_strategy_success_no_mock_all_auths() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

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

    // mint 
    let amount = 987654321i128;
    let mocked_token_client = test.token_0_admin_client.mock_all_auths();
    mocked_token_client.mint(&users[0], &amount);

    assert_eq!(test.token_0.balance(&users[0]), amount);

    let assets = defindex_contract.get_assets();
    assert_eq!(assets.len(), 1);
    let asset = assets.get(0).unwrap();
    assert_eq!(asset.strategies.len(), 1);


    let deposit_amount = 1_0_000_000i128;


    let amounts_desired = sorobanvec![&test.env, deposit_amount];
    let amounts_min = sorobanvec![&test.env, deposit_amount];
    let from =  &users[0].clone();
    let invest = false;

    defindex_contract.mock_auths(&[MockAuth {
        address: &from.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "deposit",
            args: (
                Vec::from_array(&test.env, [deposit_amount]),
                Vec::from_array(&test.env, [deposit_amount]),
                from.clone(),
                false
            ).into_val(&test.env),
            sub_invokes: &[MockAuthInvoke {
                contract: &mocked_token_client.address.clone(),
                fn_name: "transfer",
                args: sorobanvec![
                    &test.env,
                    from.clone().into_val(&test.env),
                    (defindex_contract.address).into_val(&test.env),
                    deposit_amount.into_val(&test.env),
                ],
                sub_invokes: &[],
            }],
        },
    }
    ]).deposit(&amounts_desired, &amounts_min, &from, &invest); 

    let invested_funds = defindex_contract.fetch_current_invested_funds().get(test.token_0.address.clone()).unwrap();
    let idle_funds = defindex_contract.fetch_current_idle_funds().get(test.token_0.address.clone()).unwrap();

    assert_eq!(idle_funds, deposit_amount);
    assert_eq!(invested_funds, 0);

    let withdraw_amount = deposit_amount/2;
    defindex_contract.mock_auths(&[MockAuth {
        address: &from.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "withdraw",
            args: (
                withdraw_amount,
                from,
            ).into_val(&test.env),
            sub_invokes: &[],
        },
    }
    ]).withdraw(&withdraw_amount, &from.clone());

    let invested_funds = defindex_contract.fetch_current_invested_funds().get(test.token_0.address.clone()).unwrap();
    let idle_funds = defindex_contract.fetch_current_idle_funds().get(test.token_0.address.clone()).unwrap();

    assert_eq!(idle_funds, deposit_amount - withdraw_amount);
    assert_eq!(invested_funds, 0);    
}
