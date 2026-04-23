---
description: ⏱️ 5 min read
---

# Using the Factory (Advanced)

This guide documents every argument involved in deploying a DeFindex vault — whether you call the factory smart contract directly or use the DeFindex REST API. Read the [Vault Creation Requirements](README.md#vault-creation-requirements) section first before proceeding.

---

## Quick Reference: Contract Addresses

### Testnet

Testnet addresses may be not valid after the June 17 or December 16, 2026, testnet resets. If that's the case let us know via Discord so we can update the docs.

| Contract | Address |
|---|---|
| Factory | `CDSCWE4GLNBYYTES2OCYDFQA2LLY4RBIAX6ZI32VSUXD7GO6HRPO4A32` |
| Soroswap Router | `CCJUD55AG6W5HAI5LRVNKAE5WDP5XGZBUDS5WNTIVDU7O264UZZE7BRD` |
| XLM (native SAC) | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` |
| BlendUSDC | `CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU` |
| CETES token | `CC72F57YTPX76HAA64JQOEGHQAPSADQWSY5DWVBR66JINPFDLNCQYHIC` |
| USDC Blend Strategy | `CALLOM5I7XLQPPOPQMYAHUWW4N7O3JKT42KQ4ASEEVBXDJQNJOALFSUY` |
| XLM Blend Strategy | `CDVLOSPJPQOTB6ZCWO5VSGTOLGMKTXSFWYTUP572GTPNOWX4F76X3HPM` |
| CETES Blend Strategy | `CCP4RBDWPRNO2LWO23XFU4BBLGA73J5N3BK7EHRJUHVN33YEMMFB2MBE` |

### Mainnet

| Contract | Address |
|---|---|
| Factory | `CDKFHFJIET3A73A2YN4KV7NSV32S6YGQMUFH3DNJXLBWL4SKEGVRNFKI` |
| Soroswap Router | `CAG5LRYQ5JVEUI5TEID72EYOVX44TTUJT5BQR2J6J77FH65PCCFAJDDH` |
| XLM (native SAC) | `CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC` |
| USDC (Circle) | `CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75` |
| USDC Fixed Pool Strategy | `CDB2WMKQQNVZMEBY7Q7GZ5C7E7IAFSNMZ7GGVD6WKTCEWK7XOIAVZSAP` |
| EURC Fixed Pool Strategy | `CC5CE6MWISDXT3MLNQ7R3FVILFVFEIH3COWGH45GJKL6BD2ZHF7F7JVI` |
| XLM Fixed Pool Strategy | `CDPWNUW7UMCSVO36VAJSQHQECISPJLCVPDASKHRC5SEROAAZDUQ5DG2Z` |
| USDC YieldBlox Strategy | `CCSRX5E4337QMCMC3KO3RDFYI57T5NZV5XB3W3TWE4USCASKGL5URKJL` |
| XLM YieldBlox Strategy | `CBDOIGFO2QOOZTWQZ7AFPH5JOUS2SBN5CTTXR665NHV6GOCM6OUGI5KP` |
| CETES YieldBlox Strategy | `CBTSRJLN5CVVOWLTH2FY5KNQ47KW5KKU3VWGASDN72STGMXLRRNHPRIL` |

For the complete list of mainnet strategies, see [Mainnet Deployment](../../contract-deployments/mainnet-deployment.md).

---

## Vault Role IDs

When calling the factory contract directly, roles are passed as a `Map<u32, Address>`. The role IDs are:

| ID | Role | Description |
|---|---|---|
| `0` | Emergency Manager | Can pause strategies and rescue funds in emergencies |
| `1` | Fee Receiver | Receives vault performance fees |
| `2` | Manager | Full administrative control over the vault |
| `3` | Rebalance Manager | Can rebalance asset allocations across strategies |

All four roles must be assigned. The same address can be used for multiple roles.

---

## Method 1: Direct Factory Contract Call

Use this method when integrating at the smart-contract level (e.g., building your own deployment script or using `stellar-cli`).

### Function: `create_defindex_vault`

```rust
fn create_defindex_vault(
    e: Env,
    roles: Map<u32, Address>,
    vault_fee: u32,
    assets: Vec<AssetStrategySet>,
    soroswap_router: Address,
    name_symbol: Map<String, String>,
    upgradable: bool,
) -> Result<Address, FactoryError>
```

#### Parameter Reference

| Parameter | Type | Description |
|---|---|---|
| `roles` | `Map<u32, Address>` | Maps role IDs (0–3) to Stellar addresses. All four roles are required. |
| `vault_fee` | `u32` | Vault fee in basis points (1 bps = 0.01%). Max: 10,000 (100%). |
| `assets` | `Vec<AssetStrategySet>` | The assets the vault manages and their associated strategies. |
| `soroswap_router` | `Address` | Address of the Soroswap router used for internal swaps. |
| `name_symbol` | `Map<String, String>` | Metadata: must contain keys `"name"` and `"symbol"`. |
| `upgradable` | `bool` | If `true`, the Manager can upgrade the vault's WASM without user signatures. |

#### `AssetStrategySet` Structure

```rust
struct AssetStrategySet {
    address: Address,          // The token contract address (e.g., USDC or XLM SAC)
    strategies: Vec<Strategy>, // Strategies that manage this asset
}

struct Strategy {
    address: Address, // Strategy contract address
    name: String,     // Human-readable name (stored on-chain)
    paused: bool,     // Set to false on creation; strategies start active
}
```

#### Example: `stellar-cli` (Testnet, USDC vault)

```bash
stellar contract invoke \
  --rpc-url https://soroban-testnet.stellar.org \
  --network-passphrase 'Test SDF Network ; September 2015' \
  --id CDSCWE4GLNBYYTES2OCYDFQA2LLY4RBIAX6ZI32VSUXD7GO6HRPO4A32 \
  --source-account deployer \
  -- \
  create_defindex_vault \
  --roles '{"0":"GCKFBEIY...","1":"GCKFBEIY...","2":"GCKFBEIY...","3":"GCKFBEIY..."}' \
  --vault_fee 100 \
  --assets '[{"address":"CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU","strategies":[{"address":"CALLOM5I7XLQPPOPQMYAHUWW4N7O3JKT42KQ4ASEEVBXDJQNJOALFSUY","name":"BlendUSDC Strategy","paused":false}]}]' \
  --soroswap_router CCJUD55AG6W5HAI5LRVNKAE5WDP5XGZBUDS5WNTIVDU7O264UZZE7BRD \
  --name_symbol '{"name":"My USDC Vault","symbol":"MUSDC"}' \
  --upgradable true
```

> Replace `GCKFBEIY...` with your actual Stellar addresses for each role.

### Function: `create_defindex_vault_deposit`

Creates a vault **and** makes the initial deposit in one transaction.

```rust
fn create_defindex_vault_deposit(
    e: Env,
    caller: Address,
    roles: Map<u32, Address>,
    vault_fee: u32,
    assets: Vec<AssetStrategySet>,
    soroswap_router: Address,
    name_symbol: Map<String, String>,
    upgradable: bool,
    amounts: Vec<i128>,
) -> Result<Address, FactoryError>
```

Additional parameter over `create_defindex_vault`:

| Parameter | Type | Description |
|---|---|---|
| `caller` | `Address` | The address that signs the transaction and makes the deposit. |
| `amounts` | `Vec<i128>` | Initial deposit amounts in stroops, one per asset in the same order as `assets`. Minimum 1001 per asset. |

---

## Method 2: API — `POST /factory/create-vault`

Builds an unsigned XDR to deploy a vault. You must sign the XDR and submit it via `POST /send`.

```http
POST https://api.defindex.io/factory/create-vault?network=testnet
Authorization: Bearer <API_KEY>
Content-Type: application/json
```

### Request Body

```json
{
  "roles": {
    "manager": "GACKTN5D...",
    "emergencyManager": "GACKTN5D...",
    "rebalanceManager": "GACKTN5D...",
    "feeReceiver": "GACKTN5D..."
  },
  "vaultFeeBps": 100,
  "assets": [
    {
      "address": "CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU",
      "strategies": [
        {
          "address": "CALLOM5I7XLQPPOPQMYAHUWW4N7O3JKT42KQ4ASEEVBXDJQNJOALFSUY",
          "name": "BlendUSDC Strategy",
          "paused": false
        }
      ]
    }
  ],
  "name": "My USDC Vault",
  "symbol": "MUSDC",
  "upgradable": true,
  "caller": "GACKTN5D..."
}
```

### Field Reference

| Field | Type | Required | Description |
|---|---|---|---|
| `roles.manager` | `string` | Yes | Stellar address for the Manager role |
| `roles.emergencyManager` | `string` | Yes | Stellar address for the Emergency Manager role |
| `roles.rebalanceManager` | `string` | Yes | Stellar address for the Rebalance Manager role |
| `roles.feeReceiver` | `string` | Yes | Stellar address for the Fee Receiver role |
| `vaultFeeBps` | `number` | Yes | Vault fee in basis points (0–10000). `100` = 1% |
| `assets` | `Asset[]` | Yes | Array of assets the vault will manage |
| `assets[].address` | `string` | Yes | Token contract address for this asset |
| `assets[].strategies` | `Strategy[]` | Yes | Strategies that manage this asset |
| `assets[].strategies[].address` | `string` | Yes | Strategy contract address |
| `assets[].strategies[].name` | `string` | Yes | Human-readable name stored on-chain |
| `assets[].strategies[].paused` | `boolean` | Yes | Whether the strategy starts paused. Use `false` |
| `name` | `string` | Yes | Vault display name (stored on-chain) |
| `symbol` | `string` | Yes | Vault token symbol for dfTokens (e.g., `MUSDC`) |
| `upgradable` | `boolean` | Yes | Whether the vault contract can be upgraded by the Manager |
| `caller` | `string` | Yes | Stellar address of the deployer (signs the transaction) |

### Response

```json
{
  "xdr": "AAAAAgAAAAB...",
  "simulationResponse": { ... },
  "error": null
}
```

Sign the returned `xdr` with the `caller`'s key and submit via `POST /send?network=testnet`.

---

## Method 3: API — `POST /factory/create-vault-deposit`

Creates a vault **and** performs the initial deposit atomically. This is the recommended approach since it handles Steps 3 and 4 in a single transaction.

```http
POST https://api.defindex.io/factory/create-vault-deposit?network=testnet
Authorization: Bearer <API_KEY>
Content-Type: application/json
```

### Request Body

```json
{
  "roles": {
    "manager": "GACKTN5D...",
    "emergencyManager": "GACKTN5D...",
    "rebalanceManager": "GACKTN5D...",
    "feeReceiver": "GACKTN5D..."
  },
  "vaultFeeBps": 100,
  "assets": [
    {
      "address": "CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU",
      "strategies": [
        {
          "address": "CALLOM5I7XLQPPOPQMYAHUWW4N7O3JKT42KQ4ASEEVBXDJQNJOALFSUY",
          "name": "BlendUSDC Strategy",
          "paused": false
        }
      ]
    }
  ],
  "name": "My USDC Vault",
  "symbol": "MUSDC",
  "upgradable": true,
  "caller": "GACKTN5D...",
  "depositAmounts": [10000000]
}
```

### Additional Field

| Field | Type | Required | Description |
|---|---|---|---|
| `depositAmounts` | `number[]` | Yes | Initial deposit amounts in **stroops**, one per asset in the same order as `assets`. Must be ≥ 1001 per asset. |

All other fields are identical to `create-vault` above.

> **Decimal reference:** 1 USDC = `10_000_000` stroops (7 decimals). 1 XLM = `10_000_000` stroops. The minimum `1001` stroops equals **0.0001001 units**.

### Response

Same shape as `create-vault`:

```json
{
  "xdr": "AAAAAgAAAAB...",
  "simulationResponse": { ... },
  "error": null
}
```

---

## Method 4: API — `POST /factory/create-vault-auto-invest`

Creates a vault, makes the initial deposit, **and** immediately rebalances (invests) into strategies — all in one batched transaction. At the end it also transfers the Manager role to the final address. This is the most convenient option for fully automated deployments.

```http
POST https://api.defindex.io/factory/create-vault-auto-invest?network=testnet
Authorization: Bearer <API_KEY>
Content-Type: application/json
```

### Request Body

```json
{
  "caller": "GBZXUKUY...",
  "roles": {
    "manager": "GBAJGSZQ...",
    "emergencyManager": "GBAJGSZQ...",
    "rebalanceManager": "GBAJGSZQ...",
    "feeReceiver": "GBAJGSZQ..."
  },
  "name": "Auto-Invest USDC Vault",
  "symbol": "AIUSDC",
  "vaultFee": 100,
  "upgradable": true,
  "assets": [
    {
      "address": "CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU",
      "symbol": "USDC",
      "amount": 10000000,
      "strategies": [
        {
          "address": "CALLOM5I7XLQPPOPQMYAHUWW4N7O3JKT42KQ4ASEEVBXDJQNJOALFSUY",
          "name": "BlendUSDC Strategy",
          "amount": 10000000
        }
      ]
    }
  ]
}
```

### Field Reference

| Field | Type | Required | Description |
|---|---|---|---|
| `caller` | `string` | Yes | Deployer's Stellar address. Signs the transaction. The manager role is transferred from this address to `roles.manager` at the end. |
| `roles.manager` | `string` | Yes | Final Manager address after deployment |
| `roles.emergencyManager` | `string` | Yes | Emergency Manager address |
| `roles.rebalanceManager` | `string` | Yes | Rebalance Manager address |
| `roles.feeReceiver` | `string` | Yes | Fee Receiver address |
| `name` | `string` | Yes | Vault display name |
| `symbol` | `string` | Yes | dfToken symbol (e.g., `AIUSDC`) |
| `vaultFee` | `number` | Yes | Vault fee in basis points. Note: this field is named `vaultFee` (not `vaultFeeBps`) for this endpoint. |
| `upgradable` | `boolean` | Yes | Whether the vault contract can be upgraded |
| `assets` | `Asset[]` | Yes | Assets to manage |
| `assets[].address` | `string` | Yes | Token contract address |
| `assets[].symbol` | `string` | Yes | Human-readable token symbol |
| `assets[].amount` | `number` | Yes | Total deposit amount for this asset in stroops. Must be ≥ 1001 and equal to the sum of all strategy amounts. |
| `assets[].strategies` | `Strategy[]` | Yes | Strategies for this asset |
| `assets[].strategies[].address` | `string` | Yes | Strategy contract address |
| `assets[].strategies[].name` | `string` | Yes | Strategy name |
| `assets[].strategies[].amount` | `number` | Yes | Amount (in stroops) to invest into this strategy. The sum of all strategy amounts must equal `assets[].amount`. |

> **Strategy amounts must sum to the asset amount.** If `assets[0].amount = 10000000`, then all `strategies[].amount` values for that asset must add up to `10000000`.

### Response

```json
{
  "xdr": "AAAAAgAAAAA...",
  "predictedVaultAddress": "CCQ2BCKKDX7HSF5TULLRFRKS4RYIC5ZZGYYTBR3XFDLZ6MMZFRJNXIEA",
  "warning": "The vault address is predicted from simulation. Actual address may differ if network state changes."
}
```

The `predictedVaultAddress` is derived from simulation and is accurate in most cases, but treat it as advisory until the transaction confirms on-chain.

---

## Complete Testnet Example (TypeScript)

This example deploys a BlendUSDC vault on testnet using the `create-vault-deposit` endpoint.

> **Before running**: Make sure you have BlendUSDC in your wallet. Go to [testnet.blend.capital](https://testnet.blend.capital), connect your Freighter wallet (on Testnet), and use the faucet to receive BlendUSDC.

```typescript
const API_KEY = process.env.DEFINDEX_API_KEY!;
const DEPLOYER_ADDRESS = "GACKTN5D..."; // Your Stellar testnet address

const body = {
  roles: {
    manager: DEPLOYER_ADDRESS,
    emergencyManager: DEPLOYER_ADDRESS,
    rebalanceManager: DEPLOYER_ADDRESS,
    feeReceiver: DEPLOYER_ADDRESS,
  },
  vaultFeeBps: 100, // 1%
  assets: [
    {
      // BlendUSDC on testnet
      address: "CAQCFVLOBK5GIULPNZRGATJJMIZL5BSP7X5YJVMGCPTUEPFM4AVSRCJU",
      strategies: [
        {
          address: "CALLOM5I7XLQPPOPQMYAHUWW4N7O3JKT42KQ4ASEEVBXDJQNJOALFSUY",
          name: "Blend USDC Strategy",
          paused: false,
        },
      ],
    },
  ],
  name: "My USDC Vault",
  symbol: "MUSDC",
  upgradable: true,
  caller: DEPLOYER_ADDRESS,
  depositAmounts: [1001], // Minimum first deposit (1001 stroops)
};

// Step 1: Build the transaction
const res = await fetch(
  "https://api.defindex.io/factory/create-vault-deposit?network=testnet",
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_KEY}`,
    },
    body: JSON.stringify(body),
  }
);
const { xdr } = await res.json();

// Step 2: Sign with your wallet (Freighter / Privy / Crossmint)
const signedXdr = await signTransaction(xdr, { network: "TESTNET" });

// Step 3: Submit
const sendRes = await fetch(
  "https://api.defindex.io/send?network=testnet",
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_KEY}`,
    },
    body: JSON.stringify({ xdr: signedXdr }),
  }
);
const result = await sendRes.json();
console.log("Vault deployed:", result.txHash);
```

---

## Complete Mainnet Example (TypeScript)

```typescript
const body = {
  roles: {
    manager: "GMANAGER...",
    emergencyManager: "GEMERGENCY...",
    rebalanceManager: "GREBALANCE...",
    feeReceiver: "GFEERECEIVER...",
  },
  vaultFeeBps: 50, // 0.5%
  assets: [
    {
      // Real USDC on mainnet (Circle / Stellar SAC)
      address: "CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75",
      strategies: [
        {
          address: "CDB2WMKQQNVZMEBY7Q7GZ5C7E7IAFSNMZ7GGVD6WKTCEWK7XOIAVZSAP",
          name: "Blend USDC Fixed Strategy",
          paused: false,
        },
      ],
    },
  ],
  name: "My USDC Vault",
  symbol: "MUSDC",
  upgradable: true,
  caller: "GDEPLOYER...",
  depositAmounts: [1001], // 0.0001001 USDC — the required minimum
};

const res = await fetch(
  "https://api.defindex.io/factory/create-vault-deposit?network=mainnet",
  {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_KEY}`,
    },
    body: JSON.stringify(body),
  }
);
const { xdr } = await res.json();
// ... sign and send as above
```

---

## After Deployment: First Rebalance

After the vault is deployed and the first deposit is made, call the `rebalance` function to invest funds into your chosen strategies. Without this step, all deposited funds remain **idle** (uninvested) in the vault.

See the [Rebalance section in the main Create a Vault guide](README.md#step-5-first-rebalance) for how to do this via the API or `stellar-cli`.

> If you used `create-vault-auto-invest`, the initial rebalance is already done for you as part of the deployment transaction.

---

## Testnet vs Mainnet Differences at a Glance

| Aspect | Testnet | Mainnet |
|---|---|---|
| USDC | BlendUSDC (`CAQCFV...`) — get from testnet.blend.capital | Circle USDC (`CCW67T...`) |
| CETES | Testnet CETES (`CC72F5...`) — get from testnet.blend.capital | Real CETES token |
| XLM | Testnet XLM — get from Friendbot | Real XLM |
| Factory | `CDSCWE4...` | `CDKFHFJ...` |
| API `network` param | `testnet` | `mainnet` |
| Network passphrase | `Test SDF Network ; September 2015` | `Public Global Stellar Network ; September 2015` |
| RPC URL | `https://soroban-testnet.stellar.org` | Provider-dependent |

---

## Troubleshooting

**`StrategyDoesNotSupportAsset` (error 102)**\
The strategy address you provided does not support the asset token you specified. Double-check that the strategy address matches the asset and the strategy/asset/network relation is correct. For example, the USDC strategy cannot be used with the XLM token address or, the USDC strategy on testnet cannot be used with the USDC token address from mainnet.

**Transaction fails with simulation error on testnet**\
You may not have a BlendUSDC trustline. Visit [testnet.blend.capital](https://testnet.blend.capital), connect your wallet, and add the BlendUSDC asset before trying again.

**`RolesIncomplete` (error 104)**\
All four roles (0–3) must be provided. If you leave any out, the factory will reject the transaction.

**`FeeTooHigh` (factory error 406)**\
The `vaultFeeBps` value exceeds the maximum allowed. Keep it at or below 10,000 (100%).

**`AmountNotAllowed` (error 110)**\
The initial deposit amount is zero or negative. Provide at least 1001 stroops per asset.

For additional errors, see the [Troubleshooting Guide](../troubleshooting.md).
