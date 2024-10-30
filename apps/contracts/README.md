## DeFindex Contracts

First, run the development server:

```bash
yarn build
```

### Building contracts inside the container
To build the contracts inside the container you can do 
```bash
make build
```
and to test them run

```bash 
make test
```
if you want to test or build each contract separatedly you can do the same inside the contract directory

### Tests on typescript

to deploy the factory you can use 

```bash 
yarn deploy-factory <network>
```

Make sure you have the hodl strategy deployed, if not, you can run:
    
```bash
yarn deploy-hodl <network>
```

to test the factory to deploy a DeFindex Vault, and deposit there two times, you can run:

```bash
yarn test <network>
```
If you only want to test a specific vault you should modify the contract address in the test file `apps/contracts/src/tests/testOnlyVault.ts` and run the following command:

```bash
yarn test-vault <network>
```

### Generate Docs
```bash 
cargo doc --package defindex-strategy-core --package defindex-factory --package defindex-vault --no-deps
```

to publish them, run this to copy all files into /rust_docs
```bash
cp -rf /workspace/apps/contracts/target/doc/* /workspace/apps/rust_docs/
```