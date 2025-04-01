use soroban_sdk::{testutils::Address as _, vec as svec, Address, BytesN, Map, String};
use crate::{blend_strategy::{create_blend_strategy_contract, BlendStrategyClient}, factory::{AssetStrategySet, Strategy}, fixed_strategy::{create_fixed_strategy_contract, FixedStrategyClient}, hodl_strategy::create_hodl_strategy_contract, setup::{blend_setup::{create_blend_pool, BlendFixture, BlendPoolClient}, create_soroswap_factory, create_soroswap_pool, create_soroswap_router, VAULT_FEE}, test::{limits::check_limits, EnvTestUtils, IntegrationTest, DAY_IN_LEDGERS}, token::create_token, vault::defindex_vault_contract::{Instruction, VaultContractClient}};

#[test]
fn n_assets_one_strategy_hodl() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

    let mut tokens = Vec::new();
    let mut token_clients = Vec::new();

    let num_tokens = 9;

    for _ in 0..num_tokens {
        let token_admin = Address::generate(&setup.env);
        let (token, token_admin_client) = create_token(&setup.env, &token_admin);
        tokens.push(token);
        token_clients.push(token_admin_client);
    }

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

    for i in 0..num_tokens {
        let strategy_contract = create_hodl_strategy_contract(&setup.env, &tokens.get(i).unwrap().address);
        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, "HodlStrategy"),
            paused: false,
        });
    }

    let mut assets = svec![&setup.env];
    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        assets.push_back(AssetStrategySet {
            address: token.address.clone(),
            strategies: svec![&setup.env, strategy],
        });
    }


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

    let mut amounts_desired = svec![&setup.env];
    let mut amounts_min = svec![&setup.env];

    for i in 0..num_tokens {
        let token_admin_client = token_clients.get(i).unwrap();
        token_admin_client.mint(user, &user_starting_balance);

        let desired_amount = 10000000_0_000_000i128;

        amounts_desired.push_back(desired_amount);
        amounts_min.push_back(desired_amount);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(&amounts_desired, &amounts_min, user, &false);
    check_limits(&setup.env, "Deposit");


    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    let batch_size = num_tokens / 2;

    for i in 0..batch_size {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i.try_into().unwrap()).unwrap().address.clone(),
            user_starting_balance,
        ));
    }

    // Rebalance first batch
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest Batch 1");

    let mut invest_instructions = svec![&setup.env];

    for i in batch_size..num_tokens {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i.try_into().unwrap()).unwrap().address.clone(),
            user_starting_balance,
        ));
    }

    // Rebalance second batch
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest Batch 2");

    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        let balance = token.balance(&strategy.address);
        println!("Strategy {} balance: {}", i, balance);
        assert!(balance > 0, "Strategy {} has zero balance", i);
    }

    // Deposit and Invest
    let mut amounts_desired = svec![&setup.env];
    let mut amounts_min = svec![&setup.env];

    for i in 0..num_tokens {
        let token_admin_client = token_clients.get(i).unwrap();
        token_admin_client.mint(user, &user_starting_balance);

        let desired_amount = 10000000_0_000_000i128;

        amounts_desired.push_back(desired_amount);
        amounts_min.push_back(desired_amount);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(&amounts_desired, &amounts_min, user, &true);
    check_limits(&setup.env, "Deposit and Invest");

    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        let balance = token.balance(&strategy.address);
        println!("Strategy {} balance: {}", i, balance);
        assert!(balance > user_starting_balance, "Strategy {} has zero balance", i);
    }

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    vault_contract.withdraw(&balance, &user);
    check_limits(&setup.env, "Withdraw");

    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        let balance = token.balance(&strategy.address);
        println!("Strategy {} balance after withdrawal: {}", i, balance);
        assert!(balance < user_starting_balance, "Strategy {} balance did not decrease", i);
    }
}

#[test]
fn n_assets_one_strategy_fixed() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

    let mut tokens = Vec::new();
    let mut token_clients = Vec::new();

    let num_tokens = 8; // CHANGE THIS TO CHECK THE LIMITS

    for _ in 0..num_tokens {
        let token_admin = Address::generate(&setup.env);
        let (token, token_admin_client) = create_token(&setup.env, &token_admin);
        tokens.push(token);
        token_clients.push(token_admin_client);
    }

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

    for i in 0..num_tokens {
        let token_admin_client = token_clients.get(i).unwrap();
        let strategy_contract = create_fixed_strategy_contract(&setup.env, &tokens.get(i).unwrap().address, 1000u32, &token_admin_client);
        strategies.push_back(Strategy {
            address: strategy_contract.address.clone(),
            name: String::from_str(&setup.env, "FixedStrategy"),
            paused: false,
        });
    }

    let mut assets = svec![&setup.env];
    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        assets.push_back(AssetStrategySet {
            address: token.address.clone(),
            strategies: svec![&setup.env, strategy],
        });
    }


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

    let mut amounts_desired = svec![&setup.env];
    let mut amounts_min = svec![&setup.env];

    for i in 0..num_tokens {
        let token_admin_client = token_clients.get(i).unwrap();
        token_admin_client.mint(user, &user_starting_balance);

        let desired_amount = 10000000_0_000_000i128;

        amounts_desired.push_back(desired_amount);
        amounts_min.push_back(desired_amount);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(&amounts_desired, &amounts_min, user, &false);
    check_limits(&setup.env, "Deposit");

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];

    for i in 0..num_tokens {
        invest_instructions.push_back(Instruction::Invest(
            strategies.get(i.try_into().unwrap()).unwrap().address.clone(),
            user_starting_balance,
        ));
    }

    // Rebalance first batch
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        let balance = token.balance(&strategy.address);
        println!("Strategy {} balance: {}", i, balance);
        assert!(balance > 0, "Strategy {} has zero balance", i);
    }

    // Deposit and Invest
    let mut amounts_desired = svec![&setup.env];
    let mut amounts_min = svec![&setup.env];

    for i in 0..num_tokens {
        let token_admin_client = token_clients.get(i).unwrap();
        token_admin_client.mint(user, &user_starting_balance);

        let desired_amount = 10000000_0_000_000i128;

        amounts_desired.push_back(desired_amount);
        amounts_min.push_back(desired_amount);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(&amounts_desired, &amounts_min, user, &true);
    check_limits(&setup.env, "Deposit and Invest");

    for i in 0..num_tokens {
        let token = tokens.get(i).unwrap();
        let strategy = strategies.get(i.try_into().unwrap()).unwrap();
        let balance = token.balance(&strategy.address);
        println!("Strategy {} balance: {}", i, balance);
        assert!(balance > user_starting_balance, "Strategy {} has zero balance", i);
    }

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    vault_contract.withdraw(&balance, &user);
    check_limits(&setup.env, "Withdraw");
}

#[test]
fn n_assets_one_strategy_blend() {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    setup.env.cost_estimate().budget().reset_unlimited();

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);
    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let admin = Address::generate(&setup.env);

    let (blnd, blnd_client) = create_token(&setup.env, &admin);
    let (usdc, usdc_client) = create_token(&setup.env, &admin);
    let (xlm, xlm_client) = create_token(&setup.env, &admin);

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

    let blend_fixture = BlendFixture::deploy(&setup.env, &admin, &blnd.address, &usdc.address);

    let pool = create_blend_pool(&setup.env, &blend_fixture, &admin, &usdc_client, &xlm_client);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let manager = Address::generate(&setup.env);

    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "BlendVault");
    let vault_symbol = String::from_str(&setup.env, "BLNDVLT");

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let usdc_strategy = create_blend_strategy_contract(
        &setup.env,
        &usdc.address,
        &pool,
        &blnd.address,
        &soroswap_router.address,
        40_0000000i128
    );
    let usdc_strategy_contract = BlendStrategyClient::new(&setup.env, &usdc_strategy);

    let xlm_strategy = create_blend_strategy_contract(
        &setup.env,
        &xlm.address,
        &pool,
        &blnd.address,
        &soroswap_router.address,
        40_0000000i128
    );
    let xlm_strategy_contract = BlendStrategyClient::new(&setup.env, &xlm_strategy);

    let num_tokens = 2;

    let mut assets = svec![&setup.env];

    assets.push_back(AssetStrategySet {
        address: usdc.address.clone(),
        strategies: svec![&setup.env, Strategy {
            address: usdc_strategy_contract.address.clone(),
            name: String::from_str(&setup.env, "BlendUSDC"),
            paused: false,
        }],
    });
    assets.push_back(AssetStrategySet {
        address: xlm.address.clone(),
        strategies: svec![&setup.env, Strategy {
            address: xlm_strategy_contract.address.clone(),
            name: String::from_str(&setup.env, "BlendXLM"),
            paused: false,
        }],
    });
    

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

    let mut amounts_desired = svec![&setup.env];
    let mut amounts_min = svec![&setup.env];

    for _ in 0..num_tokens {
        usdc_client.mint(user, &user_starting_balance);
        xlm_client.mint(user, &user_starting_balance);

        let desired_amount = 10000000_0_000_000i128;

        amounts_desired.push_back(desired_amount);
        amounts_min.push_back(desired_amount);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(&amounts_desired, &amounts_min, user, &false);
    check_limits(&setup.env, "Deposit");

    // Prepare rebalance instructions for all strategies
    let mut invest_instructions = svec![&setup.env];
    let batch_size = num_tokens;

    for i in 0..batch_size {
        let asset = assets.get(i.try_into().unwrap()).unwrap();
        let strategy = asset.strategies.get(0).unwrap();
        invest_instructions.push_back(Instruction::Invest(
            strategy.address.clone(),
            user_starting_balance,
        ));
    }

    // Rebalance first batch
    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.rebalance(&manager, &invest_instructions);
    check_limits(&setup.env, "Invest");

    // let mut invest_instructions = svec![&setup.env];

    // for i in batch_size..num_tokens {
    //     invest_instructions.push_back(Instruction::Invest(
    //         strategies.get(i.try_into().unwrap()).unwrap().address.clone(),
    //         user_starting_balance,
    //     ));
    // }

    // // Rebalance second batch
    // setup.env.cost_estimate().budget().reset_unlimited();
    // vault_contract.rebalance(&manager, &invest_instructions);
    // check_limits(&setup.env, "Invest Batch 2");

    let mut amounts_desired = svec![&setup.env];
    let mut amounts_min = svec![&setup.env];

    for _ in 0..num_tokens {
        usdc_client.mint(user, &user_starting_balance);
        xlm_client.mint(user, &user_starting_balance);

        let desired_amount = 10000000_0_000_000i128;

        amounts_desired.push_back(desired_amount);
        amounts_min.push_back(desired_amount);
    }

    setup.env.cost_estimate().budget().reset_unlimited();
    vault_contract.deposit(&amounts_desired, &amounts_min, user, &true);
    check_limits(&setup.env, "Deposit and Invest");

    setup.env.jump(DAY_IN_LEDGERS * 7);

    // Simulate a user withdrawal touching all strategies
    setup.env.cost_estimate().budget().reset_unlimited();
    let balance = vault_contract.balance(&user);
    let _ = vault_contract.try_withdraw(&(balance/2), &user);
    check_limits(&setup.env, "Withdraw");
}