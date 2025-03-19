use soroban_sdk::{testutils::Address as _, Address, Env, BytesN};
use crate::error::FactoryError;
use crate::storage::DataKey;
use crate::test::{create_defindex_factory, defindex_vault_contract, DeFindexFactoryTest};
use crate::constants::MAX_DEFINDEX_FEE;

fn retrieve_value(env: &Env, key: DataKey) -> Result<BytesN<32>, FactoryError> {
    env.storage().instance().get(&key).ok_or(FactoryError::NotInitialized)
}

#[test]
fn get_storage() {
    let test = DeFindexFactoryTest::setup();

    let factory_admin = test.factory_contract.admin();
    let factory_defindex_receiver = test.factory_contract.defindex_receiver();
    let key = DataKey::DeFindexWasmHash;
    let vault_wasm = test.env.as_contract(&test.factory_contract.address,|| retrieve_value(&test.env, key));

    assert_eq!(vault_wasm.unwrap(), test.defindex_wasm_hash);
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