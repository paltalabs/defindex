---
description: ⏱️ 4 min read
---

# Sponsored Transactions (Fee Bump)

## Introduction

Sponsored transactions allow a **sponsor account** to pay Stellar transaction fees on behalf of another user. This is essential when working with **smart accounts** (Soroban contract-based accounts) that cannot pay fees themselves. Using the fee-bump pattern, you can build seamless user experiences where end users never need to hold XLM for gas.

This guide walks you through implementing fee-bump deposits and withdrawals with the DeFindex API.

## Why Are Sponsored Transactions Needed?

### Smart Accounts and Transaction Fees

On Stellar, there are two types of accounts:

* **Native accounts** (`G...` addresses) — Standard Stellar accounts that hold XLM and can pay transaction fees. They also serve as the **source account** (sequence number provider) for transactions.
* **Smart accounts** (`C...` addresses) — Soroban contract-based accounts. These accounts **cannot be the source account** of a transaction and **cannot pay transaction fees** because Stellar requires fees and sequence numbers from a native `G...` account.

> [!IMPORTANT]
> **Source** = always a native `G...` account (provides the sequence number). In a fee-bump transaction, the **sponsor** pays the fee, not the source account.
> **Caller (from)** = can be `G...` or `C...` (the account that authorizes the vault operation)

When a DeFindex vault is operated by a smart account, the transaction will fail if no one covers the fee.

### The Fee-Bump Solution

Stellar's **fee-bump transaction** wraps an existing (inner) transaction with an outer envelope that specifies a different fee-paying account:

```
┌──────────────────────────────────────┐
│  Fee-Bump Transaction (outer)        │
│  Fee Source: Sponsor (G...)          │
│                                      │
│  ┌──────────────────────────────┐    │
│  │  Inner Transaction           │    │
│  │  Source: Native account (G..)│    │
│  │  Caller/From: C... or G...   │    │
│  │  Operations: deposit/        │    │
│  │    withdraw, etc.            │    │
│  │  Signed by: Caller           │    │
│  └──────────────────────────────┘    │
│                                      │
│  Signed by: Sponsor                  │
└──────────────────────────────────────┘
```

The inner transaction's **source account** is always a native `G...` account (which provides the sequence number). The **caller** signs the inner transaction to authorize the vault operation. The **sponsor** wraps it in a fee-bump and signs the outer transaction to pay the fee. The network processes both as a single unit.

## Prerequisites

* **`@stellar/stellar-sdk`** `^14.3.0`
* **Two Stellar keypairs:**
  * **Sponsor** — A native `G...` account funded with XLM to pay fees
  * **Caller** — The account executing vault operations (can be `G...` or `C...`)
* **DeFindex API key** (get it at the  [API Dashboard](https://api.defindex.io/login))

### Environment Configuration

Create a `.env` file based on the following template:

```bash
# Network: testnet or mainnet
NETWORK=testnet

# Sponsor keypair secret (pays transaction fees)
SPONSOR_SECRET=SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

# Caller secret (signs the inner transaction; see note below)
CALLER_SECRET=SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

# Defindex API credentials
DEFINDEX_API_KEY=your_api_key_here
DEFINDEX_API_URL=https://api.defindex.io

# Vault contract address on Stellar
VAULT_ADDRESS=CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

> **Note on `CALLER_SECRET` and smart accounts:**
> `CALLER_SECRET` expects a Stellar secret key (`S...`) for native `G...` accounts. If the caller is a **smart account** (`C...`), authorization comes from wallet interactions (Freighter, xBull, etc.), not a raw private key. In that scenario, present the unsigned XDR to the user's wallet for signing instead of using `Keypair.fromSecret(...)`.

## Deposit with Fee Bump

### Step 1: Initialize API Client and Keypairs

```typescript
import {
  Keypair,
  Networks,
  Transaction,
  TransactionBuilder,
} from "@stellar/stellar-sdk";
import { config } from "dotenv";

config();

const network = process.env.NETWORK?.toLowerCase() || "testnet";
const isMainnet = network === "mainnet";
const stellarNetwork = isMainnet ? Networks.PUBLIC : Networks.TESTNET;

const API_BASE = process.env.DEFINDEX_API_URL as string;
const API_KEY = process.env.DEFINDEX_API_KEY as string;

const sponsorKeypair = Keypair.fromSecret(process.env.SPONSOR_SECRET as string);
// Assumes caller is a native G... account. For smart accounts (C...), use wallet signing instead.
const callerKeypair = Keypair.fromSecret(process.env.CALLER_SECRET as string);

// Lightweight helper for DeFindex API calls. Alternatively, you can use the @defindex/sdk package.
async function api<T>(path: string, options?: { method?: string; body?: unknown }): Promise<T> {
  const separator = path.includes("?") ? "&" : "?";
  const url = `${API_BASE}${path}${separator}network=${network}`;

  const res = await fetch(url, {
    method: options?.method ?? "GET",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${API_KEY}`,
    },
    ...(options?.body ? { body: JSON.stringify(options.body) } : {}),
  });

  if (!res.ok) {
    const errorBody = await res.text();
    throw new Error(`API ${res.status}: ${errorBody}`);
  }

  return res.json() as Promise<T>;
}
```

### Step 2: Get Unsigned Deposit Transaction

> **API Reference:** [`POST /vault/{address}/deposit`](https://api.defindex.io/docs#tag/Vault/operation/VaultController_deposit) — `DepositDto`

```typescript
const vaultAddress = process.env.VAULT_ADDRESS as string;

const depositResponse = await api<{ xdr: string }>(
  `/vault/${vaultAddress}/deposit`,
  {
    method: "POST",
    body: {
      amounts: [10000000],               // Amount in stroops
      invest: true,                      // Auto-invest into strategies
      caller: callerKeypair.publicKey(),
    },
  }
);
```

### Step 3: Sign Inner Transaction with Caller

```typescript
const transaction = TransactionBuilder.fromXDR(
  depositResponse.xdr,
  stellarNetwork
) as Transaction;

transaction.sign(callerKeypair);
```

### Step 4: Create and Sign Fee-Bump with Sponsor

```typescript
const innerTxFee = parseInt(transaction.fee);

const feeBumpTx = TransactionBuilder.buildFeeBumpTransaction(
  sponsorKeypair,
  innerTxFee.toString(),
  transaction,
  stellarNetwork
);

feeBumpTx.sign(sponsorKeypair);
```

### Step 5: Submit the Transaction

> **API Reference:** [`POST /send`](https://api.defindex.io/docs) — `SendXdrDto`

```typescript
const feeBumpXdr = feeBumpTx.toXDR();
const response = await api<{ txHash: string }>(
  "/send",
  { method: "POST", body: { xdr: feeBumpXdr } }
);

console.log("Transaction hash:", response.txHash);
```

## Withdraw with Fee Bump

The withdrawal flow is the same pattern, but first queries the user's vault balance to determine how much of the underlying assets to withdraw.

### Step 1: Get Vault Balance

> **API Reference:** [`GET /vault/{address}/balance`](https://api.defindex.io/docs#tag/Vault/operation/VaultController_getVaultBalance)

```typescript
const balanceResponse = await api<{ balances: number[] }>(
  `/vault/${vaultAddress}/balance?from=${callerKeypair.publicKey()}`
);

const amountsToWithdraw = balanceResponse.balances;
```

### Step 2: Get Unsigned Withdrawal Transaction

> **API Reference:** [`POST /vault/{address}/withdraw`](https://api.defindex.io/docs#tag/Vault/operation/VaultController_withdraw) — `WithdrawDto`

```typescript
const withdrawResponse = await api<{ xdr: string }>(
  `/vault/${vaultAddress}/withdraw`,
  {
    method: "POST",
    body: {
      amounts: amountsToWithdraw,
      caller: callerKeypair.publicKey(),
      slippageBps: 100,                  // Optional: max slippage in basis points (100 = 1%)
    },
  }
);
```

### Step 3: Sign, Wrap, and Submit

The signing and fee-bump steps are identical to the deposit flow:

```typescript
// Sign inner transaction with caller
const transaction = TransactionBuilder.fromXDR(
  withdrawResponse.xdr,
  stellarNetwork
) as Transaction;
transaction.sign(callerKeypair);

// Create fee-bump with sponsor
const innerTxFee = parseInt(transaction.fee);
const feeBumpTx = TransactionBuilder.buildFeeBumpTransaction(
  sponsorKeypair,
  innerTxFee.toString(),
  transaction,
  stellarNetwork
);
feeBumpTx.sign(sponsorKeypair);

// Submit
const response = await api<{ txHash: string }>(
  "/send",
  { method: "POST", body: { xdr: feeBumpTx.toXDR() } }
);
console.log("Transaction hash:", response.txHash);
```

## Fee Considerations

* `buildFeeBumpTransaction` takes a **per-operation base fee**, not a total fee. The SDK internally multiplies by `(numOperations + 1)` to compute the total fee-bump fee ([CAP-0015 rule](https://github.com/stellar/stellar-protocol/blob/master/core/cap-0015.md)).
* For a typical Soroban transaction with 1 operation, passing `parseInt(transaction.fee)` as the base fee produces a **total** fee-bump fee of `2 × innerFee` (since `1 op + 1 = 2`). This satisfies the protocol's fee-rate check because the per-operation rate equals the inner transaction's rate.
* The DeFindex API builds the inner transaction with a simulated resource fee and a correct inclusion fee. Passing `parseInt(transaction.fee)` as the base fee ensures you meet the required minimum.
* To **increase priority** during network congestion, pass a higher base fee:

```typescript
const feeBumpTx = TransactionBuilder.buildFeeBumpTransaction(
  sponsorKeypair,
  (innerTxFee * 2).toString(), // 2× base fee → 4× total fee (for a 1-op tx) for higher priority
  transaction,
  stellarNetwork
);
```

## Common Issues

| Issue | Cause | Solution |
|---|---|---|
| `tx_insufficient_fee` | Fee-bump per-operation fee rate is lower than the inner transaction's rate | Ensure the base fee passed to `buildFeeBumpTransaction` is ≥ `parseInt(transaction.fee)`. The SDK handles the `(numOps + 1)` multiplication internally |
| `tx_bad_auth` | Inner transaction not signed by caller | Ensure `transaction.sign(callerKeypair)` is called before building the fee-bump |
| Wrong signing order | Fee-bump created before signing inner tx | Always sign the inner transaction first, then build the fee-bump |
| `429 Too Many Requests` | API rate limit exceeded | Implement exponential backoff (see [Troubleshooting](../troubleshooting.md#rate-limiting)) |
| `tx_bad_seq` | Stale sequence number | Re-fetch the unsigned transaction from the API and retry. Note that Soroban resource estimates (footprint, CPU/memory limits) are also ledger-specific, so simply adjusting the sequence number on a stale transaction is not enough — you must request a fresh XDR |
| `tx_too_late` | Transaction timebounds expired before submission | The XDR was held too long. Re-fetch a fresh unsigned transaction from the API and sign/submit promptly |

## Production Notes

* **Channel accounts for concurrent sponsors:** If the sponsor account submits multiple fee-bump transactions in parallel, sequence-number collisions will cause failures. Use [channel accounts](https://developers.stellar.org/docs/encyclopedia/channel-accounts). a pool of funded `G...` accounts — so each concurrent submission uses its own sequence number.
* **XDR verification before signing:** In production, the sponsor should decode and inspect the inner transaction XDR before signing. Verify that the operations, amounts, and destination contracts match expected values. Never blindly sign arbitrary XDRs.

## Complete Example Repository

A fully working example with deposit and withdraw scripts is available at:

[**paltalabs/examples/defindex-sdk-deposit-withdraw-fee-bump**](https://github.com/paltalabs/examples/tree/main/defindex-sdk-deposit-withdraw-fee-bump)

To run it:

```bash
git clone https://github.com/paltalabs/examples.git
cd examples/defindex-sdk-deposit-withdraw-fee-bump
npm install
cp .env.example .env
# Edit .env with your keys and vault address

npm run deposit    # Run deposit example
npm run withdraw   # Run withdraw example
```

## Additional Resources

* [Stellar Fee-Bump Transactions](https://developers.stellar.org/docs/build/guides/transactions/fee-bump-transactions)
* [Deposit](../smart-contracts/deposit.md)
* [Withdraw](../smart-contracts/withdraw.md)
* [DeFindex API Documentation](https://api.defindex.io/docs)
