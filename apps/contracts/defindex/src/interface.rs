use soroban_sdk::{Address, Env, Vec};

use crate::{storage::StrategyParams, ContractError};

pub trait VaultTrait {
  fn initialize(
      e: Env, 
      emergency_manager: Address, 
      fee_receiver: Address, 
      manager: Address,
      tokens: Vec<Address>,
      ratios: Vec<u32>,
      strategies: Vec<StrategyParams>
  ) -> Result<(), ContractError>;

  fn deposit(e: Env, amount: i128, from: Address) -> Result<(), ContractError>;

  fn withdraw(
      e: Env,
      from: Address,
  ) -> Result<(), ContractError>;

  fn emergency_withdraw(
      e: Env,
      from: Address,
  ) -> Result<(), ContractError>;

  fn get_strategies(e: Env) -> Vec<StrategyParams>;

  fn current_invested_funds(e: Env) -> i128;
}

pub trait AdminInterfaceTrait {
  fn set_fee_receiver(e: Env, caller: Address, fee_receiver: Address);

  fn get_fee_receiver(e: Env) -> Result<Address, ContractError>;

  fn set_manager(e: Env, manager: Address);

  fn get_manager(e: Env) -> Result<Address, ContractError>;

  fn set_emergency_manager(e: Env, emergency_manager: Address);

  fn get_emergency_manager(e: Env) -> Result<Address, ContractError>;
}