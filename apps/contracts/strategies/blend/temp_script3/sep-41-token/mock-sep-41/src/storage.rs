use soroban_sdk::{
    contracttype, symbol_short, unwrap::UnwrapOptimized, Address, Env, IntoVal, String, Symbol,
    TryFromVal, Val,
};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 31 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 120 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - 20 * DAY_IN_LEDGERS;

const METADATA_KEY: Symbol = symbol_short!("METADATA");
const ADMIN_KEY: Symbol = symbol_short!("ADMIN");

#[derive(Clone)]
#[contracttype]
pub struct TokenMetadata {
    pub decimal: u32,
    pub name: String,
    pub symbol: String,
}

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[contracttype]
pub struct AllowanceValue {
    pub amount: i128,
    pub expiration_ledger: u32,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Allowance(AllowanceDataKey),
    Balance(Address),
    Nonce(Address),
    State(Address),
}

/// Bump the instance lifetime by the defined amount
pub fn extend_instance(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

/// Fetch an entry in persistent storage that has a default value if it doesn't exist
fn get_persistent_default<K: IntoVal<Env, Val>, V: TryFromVal<Env, Val>>(
    e: &Env,
    key: &K,
    default: V,
    bump_threshold: u32,
    bump_amount: u32,
) -> V {
    if let Some(result) = e.storage().persistent().get::<K, V>(key) {
        e.storage()
            .persistent()
            .extend_ttl(key, bump_threshold, bump_amount);
        result
    } else {
        default
    }
}

//********** Instance **********//

// Admin

pub fn get_admin(e: &Env) -> Address {
    e.storage().instance().get(&ADMIN_KEY).unwrap_optimized()
}

pub fn has_admin(e: &Env) -> bool {
    e.storage().instance().has(&ADMIN_KEY)
}

pub fn set_admin(e: &Env, admin: &Address) {
    e.storage().instance().set(&ADMIN_KEY, &admin);
}

// Metadata

pub fn get_metadata(e: &Env) -> TokenMetadata {
    e.storage().instance().get(&METADATA_KEY).unwrap_optimized()
}

pub fn set_metadata(e: &Env, metadata: &TokenMetadata) {
    e.storage().instance().set(&METADATA_KEY, metadata);
}

//********** Persistent **********//

// Balance

pub fn get_balance(e: &Env, address: &Address) -> i128 {
    get_persistent_default(
        e,
        &DataKey::Balance(address.clone()),
        0_i128,
        BALANCE_LIFETIME_THRESHOLD,
        BALANCE_BUMP_AMOUNT,
    )
}

pub fn set_balance(e: &Env, address: &Address, balance: &i128) {
    e.storage()
        .persistent()
        .set(&DataKey::Balance(address.clone()), balance);
}

//********** Temporary **********//

// Allowance

pub fn get_allowance(e: &Env, from: &Address, spender: &Address) -> AllowanceValue {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });
    let temp = e.storage().temporary().get(&key);
    temp.unwrap_or_else(|| AllowanceValue {
        amount: 0,
        expiration_ledger: 0,
    })
}

pub fn set_allowance(
    e: &Env,
    from: &Address,
    spender: &Address,
    amount: i128,
    expiration_ledger: u32,
) {
    let key = DataKey::Allowance(AllowanceDataKey {
        from: from.clone(),
        spender: spender.clone(),
    });
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };
    e.storage().temporary().set(&key, &allowance);
    if amount > 0 {
        let ledgers_to_live = expiration_ledger
            .checked_sub(e.ledger().sequence())
            .unwrap_optimized();
        e.storage()
            .temporary()
            .extend_ttl(&key, ledgers_to_live, ledgers_to_live);
    }
}
