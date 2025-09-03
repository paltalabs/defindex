---
cover: ../../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
---

# Create a Vault

DeFindex Vaults let **wallet builders** design yield products tailored to their users.\
With a vault, you choose the **assets**, **strategies**, **allocation**, and **fees**—and you can rebalance positions or rescue funds at any time. From the end-user’s perspective, it’s just **deposit and withdraw**.

***

####

### Step 1: Assign Vault Roles, Fees, and Upgradability

Before deployment, you must configure the following **roles** (each tied to an address):

* **Vault Manager** – primary owner, manages settings, upgrades, and other roles (_use a multisig for security_)
* **Rebalance Manager** – allocates funds across strategies (_can be automated_)
* **Fee Receiver** – collects performance fees (_use a secure, dedicated wallet_)
* **Emergency Manager** – rescues funds and pauses risky strategies (_automate for faster response_)

***

**Fees**

* Fees are expressed in **basis points** (1 bp = 0.01%).
* The **Vault Fee** is applied to strategy earnings and assigned to the Fee Receiver.

***

**Upgradability**

* You may choose whether the vault contract is **upgradable**.
* If enabled, the contract’s WASM code can be updated **without user signatures**.
* This allows improvements or bug fixes without interrupting vault operations.

***

### Step 2: Select Assets and Strategies

DeFindex offers **curated and audited strategies**, currently live for:

* **Blend Autocompound – Fixed Pool**: USDC, EURC, XLM
* **Blend Autocompound – YieldBlox Pool**: USDC, EURC, XLM

👉 Need support for additional pools? Just ping us.

***

### Step 3: Deploy the Vault

You can deploy your Vault in two ways:

* [**Using GUI (Basic)**](using-gui-basic.md)
* [Using the Factory Contract (Advanced)](using-the-factory-advanced.md)

### Step 4: Do a First Investment&#x20;

A **minimum first investment of 1001 units** of your supported asset is required.

* **Why?**
  * **1000 units** will be permanently locked in the Vault for security.
* **Example:**\
  If you are depositing **USDC**, this equals just **0.0010001 USDC** — practically nothing!

This first investment is necessary before you can proceed to the next step: the **first rebalance**.

## Step 5: First Rebalance

After deployment, perform the **first rebalance** to define allocations across chosen strategies.



You can do the first rebalance using the script `vault_usage_example.ts` (discussed on next section) or by using the [stellar-cli](https://developers.stellar.org/docs/build/guides/cli).

First, you need to setup your keys, make sure the rebalancer manager role defined previously is the one you are going to setup. For example, you can set it up using secret key by:

```bash
stellar keys add --secret-key rebalancer
```

then, you will be prompted to write your secret key.

Next, let's make the rebalance using testnet as example. You can do that by:

```bash
stellar contract invoke \
  --rpc-url https://soroban-testnet.stellar.org/ \
  --network-passphrase 'Test SDF Network ; September 2015' \
  --id <CONTRACT_ID> \
  --source-account rebalancer \
  -- \
  rebalance \
  --caller <REBALANCER_ADDRESS> \
  --instructions '[{"Invest":["<STRATEGY_ADDRESS>", "<AMOUNT_IN_STROOPS>"]}]'
```

you can find the strategy addresses on `~/public/<network>.contracts.json`.

And that's all!



### Interacting with the Factory Contract (Advanced)

If you prefer to interact directly with the DeFindex Factory contract to create a vault, here's a step-by-step guide:

1. **Locate the Factory contract**: Find the DeFindex Factory contract address in the [`~/public/`](https://github.com/paltalabs/defindex/tree/main/public) folder.
2. **Prepare the transaction**: Use your preferred method to prepare a transaction that interacts with the Factory contract. You will need to provide the following parameters:
   * `roles`: A `Map` containing role identifiers (`u32`) and their respective addresses (`Address`). Example: `{1: "GCINP...", 2: "GCINP..."}`.
   * `vault_fee`: The commission rate in basis points (1 basis point = 0.01%). Example: `100` for a 1% fee.
   * `assets`: A vector of [`AssetStrategySet`](../../../contracts/common/src/models.rs) structures that define the strategies and assets managed by the vault.
     *   **Structure of AssetStrategySet**:

         ```rust
         struct AssetStrategySet {
             address: Address,  // The address of the asset (token)
             strategies: Vec<Strategy>,  // A vector of strategies for this asset
         }

         struct Strategy {
             address: Address,  // The address of the strategy contract
             name: String,      // The name of the strategy
             paused: bool,      // Whether the strategy is initially paused
         }
         ```
     *   **Example**:

         ```json
         {
           "address": "CBZ5WXLMCH...",  // USDC token address
           "strategies": [
             {
               "address": "CCIN4WQP5Z...",  // Lending strategy address
               "name": "DummyStrategy",
               "paused": false
             },
             {
               "address": "CD2QVXMN7Y...",  // Yield farming strategy address
               "name": "DummyStrategy2",
               "paused": false
             }
           ]
         }
         ```
   * `soroswap_router`: The address of the Soroswap router (`Address`) that facilitates exchanges within the vault. (You can find the address [here](https://api.soroswap.finance/api/mainnet/router))
   * `name_symbol`: A `Map` containing the name and symbol of the vault. Example: `{"name": "MyVault", "symbol": "MVLT"}`.
   * `upgradable`: A boolean indicating whether the vault contract will support upgrades. Example: `true` or `false`.
3. **Submit the transaction**: Once you have prepared the transaction with the required parameters, sign and send it using your preferred method.
4. **Wait for confirmation**: After submitting the transaction, wait for it to be confirmed on the blockchain. Once confirmed, your vault will be active, and you can start interacting with it.
