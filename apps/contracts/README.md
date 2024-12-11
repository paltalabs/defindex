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

Before running the tests, you need to deploy the contracts, you can do this by running:

```bash 
cd apps/contracts
make build
```

```bash 
yarn deploy-factory <network>
```
```bash 
yarn deploy-hodl <network>
```
```bash 
yarn deploy-fixed <network>
```
```bash 
yarn deploy-blend <network>
```

once you have deployed all the contracts you can run all the tests by running:

```bash
yarn test testnet -a
```
If you want to see all the avaliable test you can do so by running:

```bash
yarn test testnet -h
```
it will show the next message where you can see all the available tests and the specific flags to run them.
  
  ```bash


### Generate Docs
```bash 
cargo doc --package defindex-strategy-core --package defindex-factory --package defindex-vault --no-deps
```

to publish them, run this to copy all files into /rust_docs
```bash
cp -rf /workspace/apps/contracts/target/doc/* /workspace/apps/rust_docs/
```