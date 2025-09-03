default: build

all: test

test: build
	cargo test

build:
	cargo build --target wasm32v1-none --release
	stellar contract optimize --wasm ../../target/wasm32v1-none/release/fixed_apr_strategy.wasm
	@rm ../../target/wasm32v1-none/release/fixed_apr_strategy.wasm

fmt:
	cargo fmt --all --check

clean:
	cargo clean