#![cfg(test)]
pub extern crate std;

pub const ONE_DAY_IN_SECONDS: u64 = 86_400;
pub const REWARD_THRESHOLD: i128 = 40_0000000;

use crate::{
    blend_pool::{self, BlendPoolClient, Request},
    storage::ONE_DAY_IN_LEDGERS,
    BlendStrategy,
};
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::{
    testutils::{BytesN as _, Ledger as _, LedgerInfo, Address as _},
    token::StellarAssetClient,
    vec, Address, BytesN, Env, IntoVal, String, Symbol, Val, Vec,
};

use blend_contract_sdk::pool::{
    Client as PoolClient, ReserveConfig, ReserveEmissionMetadata,
}; 
use blend_contract_sdk::testutils::BlendFixture;


mod blend_factory_pool {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/pool_factory.wasm");
}

mod blend_emitter {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/emitter.wasm");
}

mod blend_backstop {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/backstop.wasm");
}

mod blend_comet {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/comet.wasm");
}

pub(crate) fn register_blend_strategy(
    e: &Env,
    asset: &Address,
    blend_pool: &Address,
    reserve_id: &u32,
    blend_token: &Address,
    soroswap_router: &Address,
    claim_ids: Vec<u32>,
    reward_threshold: i128,
) -> Address {
    let init_args: Vec<Val> = vec![
        e,
        blend_pool.into_val(e),
        reserve_id.into_val(e),
        blend_token.into_val(e),
        soroswap_router.into_val(e),
        claim_ids.into_val(e),
        reward_threshold.into_val(e),
    ];

    let args = (asset, init_args);
    e.register(BlendStrategy, args)
}


pub(crate) fn create_blend_pool(
    e: &Env,
    blend_fixture: &BlendFixture,
    admin: &Address,
    usdc: &MockTokenClient,
    xlm: &MockTokenClient,
) -> Address {
    // Mint usdc to admin
    usdc.mint(&admin, &200_000_0000000);
    // Mint xlm to admin
    xlm.mint(&admin, &200_000_0000000);

    // set up oracle
    let (oracle, oracle_client) = create_mock_oracle(e);
    oracle_client.set_data(
        &admin,
        &Asset::Other(Symbol::new(&e, "USD")),
        &vec![
            e,
            Asset::Stellar(usdc.address.clone()),
            Asset::Stellar(xlm.address.clone()),
        ],
        &7,
        &300,
    );
    oracle_client.set_price_stable(&vec![e, 1_000_0000, 100_0000]);
    let salt = BytesN::<32>::random(&e);
    let pool = blend_fixture.pool_factory.deploy(
        &admin,
        &String::from_str(e, "TEST"),
        &salt,
        &oracle,
        &0,
        &4,
        &1_0000000
    );
    let pool_client = PoolClient::new(e, &pool);
    blend_fixture
        .backstop
        .deposit(&admin, &pool, &20_0000_0000000);
    let reserve_config = ReserveConfig {
        c_factor: 900_0000,
        decimals: 7,
        index: 0,
        l_factor: 900_0000,
        max_util: 900_0000,
        reactivity: 0,
        r_base: 100_0000,
        r_one: 0,
        r_two: 0,
        r_three: 0,
        util: 0,
        collateral_cap: 170_141_183_460_469_231_731_687_303_715_884_105_727,
        enabled: true,
    };
    pool_client.queue_set_reserve(&usdc.address, &reserve_config);
    pool_client.set_reserve(&usdc.address);
    pool_client.queue_set_reserve(&xlm.address, &reserve_config);
    pool_client.set_reserve(&xlm.address);
    let emission_config = vec![
        e,
        ReserveEmissionMetadata {
            res_index: 0,
            res_type: 0,
            share: 250_0000,
        },
        ReserveEmissionMetadata {
            res_index: 0,
            res_type: 1,
            share: 250_0000,
        },
        ReserveEmissionMetadata {
            res_index: 1,
            res_type: 0,
            share: 250_0000,
        },
        ReserveEmissionMetadata {
            res_index: 1,
            res_type: 1,
            share: 250_0000,
        },
    ];
    pool_client.set_emissions_config(&emission_config);
    pool_client.set_status(&0);
    blend_fixture.backstop.add_reward(&pool, &None);

    // wait a week and start emissions
    e.jump(ONE_DAY_IN_LEDGERS * 7);
    blend_fixture.emitter.distribute();
    return pool;
}

/// Create a Blend Strategy
pub(crate) fn create_blend_strategy(
    e: &Env,
    underlying_asset: &Address,
    blend_pool: &Address,
    reserve_id: &u32,
    blend_token: &Address,
    soroswap_router: &Address,
) -> Address {
    let address = register_blend_strategy(
        e,
        underlying_asset,
        blend_pool,
        reserve_id,
        blend_token,
        soroswap_router,
        vec![e, 0u32, 1u32, 2u32, 3u32],
        REWARD_THRESHOLD
    );

    address
}

pub trait EnvTestUtils {
    /// Jump the env by the given amount of ledgers. Assumes 5 seconds per ledger.
    fn jump(&self, ledgers: u32);

    /// Jump the env by the given amount of seconds. Incremends the sequence by 1.
    fn jump_time(&self, seconds: u64);

    /// Set the ledger to the default LedgerInfo
    ///
    /// Time -> 1441065600 (Sept 1st, 2015 12:00:00 AM UTC)
    /// Sequence -> 100
    fn set_default_info(&self);
}

impl EnvTestUtils for Env {
    fn jump(&self, ledgers: u32) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(ledgers as u64 * 5),
            protocol_version: 22,
            sequence_number: self.ledger().sequence().saturating_add(ledgers),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * ONE_DAY_IN_LEDGERS,
            min_persistent_entry_ttl: 30 * ONE_DAY_IN_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_IN_LEDGERS,
        });
    }

    fn jump_time(&self, seconds: u64) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(seconds),
            protocol_version: 22,
            sequence_number: self.ledger().sequence().saturating_add(1),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * ONE_DAY_IN_LEDGERS,
            min_persistent_entry_ttl: 30 * ONE_DAY_IN_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_IN_LEDGERS,
        });
    }

    fn set_default_info(&self) {
        self.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 12:00:00 AM UTC
            protocol_version: 22,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * ONE_DAY_IN_LEDGERS,
            min_persistent_entry_ttl: 30 * ONE_DAY_IN_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_IN_LEDGERS,
        });
    }
}

/// Oracle
use sep_40_oracle::testutils::{Asset, MockPriceOracleClient, MockPriceOracleWASM};

pub fn create_mock_oracle<'a>(e: &Env) -> (Address, MockPriceOracleClient<'a>) {
    let contract_id = Address::generate(e);
    e.register_at(&contract_id, MockPriceOracleWASM, ());
    (
        contract_id.clone(),
        MockPriceOracleClient::new(e, &contract_id),
    )
}

/// Mock pool to test b_rate updates
pub mod mockpool {

    use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol};

    use super::EnvTestUtils;

    const BRATE: Symbol = symbol_short!("b_rate");
    #[derive(Clone, Debug)]
    #[contracttype]
    pub struct Reserve {
        pub asset: Address,        // the underlying asset address
        pub config: ReserveConfig, // the reserve configuration
        pub data: ReserveData,     // the reserve data
        pub scalar: i128,
    }

    #[derive(Clone, Debug, Default)]
    #[contracttype]
    pub struct ReserveConfig {
        pub index: u32,           // the index of the reserve in the list
        pub decimals: u32,        // the decimals used in both the bToken and underlying contract
        pub c_factor: u32, // the collateral factor for the reserve scaled expressed in 7 decimals
        pub l_factor: u32, // the liability factor for the reserve scaled expressed in 7 decimals
        pub util: u32,     // the target utilization rate scaled expressed in 7 decimals
        pub max_util: u32, // the maximum allowed utilization rate scaled expressed in 7 decimals
        pub r_base: u32, // the R0 value (base rate) in the interest rate formula scaled expressed in 7 decimals
        pub r_one: u32,  // the R1 value in the interest rate formula scaled expressed in 7 decimals
        pub r_two: u32,  // the R2 value in the interest rate formula scaled expressed in 7 decimals
        pub r_three: u32, // the R3 value in the interest rate formula scaled expressed in 7 decimals
        pub reactivity: u32, // the reactivity constant for the reserve scaled expressed in 7 decimals
        pub collateral_cap: i128, // the total amount of underlying tokens that can be used as collateral
        pub enabled: bool,        // the flag of the reserve
    }

    #[derive(Clone, Debug, Default)]
    #[contracttype]
    pub struct ReserveData {
        pub d_rate: i128,   // the conversion rate from dToken to underlying with 12 decimals
        pub b_rate: i128,   // the conversion rate from bToken to underlying with 12 decimals
        pub ir_mod: i128,   // the interest rate curve modifier with 7 decimals
        pub b_supply: i128, // the total supply of b tokens, in the underlying token's decimals
        pub d_supply: i128, // the total supply of d tokens, in the underlying token's decimals
        pub backstop_credit: i128, // the amount of underlying tokens currently owed to the backstop
        pub last_time: u64, // the last block the data was updated
    }

    #[contract]
    pub struct MockPool;

    #[contractimpl]
    impl MockPool {
        pub fn __constructor(e: Env, b_rate: i128) {
            e.storage().instance().set(&BRATE, &b_rate);
        }

        pub fn set_b_rate(e: Env, b_rate: i128) {
            e.storage().instance().set(&BRATE, &b_rate);
        }

        /// Note: We're only interested in the `b_rate`
        pub fn get_reserve(e: Env, reserve: Address) -> Reserve {
            let mut r_data = ReserveData::default();
            r_data.b_rate = e.storage().instance().get(&BRATE).unwrap_or(0);
            Reserve {
                asset: reserve,
                config: ReserveConfig::default(),
                data: r_data,
                scalar: 0,
            }
        }
    }

    pub fn register_mock_pool_with_b_rate(e: &Env, b_rate: i128) -> MockPoolClient {
        let pool_address = e.register(MockPool {}, (b_rate,));
        MockPoolClient::new(e, &pool_address)
    }

    /// Updates the mock pool's b_rate and also updates
    /// the timestamp to make sure `reserve_vault::update_rate` doesn't return early.
    pub fn set_b_rate(e: &Env, mock_pool_client: &MockPoolClient, b_rate: i128) {
        e.jump(5);
        mock_pool_client.set_b_rate(&b_rate);
    }
}
mod blend;
