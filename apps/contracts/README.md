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
    
3.  **Select Strategies:** You can comment the strategies you dont want to deploy in the array in `src/deploy_vault.ts` to deploy different strategies. 
        
4.  **Deploy Blend Vault:**
    ```
    yarn deploy-vault <network> <asset>
    ```

## Deploying strategies
You can deploy the strategies by running:
```
yarn deploy-strategies <network> <asset_symbol> <number_of_strategies> <force_install> # number of strategies deployed, to run tests use with value 2
```
You can comment the strategies you dont want to deploy in the array in `src/strategies/deploy_strategies.ts` to deploy different strategies.
if number of strategies is not specified, it will deploy one strategy.

leave force_install empty if the wasm for the strategy is already installed(hasnt changed).

## Tests on typescript
Make sure that you have configured the `.env` file and set your configs at `configs.json` file
Before running the tests, you need to deploy the contracts, you can do this by running:
```bash
cd  apps/contracts
make  build
yarn deploy-factory <network>
yarn deploy-strategies <network> <asset_symbol> 2 true # number of strategies deployed, to run tests use with value 2
yarn publish-addresses <network>
yarn deploy-vault <network> <asset_symbol>
```
#### Multi deploy blend
This is for testing purposes.
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

## Production deployment

### Factory
Make sure you have compiled the contracts:
```
make  build
```
and then you can deploy the factory with the following command:
```
yarn deploy-factory <network>
```
Copy the deployed factory address from the output on `.soroban/<network>.contracts.json` and paste it in `public/<network>.contracts.json`
or just run the following command to copy it automatically:

```
    yarn publish-addresses <network>
```

### blend strategies
First you need to complete the following steps:
1. review the `blend_deploy_config.json` file to ensure that the strategies are correctly configured. In this file you can see a list of the strategies to deploy and the parameters for each one.
2. ensure that the addresses in the `<network>.contracts.json` file are correct and match the ones in the `blend_deploy_config.json` file.
3. ensure that the blend_keeper and blend_deployer secret keys are set in the `.env` file. Also, make sure that the `BLEND_KEEPER_SECRET_KEY` and `BLEND_DEPLOYER_SECRET_KEY` has trustlines set with all the assets that will be used in the strategies.
4. run the deploy_blend script to deploy the strategies:
```
yarn deploy-blend <network>
```
5. Then, to make it available for the frontend, you need to copy the new deployed strategies from `.soroban/<network>.contracts.json` into the `~/public/<network>.contracts.json` file.

### Deploy vault
Before deploying a vault, ensure that you have the `~/public/<network>.contracts.json` file updated with the latest contract addresses. This file should contain the addresses of all the necesary contracts deployed on the specified network, such as the factory, strategies, and blend addresses.

To deploy a vault, you need to run the following command:
```
yarn deploy-vault <network> <asset>
```
where `<network>` is the network you want to deploy to (e.g., `testnet`, `mainnet`) and `<asset>` is the asset you want to use (e.g., `usdc`, `xlm`). 
>[!NOTE] Make sure to double-check that the configuration in the `./configs.json` and the addresses at the public/`<network>`.contracts.json files are correct before deploying.

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