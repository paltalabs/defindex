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

## Deployments

---

### Configuration files

Before deploying anything, understand the three configuration sources:

#### `configs.json`

Located at `apps/contracts/configs.json`. Contains network-level settings and vault role assignments. Each network entry includes:

| Field | Description |
|---|---|
| `network` | Network identifier: `mainnet`, `testnet`, `standalone` |
| `horizon_rpc_url` | Horizon REST API endpoint |
| `soroban_rpc_url` | Soroban RPC endpoint |
| `soroban_network_passphrase` | Network passphrase for transaction signing |
| `friendbot_url` | (testnet only) Friendbot URL for account funding |
| `defindex_factory_admin` | Public key of the factory admin |
| `defindex_fee` | Protocol fee in basis points (e.g. `2000` = 20%) |
| `defindex_fee_receiver` | Public key that receives protocol fees |
| `vault_fee_receiver` | Public key that receives vault-level fees |
| `vault_manager` | Can call operational functions (e.g. `investVault`) |
| `vault_emergency_manager` | Can pause the vault in emergencies |
| `vault_rebalance_manager` | Can rebalance allocations between strategies |
| `blend_keeper` | Account authorized to call `harvest` on Blend strategies |
| `vault_name` | Display name stored in the vault contract |
| `vault_symbol` | Token symbol for vault shares (e.g. `DFXV`) |

> The deployer's secret key must be set in `.env` as `DEPLOYER_SECRET_KEY`. On testnet/standalone, the admin account is airdropped automatically.

---

#### `public/<network>.contracts.json`

Located at `public/testnet.contracts.json` (root of the monorepo, **not** inside `apps/contracts`). This is the **shared address book** used as input by `deploy_vault.ts`. It is also written to by `yarn publish-addresses` (which copies from `apps/contracts/.soroban/`).

It must contain the addresses of all external dependencies:

```json
{
  "ids": {
    "soroswap_router":         "<address>",
    "blend_fixed_xlm_usdc_pool": "<address>",
    "blend_pool_usdc":         "<address>",
    "blend_pool_cetes":        "<address>",
    "blnd_token":              "<address>",
    "cetes_token":             "<address>",
    "soroswap_usdc":           "<address>",
    "XLM_blend_strategy":      "<address after strategy deploy>",
    "USDC_blend_strategy":     "<address after strategy deploy>",
    "CETES_blend_strategy":    "<address after strategy deploy>"
  }
}
```

**Strategy addresses** (keys like `<ASSET>_blend_strategy`) must be added manually after running `deploy-blend`. The vault deployment reads from this file to find the strategy address.

---

#### `apps/contracts/public/<network>.contracts.json`

A secondary address book scoped to the contracts workspace. Read by `deploy_blend.ts` and `constants.ts` to load token and pool addresses. Must contain the external dependency addresses (tokens, pools, router). **Must be populated manually** — `yarn publish-addresses` writes to the root-level `public/`, not to this file.

---

#### `.soroban/<network>.contracts.json`

Located at `apps/contracts/.soroban/`. This is the **internal address book** written automatically by deployment scripts. It stores every deployed contract address using the key format:

- Strategies: `<asset_symbol>_blend_<name>_<pool_name>_strategy`
  e.g. `cetes_blend_regional_starter_pack_rsp_strategy`
- Vaults: `<asset>_paltalabs_vault`
  e.g. `cetes_paltalabs_vault`

Do not edit this file manually — it is managed by the scripts.

---

#### `src/strategies/blend_deploy_config.json`

Contains the list of Blend strategies to deploy, organized by network. Each strategy entry:

```json
{
  "name": "regional_starter_pack",
  "keeper": "<keeper public key>",
  "asset": "<token contract address>",
  "asset_symbol": "CETES",
  "reward_threshold": "40",
  "blend_pool_address": "<blend pool contract address>",
  "blend_pool_name": "rsp"
}
```

| Field | Description |
|---|---|
| `name` | Strategy variant name, used in the resulting contract key |
| `keeper` | Public key authorized to call `harvest` |
| `asset` | The underlying token address the strategy manages |
| `asset_symbol` | Symbol used to filter deploys and build contract keys |
| `reward_threshold` | Minimum BLND reward amount that triggers auto-compounding. **Note:** currently hardcoded to `40` in `deploy_blend.ts`; this field is not yet read by the deploy script. |
| `blend_pool_address` | The Blend pool where assets are deposited |
| `blend_pool_name` | Short name for the pool, used in the resulting contract key |

The resulting contract key is: `<asset_symbol>_blend_<name>_<pool_name>_strategy`

> `install_contract: "true"` at the network level controls whether the WASM is re-uploaded. Set to `"false"` to reuse an already-installed WASM hash.

### Deploying a Blend strategy

**1. Configure `blend_deploy_config.json`**
Add or verify the entry for your asset under the target network. Example for CETES on testnet:

```json
{
  "name": "regional_starter_pack",
  "keeper": "G...",
  "asset": "CC72F57YTPX76HAA64JQOEGHQAPSADQWSY5DWVBR66JINPFDLNCQYHIC",
  "asset_symbol": "CETES",
  "reward_threshold": "40",
  "blend_pool_address": "CAPBMXIQTICKWFPWFDJWMAKBXBPJZUKLNONQH3MLPLLBKQ643CYN5PRW",
  "blend_pool_name": "rsp"
}
```

**2. Ensure the deployer has a trustline for the asset**
The deployer account (from `DEPLOYER_SECRET_KEY`) must have a trustline for the asset token and hold at least a small balance (~1001 stroops worth), which is required for the bootstrap deposit made automatically on first deploy.

**3. Run the deploy script**
To deploy all strategies for a network:
```bash
npm run deploy-blend -- <network>
```

To deploy only a specific asset (avoids re-deploying existing strategies):
```bash
npm run deploy-blend -- <network> <ASSET_SYMBOL>
```

Example:
```bash
npm run deploy-blend -- testnet CETES
```

The script will:
- Airdrop the admin account (testnet only)
- Install the `blend_strategy` WASM (if `install_contract: "true"`)
- Deploy the strategy contract
- Make an automatic bootstrap deposit to prevent the first-depositor vulnerability
- Save the address to `.soroban/<network>.contracts.json`

**4. Publish the strategy address**
After deploying, copy the new strategy address from `.soroban/<network>.contracts.json` and add it to `public/<network>.contracts.json` (root level) using the key `<ASSET_SYMBOL>_blend_strategy`:

```json
"CETES_blend_strategy": "C..."
```

---

### Deployment instructions

**Prerequisites:**

- Factory deployed and address in `public/<network>.contracts.json`
- Strategy deployed and its address added to `public/<network>.contracts.json` as `<ASSET>_blend_strategy`
- `configs.json` has correct vault roles for the target network

**Run the deploy script:**
```bash
npm run deploy-vault -- <network> <ASSET_SYMBOL>
```

Example:
```bash
npm run deploy-vault -- testnet XLM
```

The script reads:

- The strategy address from `public/<network>.contracts.json` → key `<ASSET>_blend_strategy`
- The factory address from the same file → key `defindex_factory`
- Vault roles (manager, fee receiver, etc.) from `configs.json`

On success it prints the vault address and saves it to `.soroban/<network>.contracts.json` as `<asset>_paltalabs_vault`.

> After vault deployment, users deposit into the vault and the vault manager calls `investVault` to allocate funds to the strategy. This is a separate operational step.

---

### Full deployment sequence (new asset)

```bash
# 1. Build contracts
make build

# 2. Deploy factory (first time only)
npm run deploy-factory -- testnet
npm run publish-addresses -- testnet

# 3. Add strategy config to blend_deploy_config.json (manual)

# 4. Deploy the strategy
npm run deploy-blend -- testnet CETES

# 5. Add strategy address to public/testnet.contracts.json (manual)
#    "CETES_blend_strategy": "<address from .soroban/testnet.contracts.json>"

# 6. Deploy the vault
npm run deploy-vault -- testnet CETES
```

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