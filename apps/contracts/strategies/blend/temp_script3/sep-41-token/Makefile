default: build

test: build
	cargo test --all --tests

build:
	cargo rustc --manifest-path=mock-sep-41/Cargo.toml --crate-type=cdylib --target=wasm32-unknown-unknown --release
	soroban contract optimize \
		--wasm target/wasm32-unknown-unknown/release/mock_sep_41_token.wasm \
		--wasm-out sep-41/src/testutils/mock_sep_41_token.wasm
	cargo build -p sep-41-token
	cargo build -p sep-41-token --features testutils

fmt:
	cargo fmt --all

clean:
	cargo clean
