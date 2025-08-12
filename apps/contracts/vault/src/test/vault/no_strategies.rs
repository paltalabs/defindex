use soroban_sdk::{
    vec as sorobanvec, Address, Map, String, Vec,
    testutils::{Address as _},
};

use crate::test::{
    create_defindex_vault,
    defindex_vault::{AssetStrategySet, CurrentAssetInvestmentAllocation, RolesDataKey, Strategy}, DeFindexVaultTest,
};

#[test]
fn with_one_asset_no_strategies(){
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params: Vec<Strategy> = sorobanvec![&test.env];

    // Strategy Set defines an underlying asset, with an empty Strategy Vector for that asset
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params.clone()
        }
    ];
    
    // All roles are burned. TODO: Burn this address setting weight=0
    let burn_address = Address::generate(&test.env);

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(RolesDataKey::Manager as u32, burn_address.clone());
    roles.set(RolesDataKey::EmergencyManager as u32, burn_address.clone());
    roles.set(RolesDataKey::VaultFeeReceiver as u32, burn_address.clone());
    roles.set(RolesDataKey::RebalanceManager as u32, burn_address.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets, //only one asset, no strategies
        roles, // burned addresses
        0u32, // vault fee =0
        burn_address.clone(), // defindex_protocol_receiver: Address,
        0u32, //   defindex_protocol_rate: u32,
        burn_address.clone(), // soroswap_router: Address, ,
        name_symbol,
        false // upgradable: bool,
    );
    
    // ----------------
    //  Check initial parameters
    let vault_assets = defindex_contract.get_assets();
    let asset = vault_assets.get(0).unwrap();
    let vault_strategies = asset.strategies;

    let total_managed_funds = defindex_contract.fetch_total_managed_funds();
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;
    let current_idle_funds = test.token_0.balance(&defindex_contract.address);

    let mut expected_total_managed_funds: Vec<CurrentAssetInvestmentAllocation> = Vec::new(&test.env);
    expected_total_managed_funds.push_back(
        CurrentAssetInvestmentAllocation {
            asset: test.token_0.address.clone(),
            total_amount: 0i128,
            idle_amount: 0i128,
            invested_amount: 0i128,
            strategy_allocations: sorobanvec![&test.env]
        },
    );
    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);

    assert_eq!(vault_assets.len(), 1);
    assert_eq!(vault_strategies.len(), strategy_params.len());

    assert_eq!(total_managed_funds, expected_total_managed_funds);
    assert_eq!(current_invested_funds, expected_current_invested_funds.get(test.token_0.address.clone()).unwrap());
    assert_eq!(current_idle_funds, 0i128);
    
    // ----------------
    // Deposits
    
    // USER 0: 5M tokens
    let users = DeFindexVaultTest::generate_random_users(&test.env, 3);
    let amount0 = 5_000_000i128;
    test.token_0_admin_client.mint(&users[0], &amount0);
    let _deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount0],
        &sorobanvec![&test.env, amount0],
        &users[0],
        &false,
    );

    let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    let current_invested_funds = defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount;

    let mut expected_current_invested_funds: Map<Address, i128> = Map::new(&test.env);
    expected_current_invested_funds.set(test.token_0.address.clone(), 0i128);

    // All funds should be idle.
    assert_eq!(current_idle_funds, amount0);
    assert_eq!(current_invested_funds, expected_current_invested_funds.get(test.token_0.address.clone()).unwrap());

    // Vault shares should be equal to deposited amounts
    // The first depositor is the only one that receives 1000 less shares than the deposited amount
    assert_eq!(defindex_contract.balance(&users[0]), amount0 - 1000i128);

    // USER 1: 1000 tokens
    let amount1 = 1000i128;
    test.token_0_admin_client.mint(&users[1], &amount1);
    let _deposit_result = defindex_contract.deposit(
        &sorobanvec![&test.env, amount1],
        &sorobanvec![&test.env, amount1],
        &users[1],
        &true,
    );
    // Vault shares should be equal to deposited amounts
    assert_eq!(defindex_contract.balance(&users[1]), amount1);
    // All funds are still in idle (sum of both deposits)
    assert_eq!(test.token_0.balance(&defindex_contract.address), amount0 + amount1);
    // Total amounts should be the sum of both deposits
    assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().total_amount, amount0 + amount1);
    // Idle amounts should be the sum of both deposits
    assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().idle_amount, amount0 + amount1);
    // Invested amounts should be 0
    assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount, 0);
    // Strategy allocations should be empty
    assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().strategy_allocations.len(), 0);
    // Total shares supply should be the sum of both deposits
    assert_eq!(defindex_contract.total_supply(), amount0 + amount1);
    // get_asset_amounts_per_shares shoukd be 1
    assert_eq!(defindex_contract.get_asset_amounts_per_shares(&1i128).get(0).unwrap(), 1);
   
    // USER 2: 1,000,000,000,000 tokens
   let amount2 = 1_000_000_000_000i128;
   test.token_0_admin_client.mint(&users[2], &amount2);
   let _deposit_result = defindex_contract.deposit(
    &sorobanvec![&test.env, amount2],
    &sorobanvec![&test.env, amount2],
    &users[2],
    &true,
   );
   // Check balances for all users and common stats
   assert_eq!(test.token_0.balance(&defindex_contract.address), amount0 + amount1 + amount2);
   assert_eq!(defindex_contract.balance(&users[2]), amount2);
   assert_eq!(defindex_contract.balance(&users[1]), amount1);
   assert_eq!(defindex_contract.balance(&users[0]), amount0 - 1000i128); // user 0 is the only one that gets less shares than the deposited amount
   assert_eq!(defindex_contract.balance(&defindex_contract.address), 1000i128);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().total_amount, amount0 + amount1 + amount2);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().idle_amount, amount0 + amount1 + amount2);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount, 0);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().strategy_allocations.len(), 0);
   assert_eq!(defindex_contract.total_supply(), amount0 + amount1 + amount2);
   assert_eq!(defindex_contract.get_asset_amounts_per_shares(&1i128).get(0).unwrap(), 1);

   // WITHDRAW
   // USER 0: 1000 shares
   let withdraw_amount_0 = defindex_contract.balance(&users[0]);
   let _withdraw_result = defindex_contract.withdraw(
    &withdraw_amount_0,
    &sorobanvec![&test.env, withdraw_amount_0],
    &users[0].clone(),
   );
   // Check balances for all users and common stats
   assert_eq!(test.token_0.balance(&defindex_contract.address), amount0 + amount1 + amount2 - withdraw_amount_0);
   assert_eq!(defindex_contract.balance(&users[2]), amount2);
   assert_eq!(defindex_contract.balance(&users[1]), amount1);
   assert_eq!(defindex_contract.balance(&users[0]), amount0 - 1000i128 - withdraw_amount_0); // user 0 is the only one that gets less shares than the deposited amount
   assert_eq!(defindex_contract.balance(&defindex_contract.address), 1000i128);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().total_amount, amount0 + amount1 + amount2 - withdraw_amount_0);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().idle_amount, amount0 + amount1 + amount2 - withdraw_amount_0);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount, 0);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().strategy_allocations.len(), 0);
   assert_eq!(defindex_contract.total_supply(), amount0 + amount1 + amount2 - withdraw_amount_0);
   assert_eq!(defindex_contract.get_asset_amounts_per_shares(&1i128).get(0).unwrap(), 1);

   // USER 1: all its balance
   let withdraw_amount_1 = defindex_contract.balance(&users[1]);
   let _withdraw_result = defindex_contract.withdraw(
    &withdraw_amount_1,
    &sorobanvec![&test.env, withdraw_amount_1],
    &users[1].clone(),
   );
   // Check balances for all users and common stats
   assert_eq!(test.token_0.balance(&defindex_contract.address), amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1);
   assert_eq!(defindex_contract.balance(&users[2]), amount2);
   assert_eq!(defindex_contract.balance(&users[1]), amount1 - withdraw_amount_1);
   assert_eq!(defindex_contract.balance(&users[0]), amount0 - 1000i128 - withdraw_amount_0); // user 0 is the only one that gets less shares than the deposited amount
   assert_eq!(defindex_contract.balance(&defindex_contract.address), 1000i128);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().total_amount, amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().idle_amount, amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1);

   // USER 2: all its balance
   let withdraw_amount_2 = defindex_contract.balance(&users[2]);
   let _withdraw_result = defindex_contract.withdraw(
    &withdraw_amount_2,
    &sorobanvec![&test.env, withdraw_amount_2],
    &users[2].clone(),
   );
   // Check balances for all users and common stats
   assert_eq!(test.token_0.balance(&defindex_contract.address), amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1 - withdraw_amount_2);
   assert_eq!(defindex_contract.balance(&users[2]), amount2 - withdraw_amount_2);
   assert_eq!(defindex_contract.balance(&users[1]), amount1 - withdraw_amount_1);
   assert_eq!(defindex_contract.balance(&users[0]), amount0 - 1000i128 - withdraw_amount_0); // user 0 is the only one that gets less shares than the deposited amount
   assert_eq!(defindex_contract.balance(&defindex_contract.address), 1000i128);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().total_amount, amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1 - withdraw_amount_2); 
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().idle_amount, amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1 - withdraw_amount_2);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().invested_amount, 0);
   assert_eq!(defindex_contract.fetch_total_managed_funds().get(0).unwrap().strategy_allocations.len(), 0);
   assert_eq!(defindex_contract.total_supply(), amount0 + amount1 + amount2 - withdraw_amount_0 - withdraw_amount_1 - withdraw_amount_2);
   assert_eq!(defindex_contract.get_asset_amounts_per_shares(&1i128).get(0).unwrap(), 1);

    // let withdraw_amount = defindex_contract.try_get_asset_amounts_per_shares(&vault_shares).unwrap().unwrap().get(0).unwrap();

    // let min_amounts_out = sorobanvec![&test.env, withdraw_amount];
    
    // let _withdraw_result = defindex_contract.withdraw(
    //     &withdraw_amount,
    //     &min_amounts_out,
    //     &users[0].clone(),
    // ); 

    // let current_idle_funds = test.token_0.balance(&defindex_contract.address);
    // assert_eq!(current_idle_funds, 1000i128);
}