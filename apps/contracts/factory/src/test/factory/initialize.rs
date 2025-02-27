use soroban_sdk::{testutils::Address as _, Address, Env};
use crate::error::FactoryError;
use crate::test::{create_defindex_factory, defindex_vault_contract, DeFindexFactoryTest, DeFindexFactoryClient};
use crate::constants::MAX_DEFINDEX_FEE;
extern crate std;

impl std::fmt::Debug for DeFindexFactoryClient<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DeFindexFactoryClient {{ ... }}")
    }
}


#[test]
fn get_storage() {
    let test = DeFindexFactoryTest::setup();

    let factory_admin = test.factory_contract.admin();
    let factory_defindex_receiver = test.factory_contract.defindex_receiver();

    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);
}

#[test]
#[should_panic(expected = "HostError: Error(Context, InvalidAction)")] //FactoryError::FeeTooHigh (#406)
fn initialize_excesive_fees(){
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let defindex_receiver = Address::generate(&env);

    let defindex_fee = MAX_DEFINDEX_FEE + 1;
    let defindex_wasm_hash = env
        .deployer()
        .upload_contract_wasm(defindex_vault_contract::WASM);

    let _factory_contract = create_defindex_factory(
        &env,
        &admin,
        &defindex_receiver,
        defindex_fee,
        &defindex_wasm_hash,
    ); //This should panic
}