---
description: ⏱️ 3 min read
cover: ../../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
---

# Create a Vault

DeFindex Vaults let **wallet builders** design yield products tailored to their users.\
With a vault, you choose the **assets**, **strategies**, **allocation**, and **fees**—and you can rebalance positions or rescue funds at any time. From the end-user's perspective, it's just **deposit and withdraw**.

***

## Vault Creation Requirements

Before you deploy a vault, make sure you meet the following requirements. These details are often the source of confusion for new builders.

### Network

DeFindex operates on **Stellar Mainnet** and **Stellar Testnet**. The two networks are completely independent and use different contract addresses and token types.

| Requirement        | Testnet                                                                    | Mainnet                                                                    |
| ------------------ | -------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| Network Passphrase | `Test SDF Network ; September 2015`                                        | `Public Global Stellar Network ; September 2015`                           |
| Factory contract   | See [Testnet Deployment](../../contract-deployments/testnet-deployment.md) | See [Mainnet Deployment](../../contract-deployments/mainnet-deployment.md) |
| RPC URL            | `https://soroban-testnet.stellar.org`                                      | `https://soroban.stellar.org` or another provider                          |
| Horizon URL        | `https://horizon-testnet.stellar.org`                                      | `https://horizon.stellar.org`                                              |

### Tokens

**On Testnet**, DeFindex strategies use a test USDC issued by the Blend Capital testnet deployment — referred to here as **BlendUSDC**. This is **not** Soroswap or regular USDC; it is a separate test token you must obtain from [testnet.blend.capital](https://testnet.blend.capital).

**On Mainnet**, strategies use real USDC (Circle) and other well-known tokens.

#### Token addresses

| Token            | Network           | Contract Address                                           |
| ---------------- | ----------------- | ---------------------------------------------------------- |
| XLM (native SAC) | Testnet & Mainnet | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` |
| BlendUSDC        | **Testnet only**  | `CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU` |
| USDC (Circle)    | **Mainnet only**  | `CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75` |
| CETES            | Testnet           | `CC72F57YTPX76HAA64JQOEGHQAPSADQWSY5DWVBR66JINPFDLNCQYHIC` |

For a full list of deployed strategy contract addresses, see the [Contract Deployments](../../contract-deployments/) section.

### Getting Testnet Tokens

You need testnet tokens to deploy and interact with a vault on testnet:

1.  **XLM (for fees)**: Use [Friendbot](https://friendbot.stellar.org/) to fund your testnet account.

    ```
    https://friendbot.stellar.org/?addr=YOUR_STELLAR_ADDRESS
    ```
2. **BlendUSDC** (required for USDC vaults on testnet):
   * Go to [testnet.blend.capital](https://testnet.blend.capital).
   * Connect your Freighter wallet (switch to Testnet in wallet settings first).
   * Click **"Faucet"** or use the asset menu to add a BlendUSDC trustline and receive test tokens.
3. **CETES** (required for CETES vaults on testnet):
   * Contact Etherfuse team to get the official test CETES

> **Why BlendUSDC?** On testnet, DeFindex strategies are deployed against the Blend Capital testnet pools, which use their own test USDC. Real Circle USDC does not support large amounts minting on Stellar Testnet.

### Funding Requirements

A **minimum first deposit of 1001 units** (in the asset's smallest denomination — stroops) of your vault's underlying asset is required immediately after deployment.

* On the **first deposit**, the vault locks **1000 shares** as an inflation-attack defense mechanism.
* **Example**: For a USDC vault (7 decimals), 1001 stroops = **0.0001001 USDC** — practically free.
* **Example**: For an XLM vault (7 decimals), 1001 stroops = **0.0001001 XLM** — also negligible.

This is required to prevent the [ERC-4626 inflation attack](https://blog.openzeppelin.com/a-novel-defense-against-erc4626-inflation-attacks) (the same class of attack applies to Soroban vaults).

> Funds are **not** automatically invested on deposit. After the first deposit you must perform a [**first rebalance**](./#step-5-first-rebalance) to allocate funds across strategies.

{% hint style="info" %}
A first deposit of 20 USDC prevents historical APY endpoint from having approximation errors
{% endhint %}

***

### Step 1: Assign Vault Roles, Fees, and Upgradability

Before deployment, you must configure the following **roles** (each tied to an address):

* **Manager** – primary owner, manages settings, upgrades, and other roles (use a secure wallet, ex. multisig, cold or MPC)
* **Emergency Manager** – rescues funds and pauses risky strategies (use a hot wallet with fast access or automate it for faster response)
* **Rebalance Manager** – allocates funds across strategies (use a hot wallet or it can be managed by a third party)
* **Fee Receiver** – collects performance fees (_use a secure, dedicated wallet_)

> All four roles must be assigned. You may use the same address for multiple roles, but this is not recommended for production.

***

**Fees**

* Fees are expressed in **basis points** (1 bp = 0.01%).
* The **Vault Fee** is applied to strategy earnings and assigned to the Fee Receiver.
* Maximum allowed fee: **10,000 bps (100%)** — in practice, keep this well below 100%.

***

**Upgradability**

* Choose whether this vault can be upgraded after deployment.
* If enabled, you can migrate to a new vault version in the future.
* Your users' funds and positions remain unaffected — no action required on their end.

***

### Step 2: Select Assets and Strategies

DeFindex offers **curated and audited strategies**, currently live for:

* **Blend Autocompound – Fixed Pool**: USDC, EURC, XLM
* **Blend Autocompound – YieldBlox Pool**: USDC, EURC, XLM, CETES, USTRY, AQUA
* **Blend Autocompound – Orbit Pool**: XLM, CETES, USTRY, oUSD

Each vault supports one or more assets, and each asset can be backed by one or more strategies.

> Need support for additional pools? Just ping us on [Discord](https://discord.gg/e2qAhJCBmx).

***

### Step 3: Deploy the Vault

You can deploy your Vault in two ways:

* [**Using GUI (Basic)**](using-gui-basic.md)
* [**Using the Factory Contract or API (Advanced)**](using-the-factory-advanced.md)

### Step 4: Do a First Deposit

A **minimum first deposit of 1001 units** of your supported asset is required.

* **Why?**
  * **1001 units** will be permanently locked in the Vault for security.
* **Example:**\
  If you are depositing **USDC**, this equals just **0.0001001 USDC** — practically nothing!

This is because these vaults are protected from something called "inflation attacks" you can read more about this kind of attacks on [OpenZeppelin blog](https://blog.openzeppelin.com/a-novel-defense-against-erc4626-inflation-attacks)

### Step 5: First Rebalance

After deployment, perform the **first rebalance** to define allocations across chosen strategies. This may be a bit confusing right? but How the vault is going to know how to distribute the funds across the different strategies? This step only need to be done once.

You have 3 ways to make the first rebalance: using API, using a script ([discussed here](../smart-contracts/#using-the-example-script-vault_usage_example.ts)) or using [stellar-cli](https://developers.stellar.org/docs/build/guides/cli)

#### Using API

Make sure you have an `API_KEY` to call the API. And then, call the rebalance function

```
curl --location 'https://api.defindex.io//vault/${VAULT_ADDRESS}/rebalance?network=mainnet' \
--header 'Content-Type: application/json' \
--header 'Authorization: Bearer ${JWT_TOKEN}' \
--data '{
    "caller": "${MANAGER_OR_REBALANCE_MANAGER}$",
    "instructions": [
        {
            "type": "Invest",
            "strategy_address": "${STRATEGY_ADDRESS}$",
            "amount": 1000000
        }
    ]
}'

```

Where `VAULT_ADDRESS` is the address of the recently deployed vault, `JWT_TOKEN` is the API key, the `MANAGER_OR_REBALANCE_MANAGER` is what you defined when creating the vault. The strategy address is the one you want to invest on. For a list of all the addresses you can check -> [here](../../../../public/mainnet.contracts.json).

This will return you an unsigned XDR, that can be signed using your preferred method of signing. One simple method could be using [Stellar Laboratory](https://lab.stellar.org/transaction/sign). Simply copy and paste the unsigned XDR and sign it.

#### Using stellar-cli

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

you can find the strategy addresses on [`~/public/<network>.contracts.json`](../../../../public/mainnet.contracts.json).

And that's all!
