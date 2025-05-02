# DeFindex Contracts
To use this application use the Defindex Docker container by running:
 ``` bash
docker compose up -d #Start the containers
bash run.sh #Connecte to the workspace container
``` 

## Building contracts inside the container

To build the contracts inside the container navigate to `apps/contracts` in your terminal and run
```bash
make  build
```
and to test them run  
```bash
make  test
```
if you want to test or build each contract separatedly you can do the same inside the contract directory.


## Deploying a vault
Deploying a **Defindex** vault requires careful configuration. Follow these steps precisely:"

1.  **Environment Setup:** Ensure the following environment variables are set in your `.env` file:
    
    -   `DEPLOYER_SECRET_KEY`: The administrator's secret key.
        
    -   `MAINNET_RPC_URL`: The URL of your Ethereum mainnet RPC provider.
        
2.  **Configuration:** Verify that the `configs.json` file has the correct settings for mainnet deployment.
    
3.  **Contract Addresses:** Make sure that the `public/mainnet.contracts.json` file is up to date.
        
4.  **Deploy Blend Vault:**
    ```
    yarn deploy-blend-vault <network> <asset>
    ```

## Tests on typescript
Make sure that you have configured the `.env` file and set your configs at `configs.json` file
Before running the tests, you need to deploy the contracts, you can do this by running:
```bash
cd  apps/contracts
make  build
yarn deploy-factory <network>
yarn deploy-strategies <network> <asset_symbol> 2 # number of strategies deployed, to run tests use with value 2
yarn publish-addresses <network>
yarn deploy-vault <network> <asset_symbol>
```
#### Multi deploy blend
```
# yarn multi-deploy-blend <network> <number of strategies >= 2> <asset key "usdc" / "xlm">
yarn multi-deploy-blend testnet 2 usdc
```
once you have deployed all the contracts you can run all the tests by running:
```bash
yarn  test  testnet  -a
```
If you want to see all the avaliable test you can do so by running:
```bash
yarn  test  testnet  -h
```
it will show the next message where you can see all the available tests and the specific flags to run them.


## Generate Docs
```bash
cargo doc --package defindex-strategy-core --package defindex-factory --package defindex-vault --no-deps
```
to publish them, run this to copy all files into /rust_docs
```bash
cp  -rf  /workspace/apps/contracts/target/doc/*  /workspace/apps/rust_docs/
```
## Scout Audit
```bash
cd  apps/contracts/factory/
cargo  scout-audit
```