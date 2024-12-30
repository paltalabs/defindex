extern crate std;
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1,
    defindex_vault::{
        AssetInvestmentAllocation, AssetStrategySet, Instruction, RolesDataKey, StrategyAllocation
    },
    DeFindexVaultTest,
};
use soroban_sdk::{vec as sorobanvec, Address, Map, String, Vec};

#[test]
fn budget() {
    let test = DeFindexVaultTest::setup();

    test.env.budget().reset_unlimited();

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
    );
    
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "create_defindex_vault()                                   | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();

    // deposit
    let amount0 = 5_0_000_000i128;
    let amount1 = 4_0_000_000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    test.token_0_admin_client.mint(&users[0], &99987654321i128);
    test.token_1_admin_client.mint(&users[0], &99987654321i128);

    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "deposit()                                                 | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();
    // deposit
    let amount0 = 5_0_000_000i128;
    let amount1 = 4_0_000_000i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    test.token_0_admin_client.mint(&users[0], &99987654321i128);
    test.token_1_admin_client.mint(&users[0], &99987654321i128);

    let _ = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &true,
    );

    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("deposit_and_invest()                                                 | cpu: {},      mem: {}", cpu, mem);

    test.env.budget().reset_unlimited();

    // withdraw
    let _ = defindex_contract.withdraw(&5_0_000i128, &users[0]);

    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "withdraw()                                                | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();

    // invest
    let asset_investments = sorobanvec![
        &test.env,
        Some(AssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_0.address.clone(),
                    amount: 100,
                }),
            ]
        }),
        Some(AssetInvestmentAllocation {
            asset: test.token_1.address.clone(),
            strategy_allocations: sorobanvec![
                &test.env,
                Some(StrategyAllocation {
                    strategy_address: test.strategy_client_token_1.address.clone(),
                    amount: 200,
                }),
            ]
        })
    ];

    let _ = defindex_contract.invest(&asset_investments);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "invest()                                                  | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();

    // rebalance withdraw
    let withdraw_instructions = sorobanvec![
        &test.env,
        Instruction::Withdraw(test.strategy_client_token_0.address.clone(), 100),
    ];
    let _ = defindex_contract.rebalance(&test.rebalance_manager, &withdraw_instructions);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "rebalance_withdraw()                                      | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();

    // rebalance invest

    let invest_instructions = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
    ];

    let _ = defindex_contract.rebalance(&test.rebalance_manager, &invest_instructions);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "rebalance_invest()                                        | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();

    //Rebalance with several instructions one strategy

    let several_instructions_one_strategy = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
    ];

    let _ = defindex_contract.rebalance(&test.rebalance_manager, &several_instructions_one_strategy);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "rebalance_invest_several_instructions_one_strategy()      | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();

    // Rebalance several instructions two strategies

    let several_instructions_two_strategy = sorobanvec![
        &test.env,
        Instruction::Invest(test.strategy_client_token_1.address.clone(), 100),
        Instruction::Invest(test.strategy_client_token_1.address.clone(), 100),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
        Instruction::Invest(test.strategy_client_token_0.address.clone(), 100),
    ];

    let _ = defindex_contract.rebalance(&test.rebalance_manager, &several_instructions_two_strategy);
    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!(
        "rebalance_invest_several_instructions_two_strategy()      | cpu: {},      mem: {}",
        cpu,
        mem
    );

    test.env.budget().reset_unlimited();
}
