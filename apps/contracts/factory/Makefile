# Declare the dependencies of the factory on defindex
default: build

all: test

test: build
	cargo test

build: ../target/wasm32v1-none/release/defindex_vault.optimized.wasm
	cargo build --target wasm32v1-none --release
	soroban contract optimize --wasm ../target/wasm32v1-none/release/defindex_factory.wasm
	@rm ../target/wasm32v1-none/release/defindex_factory.wasm

../target/wasm32v1-none/release/defindex_vault.optimized.wasm:
	$(MAKE) -C ../defindex build

fmt:
	cargo fmt --all --check

clean:
	@rm -f ../target/wasm32v1-none/release/defindex_vault.optimized.wasm