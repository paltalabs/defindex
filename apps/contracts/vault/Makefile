default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32v1-none --release
	soroban contract optimize --wasm ../target/wasm32v1-none/release/defindex_vault.wasm
	@rm ../target/wasm32v1-none/release/defindex_vault.wasm
fmt:
	cargo fmt --all --check

clean:
	cargo clean
