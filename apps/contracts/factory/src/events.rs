//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Address, Env, Vec};
use crate::defindex::StrategyParams;

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub admin: Address,
    pub defindex_receiver: Address,
}

pub(crate) fn emit_initialized(e: &Env, admin: Address, defindex_receiver: Address) {
    let event: InitializedEvent = InitializedEvent {
        admin,
        defindex_receiver,
    };
    e.events()
        .publish(("DeFindexFactory", symbol_short!("init")), event);
}

// CREATE DEFINDEX VAULT EVENT
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreateDeFindexEvent {
    pub emergency_manager: Address, 
    pub fee_receiver: Address, 
    pub manager: Address,
    pub tokens: Vec<Address>,
    pub ratios: Vec<u32>,
    pub strategies: Vec<StrategyParams>,
}

/// Publishes an `CreateDeFindexEvent` to the event stream.
pub(crate) fn emit_create_defindex_vault(
    e: &Env, 
    emergency_manager: Address, 
    fee_receiver: Address, 
    manager: Address,
    tokens: Vec<Address>,
    ratios: Vec<u32>,
    strategies: Vec<StrategyParams>,
) {
    let event = CreateDeFindexEvent { 
      emergency_manager,
      fee_receiver,
      manager,
      tokens,
      ratios,
      strategies,
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
    let event = NewDeFindexReceiverEvent { new_defindex_receiver };

    e.events()
        .publish(("DeFindexFactory", symbol_short!("nreceiver")), event);
}