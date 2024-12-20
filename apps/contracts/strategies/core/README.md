# DeFindex Strategy Core

The defindex-strategy-core package is a foundational library designed to facilitate the development of strategies for DeFindex. It provides reusable abstractions and utilities that streamline the creation, management, and integration of strategies into the DeFindex ecosystem.

### Features

- **Reusable Events**: Predefined events to log actions such as deposits, withdrawals, and harvests.
- **Custom Errors**: A unified error handling system to ensure consistent and informative feedback across strategies.
- **Core Abstractions**: Base traits and utilities to define and implement strategies with minimal boilerplate.

### Structure

This package includes the following modules:
1. **Error**: Provides custom error types to handle various edge cases and ensure smooth execution.
2. **Event**: Includes predefined events for logging and monitoring strategy activity.
3. **Core Traits**: Defines the DeFindexStrategyTrait, which serves as the contract for developing new strategies.

### Installation

Add the defindex-strategy-core package to your Cargo.toml dependencies:

```toml
[dependencies]
defindex-strategy-core = "0.2.0"
```

### Usage

Here is a simple example of how to use this package to build a custom strategy:

1. Import the Core Library
```rust
use defindex_strategy_core::{DeFindexStrategyTrait, StrategyError, event};
```

2. Implement the Strategy Trait

Define your custom strategy by implementing the DeFindexStrategyTrait:
```rust
#[contract]
struct MyCustomStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for MyCustomStrategy {
    fn __constructor(e: Env, asset: Address, init_args: Vec<Val>) {
        // Initialization logic
    }

    fn deposit(e: Env, amount: i128, from: Address) -> Result<i128, StrategyError> {
        // Deposit logic
        Ok(0) // it must return the vault balance, same response as balance() 
    }

    fn withdraw(e: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError> {
        // Withdrawal logic
        Ok(amount) // it must return the vault balance, same response as balance() 
    }

    fn balance(e: Env, from: Address) -> Result<i128, StrategyError> {
        // Balance check logic
        Ok(0)
    }

    fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
        // Harvest logic
        Ok(())
    }
}
```

3. Emit Events

Use the event module to log actions:
```rust
event::emit_deposit(&e, String::from("MyCustomStrategy"), amount, from.clone());
```