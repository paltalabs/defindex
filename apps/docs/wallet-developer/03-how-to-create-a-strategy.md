---
cover: ../.gitbook/assets/image 31.png
coverY: 0
---

# Building the Blend Strategy

## **Introduction**

Welcome to this guide on implementing the **Blend Strategy** for DeFindex. This tutorial is designed to provide a comprehensive walkthrough of the Blend Strategy smart contract, which integrates with the **Blend Protocol**, a lending and borrowing platform.

Inspired by the “fee-vault” contract developed by Script3, the Blend Strategy handles deposits, withdrawals, rewards harvesting, and balance tracking, making it a vital part of DeFindex’s modular architecture.

***

## **Why a Strategy?**

A strategy in DeFindex acts as a **proxy** between the Vault and an external protocol. This design is essential because:

1. **Protocol-specific Authorization**: The Vault cannot directly authorize interactions with external protocols like Blend.
2. **Position Management**: The Strategy holds positions for each interacting vault and tracks them using shares.
3. **Standardized Outputs**: The Strategy always converts internal shares to **underlying asset balances** for the Vault to ensure consistency.

***

### **Getting Started**

To implement the Blend Strategy, you need to be familiar with **Soroban smart contract development** and **Rust**. If you're new to Soroban, start with the official [Soroban Getting Started Guide](https://developers.stellar.org/docs/build/smart-contracts/getting-started/setup).

#### **Cargo.toml**

Here’s the `Cargo.toml` for the Blend Strategy:

```toml
[package]
name = "blend_strategy"
version = "0.1.0"
authors = ["coderipper <joaquin@paltalabs.io>"]
license = "GPL-3.0"
edition = "2021"
publish = false
repository = "https://github.com/paltalabs/defindex"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "22.0.0-rc.2.1"
defindex-strategy-core = "0.2.0"
soroban-fixed-point-math = "1.3.0"
  
[dev-dependencies]
soroban-sdk = { workspace = true, features = ["testutils"] }
sep-40-oracle = { version = "1.2.0", features = ["testutils"] }
sep-41-token = { version = "1.2.0", features = ["testutils"] }  
```

***

### **Project Setup**

Below, we’ll break the Blend Strategy into its components, explaining each part with the corresponding code.

### **1. lib.rs: Core Logic**

The **Blend Strategy** implements the DeFindexStrategyTrait and provides all core functionality, including deposits, withdrawals, harvesting, and balance tracking.

**Code:**

```rust
#![no_std]
use blend_pool::perform_reinvest;
use constants::{MIN_DUST, SCALAR_9};
use reserves::StrategyReserves;
use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env, IntoVal, String, Val, Vec};

mod blend_pool;
mod constants;
mod reserves;
mod soroswap;
mod storage;

use storage::{extend_instance_ttl, has_config, Config};

pub use defindex_strategy_core::{
	DeFindexStrategyTrait,
	StrategyError,
	event
};  

pub fn check_nonnegative_amount(amount: i128) -> Result<(), StrategyError> {
	if amount < 0 {
		Err(StrategyError::NegativeNotAllowed)
	} else {
		Ok(())
	}
}

fn check_initialized(e: &Env) -> Result<(), StrategyError> {
	if has_config(e) {
		Ok(())
	} else {
		Err(StrategyError::NotInitialized)
	}
}

const STARETEGY_NAME: &str = "BlendStrategy";

#[contract]
struct BlendStrategy;

#[contractimpl]
impl DeFindexStrategyTrait for BlendStrategy {
	fn __constructor(
		e: Env,
		asset: Address,
		init_args: Vec<Val>,
		) {
		// Getting init args from the Vec<Val>
		let blend_pool_address: Address = init_args.get(0).ok_or(StrategyError::InvalidArgument).unwrap().into_val(&e);
		let reserve_id: u32 = init_args.get(1).ok_or(StrategyError::InvalidArgument).unwrap().into_val(&e);
		let blend_token: Address = init_args.get(2).ok_or(StrategyError::InvalidArgument).unwrap().into_val(&e);
		let soroswap_router: Address = init_args.get(3).ok_or(StrategyError::InvalidArgument).unwrap().into_val(&e);

		let config = Config {
			asset: asset.clone(),
			pool: blend_pool_address,
			reserve_id,
			blend_token,
			router: soroswap_router,
		};
		
		// Storing the configuration in Config		
		storage::set_config(&e, config);
	}

	// It returns the underlying asset
	fn asset(e: Env) -> Result<Address, StrategyError> {
		check_initialized(&e)?;
		extend_instance_ttl(&e);
		
		Ok(storage::get_config(&e).asset)
	}

	fn deposit(
		e: Env,
		amount: i128,
		from: Address,
	) -> Result<i128, StrategyError> {
		check_initialized(&e)?;
		check_nonnegative_amount(amount)?;
		extend_instance_ttl(&e);
		from.require_auth();
		
		if amount < MIN_DUST {
			return Err(StrategyError::InvalidArgument); 
		}
		
		let config = storage::get_config(&e);
		// It claims any available BLND tokens and if its greater than the threshold it swaps them to the underlying asset and reinvest into the pool
		blend_pool::claim(&e, &e.current_contract_address(), &config);
		perform_reinvest(&e, &config)?;		
		  
		let reserves = storage::get_strategy_reserves(&e);				  
		
		// transfer tokens from the vault to the strategy contract
		TokenClient::new(&e, &config.asset).transfer(&from, &e.current_contract_address(), &amount);
		
		let b_tokens_minted = blend_pool::supply(&e, &from, &amount, &config);
		  
		// Keeping track of the total deposited amount and the total bTokens owned by the strategy depositors
		let vault_shares = reserves::deposit(&e, reserves.clone(), &from, amount, b_tokens_minted);
		
		// Getting the underlying asset balance from the shares holded by the "from" address
		let underlying_balance = shares_to_underlying(vault_shares, reserves);
		
		event::emit_deposit(&e, String::from_str(&e, STARETEGY_NAME), amount, from);
		// It is required by the vault that the strategy returns the balance of the "from" address to keep track of the status and health of the strategy
		Ok(underlying_balance)
	}

  

	fn harvest(e: Env, from: Address) -> Result<(), StrategyError> {
		check_initialized(&e)?;
		extend_instance_ttl(&e);
		
		let config = storage::get_config(&e);
		
		// Claims BLND tokens
		let harvested_blend = blend_pool::claim(&e, &e.current_contract_address(), &config);
		// If the threshold is greater than X it will swap and reinvest the claimed BLND tokens
		perform_reinvest(&e, &config)?;
	  
		event::emit_harvest(&e, String::from_str(&e, STARETEGY_NAME), harvested_blend, from);
	
		Ok(())
	}

  

	fn withdraw(
		e: Env,
		amount: i128,
		from: Address,
		to: Address,
	) -> Result<i128, StrategyError> {
		check_initialized(&e)?;
		check_nonnegative_amount(amount)?;
		extend_instance_ttl(&e);
		from.require_auth();
		
		// protect against rouding of reserve_vault::update_rate, as small amounts
		// can cause incorrect b_rate calculations due to the pool rounding
		if amount < MIN_DUST {
			return Err(StrategyError::InvalidArgument)
		}
		
		let reserves = storage::get_strategy_reserves(&e);  
		
		let config = storage::get_config(&e);
		
		// It withdraws the underlying asset from the blend pool
		let (tokens_withdrawn, b_tokens_burnt) = blend_pool::withdraw(&e, &to, &amount, &config);

		// It updates the vault shares and withdrawed amounts
		let vault_shares = reserves::withdraw(&e, reserves.clone(), &from, tokens_withdrawn, b_tokens_burnt);

		// Getting the underlying asset balance from the shares holded by the "from" address
		let underlying_balance = shares_to_underlying(vault_shares, reserves);
		  
		event::emit_withdraw(&e, String::from_str(&e, STARETEGY_NAME), amount, from);
		
		Ok(underlying_balance)
	}

  

	fn balance(
		e: Env,
		from: Address,
	) -> Result<i128, StrategyError> {
		check_initialized(&e)?;
		extend_instance_ttl(&e);
		
		// Get the vault's shares
		let vault_shares = storage::get_vault_shares(&e, &from);
		
		// Get the strategy's total shares and bTokens
		let reserves = storage::get_strategy_reserves(&e);
		let underlying_balance = shares_to_underlying(vault_shares, reserves);
		
		Ok(underlying_balance)
	}
		
}


fn shares_to_underlying(shares: i128, reserves: StrategyReserves) -> i128 {
	let total_shares = reserves.total_shares;
	let total_b_tokens = reserves.total_b_tokens;
	
	if total_shares == 0 || total_b_tokens == 0 {	
		// No shares or bTokens in the strategy
		return 0i128;
	}

	// Calculate the bTokens corresponding to the vault's shares
	let vault_b_tokens = (shares * total_b_tokens) / total_shares;
	
	// Use the b_rate to convert bTokens to underlying assets
	(vault_b_tokens * reserves.b_rate) / SCALAR_9
}
```

***

### **2. Storage Module**

The `storage.rs` file is fundamental to the Blend Strategy as it handles the configuration, reserves, and vault position data. This module is the first introduced into the contract, as it’s initialized by the constructor to store the strategy’s configuration. Let’s break down its purpose and implementation.

#### **Purpose**

1. **Configuration Management**:
   * Stores essential information like the underlying asset, Blend Pool address, and reserve ID.
   * Used to retrieve the configuration during operations like deposits and withdrawals.
2. **Vault Position Tracking**:
   * Tracks the number of shares each vault or user owns.
   * Shares represent a user’s proportionate stake in the strategy’s reserves.
3. **Reserves Management**:
   * Maintains the total shares, bTokens, and bRate (exchange rate) for the strategy.

**Code Walkthrough**

Here’s the complete `storage.rs` file with detailed explanations:

```rust
use soroban_sdk::{contracttype, Address, Env};
use crate::reserves::StrategyReserves;

#[contracttype]
pub struct Config {
    pub asset: Address,         // The underlying asset managed by the strategy
    pub pool: Address,          // Blend Pool address
    pub reserve_id: u32,        // Reserve ID for the Blend Pool
    pub blend_token: Address,   // Blend token address for rewards
    pub router: Address,        // Soroswap Router address for swaps

}


#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Config,              // Key for storing the strategy configuration
    Reserves,            // Key for storing strategy reserves
    VaultPos(Address),   // Key for storing vault positions (per user or vault)

}

  

pub const DAY_IN_LEDGERS: u32 = 17280; // Number of ledgers in a day
pub const INSTANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS; // TTL extension for 30 days
pub const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

const LEDGER_BUMP: u32 = 120 * DAY_IN_LEDGERS; // TTL bump for persistent storage
const LEDGER_THRESHOLD: u32 = LEDGER_BUMP - 20 * DAY_IN_LEDGERS;

pub fn extend_instance_ttl(e: &Env) {
    e.storage()
        .instance()
        .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

// Config Management
pub fn set_config(e: &Env, config: Config) {
    e.storage().instance().set(&DataKey::Config, &config);
}

pub fn get_config(e: &Env) -> Config {
    e.storage().instance().get(&DataKey::Config).unwrap()
}

pub fn has_config(e: &Env) -> bool {
    e.storage().instance().has(&DataKey::Config)
}

// Vault Position Management
/// Set the number of shares a user or vault owns.
pub fn set_vault_shares(e: &Env, address: &Address, shares: i128) {
    let key = DataKey::VaultPos(address.clone());
    e.storage().persistent().set::<DataKey, i128>(&key, &shares);
    e.storage()
        .persistent()
        .extend_ttl(&key, LEDGER_THRESHOLD, LEDGER_BUMP);
}

/// Get the number of shares a user or vault owns.
pub fn get_vault_shares(e: &Env, address: &Address) -> i128 {
    let result = e.storage().persistent().get::<DataKey, i128>(&DataKey::VaultPos(address.clone()));
    match result {
        Some(shares) => {
            e.storage()
                .persistent()
                .extend_ttl(&DataKey::VaultPos(address.clone()), LEDGER_THRESHOLD, LEDGER_BUMP);
            shares
        }
        None => 0,
    }
}

// Reserves Management
pub fn set_strategy_reserves(e: &Env, new_reserves: StrategyReserves) {
    e.storage().instance().set(&DataKey::Reserves, &new_reserves);
}
  
pub fn get_strategy_reserves(e: &Env) -> StrategyReserves {
    e.storage().instance().get(&DataKey::Reserves).unwrap_or(
        StrategyReserves {
            total_shares: 0,
            total_b_tokens: 0,
            b_rate: 0,
        }
    )
}
```

**Key Points**

1. **Configuration**:
   * The Config struct holds all necessary parameters for the strategy.
   * The constructor uses set\_config to initialize these values.
2. **Vault Positions**:
   * Shares are stored with the VaultPos key and are specific to each vault or user.
   * Precision is managed with 7 decimal places to ensure accuracy.
3. **Reserves**:
   * Reserves track the strategy’s overall state, including total shares, bTokens, and the current exchange rate (bRate).
   * If reserves are missing, default values are used.

***

### **3. Blend Pool Interactions**

The `blend_pool.rs` file is responsible for managing all interactions with the Blend Pool smart contract. This includes supplying and withdrawing assets, claiming rewards, and reinvesting harvested tokens. Each function is tightly integrated with the strategy's storage and configuration to ensure smooth operation.

***

#### **Purpose**

1. **Supply and Withdraw Assets**:
   * Handles depositing and withdrawing the underlying asset to/from the Blend Pool.
   * Tracks `bTokens` received or burned during these operations.
2. **Claim Rewards**:
   * Retrieves rewards (e.g., BLND tokens) accrued in the Blend Pool.
3. **Reinvest Rewards**:
   * Converts rewards into the underlying asset and reinvests them into the Blend Pool.

***

#### **Code Walkthrough**

```rust
use defindex_strategy_core::StrategyError;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    panic_with_error, token::TokenClient, vec, Address, Env, IntoVal, Symbol, Vec,
};

use crate::{
    constants::REWARD_THRESHOLD,
    reserves,
    soroswap::internal_swap_exact_tokens_for_tokens,
    storage::{self, Config},
};

// Importing the Contract WASM file from Blend Pool
soroban_sdk::contractimport!(
    file = "../external_wasms/blend/blend_pool.wasm"
);
pub type BlendPoolClient<'a> = Client<'a>;

// Define the RequestType enum with explicit u32 values
#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum RequestType {
    Supply = 0,
    Withdraw = 1,
}

impl RequestType {
    fn to_u32(self) -> u32 {
        self as u32
    }
}

// Deposits the underlying asset into the Blend Pool and returns the number of bTokens minted.
pub fn supply(e: &Env, from: &Address, amount: &i128, config: &Config) -> i128 {
    let pool_client = BlendPoolClient::new(e, &config.pool);

    // Get deposit amount pre-supply used to then calculate the bTokens received
    let pre_supply = pool_client
        .get_positions(&e.current_contract_address())
        .supply
        .get(config.reserve_id)
        .unwrap_or(0);

	//  Creating the request for the Blend Pool (this can be checked in Blend Documentation)
    let requests: Vec<Request> = vec![&e, Request {
        address: config.asset.clone(),
        amount: amount.clone(),
        request_type: RequestType::Supply.to_u32(),
    }];

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: config.asset.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    config.pool.clone(),
                    amount.clone()).into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);
  
    let new_positions = pool_client.submit(
        &e.current_contract_address(),
        &e.current_contract_address(),
        &from,
        &requests
    );
  
    // Calculate the amount of bTokens received
    let b_tokens_amount = new_positions.supply.get_unchecked(config.reserve_id) - pre_supply;

    b_tokens_amount
}

// Withdraws the underlying asset from the Blend Pool and calculates the actual amount received.
pub fn withdraw(e: &Env, to: &Address, amount: &i128, config: &Config) -> (i128, i128) {
    let pool_client = BlendPoolClient::new(e, &config.pool);
    
    // Get withdraw amount pre-withdraw used to then calculate the bTokens burned
    let pre_withdraw_btokens = pool_client
        .get_positions(&e.current_contract_address())
        .supply
        .get(config.reserve_id)
        .unwrap_or_else(|| panic_with_error!(e, StrategyError::InsufficientBalance));

    // Get balance pre-withdraw, as the pool can modify the withdrawal amount
    let pre_withdrawal_balance = TokenClient::new(&e, &config.asset).balance(&to);

    let requests: Vec<Request> = vec![&e, Request {
        address: config.asset.clone(),
        amount: amount.clone(),
        request_type: RequestType::Withdraw.to_u32(),
    }];

    let new_positions = pool_client.submit(
        &e.current_contract_address(),
        &e.current_contract_address(),
        &to,
        &requests
    );

    // Calculate the amount of tokens withdrawn and bTokens burnt
    let post_withdrawal_balance = TokenClient::new(&e, &config.asset).balance(&to);

    let real_amount = post_withdrawal_balance - pre_withdrawal_balance;
  
	// Calculates the amount of bToken burned
    let b_tokens_amount = pre_withdraw_btokens - new_positions.supply.get(config.reserve_id).unwrap_or(0);

    (real_amount, b_tokens_amount)

}

// Claims rewards (e.g., BLND tokens) from the Blend Pool.
pub fn claim(e: &Env, from: &Address, config: &Config) -> i128 {
    let pool_client = BlendPoolClient::new(e, &config.pool);
    pool_client.claim(from, &vec![&e, 0u32, 1u32, 2u32, 3u32], from)
}

// Converts rewards into the underlying asset and reinvests them into the Blend Pool.
pub fn perform_reinvest(e: &Env, config: &Config) -> Result<bool, StrategyError> {
	// Getting the BLND Token balance to check if it needs to reinvest
    let blnd_balance = TokenClient::new(e, &config.blend_token).balance(&e.current_contract_address());
  
    // If balance does not exceed threshold, skip reinvest
    if blnd_balance < REWARD_THRESHOLD {
        return Ok(false);
    }

    // Swap BLND to the underlying asset
    let mut swap_path: Vec<Address> = vec![&e];
    swap_path.push_back(config.blend_token.clone());
    swap_path.push_back(config.asset.clone());

    let deadline = e.ledger().timestamp() + 600;

	// Swaps the BLND token into the underlying asset eg. USDC
    let swapped_amounts = internal_swap_exact_tokens_for_tokens(
        e,
        &blnd_balance,
        &0i128,
        swap_path,
        &e.current_contract_address(),
        &deadline,
        config,
    )?;

    let amount_out: i128 = swapped_amounts
        .get(1)
        .ok_or(StrategyError::InvalidArgument)?
        .into_val(e);
  
    // Supplying underlying asset into blend pool
    let b_tokens_minted = supply(&e, &e.current_contract_address(), &amount_out, &config);

    let reserves = storage::get_strategy_reserves(&e);
    reserves::harvest(&e, reserves, amount_out, b_tokens_minted);

    Ok(true)
}
```

**Key Points**

1. **Supply and Withdraw**:
   * Use RequestType to define the operation.
   * Ensure accurate tracking of bTokens for precise position management.
2. **Claim**:
   * Hardcoded reserve token IDs are used as placeholders for now
3. **Reinvest**:
   * Converts rewards to maximize returns.
   * Leverages Soroswap to swap BLND for the underlying asset.

***

### **5. Token Swapping with Soroswap**

This module handles token swaps, converting rewards (e.g., BLND tokens) into the underlying asset during the **harvest** process to reinvest them into the Blend Pool.

***

#### **Code Walkthrough**

```rust
use defindex_strategy_core::StrategyError;
use soroban_sdk::{
    auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation},
    vec, Address, Env, IntoVal, Symbol, Val, Vec,
};

use crate::storage::Config;

// Handles swaps using the Soroswap Router
pub fn internal_swap_exact_tokens_for_tokens(
    e: &Env,
    amount_in: &i128,
    amount_out_min: &i128,
    path: Vec<Address>,
    to: &Address,
    deadline: &u64,
    config: &Config,
) -> Result<Vec<i128>, StrategyError> {
    
    let mut swap_args: Vec<Val> = vec![&e];
    swap_args.push_back(amount_in.into_val(e));
    swap_args.push_back(amount_out_min.into_val(e));
    swap_args.push_back(path.into_val(e));
    swap_args.push_back(to.to_val());
    swap_args.push_back(deadline.into_val(e));

    let pair_address: Address = e.invoke_contract(
        &config.router,
        &Symbol::new(&e, "router_pair_for"),
        vec![&e, path.get(0).unwrap().into_val(e), path.get(1).unwrap().into_val(e)],
    );

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: path.get(0).unwrap().clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    pair_address,
                    amount_in.clone(),
                ).into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);
  
    e.invoke_contract(
        &config.router,
        &Symbol::new(&e, "swap_exact_tokens_for_tokens"),
        swap_args,
    )
}
```

**Key Points**

* Swaps BLND tokens into the underlying asset during harvest.
* Uses the Soroswap Router contract.

**References**

* [DeFindex GitHub Repository](https://github.com/paltalabs/defindex/)
* [Script3 “Fee Vault” Contract](https://github.com/script3/fee-vault/)
* [DeFindex Whitepaper - Strategy Section](https://docs.defindex.io/whitepaper/10-whitepaper/03-the-defindex-approach/02-contracts/02-strategy-contract)

The Blend Strategy for DeFindex showcases the power of modular architecture in decentralized finance. By acting as a proxy between the Vault and external protocols, the strategy ensures seamless integration while maintaining flexibility and security.

This guide provides a complete walkthrough for implementing the Blend Strategy, covering: • Initialization and storage management. • Interactions with the Blend Pool. • Reinvestment logic using Soroswap.

With this foundation, you can build custom strategies tailored to specific protocols and assets, expanding DeFindex’s utility. Remember to follow best practices, rigorously test your strategies, and monitor deployments to ensure optimal performance.

If you have questions or need help, join the conversation on our [DeFindex Discord](https://discord.gg/CUC26qUTw7) or connect with us on the [PaltaLabs Discord](https://discord.com/invite/4F4pWFkkyZ). We’re here to help you build and innovate. Happy coding! 🚀
