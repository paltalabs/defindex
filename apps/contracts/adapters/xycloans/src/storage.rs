use soroban_sdk::{contracttype, Env, Address};

#[derive(Clone)]
#[contracttype]

enum DataKey {
    Initialized,
    SoroswapRouterAddress,
    SoroswapFactoryAddress,
    XycloansPoolAddress,
    Token0,
    Token1
}

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}

pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}

// Soroswap Router Address
pub fn set_soroswap_router_address(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::SoroswapRouterAddress, &address);
}

pub fn get_soroswap_router_address(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::SoroswapRouterAddress).unwrap()
}

// Soroswap Factory Address
pub fn set_soroswap_factory_address(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::SoroswapFactoryAddress, &address);
}

pub fn get_soroswap_factory_address(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::SoroswapFactoryAddress).unwrap()
}

// Xycloans Pool Address
pub fn set_xycloans_pool_address(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::XycloansPoolAddress, &address);
}

pub fn get_xycloans_pool_address(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::XycloansPoolAddress).unwrap()
}

// Tokens
pub fn set_pool_token(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Token0, &address);
}

pub fn get_pool_token(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Token0).unwrap()
}

pub fn set_token_in(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::Token1, &address);
}

pub fn get_token_in(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::Token1).unwrap()
}