use soroban_sdk::{vec, Address, BytesN, Map, String, Vec};

use crate::test::{create_asset_params, DeFindexFactoryTest};

#[test]
fn create_success() {
    let test = DeFindexFactoryTest::setup();

    let asset_params = create_asset_params(&test);
    let salt = BytesN::from_array(&test.env, &[0; 32]);

    let mut roles: Map<u32, Address> = Map::new(&test.env);
    roles.set(0u32, test.emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, test.fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, test.manager.clone()); // Manager enum = 2
    roles.set(3u32, test.rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    // Create first vault
    test.factory_contract.create_defindex_vault(
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager,
        &name_symbol,
        &true
    );

    // Create second vault with different salt
    let salt_2 = BytesN::from_array(&test.env, &[1; 32]);
    test.factory_contract.create_defindex_vault(
        &roles,
        &2000u32,
        &asset_params,
        &salt_2,
        &test.emergency_manager,
        &name_symbol,
        &true
    );

    // Create third vault with different salt
    let salt_3 = BytesN::from_array(&test.env, &[2; 32]);
    test.factory_contract.create_defindex_vault(
        &roles,
        &2000u32,
        &asset_params,
        &salt_3,
        &test.emergency_manager,
        &name_symbol,
        &true
    );

    let deployed_vaults = test.factory_contract.deployed_vaults();
    assert_eq!(deployed_vaults.len(), 3);

    // Verify vaults are stored in order of creation
    let first_vault = deployed_vaults.get(0);
    let second_vault = deployed_vaults.get(1);
    let third_vault = deployed_vaults.get(2);

    assert!(first_vault.is_some());
    assert!(second_vault.is_some());
    assert!(third_vault.is_some());
    assert_ne!(first_vault, second_vault);
    assert_ne!(second_vault, third_vault);
    assert_ne!(first_vault, third_vault);
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
    roles.set(0u32, test.emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, test.fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, test.manager.clone()); // Manager enum = 2
    roles.set(3u32, test.rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&test.env);
    name_symbol.set(String::from_str(&test.env, "name"), String::from_str(&test.env, "dfToken"));
    name_symbol.set(String::from_str(&test.env, "symbol"), String::from_str(&test.env, "DFT"));

    // Create first vault with deposit
    test.factory_contract.create_defindex_vault_deposit(
        &test.manager,
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager,
        &name_symbol,
        &true,
        &amounts,
    );

    // Mint more tokens for second vault
    test.token0_admin_client.mint(&test.manager, &amount_0);
    test.token1_admin_client.mint(&test.manager, &amount_1);

    // Create second vault with deposit using different salt
    let salt_2 = BytesN::from_array(&test.env, &[1; 32]);
    test.factory_contract.create_defindex_vault_deposit(
        &test.manager,
        &roles,
        &2000u32,
        &asset_params,
        &salt_2,
        &test.emergency_manager,
        &name_symbol,
        &true,
        &amounts,
    );

    let deployed_vaults = test.factory_contract.deployed_vaults();
    assert_eq!(deployed_vaults.len(), 2);

    // Verify first vault balances
    let first_vault = deployed_vaults.get(0).unwrap();
    let token_0_first_vault_balance = test.token0.balance(&first_vault);
    let token_1_first_vault_balance = test.token1.balance(&first_vault);
    assert_eq!(token_0_first_vault_balance, amount_0);
    assert_eq!(token_1_first_vault_balance, amount_1);

    // Verify second vault balances
    let second_vault = deployed_vaults.get(1).unwrap();
    let token_0_second_vault_balance = test.token0.balance(&second_vault);
    let token_1_second_vault_balance = test.token1.balance(&second_vault);
    assert_eq!(token_0_second_vault_balance, amount_0);
    assert_eq!(token_1_second_vault_balance, amount_1);
}
