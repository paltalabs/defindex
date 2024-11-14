extern crate std;
use crate::factory::DeFindexFactoryClient;
use soroban_sdk::{
    Env, 
    Address, 
    testutils::Address as _,
};
use std::vec as std_vec;

use crate::factory::create_factory_contract;
use crate::vault::defindex_vault_contract;

pub static ONE_YEAR_IN_SECONDS: u64 = 31_536_000;
pub static DEFINDEX_FEE: u32 = 50;

pub struct IntegrationTest<'a> {
    pub env: Env,
    pub factory_contract: DeFindexFactoryClient<'a>,
    pub admin: Address,
    pub defindex_receiver: Address,
    pub defindex_fee: u32
}

impl<'a> IntegrationTest<'a> {
    pub fn setup() -> Self {
        let env = Env::default();
        
        let admin = Address::generate(&env);
        let defindex_receiver = Address::generate(&env);

        let vault_wasm_hash = env.deployer().upload_contract_wasm(defindex_vault_contract::WASM);
        let defindex_fee = DEFINDEX_FEE;

        let factory_contract = create_factory_contract(&env, &admin, &defindex_receiver, &defindex_fee, &vault_wasm_hash);

        env.budget().reset_unlimited();

        IntegrationTest {
            env,
            factory_contract,
            admin,
            defindex_receiver,
            defindex_fee
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

#[cfg(test)]
mod test_vault_one_hodl_strategy;
mod test_vault_one_fixed_strategy;