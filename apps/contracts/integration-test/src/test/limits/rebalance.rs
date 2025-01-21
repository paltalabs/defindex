use soroban_sdk::{testutils::Address as _, vec as svec, Address, BytesN, Map, String};

use crate::{blend_strategy::{create_blend_strategy_contract, BlendStrategyClient}, factory::{AssetStrategySet, Strategy}, fixed_strategy::create_fixed_strategy_contract, hodl_strategy::create_hodl_strategy_contract, setup::{blend_setup::{create_blend_pool, BlendFixture, BlendPoolClient}, create_soroswap_factory, create_soroswap_pool, create_soroswap_router, VAULT_FEE}, test::{EnvTestUtils, IntegrationTest, DAY_IN_LEDGERS}, token::create_token, vault::defindex_vault_contract::{Instruction, VaultContractClient}};

use super::check_limits;

#[test]
fn asset_one_strategy_hodl_rebalance() {
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

    let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);
    strategies.push_back(Strategy {
        address: strategy_contract.address.clone(),
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;

    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );

    let num_investments = 29;

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (deposit_amount - 1000) / num_investments as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    // Checking unwind limit
    let balance_on_strategy = strategy_contract.balance(&vault_contract.address);
    let num_unwinds = 29;

    let mut unwind_instructions = svec![&setup.env];
    for _ in 0..num_unwinds {
        unwind_instructions.push_back(Instruction::Unwind(
            strategies.first().unwrap().address.clone(),
            (balance_on_strategy - 1000) / num_unwinds as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwind_instructions);
    check_limits(&setup.env, "Unwind");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn asset_one_strategy_hodl_rebalance_panic_invest() {
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

    let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);
    strategies.push_back(Strategy {
        address: strategy_contract.address.clone(),
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;

    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );

    let num_investments = 30;

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (deposit_amount - 1000) / num_investments as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn asset_one_strategy_hodl_rebalance_panic_unwind() {
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

    let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);
    strategies.push_back(Strategy {
        address: strategy_contract.address.clone(),
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;

    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );

    let num_investments = 30;

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (deposit_amount - 1000) / num_investments as i128,
        ));
    }

    // Rebalance
    vault_contract.rebalance(&manager, &invest_instructions);

    // Checking unwind limit
    let balance_on_strategy = strategy_contract.balance(&vault_contract.address);
    let num_unwinds = 30;

    let mut unwind_instructions = svec![&setup.env];
    for _ in 0..num_unwinds {
        unwind_instructions.push_back(Instruction::Unwind(
            strategies.first().unwrap().address.clone(),
            (balance_on_strategy - 1000) / num_unwinds as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwind_instructions);
    check_limits(&setup.env, "Unwind");
}

#[test]
fn asset_one_strategy_fixed_rebalance() {
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

    let strategy_contract = create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);
    strategies.push_back(Strategy {
        address: strategy_contract.address.clone(),
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );

    let num_investments = 24;

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (deposit_amount - 1000) / num_investments as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // harvest on strategy
    strategy_contract.harvest(&manager);

    vault_contract.report();
    vault_contract.lock_fees(&None);
    vault_contract.distribute_fees();

    let balance_on_strategy = strategy_contract.balance(&vault_contract.address);
    let num_unwinds = 25;

    // Prepare rebalance instructions for all strategies
    let mut unwind_instructions = svec![&setup.env];
    for _ in 0..num_unwinds {
        unwind_instructions.push_back(Instruction::Unwind(
            strategies.first().unwrap().address.clone(),
            (balance_on_strategy - 1000) / num_unwinds as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwind_instructions);
    check_limits(&setup.env, "Unwind");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn asset_one_strategy_fixed_rebalance_panic_invest() {
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

    let strategy_contract = create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);
    strategies.push_back(Strategy {
        address: strategy_contract.address.clone(),
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );

    let num_investments = 25;

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (deposit_amount - 1000) / num_investments as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn asset_one_strategy_fixed_rebalance_panic_unwind() {
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

    let strategy_contract = create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);
    strategies.push_back(Strategy {
        address: strategy_contract.address.clone(),
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: strategies.clone(),
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let user_starting_balance = 10000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    token_admin_client.mint(user, &user_starting_balance);

    let deposit_amount = 100000_0_000_000i128;
    vault_contract.deposit(
        &svec![&setup.env, deposit_amount],
        &svec![&setup.env, deposit_amount],
        &user,
        &false,
    );

    let num_investments = 24;

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (deposit_amount - 1000) / num_investments as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // harvest on strategy
    strategy_contract.harvest(&manager);

    vault_contract.report();
    vault_contract.lock_fees(&None);
    vault_contract.distribute_fees();

    let balance_on_strategy = strategy_contract.balance(&vault_contract.address);
    let num_unwinds = 26;

    // Prepare rebalance instructions for all strategies
    let mut unwind_instructions = svec![&setup.env];
    for _ in 0..num_unwinds {
        unwind_instructions.push_back(Instruction::Unwind(
            strategies.first().unwrap().address.clone(),
            (balance_on_strategy - 1000) / num_unwinds as i128,
        ));
    }

    // Rebalance
    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwind_instructions);
    check_limits(&setup.env, "Unwind");
}

#[test]
fn asset_one_strategy_blend_rebalance() {
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

    let mut strategies = svec![&setup.env];

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
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

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

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &false
    );

    let num_investments = 4;

    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (starting_balance * 2 - 1000) / num_investments as i128,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    let balance_on_strategy = strategy_contract.balance(&vault_contract.address);
    let num_unwinds = 5;

    let mut unwinds_instructions = svec![&setup.env];
    for _ in 0..num_unwinds {
        unwinds_instructions.push_back(Instruction::Unwind(
            strategies.first().unwrap().address.clone(),
            (balance_on_strategy - 1000) / num_unwinds as i128,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwinds_instructions);
    check_limits(&setup.env, "Unwind");
}

#[test]
#[should_panic(expected = "CPU instructions exceeded limit")]
fn asset_one_strategy_blend_rebalance_panic_invest() {
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

    let mut strategies = svec![&setup.env];

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
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

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

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &false
    );

    let num_investments = 5;

    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (starting_balance * 2 - 1000) / num_investments as i128,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");
}

#[test]
#[should_panic(expected = "CPU instructions exceeded limit")]
fn asset_one_strategy_blend_rebalance_panic_unwind() {
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

    let mut strategies = svec![&setup.env];

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
        name: String::from_str(&setup.env, "strategy_name"),
        paused: false,
    });

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

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );

    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &false
    );

    let num_investments = 4;

    let mut invest_instructions = svec![&setup.env];
    for _ in 0..num_investments {
        invest_instructions.push_back(Instruction::Invest(
            strategies.first().unwrap().address.clone(),
            (starting_balance * 2 - 1000) / num_investments as i128,
        ));
    }

    vault_contract.rebalance(&manager, &invest_instructions);

    let balance_on_strategy = strategy_contract.balance(&vault_contract.address);
    let num_unwinds = 6;

    let mut unwinds_instructions = svec![&setup.env];
    for _ in 0..num_unwinds {
        unwinds_instructions.push_back(Instruction::Unwind(
            strategies.first().unwrap().address.clone(),
            (balance_on_strategy - 1000) / num_unwinds as i128,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwinds_instructions);
    check_limits(&setup.env, "Unwind");
}

#[test]
fn two_assets_swap_limits_rebalance() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (xlm, xlm_client) = create_token(&setup.env, &token_admin);
    let (usdc, usdc_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    // Setting up soroswap pool
    let pool_admin = Address::generate(&setup.env);
    let amount_a = 100000000000_0_000_000;
    let amount_b = 50000000000_0_000_000;
    xlm_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    create_soroswap_pool(
        &setup.env,
        &soroswap_router,
        &pool_admin,
        &xlm.address,
        &usdc.address,
        &amount_a,
        &amount_b,
    );

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

    let xlm_strategy_contract = create_hodl_strategy_contract(&setup.env, &xlm.address);
    let usdc_strategy_contract = create_hodl_strategy_contract(&setup.env, &usdc.address);

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: xlm.address.clone(),
            strategies: svec![&setup.env, Strategy {
                address: xlm_strategy_contract.address.clone(),
                name: String::from_str(&setup.env, "xlmStrat"),
                paused: false,
            }],
        },
        AssetStrategySet {
            address: usdc.address.clone(),
            strategies: svec![&setup.env, Strategy {
                address: usdc_strategy_contract.address.clone(),
                name: String::from_str(&setup.env, "usdcStrat"),
                paused: false,
            }],
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let xlm_starting_balance = 10000000_0_000_000i128;
    let usdc_starting_balance = 5000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    xlm_client.mint(user, &xlm_starting_balance);
    usdc_client.mint(user, &usdc_starting_balance);

    vault_contract.deposit(
        &svec![&setup.env, xlm_starting_balance, usdc_starting_balance],
        &svec![&setup.env, xlm_starting_balance, usdc_starting_balance],
        &user,
        &false,
    );

    // Checking SWAP Limits
    let usdc_balance_on_vault = usdc.balance(&vault_contract.address);
    let num_exact_in = 5;

    let mut exact_in_instructions = svec![&setup.env];
    for _ in 0..num_exact_in {
        exact_in_instructions.push_back(Instruction::SwapExactIn(
            usdc.address.clone(),
            xlm.address.clone(),
            usdc_balance_on_vault / num_exact_in as i128,
            0,
            setup.env.ledger().timestamp() + 3600u64,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &exact_in_instructions);
    check_limits(&setup.env, "SwapExactIn");

    let num_exact_out = 5;

    let mut exact_out_instructions = svec![&setup.env];
    for _ in 0..num_exact_out {
        exact_out_instructions.push_back(Instruction::SwapExactIn(
            xlm.address.clone(),
            usdc.address.clone(),
            usdc_balance_on_vault / num_exact_out as i128,
            0,
            setup.env.ledger().timestamp() + 3600u64,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &exact_out_instructions);
    check_limits(&setup.env, "SwapExactOut");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn two_assets_swap_limits_rebalance_panic_exact_in() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (xlm, xlm_client) = create_token(&setup.env, &token_admin);
    let (usdc, usdc_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    // Setting up soroswap pool
    let pool_admin = Address::generate(&setup.env);
    let amount_a = 100000000000_0_000_000;
    let amount_b = 50000000000_0_000_000;
    xlm_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    create_soroswap_pool(
        &setup.env,
        &soroswap_router,
        &pool_admin,
        &xlm.address,
        &usdc.address,
        &amount_a,
        &amount_b,
    );

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

    let xlm_strategy_contract = create_hodl_strategy_contract(&setup.env, &xlm.address);
    let usdc_strategy_contract = create_hodl_strategy_contract(&setup.env, &usdc.address);

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: xlm.address.clone(),
            strategies: svec![&setup.env, Strategy {
                address: xlm_strategy_contract.address.clone(),
                name: String::from_str(&setup.env, "xlmStrat"),
                paused: false,
            }],
        },
        AssetStrategySet {
            address: usdc.address.clone(),
            strategies: svec![&setup.env, Strategy {
                address: usdc_strategy_contract.address.clone(),
                name: String::from_str(&setup.env, "usdcStrat"),
                paused: false,
            }],
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let xlm_starting_balance = 10000000_0_000_000i128;
    let usdc_starting_balance = 5000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    xlm_client.mint(user, &xlm_starting_balance);
    usdc_client.mint(user, &usdc_starting_balance);

    vault_contract.deposit(
        &svec![&setup.env, xlm_starting_balance, usdc_starting_balance],
        &svec![&setup.env, xlm_starting_balance, usdc_starting_balance],
        &user,
        &false,
    );

    // Checking SWAP Limits
    let usdc_balance_on_vault = usdc.balance(&vault_contract.address);
    let num_exact_in = 6;

    let mut exact_in_instructions = svec![&setup.env];
    for _ in 0..num_exact_in {
        exact_in_instructions.push_back(Instruction::SwapExactIn(
            usdc.address.clone(),
            xlm.address.clone(),
            usdc_balance_on_vault / num_exact_in as i128,
            0,
            setup.env.ledger().timestamp() + 3600u64,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &exact_in_instructions);
    check_limits(&setup.env, "SwapExactIn");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn two_assets_swap_limits_rebalance_panic_exact_out() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.budget().reset_unlimited();

    let token_admin = Address::generate(&setup.env);
    let (xlm, xlm_client) = create_token(&setup.env, &token_admin);
    let (usdc, usdc_client) = create_token(&setup.env, &token_admin);

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    // Setting up soroswap pool
    let pool_admin = Address::generate(&setup.env);
    let amount_a = 100000000000_0_000_000;
    let amount_b = 50000000000_0_000_000;
    xlm_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    create_soroswap_pool(
        &setup.env,
        &soroswap_router,
        &pool_admin,
        &xlm.address,
        &usdc.address,
        &amount_a,
        &amount_b,
    );

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

    let xlm_strategy_contract = create_hodl_strategy_contract(&setup.env, &xlm.address);
    let usdc_strategy_contract = create_hodl_strategy_contract(&setup.env, &usdc.address);

    let assets = svec![
        &setup.env,
        AssetStrategySet {
            address: xlm.address.clone(),
            strategies: svec![&setup.env, Strategy {
                address: xlm_strategy_contract.address.clone(),
                name: String::from_str(&setup.env, "xlmStrat"),
                paused: false,
            }],
        },
        AssetStrategySet {
            address: usdc.address.clone(),
            strategies: svec![&setup.env, Strategy {
                address: usdc_strategy_contract.address.clone(),
                name: String::from_str(&setup.env, "usdcStrat"),
                paused: false,
            }],
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &salt,
        &soroswap_router.address,
        &name_symbol,
        &true,
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    // User deposit
    let xlm_starting_balance = 10000000_0_000_000i128;
    let usdc_starting_balance = 5000000_0_000_000i128;
    let users = IntegrationTest::generate_random_users(&setup.env, 1);
    let user = &users[0];
    xlm_client.mint(user, &xlm_starting_balance);
    usdc_client.mint(user, &usdc_starting_balance);

    vault_contract.deposit(
        &svec![&setup.env, xlm_starting_balance, usdc_starting_balance],
        &svec![&setup.env, xlm_starting_balance, usdc_starting_balance],
        &user,
        &false,
    );

    // Checking SWAP Limits
    let usdc_balance_on_vault = usdc.balance(&vault_contract.address);
    let num_exact_in = 5;

    let mut exact_in_instructions = svec![&setup.env];
    for _ in 0..num_exact_in {
        exact_in_instructions.push_back(Instruction::SwapExactIn(
            usdc.address.clone(),
            xlm.address.clone(),
            usdc_balance_on_vault / num_exact_in as i128,
            0,
            setup.env.ledger().timestamp() + 3600u64,
        ));
    }

    vault_contract.rebalance(&manager, &exact_in_instructions);

    let num_exact_out = 6;

    let mut exact_out_instructions = svec![&setup.env];
    for _ in 0..num_exact_out {
        exact_out_instructions.push_back(Instruction::SwapExactIn(
            xlm.address.clone(),
            usdc.address.clone(),
            usdc_balance_on_vault / num_exact_out as i128,
            0,
            setup.env.ledger().timestamp() + 3600u64,
        ));
    }

    setup.env.budget().reset_unlimited();
    vault_contract.rebalance(&manager, &exact_out_instructions);
    check_limits(&setup.env, "SwapExactOut");
}