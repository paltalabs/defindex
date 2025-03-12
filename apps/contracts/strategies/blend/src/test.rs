#![cfg(test)]
pub extern crate std;

pub const ONE_DAY_IN_SECONDS: u64 = 86_400;
pub const REWARD_THRESHOLD: i128 = 1;
pub const SCALAR_7: i128 = 1_0000000;

use crate::{
    storage::ONE_DAY_LEDGERS,
    BlendStrategy,
};
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::{
    testutils::{BytesN as _, Ledger as _, LedgerInfo, Address as _},
    vec, Address, BytesN, Env, IntoVal, String, Symbol, Val, Vec,
};
use soroban_sdk::{token::StellarAssetClient};
use soroban_fixed_point_math::FixedPoint;

// Blend Fixture
pub mod comet {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/comet.wasm");
}
pub mod backstop {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/backstop.wasm");
}
use backstop::{Client as BackstopClient};

pub fn create_backstop<'a>(
    e: &Env,
    contract_id: &Address,
    backstop_token: &Address,
    emitter: &Address,
    blnd_token: &Address,
    usdc_token: &Address,
    pool_factory: &Address,
    drop_list: &Vec<(Address, i128)>,
) -> BackstopClient<'a> {
        e.register_at(
            contract_id,
            backstop::WASM, 
            (
                backstop_token,
                emitter,
                blnd_token,
                usdc_token,
                pool_factory,
                drop_list.clone(),
            ),
        );
    BackstopClient::new(e, &contract_id)
}

pub mod emitter {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/emitter.wasm");
}


// Pool Factory
pub mod pool_factory {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/pool_factory.wasm");
}

use pool_factory::{Client as PoolFactoryClient, PoolInitMeta};

pub fn create_pool_factory<'a>(
    e: &Env,
    contract_id: &Address,
    pool_init_meta: PoolInitMeta,
) -> PoolFactoryClient<'a> {
    e.register_at(&contract_id, pool_factory::WASM, (pool_init_meta,));
    PoolFactoryClient::new(e, &contract_id)
}

pub mod pool {
    soroban_sdk::contractimport!(file = "../external_wasms/blend/pool.wasm");
}
use pool::{
    Client as PoolClient, ReserveConfig, ReserveEmissionMetadata,
};


/// Fixture for deploying and interacting with the Blend Protocol contracts in Rust tests.
pub struct BlendFixture<'a> {
    // pub dummy: Address<'a>,
    pub backstop: backstop::Client<'a>,
    pub emitter: emitter::Client<'a>,
    pub backstop_token: comet::Client<'a>,
    pub pool_factory: pool_factory::Client<'a>,
}

impl<'a> BlendFixture<'a> {
    /// Deploy a new set of Blend Protocol contracts. Mints 200k backstop
    /// tokens to the deployer that can be used in the future to create up to 4
    /// reward zone pools (50k tokens each).
    ///
    /// This function also resets the env budget via `reset_unlimited`.
    ///
    /// ### Arguments
    /// * `env` - The environment to deploy the contracts in
    /// * `deployer` - The address of the deployer
    /// * `blnd` - The address of the BLND token
    /// * `usdc` - The address of the USDC token
    pub fn deploy(
        env: &Env,
        deployer: &Address,
        blnd: &Address,
        usdc: &Address,
    ) -> BlendFixture<'a> {
        env.cost_estimate().budget().reset_unlimited();
        let backstop_id = Address::generate(&env);
        let pool_factory = Address::generate(&env);

        let emitter = env.register(emitter::WASM, ());
        let comet = env.register(comet::WASM, ());

        let blnd_client = StellarAssetClient::new(env, &blnd);
        let usdc_client = StellarAssetClient::new(env, &usdc);
        blnd_client
            .mock_all_auths()
            .mint(deployer, &(1_000_0000000 * 2001));
        usdc_client
            .mock_all_auths()
            .mint(deployer, &(25_0000000 * 2001));

        let comet_client: comet::Client<'a> = comet::Client::new(env, &comet);
        comet_client.mock_all_auths().init(
            &deployer,
            &vec![env, blnd.clone(), usdc.clone()],
            &vec![env, 0_8000000, 0_2000000],
            &vec![env, 1_000_0000000, 25_0000000],
            &0_0030000,
        );

        comet_client.mock_all_auths().join_pool(
            &199_900_0000000, // finalize mints 100
            &vec![env, 1_000_0000000 * 2000, 25_0000000 * 2000],
            deployer,
        );

        blnd_client.mock_all_auths().set_admin(&emitter);
        let emitter_client: emitter::Client<'a> = emitter::Client::new(env, &emitter);
        emitter_client
            .mock_all_auths()
            .initialize(&blnd, &backstop_id, &comet);

        let empty_vec: Vec<(Address, i128)> = vec![&env];


        let backstop_client = create_backstop(
            &env,
            &backstop_id,
            &comet,
            &emitter,
            &blnd,
            &usdc,
            &pool_factory,
            &empty_vec,
        );
        
        let pool_hash = env.deployer().upload_contract_wasm(pool::WASM);
        
        let pool_init_meta = PoolInitMeta {
            backstop: backstop_id.clone(),
            pool_hash: pool_hash.clone(),
            blnd_id: blnd.clone(),
        };

        let pool_factory_client = create_pool_factory(&env, &pool_factory.clone(), pool_init_meta);

        // start distribution period
        backstop_client.distribute();
     

        BlendFixture {
            backstop: backstop_client,
            emitter: emitter_client,
            backstop_token: comet_client,
            pool_factory: pool_factory_client,
        }
    }
}


pub(crate) fn create_blend_pool(
    e: &Env,
    blend_fixture: &BlendFixture,
    admin: &Address,
    usdc: &MockTokenClient,
    xlm: &MockTokenClient,
    blnd: &MockTokenClient,
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
    blend_fixture.backstop.add_reward(&pool, &None);
    
    pool_client.set_status(&0);

    assert_eq!(blend_fixture.emitter.get_backstop(), blend_fixture.backstop.address);
    assert_eq!(blnd.balance(&blend_fixture.backstop.address), 0);

    // start time is now
    let start_time = e.ledger().timestamp();
    assert_eq!(blend_fixture.emitter.get_last_distro(&blend_fixture.backstop.address), start_time);

    // wait a week and start emissions
    e.jump(ONE_DAY_LEDGERS * 7);
    // 7*24*60*60 = 604800
    let distribution = blend_fixture.emitter.distribute();
    assert_eq!(distribution, 604800 * 10000000);
    assert_eq!(blend_fixture.emitter.get_last_distro(&blend_fixture.backstop.address), start_time + 604800);
    assert_eq!(blnd.balance(&blend_fixture.backstop.address), distribution);

    let backstop_distribution = blend_fixture.backstop.distribute();
    assert_eq!(backstop_distribution, 604800 * 10000000);

    let pool_emissions = pool_client.gulp_emissions();
    assert_ne!(pool_emissions, 0); // We have some emissionss
    return pool;
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
            min_temp_entry_ttl: 30 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 30 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }

    fn jump_time(&self, seconds: u64) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(seconds),
            protocol_version: 22,
            sequence_number: self.ledger().sequence().saturating_add(1),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 30 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }

    fn set_default_info(&self) {
        self.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 12:00:00 AM UTC
            protocol_version: 22,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * ONE_DAY_LEDGERS,
            min_persistent_entry_ttl: 30 * ONE_DAY_LEDGERS,
            max_entry_ttl: 365 * ONE_DAY_LEDGERS,
        });
    }
}

/// Asset that `b` is within `percentage` of `a` where `percentage`
/// is a percentage in decimal form as a fixed-point number with 7 decimal
/// places
pub fn assert_approx_eq_rel(a: i128, b: i128, percentage: i128) {
    let rel_delta = b.fixed_mul_floor(percentage, SCALAR_7).unwrap();

    assert!(
        a > b - rel_delta && a < b + rel_delta,
        "assertion failed: `(left != right)` \
         (left: `{:?}`, right: `{:?}`, epsilon: `{:?}`)",
        a,
        b,
        rel_delta
    );
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

}

// Blend Strategy

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
    // let blend_pool_address: Address = init_args
    //         .get(0)
    //         .expect("Invalid argument: blend_pool_address")
    //         .into_val(&e);
    //     let reserve_id: u32 = init_args
    //         .get(1)
    //         .expect("Invalid argument: reserve_id")
    //         .into_val(&e);
    //     let blend_token: Address = init_args
    //         .get(2)
    //         .expect("Invalid argument: blend_token")
    //         .into_val(&e);
    //     let soroswap_router: Address = init_args
    //         .get(3)
    //         .expect("Invalid argument: soroswap_router")
    //         .into_val(&e);
    //     let claim_ids: Vec<u32> = init_args
    //         .get(4)
    //         .expect("Invalid argument: claim_ids")
    //         .into_val(&e);
    //     let reward_threshold: i128 = init_args
    //         .get(5)
    //         .expect("Invalid argument: reward_threshold")
    //         .into_val(&e);
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



/// Create a Blend Strategy
pub(crate) fn create_blend_strategy(
    e: &Env,
    underlying_asset: &Address,
    blend_pool: &Address,
    reserve_id: &u32,
    blend_token: &Address,
    soroswap_router: &Address,
) -> Address {
    // asset: &Address,
    // blend_pool: &Address,
    // reserve_id: &u32,
    // blend_token: &Address,
    // soroswap_router: &Address,
    // claim_ids: Vec<u32>,
    // reward_threshold: i128,
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

// Tests
mod blend;
