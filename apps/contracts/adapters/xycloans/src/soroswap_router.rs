soroban_sdk::contractimport!(
  file = "../soroswap/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;