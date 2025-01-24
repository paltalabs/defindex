extern crate std;

use crate::test::{create_asset_params, create_defindex_factory, DeFindexFactoryTest};
use soroban_sdk::{vec, Address, BytesN, Map, String, Vec};

#[test]
fn budget() {
    let test = DeFindexFactoryTest::setup();
    test.env.mock_all_auths();

    test.env.cost_estimate().budget().reset_unlimited();

    // initialize factory contract
    let factory_contract = create_defindex_factory(
        &test.env,
        &test.admin,
        &test.defindex_receiver,
        2000u32,
        &test.defindex_wasm_hash,
    );

    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();

    std::println!("create_defindex_factory()                                              | cpu: {},      mem: {}", cpu, mem);

    test.env.cost_estimate().budget().reset_unlimited();

    // create defindex vault

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

    factory_contract.create_defindex_vault(
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &name_symbol,
        &true
    );

    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("create_defindex_vault()                                                | cpu: {},      mem: {}", cpu, mem);

    test.env.cost_estimate().budget().reset_unlimited();
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
        &roles,
        &2000u32,
        &asset_params,
        &salt,
        &test.emergency_manager, //soroswap_router,
        &name_symbol,
        &true,
        &amounts,
    );

    let mem = test.env.cost_estimate().budget().memory_bytes_cost();
    let cpu = test.env.cost_estimate().budget().cpu_instruction_cost();
    std::println!("create_defindex_vault_deposit()                                        | cpu: {},      mem: {}", cpu, mem);
}
