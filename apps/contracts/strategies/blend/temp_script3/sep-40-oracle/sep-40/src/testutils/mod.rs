mod wasm {
    soroban_sdk::contractimport!(file = "./src/testutils/mock_sep_40_oracle.wasm");
}

pub use wasm::{
    Asset, Client as MockPriceOracleClient, Contract as MockPriceOracle, PriceOracleError,
    WASM as MockPriceOracleWASM,
};
