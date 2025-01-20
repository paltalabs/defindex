use soroban_sdk::{testutils::{Address as _, Ledger, MockAuth, MockAuthInvoke}, vec as svec, xdr::ContractCostType, Address, BytesN, IntoVal, Map, String, Vec};

use crate::{factory::{AssetStrategySet, Strategy}, fixed_strategy::{create_fixed_strategy_contract, FixedStrategyClient}, hodl_strategy::create_hodl_strategy_contract, setup::{create_soroswap_factory, create_soroswap_router, create_vault_one_asset_hodl_strategy, mock_mint, VAULT_FEE}, test::{limits::{check_limits, CPU_LIMIT, MEM_LIMIT}, EnvTestUtils, IntegrationTest, DAY_IN_LEDGERS, ONE_YEAR_IN_SECONDS}, token::create_token, vault::{defindex_vault_contract::{Instruction, VaultContractClient}, MINIMUM_LIQUIDITY}};

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

    let invest_instructions = invest_instructions;

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
    let num_strategies = 11; // CHANGE THIS IF U NEED TO TEST OTHER NUMBER OF STRATEGIES

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

    let invest_instructions = invest_instructions;

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