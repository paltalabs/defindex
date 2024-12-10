default: build

test: build
	cargo test --all --tests

build:
	cargo rustc --manifest-path=mock-sep-40/Cargo.toml --crate-type=cdylib --target=wasm32-unknown-unknown --release
	soroban contract optimize \
		--wasm target/wasm32-unknown-unknown/release/mock_sep_40_oracle.wasm \
		--wasm-out sep-40/src/testutils/mock_sep_40_oracle.wasm
	cargo build -p sep-40-oracle
	cargo build -p sep-40-oracle --features testutils

fmt:
	cargo fmt --all

clean:
	cargo clean
