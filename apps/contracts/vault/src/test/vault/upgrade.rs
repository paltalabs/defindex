use soroban_sdk::testutils::{MockAuth, MockAuthInvoke};
use soroban_sdk::{vec as sorobanvec, Address, IntoVal, Map, String, Vec};

use crate::test::defindex_vault::{
    self, AssetStrategySet, ContractError, RolesDataKey
};
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0,
    DeFindexVaultTest,
};

#[test]
fn upgrade_success() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // initialize with 1 assets
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
        true
    );

    let installed_new_wasm = test.env.deployer().upload_contract_wasm(defindex_vault::WASM);
    
    defindex_contract.mock_auths(&[MockAuth {
        address: &test.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "upgrade",
            args: sorobanvec!(&test.env, installed_new_wasm.clone().to_val()),
            sub_invokes: &[],
        },
    }
    ]).upgrade(&installed_new_wasm);
    
}

#[test]
#[should_panic(expected = "HostError: Error(Auth, InvalidAction)")]
fn upgrade_not_manager() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // initialize with 1 assets
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
        true
    );

    let installed_new_wasm = test.env.deployer().upload_contract_wasm(defindex_vault::WASM);
    
    defindex_contract.mock_auths(&[MockAuth {
        address: &test.emergency_manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "upgrade",
            args: sorobanvec!(&test.env, installed_new_wasm.clone().to_val()),
            sub_invokes: &[],
        },
    }
    ]).upgrade(&installed_new_wasm);
    
}

#[test]
fn upgrade_not_upgradable() {
    let test = DeFindexVaultTest::setup();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);

    // initialize with 1 assets
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
        false
    );

    let installed_new_wasm = test.env.deployer().upload_contract_wasm(defindex_vault::WASM);
    
    let result = defindex_contract.mock_auths(&[MockAuth {
        address: &test.manager.clone(),
        invoke: &MockAuthInvoke {
            contract: &defindex_contract.address.clone(),
            fn_name: "upgrade",
            args: sorobanvec!(&test.env, installed_new_wasm.clone().to_val()),
            sub_invokes: &[],
        },
    }
    ]).try_upgrade(&installed_new_wasm);

    assert_eq!(result, Err(Ok(ContractError::NotUpgradable)));
}