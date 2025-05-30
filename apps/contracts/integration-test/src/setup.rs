pub mod blend_setup;
use blend_setup::{create_blend_pool, BlendFixture, BlendPoolClient};
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke}, token::{StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient}, vec as sorobanvec, Address, Env, IntoVal, Map, String
};

mod soroswap_setup;
pub use soroswap_setup::{
    create_soroswap_pool, create_soroswap_factory, create_soroswap_router
};
use crate::{blend_strategy::{create_blend_strategy_contract, BlendStrategyClient}, factory::{AssetStrategySet, Strategy}};
use crate::fixed_strategy::{create_fixed_strategy_contract, FixedStrategyClient};
use crate::hodl_strategy::{create_hodl_strategy_contract, HodlStrategyClient};
use crate::test::IntegrationTest;
use crate::token::create_token;
use crate::vault::defindex_vault_contract::VaultContractClient;

pub struct VaultOneAseetHodlStrategy<'a> {
    pub setup: IntegrationTest<'a>,
    pub token: SorobanTokenClient<'a>,
    pub token_admin: Address,
    pub token_admin_client: SorobanTokenAdminClient<'a>,
    pub strategy_contract: HodlStrategyClient<'a>,
    pub vault_contract: VaultContractClient<'a>,
    pub manager: Address,
    pub emergency_manager: Address,
    pub fee_receiver: Address,
    pub vault_fee: u32,
}

pub static VAULT_FEE: u32 = 100;

//Soroswap setup
pub fn mock_mint(
    env: &Env,
    token_admin_client: &SorobanTokenAdminClient,
    token_admin: &Address,
    to: &Address,
    amount: &i128,
) {
    token_admin_client
        .mock_auths(&[MockAuth {
            address: &token_admin,
            invoke: &MockAuthInvoke {
                contract: &token_admin_client.address.clone(),
                fn_name: "mint",
                args: sorobanvec![&env, to.into_val(env), amount.into_val(env)],
                sub_invokes: &[],
            },
        }])
        .mint(&to, &amount);
}

pub fn create_vault_one_asset_hodl_strategy<'a>() -> VaultOneAseetHodlStrategy<'a> {
    let setup = IntegrationTest::setup();

    let token_admin = Address::generate(&setup.env);
    let (token, token_admin_client) = create_token(&setup.env, &token_admin);

     // Soroswap Setup
     let soroswap_admin = Address::generate(&setup.env);

     let amount_0: i128 = 1_000_000_000_000_000_000;

     mock_mint(
         &setup.env,
         &token_admin_client,
         &token_admin,
         &soroswap_admin,
         &amount_0,
     );

     let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
     let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    
     // let soroswap_pair = soroswap_factory.get_pair(&token_0.address, &token_1.address);

     setup.env.cost_estimate().budget().reset_unlimited();

    let strategy_contract = create_hodl_strategy_contract(&setup.env, &token.address);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
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

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &soroswap_router.address,
        &name_symbol,
        &true,
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
    pub token: SorobanTokenClient<'a>,
    pub token_admin: Address,
    pub token_admin_client: SorobanTokenAdminClient<'a>,
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

    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);

    let amount_0: i128 = 1_000_000_000_000_000_000;

    mock_mint(
        &setup.env,
        &token_admin_client,
        &token_admin,
        &soroswap_admin,
        &amount_0,
    );

    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);


    // let soroswap_pair = soroswap_factory.get_pair(&token_0.address, &token_1.address);

    setup.env.cost_estimate().budget().reset_unlimited();


    setup.env.mock_all_auths();
    let strategy_contract =
        create_fixed_strategy_contract(&setup.env, &token.address, 1000u32, &token_admin_client);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
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

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &soroswap_router.address,
        &name_symbol,
        &true
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

pub struct VaultOneBlendStrategy<'a> {
    pub setup: IntegrationTest<'a>,
    pub usdc: SorobanTokenClient<'a>,
    pub usdc_client: SorobanTokenAdminClient<'a>,
    pub blnd: SorobanTokenClient<'a>,
    pub blnd_client: SorobanTokenAdminClient<'a>,
    pub xlm: SorobanTokenClient<'a>,
    pub xlm_client: SorobanTokenAdminClient<'a>,
    pub strategy_contract: BlendStrategyClient<'a>,
    pub vault_contract: VaultContractClient<'a>,
    pub manager: Address,
    pub emergency_manager: Address,
    pub fee_receiver: Address,
    pub vault_fee: u32,
    pub blend_pool_client: BlendPoolClient<'a>,
    pub admin: Address,
    pub keeper: Address,
}

pub fn create_vault_one_blend_strategy<'a>() -> VaultOneBlendStrategy<'a> {
    let setup = IntegrationTest::setup();
    setup.env.mock_all_auths();
    
    // Soroswap Setup
    let soroswap_admin = Address::generate(&setup.env);

    let soroswap_factory = create_soroswap_factory(&setup.env, &soroswap_admin);
    let soroswap_router = create_soroswap_router(&setup.env, &soroswap_factory.address);

    let admin = Address::generate(&setup.env);
    let keeper = Address::generate(&setup.env);

    let (blnd, blnd_client) = create_token(&setup.env, &admin);
    let (usdc, usdc_client) = create_token(&setup.env, &admin);
    let (xlm, xlm_client) = create_token(&setup.env, &admin);

    // Setting up soroswap pool
    let pool_admin = Address::generate(&setup.env);
    let amount_a = 100000000_0_000_000;
    let amount_b = 50000000_0_000_000;
    blnd_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    create_soroswap_pool(
        &setup.env,
        &soroswap_router,
        &pool_admin,
        &blnd.address,
        &usdc.address,
        &amount_a,
        &amount_b,
    );
    // End of setting up soroswap pool

    let blend_fixture = BlendFixture::deploy(&setup.env, &admin, &blnd.address, &usdc.address);

    let pool = create_blend_pool(&setup.env, &blend_fixture, &admin, &usdc_client, &xlm_client);
    let pool_client = BlendPoolClient::new(&setup.env, &pool);
    let strategy = create_blend_strategy_contract(
        &setup.env,
        &usdc.address,
        &pool,
        &blnd.address,
        &soroswap_router.address,
        40_0000000,
        &keeper,
    );
    let strategy_contract = BlendStrategyClient::new(&setup.env, &strategy);

    let emergency_manager = Address::generate(&setup.env);
    let rebalance_manager = Address::generate(&setup.env);
    let fee_receiver = Address::generate(&setup.env);
    let vault_fee = VAULT_FEE;
    let vault_name = String::from_str(&setup.env, "BlendVault");
    let vault_symbol = String::from_str(&setup.env, "BLNDVLT");
    let manager = Address::generate(&setup.env);

    let assets = sorobanvec![
        &setup.env,
        AssetStrategySet {
            address: usdc.address.clone(),
            strategies: sorobanvec![
                &setup.env,
                Strategy {
                    address: strategy_contract.address.clone(),
                    name: String::from_str(&setup.env, "Blend Strategy"),
                    paused: false,
                }
            ],
        }
    ];

    let mut roles: Map<u32, Address> = Map::new(&setup.env);
    roles.set(0u32, emergency_manager.clone()); // EmergencyManager enum = 0
    roles.set(1u32, fee_receiver.clone()); // VaultFeeReceiver enum = 1
    roles.set(2u32, manager.clone()); // Manager enum = 2
    roles.set(3u32, rebalance_manager.clone()); // RebalanceManager enum = 3

    let mut name_symbol: Map<String, String> = Map::new(&setup.env);
    name_symbol.set(String::from_str(&setup.env, "name"), vault_name);
    name_symbol.set(String::from_str(&setup.env, "symbol"), vault_symbol);

    let vault_contract_address = setup.factory_contract.create_defindex_vault(
        &roles,
        &vault_fee,
        &assets,
        &soroswap_router.address,
        &name_symbol,
        &true
    );

    let vault_contract = VaultContractClient::new(&setup.env, &vault_contract_address);

    VaultOneBlendStrategy {
        setup,
        usdc,
        usdc_client,
        blnd,
        blnd_client,
        xlm,
        xlm_client,
        strategy_contract,
        vault_contract,
        manager,
        emergency_manager,
        fee_receiver,
        vault_fee,
        blend_pool_client: pool_client,
        admin,
        keeper
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
        assert_eq!(setup.factory_contract.total_vaults(), 1);

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
        assert_eq!(vault_name, String::from_str(&setup.env, "DeFindex-Vault-HodlVault"));

        let vault_symbol = enviroment.vault_contract.symbol();
        assert_eq!(vault_symbol, String::from_str(&setup.env, "HVLT"));
    }

    #[test]
    fn test_create_vault_one_asset_fixed_strategy() {
        let enviroment = create_vault_one_asset_fixed_strategy();
        let setup = enviroment.setup;
        assert_eq!(setup.factory_contract.total_vaults(), 1);

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

        let strategy_contract_balance = enviroment
            .token
            .balance(&enviroment.strategy_contract.address);
        assert_eq!(strategy_contract_balance, 100_000_000_000_0_000_000i128);

        let vault_emergency_manager = enviroment.vault_contract.get_emergency_manager();
        assert_eq!(vault_emergency_manager, enviroment.emergency_manager);

        let vault_fee_receiver = enviroment.vault_contract.get_fee_receiver();
        assert_eq!(vault_fee_receiver, enviroment.fee_receiver);

        let vault_manager = enviroment.vault_contract.get_manager();
        assert_eq!(vault_manager, enviroment.manager);

        let vault_name = enviroment.vault_contract.name();
        assert_eq!(vault_name, String::from_str(&setup.env, "DeFindex-Vault-FixedVault"));

        let vault_symbol = enviroment.vault_contract.symbol();
        assert_eq!(vault_symbol, String::from_str(&setup.env, "FVLT"));
    }

    #[test]
    fn test_create_vault_blend_strategy() {
        let enviroment = create_vault_one_blend_strategy();
        let setup = enviroment.setup;
        assert_eq!(setup.factory_contract.total_vaults(), 1);

        let strategy_token = enviroment.strategy_contract.asset();
        assert_eq!(strategy_token, enviroment.usdc.address);

        let assets = sorobanvec![
            &setup.env,
            VaultAssetStrategySet {
                address: enviroment.usdc.address.clone(),
                strategies: sorobanvec![
                    &setup.env,
                    VaultStrategy {
                        address: enviroment.strategy_contract.address.clone(),
                        name: String::from_str(&setup.env, "Blend Strategy"),
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
        assert_eq!(vault_name, String::from_str(&setup.env, "DeFindex-Vault-BlendVault"));

        let vault_symbol = enviroment.vault_contract.symbol();
        assert_eq!(vault_symbol, String::from_str(&setup.env, "BLNDVLT"));
    }
}

