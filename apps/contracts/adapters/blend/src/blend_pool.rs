soroban_sdk::contractimport!(
  file = "../external_wasms/blend/blend_pool.wasm"
);
pub type BlendPoolClient<'a> = Client<'a>;