#![no_std]

use soroban_sdk::{contractclient, contractspecfn, Address, Env, Vec, String};
pub struct Spec;

mod error;
pub use error::AdapterError;

/// Interface for SoroswapAggregatorProxy
#[contractspecfn(name = "Spec", export = false)]
#[contractclient(name = "DeFindexAdapterClient")]

pub trait DeFindexAdapterTrait {
    fn initialize(e: Env, protocol_address: Address) -> Result<(), AdapterError>;

    fn deposit(
        env: Env,
        amount: i128,
        from: Address
    ) -> Result<(), AdapterError>;

    fn balance(
        e: Env,
        from: Address,
    ) -> Result<i128, AdapterError>;
    
    fn withdraw(
        e: Env,
        from: Address,
    ) -> Result<(), AdapterError>;
}
