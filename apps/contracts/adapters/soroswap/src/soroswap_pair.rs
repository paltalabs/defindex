soroban_sdk::contractimport!(
  file = "./soroswap_pair.optimized.wasm"
);
pub type SoroswapPairClient<'a> = Client<'a>;