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
