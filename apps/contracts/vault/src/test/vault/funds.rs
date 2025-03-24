use soroban_sdk::{vec as sorobanvec, Address, Map, String, Vec};

use crate::test::{create_defindex_vault, create_fixed_strategy_params_token_0, defindex_vault::{AssetStrategySet, RolesDataKey}, DeFindexVaultTest};

use crate::storage;
use crate::funds;
use crate::report::Report;

#[test]
fn report_on_fetch_strategy_invested_funds() {
  let test = DeFindexVaultTest::setup();
  test.env.mock_all_auths();
  let strategy_params_token_0 = create_fixed_strategy_params_token_0(&test);
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

  let amount = 1000_0_000_000i128;

  let defindex_contract = create_defindex_vault(
      &test.env,
      assets,
      roles,
      2000u32,
      test.defindex_protocol_receiver.clone(),
      2500u32,
      test.soroswap_router.address.clone(),
      name_symbol,
      true
  );
  test.token_0_admin_client.mint(&defindex_contract.address, &amount);

  test.fixed_strategy_client_token_0.deposit(&amount, &defindex_contract.address);
  let expected_first_report = Report {
    prev_balance: 0,
    gains_or_losses: 0,
    locked_fee: 0,
  };

  let first_report =  test.env.as_contract(&defindex_contract.address, || storage::get_report(&test.env, &test.fixed_strategy_client_token_0.address));
  assert_eq!(first_report, expected_first_report);

  let expected_fetch_strategy_invested_funds = Ok(amount);
  let result = test.env.as_contract(&defindex_contract.address, || funds::fetch_strategy_invested_funds(&test.env, &test.fixed_strategy_client_token_0.address, true));
  assert_eq!(result, expected_fetch_strategy_invested_funds);

  let expected_new_report = Report {
    prev_balance: amount,
    gains_or_losses: 0,
    locked_fee: 0,
  };
  
  let new_report = test.env.as_contract(&defindex_contract.address, || storage::get_report(&test.env, &test.fixed_strategy_client_token_0.address));
  assert_eq!(new_report, expected_new_report);
}