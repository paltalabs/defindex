# SEP-0040 Oracle
Exposes the interface of the SEP-0040 Price Feed Oracle alongside a test price oracle contract.

SEP-0040 Definition: https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0040.md

## Safety
This is **experimental software** and is provided on an "as is" and "as available" basis.

We do **not give any warranties** and **will not be liable for any loss** incurred through any use of this codebase.

## Getting Started

Add the package to your `Cargo.toml`:

```toml
[dependencies]
sep-40-oracle = "<desired version>"
```

You can optionally include the `testutils` feature in your `dev-dependencies` to deploy a mock version of the `sep-40-oracle` for testing:

```toml
[dev_dependencies]
sep-40-oracle = { version = "<desired version>", features = ["testutils"] }
```

### Client and Trait
This package exposes a client for interacting with SEP-0040 Oracles and a trait for contracts wishing to implement a SEP-0040 Oracle.

Client usage:
```rust
use sep_40_oracle::PriceFeedClient;

let address = // address of the oracle
let price_feed_client = PriceFeedClient::new(&env, &address);
```

Trait usage:
```rust
use sep_40_oracle::PriceFeedTrait;
use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct MyPriceFeed;

#[contractimpl]
impl PriceFeedTrait for MyPriceFeed {
    // impl the trait functions
}
```

### Mock PriceFeed Oracle
This package exposes an example Soroban price feed oracle implementation. This is useful for testing protocols that depend on a `sep-0040` price feed oracle, including the ability to manipulate price feeds during a test.

A WASM version of the contract can be deployed as follows:
```rust
use sep_40_oracle::testutils::{MockPriceOracleClient, MockPriceOracleWASM};
use soroban_sdk::{testutils::Address as _, Address, Env, symbol_short, vec};

let env = Env::default();


let admin = Address::generate(&env);
let xlm = Address::generate(&env);
let oracle_id = env.register_contract_wasm(None, MockTokenWASM);
let oracle_client = MockPriceOracleClient::new(&env, &oracle_id);
oracle_client.set_data(
    &admin,
    &Asset::Other(symbol_short!("TEAPOT")),
    &vec![&env, Asset::Stellar(xlm)],
    &7,
    &(5 * 60 * 60)
);
```