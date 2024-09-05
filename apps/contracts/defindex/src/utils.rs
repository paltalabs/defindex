use soroban_sdk::Env;
pub const DAY_IN_LEDGERS: u32 = 17280;

pub fn bump_instance(e: &Env) {
  let max_ttl = e.storage().max_ttl();
  e.storage()
      .instance()
      .extend_ttl(max_ttl - DAY_IN_LEDGERS, max_ttl);
}