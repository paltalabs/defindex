use soroban_sdk::testutils::Events;
use soroban_sdk::{symbol_short, vec as sorobanvec, Address, FromVal, IntoVal, Map, String, Val, Vec};
use crate::events::{ExecuteInvestmentEvent, InvestEvent, SwapExactInEvent};

use crate::{models, report};
use crate::test::defindex_vault::{
  AssetInvestmentAllocation, AssetStrategySet, Instruction, Report, RolesDataKey, StrategyAllocation, UnwindEvent,
};
use crate::test::{
  create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1, DeFindexVaultTest
};

extern crate std;
#[test]
fn check_rebalance_events(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_strategy_params_token_0(&test);
  let strategy_params_token_1 = create_strategy_params_token_1(&test);
  // initialize with 1 assets
  let assets: Vec<AssetStrategySet> = sorobanvec![
    &test.env,
    AssetStrategySet {
        address: test.token_0.address.clone(),
        strategies: strategy_params_token_0.clone()
    },
    AssetStrategySet {
        address: test.token_1.address.clone(),
        strategies: strategy_params_token_1.clone()
    },
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
      assets.clone(),
      roles,
      2000u32,
      test.defindex_protocol_receiver.clone(),
      2500u32,
      test.defindex_factory.clone(),
      test.soroswap_router.address.clone(),
      name_symbol,
  );

  let amount = 12_3_456_789i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

  // Mint before deposit
  test.token_0_admin_client.mint(&users[0], &amount);
  test.token_1_admin_client.mint(&users[0], &amount);

  let deposit_amount = 10_0_000_000i128;

  // Deposit
  defindex_contract.deposit(
      &sorobanvec![&test.env, deposit_amount, deposit_amount],
      &sorobanvec![&test.env, deposit_amount, deposit_amount],
      &users[0],
      &true,
  );

  // Invest
  let amount_to_invest = 5_000_000i128;
  let asset_investments = sorobanvec![
    &test.env,
    Some(AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: sorobanvec![
            &test.env,
            Some(StrategyAllocation {
                strategy_address: test.strategy_client_token_0.address.clone(),
                amount: amount_to_invest,
            }),
        ],
    }),
    Some(AssetInvestmentAllocation {
        asset: test.token_1.address.clone(),
        strategy_allocations: sorobanvec![
            &test.env,
            Some(StrategyAllocation {
                strategy_address: test.strategy_client_token_1.address.clone(),
                amount: amount_to_invest,
            }),
        ],
    }),
  ];

  let invest_result = defindex_contract.invest(&asset_investments.clone());

  assert_eq!(invest_result.clone(), asset_investments.clone());

  let event = test.env.events().all().last().unwrap();

  let invest_event: ExecuteInvestmentEvent = FromVal::from_val(&test.env, &event.2);

  let expected_asset_investment: models::AssetInvestmentAllocation = models::AssetInvestmentAllocation {
    asset: test.token_0.address.clone(),
    strategy_allocations: sorobanvec![
        &test.env,
        Some(models::StrategyAllocation {
            strategy_address: test.strategy_client_token_0.address.clone(),
            amount: amount_to_invest,
        }),
    ],
  };
  let expected_asset_1_investment: models::AssetInvestmentAllocation = models::AssetInvestmentAllocation {
    asset: test.token_1.address.clone(),
    strategy_allocations: sorobanvec![
        &test.env,
        Some(models::StrategyAllocation {
            strategy_address: test.strategy_client_token_1.address.clone(),
            amount: amount_to_invest,
        }),
    ],
  };

  let expected_assets: Vec<common::models::AssetStrategySet> = sorobanvec![
    &test.env,
    common::models::AssetStrategySet {
        address: test.token_0.address.clone(),
        strategies: sorobanvec![
            &test.env,
            common::models::Strategy {
                address: test.strategy_client_token_0.address.clone(),
                name: String::from_str(&test.env, "Strategy 1"),
                paused: false,
            },
        ],
    },
    common::models::AssetStrategySet {
        address: test.token_1.address.clone(),
        strategies: sorobanvec![
            &test.env,
            common::models::Strategy {
                address: test.strategy_client_token_1.address.clone(),
                name: String::from_str(&test.env, "Strategy 1"),
                paused: false,
            },
        ],
    },
  ];
  assert_eq!(invest_event.rebalance_method, symbol_short!("invest"));
  assert_eq!(invest_event.asset_investments, sorobanvec![&test.env, Some(expected_asset_investment), Some(expected_asset_1_investment)]);
  assert_eq!(invest_event.assets, expected_assets);

  let instruction_amount_0 = 2_000i128;
  let instructions = sorobanvec![
        &test.env,
        Instruction::Withdraw(
            test.strategy_client_token_0.address.clone(),
            instruction_amount_0
        ),

    ];
  defindex_contract.rebalance(&test.rebalance_manager, &instructions.clone());

  let events = test.env.events().all();

  let rebalance_events: std::vec::Vec<(soroban_sdk::Address, soroban_sdk::Vec<soroban_sdk::Val>, soroban_sdk::Val)> = 
    events.iter().filter(
      | event| event.1 == 
        sorobanvec![&test.env, String::from_str(&test.env, "DeFindexVault").into_val(&test.env), 
        symbol_short!("rebalance").into_val(&test.env)]
      ).collect();

  let  withdraw_event: UnwindEvent = FromVal::from_val(&test.env, &rebalance_events.last().unwrap().2); 

  let expected_call_params = sorobanvec![
    &test.env,
    (test.strategy_client_token_0.address.clone(), instruction_amount_0, defindex_contract.address.clone()),
  ];

  let expected_report = Report {
    prev_balance: amount_to_invest- instruction_amount_0,
    gains_or_losses: 0,
    locked_fee:0
  };

  assert_eq!(withdraw_event.rebalance_method, symbol_short!("unwind"));
  assert_eq!(withdraw_event.call_params, expected_call_params);
  assert_eq!(withdraw_event.report, expected_report);

  let instruction_amount_1 = 1_000_000i128;

  let instructions = sorobanvec![
        &test.env,
        Instruction::Invest(
            test.strategy_client_token_0.address.clone(),
            instruction_amount_1
        ),

    ];
  defindex_contract.rebalance(&test.rebalance_manager, &instructions.clone());

  let events = test.env.events().all();
  let rebalance_events: std::vec::Vec<(soroban_sdk::Address, soroban_sdk::Vec<soroban_sdk::Val>, soroban_sdk::Val)> = 
    events.iter().filter(
      | event| event.1 == 
        sorobanvec![&test.env, String::from_str(&test.env, "DeFindexVault").into_val(&test.env), 
        symbol_short!("rebalance").into_val(&test.env)]
      ).collect();

  let  invest_event: InvestEvent = FromVal::from_val(&test.env, &rebalance_events.last().unwrap().2); 

  let expected_asset_investment = sorobanvec![
    &test.env,
    models::AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: sorobanvec![
            &test.env,
            Some(models::StrategyAllocation {
                strategy_address: test.strategy_client_token_0.address.clone(),
                amount: instruction_amount_1,
            }),
        ],
    },
  ];

  let expected_report = report::Report {
    prev_balance: amount_to_invest - instruction_amount_0 + instruction_amount_1,
    gains_or_losses: 0,
    locked_fee:0
  };

  assert_eq!(invest_event.rebalance_method, symbol_short!("invest"));
  assert_eq!(invest_event.asset_investments, expected_asset_investment);
  assert_eq!(invest_event.report, expected_report);

  let amount_in = 1_000_000;
  let instructions = sorobanvec![
    &test.env,
    Instruction::SwapExactIn(
        test.token_0.address.clone(),
        test.token_1.address.clone(),
        amount_in,
        amount_in,
        test.env.ledger().timestamp() + 3600u64
    ),
];

  defindex_contract.rebalance(&test.rebalance_manager, &instructions);

  let events = test.env.events().all();
  let rebalance_events: std::vec::Vec<(soroban_sdk::Address, soroban_sdk::Vec<soroban_sdk::Val>, soroban_sdk::Val)> = 
    events.iter().filter(
      | event| event.1 == 
        sorobanvec![&test.env, String::from_str(&test.env, "DeFindexVault").into_val(&test.env), 
        symbol_short!("rebalance").into_val(&test.env)]
      ).collect();

  let swap_exact_in_event: SwapExactInEvent = FromVal::from_val(&test.env, &rebalance_events.last().unwrap().2);
  
  let expected_path: Vec<Val> = sorobanvec![
    &test.env,
    test.token_0.address.clone().into_val(&test.env),
    test.token_1.address.clone().into_val(&test.env)
  ];
  let expected_swap_args:Vec<Val> = sorobanvec![
    &test.env,
    amount_in.into_val(&test.env),
    amount_in.into_val(&test.env),
    expected_path.into_val(&test.env),
    defindex_contract.address.clone().into_val(&test.env),
    (test.env.ledger().timestamp() + 3600u64).into_val(&test.env),
  ];

  assert_eq!(swap_exact_in_event.rebalance_method, symbol_short!("SwapEIn"));
  assert_eq!(swap_exact_in_event.swap_args, expected_swap_args);


  let amount_out = 1_000_000i128;
  let swap_exact_out_instructions = sorobanvec![
    &test.env,
    Instruction::SwapExactOut(
      test.token_0.address.clone(),
      test.token_1.address.clone(),
        amount_out,
        amount_out,
        test.env.ledger().timestamp() + 3600u64
    )];

  defindex_contract.rebalance(&test.rebalance_manager, &swap_exact_out_instructions);

  let events = test.env.events().all();
  let rebalance_events: std::vec::Vec<(soroban_sdk::Address, soroban_sdk::Vec<soroban_sdk::Val>, soroban_sdk::Val)> = 
    events.iter().filter(
      | event| event.1 == 
        sorobanvec![&test.env, String::from_str(&test.env, "DeFindexVault").into_val(&test.env), 
        symbol_short!("rebalance").into_val(&test.env)]
      ).collect();

  let swap_exact_out_event: SwapExactInEvent = FromVal::from_val(&test.env, &rebalance_events.last().unwrap().2);

  let expected_path: Vec<Val> = sorobanvec![
    &test.env,
    test.token_0.address.clone().into_val(&test.env),
    test.token_1.address.clone().into_val(&test.env),
  ];
  let expected_swap_args:Vec<Val> = sorobanvec![
    &test.env,
    amount_out.into_val(&test.env),
    amount_out.into_val(&test.env),
    expected_path.into_val(&test.env),
    defindex_contract.address.clone().into_val(&test.env),
    (test.env.ledger().timestamp() + 3600u64).into_val(&test.env),
  ];

  assert_eq!(swap_exact_out_event.rebalance_method, symbol_short!("SwapEOut"));
  assert_eq!(swap_exact_out_event.swap_args, expected_swap_args);

}

