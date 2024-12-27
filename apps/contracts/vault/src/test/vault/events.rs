use soroban_sdk::testutils::Events;
use soroban_sdk::{symbol_short, vec as sorobanvec, Address, FromVal, Map, String, Symbol, Vec};
use crate::test::defindex_vault::{
  AssetInvestmentAllocation, AssetStrategySet, RolesDataKey, StrategyAllocation
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

  let mut roles: Map<RolesDataKey, Address> = Map::new(&test.env);
  roles.set(RolesDataKey::Manager, test.manager.clone());
  roles.set(RolesDataKey::EmergencyManager, test.emergency_manager.clone());
  roles.set(RolesDataKey::VaultFeeReceiver, test.vault_fee_receiver.clone());

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

  let amount = 12_3_456_789i128;

  let users = DeFindexVaultTest::generate_random_users(&test.env, 1);

  // Mint before deposit
  test.token_0_admin_client.mint(&users[0], &amount);

  // Deposit
  defindex_contract.deposit(
      &sorobanvec![&test.env, amount],
      &sorobanvec![&test.env, amount],
      &users[0],
      &true,
  );

     // Invest
     let amount_to_invest = 10_0_000_000i128;
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

  let event_key = Symbol::from_val(&test.env, &event.1.get(1).unwrap().clone());

  let expected_key = symbol_short!("execinv");

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

