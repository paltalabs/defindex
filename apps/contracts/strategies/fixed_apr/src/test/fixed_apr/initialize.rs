// Cannot Initialize twice
extern crate std;
use crate::test::{create_fixapr_strategy, FixAprStrategyTest};

#[test]
fn check_storage() {
    let test = FixAprStrategyTest::setup();

    let strategy = create_fixapr_strategy(&test.env, &test.token.address, 1000u32, &test.token.address);

    // get asset should return underlying asset
    let underlying_asset = strategy.asset();
    assert_eq!(underlying_asset, test.token.address);
}