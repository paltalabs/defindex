use soroban_sdk::token::{StellarAssetClient, TokenClient};
use soroban_sdk::BytesN;
use soroban_sdk::{testutils::Address as _, vec as sorobanvec, Address, String};

use crate::fixed_strategy::{create_fixed_strategy_contract, FixedStrategyClient};
use crate::hodl_strategy::{create_hodl_strategy_contract, HodlStrategyClient};
use crate::test::IntegrationTest;
use crate::token::create_token;
use crate::factory::{AssetStrategySet, Strategy};
use crate::vault::defindex_vault_contract::VaultContractClient;

pub struct VaultOneAseetHodlStrategy<'a> {
    pub setup: IntegrationTest<'a>,
    pub token: TokenClient<'a>,
    pub token_admin: Address,
    pub token_admin_client: StellarAssetClient<'a>,
    pub strategy_contract: HodlStrategyClient<'a>,
    pub vault_contract: VaultContractClient<'a>,
    pub manager: Address,
    pub emergency_manager: Address,
    pub fee_receiver: Address,
    pub vault_fee: u32,
}

pub static VAULT_FEE: u32 = 100;

pub fn create_vault_one_asset_hodl_strategy<'a>() -> VaultOneAseetHodlStrategy<'a> {
    let setup = IntegrationTest::setup();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

    let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);

    let emergency_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "HodlVault");
    let vault_symbol = String::from_str(&setup.env, "HVLT");
    let manager = Address::generate(&setup.env);
    
    let assets = sorobanvec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: sorobanvec![
                &setup.env,
                Strategy {
                    address: strategy_contract.address.clone(),
                    name: String::from_str(&setup.env, "Hodl Strategy"),
                    paused: false,
                }
            ],
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &emergency_manager, 
        &fee_receiver, 
        &vault_fee, 
        &vault_name, 
        &vault_symbol, 
        &manager, 
        &assets, 
        &salt
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    VaultOneAseetHodlStrategy {
        setup,
        token,
        token_admin,
        token_admin_client,
        strategy_contract,
        vault_contract,
        manager,
        emergency_manager,
        fee_receiver,
        vault_fee,
    }
}

pub struct VaultOneAseetFixedStrategy<'a> {
    pub setup: IntegrationTest<'a>,
    pub token: TokenClient<'a>,
    pub token_admin: Address,
    pub token_admin_client: StellarAssetClient<'a>,
    pub strategy_contract: FixedStrategyClient<'a>,
    pub vault_contract: VaultContractClient<'a>,
    pub manager: Address,
    pub emergency_manager: Address,
    pub fee_receiver: Address,
    pub vault_fee: u32,
}

pub fn create_vault_one_asset_fixed_strategy<'a>() -> VaultOneAseetFixedStrategy<'a> {
    let setup = IntegrationTest::setup();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

    setup.env.mock_all_auths();
    let strategy_contract = create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);

    let emergency_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "FixedVault");
    let vault_symbol = String::from_str(&setup.env, "FVLT");
    let manager = Address::generate(&setup.env);
    
    let assets = sorobanvec![
        &setup.env,
        AssetStrategySet {
            address: token.address.clone(),
            strategies: sorobanvec![
                &setup.env,
                Strategy {
                    address: strategy_contract.address.clone(),
                    name: String::from_str(&setup.env, "Fixed Strategy"),
                    paused: false,
                }
            ],
        }
    ];

    let salt = BytesN::from_array(&setup.env, &[0; 32]);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &emergency_manager, 
        &fee_receiver, 
        &vault_fee, 
        &vault_name, 
        &vault_symbol, 
        &manager, 
        &assets, 
        &salt
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    VaultOneAseetFixedStrategy {
        setup,
        token,
        token_admin,
        token_admin_client,
        strategy_contract,
        vault_contract,
        manager,
        emergency_manager,
        fee_receiver,
        vault_fee,
    }
}

#[cfg(test)]
mod tests {
    use crate::vault::{VaultAssetStrategySet, VaultStrategy};

    use super::*;

    #[test]
    fn test_create_vault_one_asset_hodl_strategy() {
        let enviroment = create_vault_one_asset_hodl_strategy();
        let setup = enviroment.setup;
        assert_eq!(setup.factory_contract.deployed_defindexes().len(), 1);

        let strategy_token = enviroment.strategy_contract.asset();
        assert_eq!(strategy_token, enviroment.token.address);

        let assets = sorobanvec![
            &setup.env,
            VaultAssetStrategySet {
                address: enviroment.token.address.clone(),
                strategies: sorobanvec![
                    &setup.env,
                    VaultStrategy {
                        address: enviroment.strategy_contract.address.clone(),
                        name: String::from_str(&setup.env, "Hodl Strategy"),
                        paused: false,
                    }
                ],
            }
        ];

        let vault_assets = enviroment.vault_contract.get_assets();
        assert_eq!(vault_assets, assets);

        let vault_emergency_manager = enviroment.vault_contract.get_emergency_manager();
        assert_eq!(vault_emergency_manager, enviroment.emergency_manager);

        let vault_fee_receiver = enviroment.vault_contract.get_fee_receiver();
        assert_eq!(vault_fee_receiver, enviroment.fee_receiver);

        let vault_manager = enviroment.vault_contract.get_manager();
        assert_eq!(vault_manager, enviroment.manager);

        let vault_name = enviroment.vault_contract.name();
        assert_eq!(vault_name, String::from_str(&setup.env, "HodlVault"));

        let vault_symbol = enviroment.vault_contract.symbol();
        assert_eq!(vault_symbol, String::from_str(&setup.env, "HVLT"));
    }

    #[test]
    fn test_create_vault_one_asset_fixed_strategy() {
        let enviroment = create_vault_one_asset_fixed_strategy();
        let setup = enviroment.setup;
        assert_eq!(setup.factory_contract.deployed_defindexes().len(), 1);

        let strategy_token = enviroment.strategy_contract.asset();
        assert_eq!(strategy_token, enviroment.token.address);

        let assets = sorobanvec![
            &setup.env,
            VaultAssetStrategySet {
                address: enviroment.token.address.clone(),
                strategies: sorobanvec![
                    &setup.env,
                    VaultStrategy {
                        address: enviroment.strategy_contract.address.clone(),
                        name: String::from_str(&setup.env, "Fixed Strategy"),
                        paused: false,
                    }
                ],
            }
        ];
        
        let vault_assets = enviroment.vault_contract.get_assets();
        assert_eq!(vault_assets, assets);

        let strategy_contract_balance = enviroment.token.balance(&enviroment.strategy_contract.address);
        assert_eq!(strategy_contract_balance, 100_000_000_000_0_000_000i128);

        let vault_emergency_manager = enviroment.vault_contract.get_emergency_manager();
        assert_eq!(vault_emergency_manager, enviroment.emergency_manager);

        let vault_fee_receiver = enviroment.vault_contract.get_fee_receiver();
        assert_eq!(vault_fee_receiver, enviroment.fee_receiver);

        let vault_manager = enviroment.vault_contract.get_manager();
        assert_eq!(vault_manager, enviroment.manager);

        let vault_name = enviroment.vault_contract.name();
        assert_eq!(vault_name, String::from_str(&setup.env, "FixedVault"));

        let vault_symbol = enviroment.vault_contract.symbol();
        assert_eq!(vault_symbol, String::from_str(&setup.env, "FVLT"));
    }

}