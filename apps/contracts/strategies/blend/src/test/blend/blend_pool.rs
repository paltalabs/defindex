use soroban_sdk::Map;
extern crate std;
use super::utils::create_generic_strategy;
use crate::blend_pool;
#[test]
#[should_panic(expected = "#455")] // InvalidId
pub fn positions_to_supply_invalid_id(){
  let strategy = create_generic_strategy();
  let e = strategy.e;
  let positions: blend_pool::Positions = 
    blend_pool::Positions {
      collateral: Map::from_array(&e, [
        (0u32, 2509_7_147_290i128),
        (3u32, 2129_8_242_883i128),
      ]),
      liabilities: Map::from_array(&e, [
        (0u32, 251_9_345_885i128),
        (3u32, 499_9_244_017i128),
      ]),
      supply: Map::from_array(&e, [
        (0u32, 759_9_135_886i128),
        (3u32, 109_9_896_946i128),
      ]),
  };
  let reserve_id = 8;

  let result = e.as_contract(&strategy.address, || blend_pool::positions_to_supply(&e, positions, reserve_id));
  std::println!("{:?}", result);
}
#[test]
pub fn positions_to_supply_success(){
  let strategy = create_generic_strategy();
  let e = strategy.e;
  let expected_result = 759_9_135_886i128;
  let positions: blend_pool::Positions = 
    blend_pool::Positions {
      collateral: Map::from_array(&e, [
        (0u32, 2509_7_147_290i128),
        (3u32, 2129_8_242_883i128),
      ]),
      liabilities: Map::from_array(&e, [
        (0u32, 251_9_345_885i128),
        (3u32, 499_9_244_017i128),
      ]),
      supply: Map::from_array(&e, [
        (0u32, expected_result),
        (3u32, 109_9_896_946i128),
      ]),
  };
  let reserve_id = 0;

  let result = e.as_contract(&strategy.address, || blend_pool::positions_to_supply(&e, positions, reserve_id));
  assert_eq!(result, Ok(expected_result));
}