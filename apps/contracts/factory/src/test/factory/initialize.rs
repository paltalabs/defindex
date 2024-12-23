use crate::test::DeFindexFactoryTest;

#[test]
fn get_storage() {
    let test = DeFindexFactoryTest::setup();

    let factory_admin = test.factory_contract.admin();
    let factory_defindex_receiver = test.factory_contract.defindex_receiver();

    assert_eq!(factory_admin, test.admin);
    assert_eq!(factory_defindex_receiver, test.defindex_receiver);
}
