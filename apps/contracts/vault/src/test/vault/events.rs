use soroban_sdk::testutils::Events;
use soroban_sdk::{symbol_short, vec as sorobanvec, FromVal, IntoVal, String, Symbol, Vec};
use crate::events::InvestEvent;
use crate::models;
use crate::report::Report;
use crate::test::defindex_vault::{
  AssetInvestmentAllocation, AssetStrategySet, StrategyAllocation
};
use crate::test::{
  create_defindex_vault, create_strategy_params_token_0,
  DeFindexVaultTest,
};

extern crate std;
#[test]
fn check_and_execute_investments(){
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_strategy_params_token_0(&test);
  // initialize with 1 assets
  let assets: Vec<AssetStrategySet> = sorobanvec![
    &test.env,
    AssetStrategySet {
        address: test.token_0.address.clone(),
        strategies: strategy_params_token_0.clone()
    }
];

  let defindex_contract = create_defindex_vault(
      &test.env,
      assets,
      test.manager.clone(),
      test.emergency_manager.clone(),
      test.vault_fee_receiver.clone(),
      2000u32,
      test.defindex_protocol_receiver.clone(),
      2500u32,
      test.defindex_factory.clone(),
      test.soroswap_router.address.clone(),
      sorobanvec![
          &test.env,
          String::from_str(&test.env, "dfToken"),
          String::from_str(&test.env, "DFT")
      ],
  );
  let amount = 12_3_456_789i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

  // Mint before deposit
  test.token_0_admin_client.mint(&users[0], &amount);

  let deposit_amount = 10_0_000_000i128;

  // Deposit
  defindex_contract.deposit(
      &sorobanvec![&test.env, deposit_amount],
      &sorobanvec![&test.env, deposit_amount],
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
  ];

  let invest_result = defindex_contract.invest(&asset_investments);

  assert_eq!(invest_result.clone(), asset_investments);

  let event = test.env.events().all().last().unwrap();

  let investment = models::AssetInvestmentAllocation {
    asset: test.token_0.address.clone(),
    strategy_allocations: sorobanvec![&test.env, Some(models::StrategyAllocation {
        strategy_address: test.strategy_client_token_0.address.clone(),
        amount: amount_to_invest.clone(),
    })],
};
  let expected_event_2 = InvestEvent { 
    asset_investments: sorobanvec![&test.env, investment.clone()],
    rebalance_method: symbol_short!("rebalance"), 
    report: Report {
      prev_balance: 0,
      gains_or_losses: 0,
      locked_fee: 0,
    }
  };

  let expected_event =    sorobanvec![
    &test.env,
    (
        defindex_contract.address.clone(),
        (String::from_str(&test.env, "DeFindexVault"), symbol_short!("rebalance")).into_val(&test.env),
        (expected_event_2).into_val(&test.env)
    ),
  ];

  assert_eq!(sorobanvec![&test.env, event.clone()], expected_event);
  /* 

     left: Vec(Ok((Contract(CAQ5UHPIIQ2LXWNVOLLZFEQOUYIWGPGDEMNH4WJQCX3C4LPOI6EKQH5V), Vec(Ok(String(obj#2789)), Ok(Symbol(rebalance))), Map(obj#2829))))
    right: Vec(Ok((Contract(CAQ5UHPIIQ2LXWNVOLLZFEQOUYIWGPGDEMNH4WJQCX3C4LPOI6EKQH5V), Vec(Ok(String(obj#2847)), Ok(Symbol(rebalance))), Map(obj#2863))))

    pub struct InvestEvent {
      pub asset_investments: Vec<AssetInvestmentAllocation>,
      pub rebalance_method: Symbol,
      pub report: Report,
    }

    let investment = AssetInvestmentAllocation {
      asset: asset_address.clone(),
      strategy_allocations: vec![&e, Some(StrategyAllocation {
          strategy_address: strategy_address.clone(),
          amount: amount.clone(),
      })],
    };
    pub struct Report {
    pub prev_balance: i128,
    pub gains_or_losses: i128,
    pub locked_fee: i128,
}
   */
  //let invest_event: InvestEvent = FromVal::from_val(&test.env, &event.2);
  //std::println!("🟢0{:?}", invest_event);



  //Withdraw event

  let amount_to_withdraw = 5_000_000i128;


  let withdraw_result = defindex_contract.withdraw(&amount_to_withdraw, &users[0]);

  assert_eq!(withdraw_result.clone(), sorobanvec!(&test.env, amount_to_withdraw));

  let event = test.env.events().all().last().unwrap();

  let event_key = Symbol::from_val(&test.env, &event.1.get(1).unwrap().clone());

  let expected_key = symbol_short!("withdraw");

  assert_eq!(event_key, expected_key);

  /* let expected_event_data = sorobanvec![
    &test.env,
    AssetInvestmentAllocation {
        asset: test.token_0.address.clone(),
        strategy_allocations: sorobanvec![
            &test.env,
            Some(StrategyAllocation {
                strategy_address: test.strategy_client_token_0.address.clone(),
                amount: amount_to_invest,
            }),
        ],
    }];

  assert_eq!(
    sorobanvec![&test.env, event.clone()],
    sorobanvec![
      &test.env,
      (
        defindex_contract.address.clone(),
        (String::from_str(&test.env,"DeFindexVault"), symbol_short!("execinv")).into_val(&test.env),
        expected_event_data.into_val(&test.env),
      ),
    ]
  )  */

}

