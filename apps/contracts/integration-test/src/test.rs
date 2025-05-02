extern crate std;
use crate::factory::DeFindexFactoryClient;
use soroban_sdk::{
    testutils::Address as _,
    testutils::{Ledger, LedgerInfo},
    Address, Env,
};
use std::vec as std_vec;

use crate::factory::create_factory_contract;
use crate::vault::defindex_vault_contract;

pub static ONE_YEAR_IN_SECONDS: u64 = 31_536_000;
pub static DEFINDEX_FEE: u32 = 50;
pub static DAY_IN_LEDGERS: u32 = 17280;

pub struct IntegrationTest<'a> {
    pub env: Env,
    pub factory_contract: DeFindexFactoryClient<'a>,
    pub admin: Address,
    pub defindex_receiver: Address,
    pub defindex_fee: u32,
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
            min_temp_entry_ttl: 30 * DAY_IN_LEDGERS,
            min_persistent_entry_ttl: 30 * DAY_IN_LEDGERS,
            max_entry_ttl: 365 * DAY_IN_LEDGERS,
        });
    }

    fn jump_time(&self, seconds: u64) {
        self.ledger().set(LedgerInfo {
            timestamp: self.ledger().timestamp().saturating_add(seconds),
            protocol_version: 22,
            sequence_number: self.ledger().sequence().saturating_add(1),
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * DAY_IN_LEDGERS,
            min_persistent_entry_ttl: 30 * DAY_IN_LEDGERS,
            max_entry_ttl: 365 * DAY_IN_LEDGERS,
        });
    }

    fn set_default_info(&self) {
        self.ledger().set(LedgerInfo {
            timestamp: 1441065600, // Sept 1st, 2015 12:00:00 AM UTC
            protocol_version: 22,
            sequence_number: 100,
            network_id: Default::default(),
            base_reserve: 10,
            min_temp_entry_ttl: 30 * DAY_IN_LEDGERS,
            min_persistent_entry_ttl: 30 * DAY_IN_LEDGERS,
            max_entry_ttl: 365 * DAY_IN_LEDGERS,
        });
    }
}

impl<'a> IntegrationTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        env.set_default_info();

        let admin = Address::generate(&env);
        let defindex_receiver = Address::generate(&env);

        let vault_wasm_hash = env
            .deployer()
            .upload_contract_wasm(defindex_vault_contract::WASM);
        let defindex_fee = DEFINDEX_FEE;

        let factory_contract = create_factory_contract(
            &env,
            &admin,
            &defindex_receiver,
            &defindex_fee,
            &vault_wasm_hash,
        );

        env.cost_estimate().budget().reset_unlimited();

        IntegrationTest {
            env,
            factory_contract,
            admin,
            defindex_receiver,
            defindex_fee,
        }
    }

    pub fn generate_random_users(e: &Env, users_count: u32) -> std_vec::Vec<Address> {
        let mut users = std_vec![];
        for _c in 0..users_count {
            users.push(Address::generate(e));
        }
        users
    }
}

// #[cfg(test)]
#[allow(unused)]
mod vault_one_fixed_strategy;

#[allow(unused)]
mod vault_one_hodl_strategy;
#[allow(unused)]
mod vault_blend_strategy;

#[allow(unused)]
mod limits;
