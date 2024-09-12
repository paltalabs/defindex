#![no_std]

use soroban_sdk::{contractclient, contractspecfn, Address, Env, Val, Vec};
pub struct Spec;

mod error;
pub use error::StrategyError;

#[contractspecfn(name = "Spec", export = false)]
#[contractclient(name = "DeFindexStrategyClient")]

pub trait DeFindexStrategyTrait {
    /// Initializes the strategy with the required parameters.
    fn initialize(
        env: Env,
        asset: Address,
        init_args: Vec<Val>,
    ) -> Result<(), StrategyError>;

    /// Returns the underlying asset address of the strategy.
    fn asset(env: Env) -> Result<Address, StrategyError>;

    /// Allows the DeFindex to deposit assets into the strategy.
    fn deposit(
        env: Env,
        amount: i128,
        from: Address
    ) -> Result<(), StrategyError>;

    /// Generates yields for the strategy, performing any required actions.
    fn harvest(env: Env) -> Result<(), StrategyError>;

    /// Returns the balance of the strategy for the given address.
    fn balance(
        env: Env,
        from: Address,
    ) -> Result<i128, StrategyError>;
    
    /// Allows the DeFindex to withdraw assets from the strategy.
    fn withdraw(
        env: Env,
        amount: i128, // Specify the amount to withdraw.
        from: Address,
    ) -> Result<i128, StrategyError>;
}