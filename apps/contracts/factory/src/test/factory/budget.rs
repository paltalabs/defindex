extern crate std;

use crate::test::{create_asset_params, create_defindex_factory, DeFindexFactoryTest};
use soroban_sdk::{vec, BytesN, String, Vec};

#[test]
fn budget() {
    let test = DeFindexFactoryTest::setup();
    test.env.mock_all_auths();

    test.env.budget().reset_unlimited();

    // initialize factory contract
    let factory_contract = create_defindex_factory(
        &test.env,
        &test.admin,
        &test.defindex_receiver,
        2000u32,
        &test.defindex_wasm_hash,
    );

    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();

    std::println!("create_defindex_factory()                                              | cpu: {},      mem: {}", cpu, mem);

    test.env.budget().reset_unlimited();

    // create defindex vault

    let asset_params = create_asset_params(&test);

    let salt = BytesN::from_array(&test.env, &[0; 32]);

    let _ = factory_contract.create_defindex_vault(
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

    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("create_defindex_vault()                                                | cpu: {},      mem: {}", cpu, mem);

    test.env.budget().reset_unlimited();
    // create defindex vault deposit
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

    let mem = test.env.budget().memory_bytes_cost();
    let cpu = test.env.budget().cpu_instruction_cost();
    std::println!("create_defindex_vault_deposit()                                        | cpu: {},      mem: {}", cpu, mem);
}
