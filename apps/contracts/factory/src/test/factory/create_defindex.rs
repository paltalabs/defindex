use soroban_sdk::{vec, BytesN, String, Vec};

use crate::test::{create_asset_params, DeFindexFactoryTest};

#[test]
fn create_success() {
    let test = DeFindexFactoryTest::setup();

    let asset_params = create_asset_params(&test);

    let salt = BytesN::from_array(&test.env, &[0; 32]);

    test.factory_contract.create_defindex_vault(
        &test.emergency_manager,
        &test.fee_receiver,
        &2000u32,
        &test.manager,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &vec![
            &test.env,
            String::from_str(&test.env, "dfToken"),
            String::from_str(&test.env, "DFT"),
        ],
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

    test.factory_contract.create_defindex_vault_deposit(
        &test.manager,
        &test.emergency_manager,
        &test.fee_receiver,
        &2000u32,
        &test.manager,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &vec![
            &test.env,
            String::from_str(&test.env, "dfToken"),
            String::from_str(&test.env, "DFT"),
        ],
        &amounts,
    );

    let deployed_defindexes = test.factory_contract.deployed_defindexes();
    assert_eq!(deployed_defindexes.len(), 1);

    let token_0_vault_balance = test.token0.balance(&deployed_defindexes.get(0).unwrap());
    assert_eq!(token_0_vault_balance, amount_0);

    let token_1_vault_balance = test.token1.balance(&deployed_defindexes.get(0).unwrap());
    assert_eq!(token_1_vault_balance, amount_1);
}
