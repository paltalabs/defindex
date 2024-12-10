mod wasm {
    soroban_sdk::contractimport!(file = "./src/testutils/mock_sep_41_token.wasm");
}

pub use wasm::{
    Client as MockTokenClient, Contract as MockToken, TokenError, WASM as MockTokenWASM,
};
