make build
TEST_CONTRACT=$(soroban contract deploy --source admin --network testnet --wasm ../../target/wasm32-unknown-unknown/release/xycloans_adapter.optimized.wasm)
soroban contract invoke --id "$TEST_CONTRACT" --source admin --network testnet -- initialize