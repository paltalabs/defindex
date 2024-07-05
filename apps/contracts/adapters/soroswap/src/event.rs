//! Definition of the Events used in the contract
use soroban_sdk::{contracttype, symbol_short, Env};

// INITIALIZED
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitializedEvent {
    pub state: bool,
}

pub(crate) fn initialized(e: &Env, state: bool) {
    
    let event: InitializedEvent = InitializedEvent {
        state: state,
    };
    e.events().publish(("DefiIndex", symbol_short!("init")), event);
}