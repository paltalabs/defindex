use soroban_sdk::{
    testutils::BytesN as _,
    token::StellarAssetClient,
    vec, Address, BytesN, Env, String, Symbol, Vec,
};

mod blend_pool {
    soroban_sdk::contractimport!(file = "../strategies/external_wasms/blend/blend_pool.wasm");
}
pub use blend_pool::{Client as BlendPoolClient, Request, ReserveConfig, ReserveEmissionMetadata};

mod blend_factory_pool {
    soroban_sdk::contractimport!(file = "../strategies/external_wasms/blend/pool_factory.wasm");
}

mod blend_emitter {
    soroban_sdk::contractimport!(file = "../strategies/external_wasms/blend/emitter.wasm");
}

mod blend_backstop {
    soroban_sdk::contractimport!(file = "../strategies/external_wasms/blend/backstop.wasm");
}

mod blend_comet {
    soroban_sdk::contractimport!(file = "../strategies/external_wasms/blend/comet.wasm");
}

pub struct BlendFixture<'a> {
    pub backstop: blend_backstop::Client<'a>,
    pub emitter: blend_emitter::Client<'a>,
    pub _backstop_token: blend_comet::Client<'a>,
    pub pool_factory: blend_factory_pool::Client<'a>,
}

pub(crate) fn create_blend_pool(
    e: &Env,
    blend_fixture: &BlendFixture,
    admin: &Address,
    usdc: &StellarAssetClient,
    xlm: &StellarAssetClient,
) -> Address {
    // Mint usdc to admin
    usdc.mint(admin, &200_000_0000000);
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
    );
    let pool_client = BlendPoolClient::new(e, &pool);
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
    blend_fixture.backstop.add_reward(&pool, &pool);

    // wait a week and start emissions
    e.jump(DAY_IN_LEDGERS * 7);
    blend_fixture.emitter.distribute();
    blend_fixture.backstop.gulp_emissions();
    pool_client.gulp_emissions();

    // admin joins pool
    let requests = vec![
        e,
        Request {
            address: usdc.address.clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: usdc.address.clone(),
            amount: 100_000_0000000,
            request_type: 4,
        },
        Request {
            address: xlm.address.clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: xlm.address.clone(),
            amount: 100_000_0000000,
            request_type: 4,
        },
    ];
    pool_client
        .mock_all_auths()
        .submit(&admin, &admin, &admin, &requests);
    return pool;
}

/// Oracle
use sep_40_oracle::testutils::{Asset, MockPriceOracleClient, MockPriceOracleWASM};

use crate::test::{EnvTestUtils, DAY_IN_LEDGERS};

pub fn create_mock_oracle<'a>(e: &Env) -> (Address, MockPriceOracleClient<'a>) {
    let contract_id = e.register(MockPriceOracleWASM, ());
    (
        contract_id.clone(),
        MockPriceOracleClient::new(e, &contract_id),
    )
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
        let backstop = env.register(blend_backstop::WASM, ());
        let emitter = env.register(blend_emitter::WASM, ());
        let comet = env.register(blend_comet::WASM, ());
        let pool_factory = env.register(blend_factory_pool::WASM, ());
        let blnd_client = StellarAssetClient::new(env, &blnd);
        let usdc_client = StellarAssetClient::new(env, &usdc);
        blnd_client
            .mock_all_auths()
            .mint(deployer, &(1_000_0000000 * 2001));
        usdc_client
            .mock_all_auths()
            .mint(deployer, &(25_0000000 * 2001));

        let comet_client: blend_comet::Client<'a> = blend_comet::Client::new(env, &comet);
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
        let emitter_client: blend_emitter::Client<'a> = blend_emitter::Client::new(env, &emitter);
        emitter_client
            .mock_all_auths()
            .initialize(&blnd, &backstop, &comet);

        let backstop_client: blend_backstop::Client<'a> =
            blend_backstop::Client::new(env, &backstop);
        backstop_client.mock_all_auths().initialize(
            &comet,
            &emitter,
            &usdc,
            &blnd,
            &pool_factory,
            &Vec::new(env),
        );

        let pool_hash = env.deployer().upload_contract_wasm(blend_pool::WASM);

        let pool_factory_client = blend_factory_pool::Client::new(env, &pool_factory);
        pool_factory_client
            .mock_all_auths()
            .initialize(&blend_factory_pool::PoolInitMeta {
                backstop,
                blnd_id: blnd.clone(),
                pool_hash,
            });
        backstop_client.update_tkn_val();

        BlendFixture {
            backstop: backstop_client,
            emitter: emitter_client,
            _backstop_token: comet_client,
            pool_factory: pool_factory_client,
        }
    }
}
