use soroban_sdk::{vec, Address, BytesN, Map, String, Vec};

use crate::test::{create_asset_params, DeFindexFactoryTest};

#[test]
fn create_success() {
    let test = DeFindexFactoryTest::setup();

    let asset_params = create_asset_params(&test);

    let salt = BytesN::from_array(&test.env, &[0; 32]);

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(0u32, test.emergency_manager.clone());
    roles.set(1u32, test.fee_receiver.clone());
    roles.set(2u32, test.manager.clone());
    roles.set(3u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));


    test.factory_contract.create_defindex_vault(
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &name_symbol
    );

    let deployed_defindexes = test.factory_contract.deployed_defindexes();
    assert_eq!(deployed_defindexes.len(), 1);
}

#[test]
fn create_and_deposit_success() {
    let test = DeFindexFactoryTest::setup();
    test.env.mock_all_auths();

    let asset_params = create_asset_params(&test);
    let salt = BytesN::from_array(&test.env, &[0; 32]);

    let amount_0 = 1000i128;
    let amount_1 = 2000i128;

    let amounts: Vec<i128> = vec![&test.env, amount_0.clone(), amount_1.clone()];

    // Mint tokens to manager
    test.token0_admin_client.mint(&test.manager, &amount_0);
    test.token1_admin_client.mint(&test.manager, &amount_1);

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(0u32, test.emergency_manager.clone());
    roles.set(1u32, test.fee_receiver.clone());
    roles.set(2u32, test.manager.clone());
    roles.set(3u32, test.rebalance_manager.clone());

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    test.factory_contract.create_defindex_vault_deposit(
        &test.manager,
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &name_symbol,
        &amounts,
    );

    test.factory_contract.create_defindex_vault(
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &name_symbol
    );

    let deployed_defindexes = test.factory_contract.deployed_defindexes();
    assert_eq!(deployed_defindexes.len(), 1);

    let token_0_vault_balance = test.token0.balance(&deployed_defindexes.get(0).unwrap());
    assert_eq!(token_0_vault_balance, amount_0);

    let token_1_vault_balance = test.token1.balance(&deployed_defindexes.get(0).unwrap());
    assert_eq!(token_1_vault_balance, amount_1);
}
