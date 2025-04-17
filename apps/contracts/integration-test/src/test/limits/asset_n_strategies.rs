use soroban_sdk::{testutils::{Address as _, Ledger, MockAuth, MockAuthInvoke}, vec as svec, xdr::ContractCostType, Address, BytesN, IntoVal, Map, String, Vec, Bytes};

use crate::{blend_strategy::{create_blend_strategy_contract, BlendStrategyClient}, factory::{AssetStrategySet, Strategy}, fixed_strategy::{create_fixed_strategy_contract, FixedStrategyClient}, hodl_strategy::create_hodl_strategy_contract, setup::{blend_setup::{create_blend_pool, BlendFixture, BlendPoolClient, Request}, create_soroswap_factory, create_soroswap_pool, create_soroswap_router, create_vault_one_asset_hodl_strategy, mock_mint, VAULT_FEE}, test::{limits::{check_limits, check_limits_return_info, create_results_table, print_resources}, EnvTestUtils, IntegrationTest, DAY_IN_LEDGERS, ONE_YEAR_IN_SECONDS}, token::create_token, vault::{defindex_vault_contract::{Instruction, VaultContractClient, CurrentAssetInvestmentAllocation}, MINIMUM_LIQUIDITY}};

/// Formats and prints the total managed funds in a readable way
fn print_total_managed_funds(total_managed_funds: &Vec<CurrentAssetInvestmentAllocation>) {
    println!("=== Total Managed Funds ===");
    for (i, allocation) in total_managed_funds.iter().enumerate() {
        println!("Asset #{}: {:?}", i + 1, allocation.asset);
        println!("  Idle Amount: {}", allocation.idle_amount);
        println!("  Invested Amount: {}", allocation.invested_amount);
        println!("  Total Amount: {}", allocation.total_amount);
        println!("  Strategy Allocations:");
        
        for (j, strategy) in allocation.strategy_allocations.iter().enumerate() {
            println!("    Strategy #{}", j + 1);
            println!("      Address: {:?}", strategy.strategy_address);
            println!("      Amount: {}", strategy.amount);
            println!("      Paused: {}", strategy.paused);
        }
    }
    println!("==========================");
}

// 26 strategies is the maximum number of strategies that can be added to a vault before exceeding the instructions limit IN RUST TESTS
// With 26 strategies withdrawals are not possible due to the instruction limit
// 13 strategies is the maximum including withdrawals
#[test]
fn hodl() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

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
    let num_strategies = 9; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

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


    setup.env.cost_estimate().budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
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
    setup.env.cost_estimate().budget().reset_unlimited();
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
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    let min_amounts_out = svec![&setup.env, 0i128];
    vault_contract.withdraw(&balance, &min_amounts_out, &user);
    check_limits(&setup.env, "Withdraw");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn hodl_panic() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

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
    let num_strategies = 15; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

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


    setup.env.cost_estimate().budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
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
    setup.env.cost_estimate().budget().reset_unlimited();
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
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    let min_amounts_out = svec![&setup.env, 0i128];
    vault_contract.withdraw(&balance, &min_amounts_out, &user);
    check_limits(&setup.env, "Withdraw");
}

// FIXED Strategy limit is 10 including withdrawals in RUST
#[test]
fn fixed() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

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
    let num_strategies = 8; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

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


    setup.env.cost_estimate().budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
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
    setup.env.cost_estimate().budget().reset_unlimited();
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
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // harvest on all strategies
    for i in 0..num_strategies {
        setup.env.cost_estimate().budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = FixedStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&manager, &None::<Bytes>);
        check_limits(&setup.env, "Harvest");
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.distribute_fees(&manager);
    check_limits(&setup.env, "Distribute Fees");

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    let min_amounts_out = svec![&setup.env, 0i128];
    vault_contract.withdraw(&balance, &min_amounts_out, &user);
    check_limits(&setup.env, "Withdraw");
}

#[test]
#[should_panic(expected = "Memory usage exceeded limit")]
fn fixed_panic() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

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


    setup.env.cost_estimate().budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
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
    setup.env.cost_estimate().budget().reset_unlimited();
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
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // harvest on all strategies
    for i in 0..num_strategies {
        setup.env.cost_estimate().budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = FixedStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&manager, &None::<Bytes>);
        check_limits(&setup.env, "Harvest");
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.distribute_fees(&manager);
    check_limits(&setup.env, "Distribute Fees");

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    let min_amounts_out = svec![&setup.env, 0i128];
    vault_contract.withdraw(&balance, &min_amounts_out, &user);
    check_limits(&setup.env, "Withdraw");
}

// 2 Strategies is the limit for 1 asset and 2 Blend strategies 
#[test]
fn blend() {
    /* --------------------------------------------------- Setting up test environment --------------------------------------------------- */
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    
    // Soroswap Setup
    /* ------------------------------------------------------ Setting up soroswap -------------------------------------------------------- */
    let soroswap_admin = Address::generate(&setup.env);

    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let admin = Address::generate(&setup.env);
    let keeper = Address::generate(&setup.env);
    let (blnd, blnd_client) = create_token(&setup.env, &admin);
    let (usdc, usdc_client) = create_token(&setup.env, &admin);
    let (_, xlm_client) = create_token(&setup.env, &admin);

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
    /* ----------------------------------------------------- End of soroswap setup ------------------------------------------------------- */

    /* ------------------------------------------------------ Setting up Blend env ------------------------------------------------------- */
    let blend_fixture = BlendFixture::deploy(&setup.env, &admin, &blnd.address, &usdc.address);

    let pool = create_blend_pool(&setup.env, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&setup.env, &pool);

    // admins deposits 200k tokens and borrows 100k tokens for a 50% util rate
    let requests = svec![&setup.env,
        Request {
            address: usdc.address.clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: usdc.address.clone(),
            amount: 100_000_0000000,
            request_type: 4,
        }
    ];
    pool_client
        .mock_all_auths()
        .submit(&admin, &admin, &admin, &requests);

    /* ----------------------------------------------------- End of Blend env setup ------------------------------------------------------ */
    /* ----------------------------------------------------------------------------------------------------------------------------------- */

    /* -------------------------------------------------------- Setting up Vault --------------------------------------------------------- */
    let mut strategies = svec![&setup.env];
    let num_strategies = 2; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES
    for i in 0..num_strategies {
        let strategy_name = format!("Blend_{}", i);
        let strategy = create_blend_strategy_contract(
            &setup.env,
            &usdc.address,
            &pool,
            &blnd.address,
            &soroswap_router.address,
            40_0000000i128,
            &keeper,
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


    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    setup.env.cost_estimate().budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &soroswap_router.address,
        &name_symbol,
        &true
    );
    let create_vault_usage= check_limits_return_info(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);
    /* ------------------------------------------------------- End of Vault setup -------------------------------------------------------- */

    
    /* -------------------------------------------------------- Setting up users --------------------------------------------------------- */
    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    /* ------------------------------------------------------- End of users setup -------------------------------------------------------- */
    setup.env.cost_estimate().budget().reset_unlimited();
    
    /* --------------------------------------------------------- Initial deposit --------------------------------------------------------- */
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );
    let deposit_usage= check_limits_return_info(&setup.env, "Deposit");
    print_resources(&setup.env, "Deposit");
    
    /* -------------------------------------------------------- Rebalance: Invest -------------------------------------------------------- */
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    let invest_usage= check_limits_return_info(&setup.env, "Invest");

    /* ----------------------------------------------------------- Deposit and invest ---------------------------------------------------- */
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &true
    );
    let deposit_and_invest_usage= check_limits_return_info(&setup.env, "Deposit (with invest)");


    /* ---------------------------------------------------------- Rebalance: Unwind ------------------------------------------------------ */
    let mut unwind_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        unwind_instructions.push_back(Instruction::Unwind(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &unwind_instructions);
    let unwind_usage = check_limits_return_info(&setup.env, "Unwind");


    /* ------------------------- This is to restore investment amounts after unwind, it is not for limits testing ------------------------ */
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }
    vault_contract.rebalance(&manager, &invest_instructions);
    setup.env.cost_estimate().budget().reset_unlimited();


    /* -------------------------------------------------- User 2 Deposit Directly into Pool --------------------------------------------- */
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

    let balance = vault_contract.balance(&users[0]);
    let min_amounts_out = svec![&setup.env, 0i128];

    // We see the total managed funds first before withdrawing
    let total_managed_funds = vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(&total_managed_funds);
    
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.withdraw(&(balance/2), &min_amounts_out, &users[0]);
    let withdraw_usage= check_limits_return_info(&setup.env, "Withdraw user0, no idle");

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
    setup.env.jump(DAY_IN_LEDGERS * 15);

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
    
    /* ----------------------------------------------------------- Harvesting ------------------------------------------------------------ */
    std::println!("-- Harvesting --");
    let mut harvest_usage: Vec<(std::string::String, u64, u64, u32, u32, u32, u32)> = Vec::new(&setup.env);
    for i in 0..num_strategies {
        setup.env.cost_estimate().budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = BlendStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&keeper, &None::<Bytes>);
        let usage = check_limits_return_info(&setup.env, "Harvest");
        harvest_usage.push_back(usage);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    let report = vault_contract.report();
    let report_usage= check_limits_return_info(&setup.env, "Report");
    println!("report = {:?}", report);

    setup.env.cost_estimate().budget().reset_unlimited();    
    let lock_fees = vault_contract.lock_fees(&None);
    let lock_fees_usage= check_limits_return_info(&setup.env, "Lock Fees");
    println!("locked_fees = {:?}", lock_fees);

    /* ---------------------------------------------------------- Distribute Fees ------------------------------------------------------- */
    println!("-- Distributing Fees --");
    setup.env.cost_estimate().budget().reset_unlimited();    
    vault_contract.distribute_fees(&manager);
    let distribute_fees_usage= check_limits_return_info(&setup.env, "Distribute Fees");

    /* -------------------------------------------------------- Withdraw from user 1 ----------------------------------------------------- */
    // Before withdrawing, we see the total managed funds
    let total_managed_funds = vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(&total_managed_funds);
    // Let's unwind to store some of the funds in the vault
    let mut unwind_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        unwind_instructions.push_back(Instruction::Unwind(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }
    vault_contract.rebalance(&manager, &unwind_instructions);

    let total_managed_funds = vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(&total_managed_funds);
    // Withdraw funds only from idle
    let balance = vault_contract.balance(&users[1]);
    let assets_per_share = vault_contract.get_asset_amounts_per_shares(&balance);
    println!("assets_per_share = {:?}", assets_per_share);
    let min_amounts_out = svec![&setup.env, 0i128];
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.withdraw(&(balance/2), &min_amounts_out, &users[1]);
    let withdraw_only_idle= check_limits_return_info(&setup.env, "Withdraw only idle");
    
    // Withdraw all funds for user 1
    let total_managed_funds = vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(&total_managed_funds);
    let balance = vault_contract.balance(&users[1]);
    let min_amounts_out = svec![&setup.env, 0i128];
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.withdraw(&balance, &min_amounts_out, &users[1]);
    let withdraw_1_usage= check_limits_return_info(&setup.env, "Withdraw idle and invested");

    /* -------------------------------------------------------- Results table --------------------------------------------------------- */
    let usage_results = vec![
        create_vault_usage,
        deposit_usage,
        invest_usage,
        deposit_and_invest_usage,
        unwind_usage,
        lock_fees_usage,
        report_usage,
        distribute_fees_usage,
        withdraw_usage,
        withdraw_only_idle,
        withdraw_1_usage,
    ];
    create_results_table(&setup.env, usage_results);
    println!("All strategies processed successfully");
}

#[test]
#[should_panic(expected = "CPU instructions exceeded limit")]
fn blend_panic() {
    /* --------------------------------------------------- Setting up test environment --------------------------------------------------- */
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    
    // Soroswap Setup
    /* ------------------------------------------------------ Setting up soroswap -------------------------------------------------------- */
    let soroswap_admin = Address::generate(&setup.env);

    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let admin = Address::generate(&setup.env);
    let keeper = Address::generate(&setup.env);
    let (blnd, blnd_client) = create_token(&setup.env, &admin);
    let (usdc, usdc_client) = create_token(&setup.env, &admin);
    let (_, xlm_client) = create_token(&setup.env, &admin);

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
    /* ----------------------------------------------------- End of soroswap setup ------------------------------------------------------- */

    /* ------------------------------------------------------ Setting up Blend env ------------------------------------------------------- */
    let blend_fixture = BlendFixture::deploy(&setup.env, &admin, &blnd.address, &usdc.address);

    let pool = create_blend_pool(&setup.env, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&setup.env, &pool);

    // admins deposits 200k tokens and borrows 100k tokens for a 50% util rate
    let requests = svec![&setup.env,
        Request {
            address: usdc.address.clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: usdc.address.clone(),
            amount: 100_000_0000000,
            request_type: 4,
        }
    ];
    pool_client
        .mock_all_auths()
        .submit(&admin, &admin, &admin, &requests);

    /* ----------------------------------------------------- End of Blend env setup ------------------------------------------------------ */
    /* ----------------------------------------------------------------------------------------------------------------------------------- */

    /* -------------------------------------------------------- Setting up Vault --------------------------------------------------------- */
    let mut strategies = svec![&setup.env];
    let num_strategies = 3; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES
    for i in 0..num_strategies {
        let strategy_name = format!("Blend_{}", i);
        let strategy = create_blend_strategy_contract(
            &setup.env,
            &usdc.address,
            &pool,
            &blnd.address,
            &soroswap_router.address,
            40_0000000i128,
            &keeper,
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


    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    setup.env.cost_estimate().budget().reset_unlimited();
    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &soroswap_router.address,
        &name_symbol,
        &true
    );
    let create_vault_usage= check_limits_return_info(&setup.env, "Create Vault");

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);
    /* ------------------------------------------------------- End of Vault setup -------------------------------------------------------- */

    
    /* -------------------------------------------------------- Setting up users --------------------------------------------------------- */
    let users = IntegrationTest::generate_random_users(&setup.env, 3);
    
    let starting_balance = 300_0000000;
    usdc_client.mint(&users[0], &starting_balance);
    usdc_client.mint(&users[1], &starting_balance);

    /* ------------------------------------------------------- End of users setup -------------------------------------------------------- */
    setup.env.cost_estimate().budget().reset_unlimited();
    
    /* --------------------------------------------------------- Initial deposit --------------------------------------------------------- */
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[0], 
        &false
    );
    let deposit_usage= check_limits_return_info(&setup.env, "Deposit");

    
    /* -------------------------------------------------------- Rebalance: Invest -------------------------------------------------------- */
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    let invest_usage= check_limits_return_info(&setup.env, "Invest");

    /* ----------------------------------------------------------- Deposit and invest ---------------------------------------------------- */
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(
        &svec!(&setup.env, starting_balance.clone()),
        &svec!(&setup.env, starting_balance.clone()),
        &users[1], 
        &true
    );
    let deposit_and_invest_usage= check_limits_return_info(&setup.env, "Deposit (with invest)");


    /* ---------------------------------------------------------- Rebalance: Unwind ------------------------------------------------------ */
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Unwind(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    let unwind_usage = check_limits_return_info(&setup.env, "Unwind");


    /* ------------------------- This is to restore investment amounts after unwind, it is not for limits testing ------------------------ */
    let mut invest_instructions = svec![&setup.env];
    for i in 0..num_strategies {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i).unwrap().address.clone(),
            starting_balance / num_strategies as i128,
        ));
    }
    vault_contract.rebalance(&manager, &invest_instructions);
    setup.env.cost_estimate().budget().reset_unlimited();


    /* -------------------------------------------------- User 2 Deposit Directly into Pool --------------------------------------------- */
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

    let balance = vault_contract.balance(&users[0]);
    let min_amounts_out = svec![&setup.env, 0i128];

    setup.env.cost_estimate().budget().reset_unlimited();

    /* ------------------------------------------------------------ Withdraw ------------------------------------------------------------- */
    // We see the total managed funds first before withdrawing
    let total_managed_funds = vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(&total_managed_funds);
    
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.withdraw(&balance, &min_amounts_out, &users[0]);
    let withdraw_usage= check_limits_return_info(&setup.env, "Withdraw");

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
    setup.env.jump(DAY_IN_LEDGERS * 15);

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
    
    /* ----------------------------------------------------------- Harvesting ------------------------------------------------------------ */
    std::println!("-- Harvesting --");
    let mut harvest_usage: Vec<(std::string::String, u64, u64, u32, u32, u32, u32)> = Vec::new(&setup.env);
    for i in 0..num_strategies {
        setup.env.cost_estimate().budget().reset_unlimited();
        let temp_strategy_address = strategies.get(i).unwrap().address.clone();
        let temp_client = BlendStrategyClient::new(&setup.env, &temp_strategy_address);
        
        temp_client.harvest(&keeper, &None::<Bytes>);
        let usage = check_limits_return_info(&setup.env, "Harvest");
        harvest_usage.push_back(usage);
    }

    let report = vault_contract.report();
    println!("report = {:?}", report);

    let lock_fees = vault_contract.lock_fees(&None);
    println!("locked_fees = {:?}", lock_fees);

    /* ---------------------------------------------------------- Distribute Fees ------------------------------------------------------- */
    println!("-- Distributing Fees --");
    setup.env.cost_estimate().budget().reset_unlimited();    
    vault_contract.distribute_fees(&manager);
    let distribute_fees_usage= check_limits_return_info(&setup.env, "Distribute Fees");

    /* -------------------------------------------------------- Withdraw from user 1 ----------------------------------------------------- */

    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&users[1]);
    let min_amounts_out = svec![&setup.env, 0i128];
    vault_contract.withdraw(&balance, &min_amounts_out, &users[1]);
    let withdraw_1_usage= check_limits_return_info(&setup.env, "Withdraw");

    /* -------------------------------------------------------- Results table --------------------------------------------------------- */
    let usage_results = vec![
        create_vault_usage,
        deposit_usage,
        invest_usage,
        deposit_and_invest_usage,
        unwind_usage,
        withdraw_usage,
        distribute_fees_usage,
        withdraw_1_usage,
    ];
    create_results_table(&setup.env, usage_results);
    println!("All strategies processed successfully");
}
