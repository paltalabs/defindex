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
yarn deploy-fixed-usdc <network>
```
```bash 
yarn deploy-fixed-xtar <network>
```
```bash 
yarn deploy-blend <network> <asset symbol "usdc" / "xlm">
```

If you want to deploy all the contracts you can do so by running:
```
yarn deploy-factory testnet && 
yarn deploy-hodl testnet && 
yarn deploy-fixed testnet && 
yarn deploy-fixed-usdc testnet && 
yarn deploy-fixed-xtar testnet && 
yarn deploy-blend testnet xlm
```

#### Multi deploy blend
```
# yarn multi-deploy-blend <network> <number of strategies >= 2> <asset key "usdc" / "xlm">
yarn multi-deploy-blend testnet 2 usdc
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

### Deplying on Mainnet
1. Make sure you have `ADMIN_SECRET_KEY`, `MAINNET_RPC_URL`, and `TEST_USER` in .env file
2. Make sure you have the correct setting for mainnet in configs.json
3. Set up the contract addresses from other projects (i.e. Blend) on `public/mainnet.contracts.json`
4. `yarn deploy-blend mainnet xlm`
5. Test the Blend strategy with `yarn exec tsc && node dist/tests/blend/test_strategy.js mainnet xlm`
6. Deploy Vault using the blend strategy `yarn deploy-blend-vault mainnet xlm`
7. Test the Vault-Blend with `yarn test-blend-strategy mainnet xlm`

### Generate Docs
```bash 
cargo doc --package defindex-strategy-core --package defindex-factory --package defindex-vault --no-deps
```

to publish them, run this to copy all files into /rust_docs
```bash
cp -rf /workspace/apps/contracts/target/doc/* /workspace/apps/rust_docs/
```

## Scout Audit
```bash
cd apps/contracts/factory/
cargo scout-audit
```