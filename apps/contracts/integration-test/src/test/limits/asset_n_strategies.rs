use soroban_sdk::{testutils::{Address as _, Ledger, MockAuth, MockAuthInvoke}, vec as svec, xdr::ContractCostType, Address, BytesN, IntoVal, Map, String, Vec};

use crate::{blend_strategy::{create_blend_strategy_contract, BlendStrategyClient}, factory::{AssetStrategySet, Strategy}, fixed_strategy::{create_fixed_strategy_contract, FixedStrategyClient}, hodl_strategy::create_hodl_strategy_contract, setup::{blend_setup::{create_blend_pool, BlendFixture, BlendPoolClient, Request}, create_soroswap_factory, create_soroswap_pool, create_soroswap_router, create_vault_one_asset_hodl_strategy, mock_mint, VAULT_FEE}, test::{limits::check_limits, EnvTestUtils, IntegrationTest, DAY_IN_LEDGERS, ONE_YEAR_IN_SECONDS}, token::create_token, vault::{defindex_vault_contract::{Instruction, VaultContractClient}, MINIMUM_LIQUIDITY}};

// 26 strategies is the maximum number of strategies that can be added to a vault before exceeding the instructions limit IN RUST TESTS
// With 26 strategies withdrawals are not possible due to the instruction limit
// 13 strategies is the maximum including withdrawals
#[test]
fn asset_n_strategies_hodl() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let manager = Address::generate(&setup.env);

    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "MultiStrategyVault");
    let vault_symbol = String::from_str(&setup.env, "MSVLT");

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let mut strategies = svec![&setup.env];
    let num_strategies = 13; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

    for i in 0..num_strategies {
        let strategy_name = format!("Strategy_{}", i);
        let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);
        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, &strategy_name),
            paused: false,
        });
    }

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    setup.env.budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );
    check_limits(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );
    check_limits(&setup.env, "Deposit");

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            deposit_amount / num_strategies as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    vault_contract.withdraw(&balance, &user);
    check_limits(&setup.env, "Withdraw");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn asset_n_strategies_hodl_panic() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let manager = Address::generate(&setup.env);

    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "MultiStrategyVault");
    let vault_symbol = String::from_str(&setup.env, "MSVLT");

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let mut strategies = svec![&setup.env];
    let num_strategies = 14; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

    for i in 0..num_strategies {
        let strategy_name = format!("Strategy_{}", i);
        let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);
        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, &strategy_name),
            paused: false,
        });
    }

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    setup.env.budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );
    check_limits(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );
    check_limits(&setup.env, "Deposit");

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            deposit_amount / num_strategies as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    vault_contract.withdraw(&balance, &user);
    check_limits(&setup.env, "Withdraw");
}

// FIXED Strategy limit is 10 including withdrawals in RUST
#[test]
fn asset_n_strategies_fixed() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let manager = Address::generate(&setup.env);

    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "MultiStrategyVault");
    let vault_symbol = String::from_str(&setup.env, "MSVLT");

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let mut strategies = svec![&setup.env];
    let num_strategies = 10; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

    for i in 0..num_strategies {
        let strategy_name = format!("Strategy_{}", i);
        let strategy_contract = create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);
        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, &strategy_name),
            paused: false,
        });
    }

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    setup.env.budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );
    check_limits(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );
    check_limits(&setup.env, "Deposit");

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            deposit_amount / num_strategies as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // harvest on all strategies
    for i in 0..num_strategies {
        setup.env.budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = FixedStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&manager);
        check_limits(&setup.env, "Harvest");
    }

    setup.env.budget().reset_unlimited();
    vault_contract.distribute_fees();
    check_limits(&setup.env, "Distribute Fees");

    // Simulate a user withdrawal touching all strategies
    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    vault_contract.withdraw(&balance, &user);
    check_limits(&setup.env, "Withdraw");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn asset_n_strategies_fixed_panic() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let manager = Address::generate(&setup.env);

    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "MultiStrategyVault");
    let vault_symbol = String::from_str(&setup.env, "MSVLT");

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let mut strategies = svec![&setup.env];
    let num_strategies = 12; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

    for i in 0..num_strategies {
        let strategy_name = format!("Strategy_{}", i);
        let strategy_contract = create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);
        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, &strategy_name),
            paused: false,
        });
    }

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    setup.env.budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );
    check_limits(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );
    check_limits(&setup.env, "Deposit");

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            deposit_amount / num_strategies as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // harvest on all strategies
    for i in 0..num_strategies {
        setup.env.budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = FixedStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&manager);
        check_limits(&setup.env, "Harvest");
    }

    setup.env.budget().reset_unlimited();
    vault_contract.distribute_fees();
    check_limits(&setup.env, "Distribute Fees");

    // Simulate a user withdrawal touching all strategies
    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    vault_contract.withdraw(&balance, &user);
    check_limits(&setup.env, "Withdraw");
}

// 2 Strategies is the limit for 1 asset and 2 Blend strategies 
#[test]
fn asset_n_strategies_blend() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    
    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);

    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let admin = Address::generate(&setup.env);

    let (blnd, blnd_client) = create_token(&setup.env, &admin);
    let (usdc, usdc_client) = create_token(&setup.env, &admin);
    let (_, xlm_client) = create_token(&setup.env, &admin);

    // Setting up soroswap pool
    let pool_admin = Address::generate(&setup.env);
    let amount_a = 100000000_0_000_000;
    let amount_b = 50000000_0_000_000;
    blnd_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    create_soroswap_pool(
        &setup.env,
        &soroswap_router,
        &pool_admin,
        &blnd.address,
        &usdc.address,
        &amount_a,
        &amount_b,
    );
    // End of setting up soroswap pool

    let blend_fixture = BlendFixture::deploy(&setup.env, &admin, &blnd.address, &usdc.address);

    let pool = create_blend_pool(&setup.env, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&setup.env, &pool);

    let mut strategies = svec![&setup.env];
    let num_strategies = 2; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

    for i in 0..num_strategies {
        let strategy_name = format!("Blend_{}", i);
        let strategy = create_blend_strategy_contract(
            &setup.env,
            &usdc.address,
            &pool,
            &0u32,
            &blnd.address,
            &soroswap_router.address,
            svec![&setup.env, 0u32, 1u32, 2u32, 3u32]
        );
        let strategy_contract = BlendStrategyClient::new(&setup.env, &strategy);

        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, &strategy_name),
            paused: false,
        });
    }

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "BlendVault");
    let vault_symbol = String::from_str(&setup.env, "BLNDVLT");
    let manager = Address::generate(&setup.env);

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: usdc.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    setup.env.budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true
    );
    check_limits(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );
    check_limits(&setup.env, "Deposit");

    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &false
    );
    check_limits(&setup.env, "Deposit");

    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            starting_balance * 2 / num_strategies as i128,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    // user_2 deposit directly into pool
    let user_2_starting_balance = 200_0000000;
    usdc_client.mint(&users[2], &user_2_starting_balance);
    pool_client.submit(
        &users[2],
        &users[2],
        &users[2],
        &svec![
            &setup.env,
            Request {
                request_type: 0,
                address: usdc.address.clone(),
                amount: user_2_starting_balance,
            },
        ],
    );


    // admin borrow back to 50% util rate
    let borrow_amount = (user_2_starting_balance + starting_balance * 2) / 2;
    pool_client.submit(
        &admin,
        &admin,
        &admin,
        &svec![
            &setup.env,
            Request {
                request_type: 4,
                address: usdc.address.clone(),
                amount: borrow_amount,
            },
        ],
    );

    let report = vault_contract.report();
    println!("report = {:?}", report);
    /*
     * Allow 1 week to pass
     */
    setup.env.jump(DAY_IN_LEDGERS * 7);

    pool_client.submit(
        &users[2],
        &users[2],
        &users[2],
        &svec![
            &setup.env,
            Request {
                request_type: 1,
                address: usdc.address.clone(),
                amount: user_2_starting_balance * 2,
            },
        ],
    );

    std::println!("-- Harvesting --");
    // harvest on all strategies
    for i in 0..num_strategies {
        setup.env.budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = FixedStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&manager);
        check_limits(&setup.env, "Harvest");
    }

    let report = vault_contract.report();
    println!("report = {:?}", report);

    let lock_fees = vault_contract.lock_fees(&None);
    println!("locked_fees = {:?}", lock_fees);

    println!("-- Distributing Fees --");
    setup.env.budget().reset_unlimited();    
    vault_contract.distribute_fees();
    check_limits(&setup.env, "Distribute Fees");

    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&users[0]);
    vault_contract.withdraw(&balance, &users[0]);
    check_limits(&setup.env, "Withdraw");

    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&users[1]);
    vault_contract.withdraw(&balance, &users[1]);
    check_limits(&setup.env, "Withdraw");
}

#[test]
#[should_panic(expected = "CPU instructions exceeded limit")]
fn asset_n_strategies_blend_panic() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    
    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);

    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let admin = Address::generate(&setup.env);

    let (blnd, blnd_client) = create_token(&setup.env, &admin);
    let (usdc, usdc_client) = create_token(&setup.env, &admin);
    let (_, xlm_client) = create_token(&setup.env, &admin);

    // Setting up soroswap pool
    let pool_admin = Address::generate(&setup.env);
    let amount_a = 100000000_0_000_000;
    let amount_b = 50000000_0_000_000;
    blnd_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    create_soroswap_pool(
        &setup.env,
        &soroswap_router,
        &pool_admin,
        &blnd.address,
        &usdc.address,
        &amount_a,
        &amount_b,
    );
    // End of setting up soroswap pool

    let blend_fixture = BlendFixture::deploy(&setup.env, &admin, &blnd.address, &usdc.address);

    let pool = create_blend_pool(&setup.env, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&setup.env, &pool);

    let mut strategies = svec![&setup.env];
    let num_strategies = 3; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

    for i in 0..num_strategies {
        let strategy_name = format!("Blend_{}", i);
        let strategy = create_blend_strategy_contract(
            &setup.env,
            &usdc.address,
            &pool,
            &0u32,
            &blnd.address,
            &soroswap_router.address,
            svec![&setup.env, 0u32, 1u32, 2u32, 3u32]
        );
        let strategy_contract = BlendStrategyClient::new(&setup.env, &strategy);

        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, &strategy_name),
            paused: false,
        });
    }

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "BlendVault");
    let vault_symbol = String::from_str(&setup.env, "BLNDVLT");
    let manager = Address::generate(&setup.env);

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: usdc.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    setup.env.budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true
    );
    check_limits(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );
    check_limits(&setup.env, "Deposit");

    setup.env.budget().reset_unlimited();
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &false
    );
    check_limits(&setup.env, "Deposit");

    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            starting_balance * 2 / num_strategies as i128,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    // user_2 deposit directly into pool
    let user_2_starting_balance = 200_0000000;
    usdc_client.mint(&users[2], &user_2_starting_balance);
    pool_client.submit(
        &users[2],
        &users[2],
        &users[2],
        &svec![
            &setup.env,
            Request {
                request_type: 0,
                address: usdc.address.clone(),
                amount: user_2_starting_balance,
            },
        ],
    );


    // admin borrow back to 50% util rate
    let borrow_amount = (user_2_starting_balance + starting_balance * 2) / 2;
    pool_client.submit(
        &admin,
        &admin,
        &admin,
        &svec![
            &setup.env,
            Request {
                request_type: 4,
                address: usdc.address.clone(),
                amount: borrow_amount,
            },
        ],
    );

    let report = vault_contract.report();
    println!("report = {:?}", report);
    /*
     * Allow 1 week to pass
     */
    setup.env.jump(DAY_IN_LEDGERS * 7);

    pool_client.submit(
        &users[2],
        &users[2],
        &users[2],
        &svec![
            &setup.env,
            Request {
                request_type: 1,
                address: usdc.address.clone(),
                amount: user_2_starting_balance * 2,
            },
        ],
    );

    std::println!("-- Harvesting --");
    // harvest on all strategies
    for i in 0..num_strategies {
        setup.env.budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = FixedStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&manager);
        check_limits(&setup.env, "Harvest");
    }

    let report = vault_contract.report();
    println!("report = {:?}", report);

    let lock_fees = vault_contract.lock_fees(&None);
    println!("locked_fees = {:?}", lock_fees);

    println!("-- Distributing Fees --");
    setup.env.budget().reset_unlimited();    
    vault_contract.distribute_fees();
    check_limits(&setup.env, "Distribute Fees");

    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&users[0]);
    vault_contract.withdraw(&balance, &users[0]);
    check_limits(&setup.env, "Withdraw");

    setup.env.budget().reset_unlimited();
    let balance = vault_contract.balance(&users[1]);
    vault_contract.withdraw(&balance, &users[1]);
    check_limits(&setup.env, "Withdraw");
}