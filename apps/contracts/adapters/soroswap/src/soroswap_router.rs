soroban_sdk::contractimport!(
  file = "../external_wasms/soroswap/soroswap_router.optimized.wasm"
);
pub type SoroswapRouterClient<'a> = Client<'a>;