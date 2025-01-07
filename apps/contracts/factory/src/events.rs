//! Definition of the Events used in the contract
use common::models::AssetStrategySet;
use soroban_sdk::{contracttype, symbol_short, Address, Env, Map, Vec};

// CREATE DEFINDEX VAULT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateDeFindexEvent {
    pub roles: Map<u32, Address>,
    pub vault_fee: u32,
    pub assets: Vec<AssetStrategySet>,
}

/// Publishes an `CreateDeFindexEvent` to the event stream.
pub(crate) fn emit_create_defindex_vault(
    e: &Env,
    roles: Map<u32, Address>,
    vault_fee: u32,
    assets: Vec<AssetStrategySet>,
) {
    let event = CreateDeFindexEvent {
        roles,
        vault_fee,
        assets,
    };

    e.events()
        .publish(("DeFindexFactory", symbol_short!("create")), event);
}

// NEW ADMIN EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewAdminEvent {
    pub new_admin: Address,
}

pub(crate) fn emit_new_admin(e: &Env, new_admin: Address) {
    let event = NewAdminEvent { new_admin };

    e.events()
        .publish(("DeFindexFactory", symbol_short!("nadmin")), event);
}

// NEW DEFINDEX RECEIVER EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewDeFindexReceiverEvent {
    pub new_defindex_receiver: Address,
}

pub(crate) fn emit_new_defindex_receiver(e: &Env, new_defindex_receiver: Address) {
    let event = NewDeFindexReceiverEvent {
        new_defindex_receiver,
    };

    e.events()
        .publish(("DeFindexFactory", symbol_short!("nreceiver")), event);
}

// NEW DEFINDEX FEE EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NewFeeRateEvent {
    pub new_defindex_fee: u32,
}

pub(crate) fn emit_new_defindex_fee(e: &Env, new_defindex_fee: u32) {
    let event = NewFeeRateEvent { new_defindex_fee };

    e.events()
        .publish(("DeFindexFactory", symbol_short!("n_fee")), event);
}
