# Implementing a Blend Strategy for DeFindex: A Step-by-Step Guide

DeFindex is designed for flexibility, enabling developers to create custom strategies that integrate seamlessly with its Vault architecture. In this guide, we’ll explore how to implement a Blend Strategy, which interacts with a Blend Pool to manage assets. The strategy includes key features such as deposits, withdrawals and claiming rewards.

For more details on the role of strategies in DeFindex, refer to the whitepaper’s strategy section.

## Overview of the Blend Strategy

The Blend Strategy handles:

- Initialization: Sets up the strategy with the Blend Pool address and the underlying asset.
- Deposits: Transfers user assets to the Blend Pool as collateral.
- Withdrawals: Retrieves user assets from the Blend Pool.
- Harvesting: Claims rewards from the Blend Pool for reinvestment or user distribution.
- Balance Tracking: Reports the current holdings of the strategy.

## Prerequisites

- Install the defindex-strategy-core crate

This library provides a standardized interface for DeFindex strategies.

```bash
cargo add defindex-strategy-core
```

- Familiarity with Soroban SDK

Soroban SDK is used to build and interact with Stellar smart contracts.

## Understanding Blend Pool Contracts

Use the Stellar CLI to fetch the Blend Pool WASM:

```bash
stellar contract fetch --network mainnet --id <CONTRACT_ID>
```

## Implementing the Core Strategy

Start by defining the BlendStrategy structure and implementing the DeFindexStrategyTrait in lib.rs.

```rust
use defindex_strategy_core::{DeFindexStrategyTrait, StrategyError};
use soroban_sdk::{contractimpl, Address, Env, Val, Vec};

pub struct BlendStrategy;
  
#[contractimpl]
impl DeFindexStrategyTrait for BlendStrategy {

    fn initialize(env: Env, asset: Address, init_args: Vec<Val>) -> Result<(), StrategyError> {
        // Initialization logic here_
        Ok(())
    }

    fn asset(env: Env) -> Result<Address, StrategyError> {
        // Return the asset managed by this strategy_
        Ok(Address::random(&env))
    }

    fn deposit(env: Env, amount: i128, from: Address) -> Result<(), StrategyError> {
        // Logic to deposit assets into the strategy_
        Ok(())
    }

    fn harvest(env: Env, from: Address) -> Result<(), StrategyError> {
        // Reinvest rewards to optimize yields_
        Ok(())
    }
  
    fn balance(env: Env, from: Address) -> Result<i128, StrategyError> {
        // Return the current balance_
        Ok(1000)
    }
  
    fn withdraw(env: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        // Withdraw assets from the strategy_
        Ok(amount)
    }
}
```

This is the basic skeleton. Let’s move on to integrate Blend Pool functionality.

## Integrating Blend Pool

Create blend_pool.rs to manage interactions with the Blend Pool smart contract.

```rust
use soroban_sdk::{vec, Address, Env, Vec};
  
use crate::storage::{get_blend_pool, get_underlying_asset};

// Import the Blend Pool WASM_

soroban_sdk::contractimport!(
    file = "../external_wasms/blend/blend_pool.wasm"
);  
  
pub type BlendPoolClient<'a> = Client<'a>;
  
pub enum RequestType {
    SupplyCollateral = 2,
    WithdrawCollateral = 3,
}

  
pub fn submit(e: &Env, from: &Address, amount: i128, request_type: RequestType) {
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);
    let underlying_asset = get_underlying_asset(e);
  
    let requests = vec![&e, Request {
        address: underlying_asset,
        amount,
        request_type: request_type as u32,
    }];
  
    blend_pool_client.submit(from, from, from, &requests);
}

pub fn claim(e: &Env, from: &Address) {
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);
    
    blend_pool_client.claim(from, &vec![&e, 3u32], from);
}

pub fn get_positions(e: &Env, from: &Address) -> Positions {
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    blend_pool_client.get_positions(from)
}
```

## Managing Strategy Storage

In storage.rs, maintain initialization flags and asset addresses for the strategy.

```rust
use soroban_sdk::{contracttype, Address, Env};
  
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Initialized,
    UnderlyingAsset,
    BlendPool,
}

pub fn set_initialized(e: &Env) {
    e.storage().instance().set(&DataKey::Initialized, &true);
}


pub fn is_initialized(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Initialized)
}


pub fn set_underlying_asset(e: &Env, address: &Address) {
    e.storage().instance().set(&DataKey::UnderlyingAsset, &address);
}

  
pub fn get_underlying_asset(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::UnderlyingAsset).unwrap()
}
  

pub fn set_blend_pool(e: &Env, address: Address) {
    e.storage().instance().set(&DataKey::BlendPool, &address);
}


pub fn get_blend_pool(e: &Env) -> Address {
    e.storage().instance().get(&DataKey::BlendPool).unwrap()
}
```

## Updating Core Methods

Finally, update lib.rs to call the blend_pool methods for deposits, withdrawals, and harvesting.

```rust
fn initialize(
    e: Env,
    asset: Address,
    init_args: Vec<Val>,
) -> Result<(), StrategyError> {
    if storage::is_initialized(&e) {
        return Err(StrategyError::AlreadyInitialized);
    }

    // We get the pool address from init_args that we are passing when initializing the contract
    let blend_pool_address = init_args
        .get(0)
        .ok_or(StrategyError::InvalidArgument)?
        .into_val(&e);

    storage::set_initialized(&e);
    // Set the blend pool address in the storage
    storage::set_blend_pool(&e, blend_pool_address);
    // Sets the underlying asset in the strategy, refer to the Whitepaper if confused
    storage::set_underlying_asset(&e, &asset);

    // Events come from the strategy core package, so we can have an standarized events for all the defindex strategies
    event::emit_initialize(&e, String::from_str(&e, STRATEGY_NAME), asset);
    storage::extend_instance_ttl(&e);
    Ok(())
}

fn deposit(e: Env, amount: i128, from: Address) -> Result<(), StrategyError> {
    storage::check_initialized(&e)?;
    
    blend_pool::submit(&e, &from, amount, RequestType::SupplyCollateral);
    Ok(())
}


fn withdraw(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
    storage::check_initialized(&e)?;
    blend_pool::submit(&e, &from, amount, RequestType::WithdrawCollateral);
    Ok(amount)
}


fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
    storage::check_initialized(&e)?;
    blend_pool::claim(&e, &from);
    Ok(())
}
```

## Conclusion

By following this guide, you can create a Blend Strategy for DeFindex that integrates smoothly with Vaults and Blend Pools. The extensible architecture ensures that your strategy remains secure, modular, and aligned with DeFindex’s design principles. Happy coding! 🎉