use soroban_sdk::{contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    SoroswapRouterAddress,
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

// Soroswap Router Address
pub fn set_soroswap_router_address(e: &Env, address: Address) {
    e.storage()
        .instance()
        .set(&DataKey::SoroswapRouterAddress, &address);
}

pub fn get_soroswap_router_address(e: &Env) -> Address {
    e.storage()
        .instance()
        .get(&DataKey::SoroswapRouterAddress)
        .unwrap()
}
