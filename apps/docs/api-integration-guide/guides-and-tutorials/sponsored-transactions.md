---
description: ⏱️ 4 min read
---

# Sponsored Transactions (Fee Bump)

## Introduction

Sponsored transactions allow a **sponsor account** to pay Stellar transaction fees on behalf of another user. This is essential when working with **smart accounts** (Soroban contract-based accounts) that cannot pay fees themselves. Using the fee-bump pattern, you can build seamless user experiences where end users never need to hold XLM for gas.

This guide walks you through implementing fee-bump deposits and withdrawals with the DeFindex SDK.

## Why Are Sponsored Transactions Needed?

### Smart Accounts and Transaction Fees

On Stellar, there are two types of accounts:

* **Native accounts** (`G...` addresses) — Standard Stellar accounts that hold XLM and can pay transaction fees.
* **Smart accounts** (`C...` addresses) — Soroban contract-based accounts. These accounts **cannot pay transaction fees** because Stellar requires fees to be paid in XLM by a native `G...` account.

When a DeFindex vault is operated by a smart account, the transaction will fail if no one covers the fee.

### The Fee-Bump Solution

Stellar's **fee-bump transaction** wraps an existing (inner) transaction with an outer envelope that specifies a different fee-paying account:

```
┌─────────────────────────────────┐
│  Fee-Bump Transaction (outer)   │
│  Fee Source: Sponsor (G...)     │
│                                 │
│  ┌───────────────────────────┐  │
│  │  Inner Transaction        │  │
│  │  Source: Caller (C.../G.) │  │
│  │  Operations: deposit/     │  │
│  │    withdraw, etc.         │  │
│  │  Signed by: Caller        │  │
│  └───────────────────────────┘  │
│                                 │
│  Signed by: Sponsor             │
└─────────────────────────────────┘
```

The **caller** signs the inner transaction to authorize the operation. The **sponsor** wraps it in a fee-bump and signs the outer transaction to pay the fee. The network processes both as a single unit.

## Prerequisites

* **`@defindex/sdk`** `^0.1.1`
* **`@stellar/stellar-sdk`** `^14.3.0`
* **Two Stellar keypairs:**
  * **Sponsor** — A native `G...` account funded with XLM to pay fees
  * **Caller** — The account executing vault operations (can be `G...` or `C...`)
* **DeFindex API key** (contact the team on [Discord](https://discord.gg/ftPKMPm38f))

### Environment Configuration

Create a `.env` file based on the following template:

```bash
# Network: testnet or mainnet
NETWORK=testnet

# Sponsor keypair secret (pays transaction fees)
SPONSOR_SECRET=SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

# Caller keypair secret (executes the transaction)
CALLER_SECRET=SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

# Defindex API credentials
DEFINDEX_API_KEY=your_api_key_here
DEFINDEX_API_URL=https://api.defindex.io

# Vault contract address on Stellar
VAULT_ADDRESS=CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

## Deposit with Fee Bump

### Step 1: Initialize SDK and Keypairs

```typescript
import {
  DefindexSDK,
  DepositToVaultParams,
  SupportedNetworks,
} from "@defindex/sdk";
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
const supportedNetwork = isMainnet
  ? SupportedNetworks.MAINNET
  : SupportedNetworks.TESTNET;

const sponsorKeypair = Keypair.fromSecret(process.env.SPONSOR_SECRET as string);
const callerKeypair = Keypair.fromSecret(process.env.CALLER_SECRET as string);

const defindexSdk = new DefindexSDK({
  apiKey: process.env.DEFINDEX_API_KEY as string,
  baseUrl: process.env.DEFINDEX_API_URL as string,
});
```

### Step 2: Get Unsigned Deposit Transaction

```typescript
const vaultAddress = process.env.VAULT_ADDRESS as string;

const depositData: DepositToVaultParams = {
  amounts: [10],                      // Amount in stroops
  invest: true,                        // Auto-invest into strategies
  caller: callerKeypair.publicKey(),
};

const depositResponse = await defindexSdk.depositToVault(
  vaultAddress,
  depositData,
  supportedNetwork
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

```typescript
const feeBumpXdr = feeBumpTx.toXDR();
const response = await defindexSdk.sendTransaction(feeBumpXdr, supportedNetwork);

console.log("Transaction hash:", response.txHash);
```

## Withdraw with Fee Bump

The withdrawal flow is the same pattern, but first queries the user's vault balance to determine how many shares to withdraw.

### Step 1: Get Vault Balance

```typescript
import { WithdrawSharesParams } from "@defindex/sdk";

const balanceResponse = await defindexSdk.getVaultBalance(
  vaultAddress,
  callerKeypair.publicKey(),
  supportedNetwork
);

const sharesAvailable = (balanceResponse.dfTokens as unknown as any[])[0];
const sharesToWithdraw = parseInt(String(sharesAvailable), 10);
```

### Step 2: Get Unsigned Withdrawal Transaction

```typescript
const withdrawData: WithdrawSharesParams = {
  shares: sharesToWithdraw,
  caller: callerKeypair.publicKey(),
};

const withdrawResponse = await defindexSdk.withdrawShares(
  vaultAddress,
  withdrawData,
  supportedNetwork
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
const response = await defindexSdk.sendTransaction(
  feeBumpTx.toXDR(),
  supportedNetwork
);
console.log("Transaction hash:", response.txHash);
```

## Fee Considerations

* The fee-bump transaction fee must be **greater than or equal to** the inner transaction fee. If you set it lower, the network will reject the transaction.
* The DeFindex API builds the inner transaction with a simulated resource fee and a correct inclusion fee. Passing `parseInt(transaction.fee)` as the fee-bump fee ensures you match the required minimum.
* To **increase priority** during network congestion, pass a higher fee value:

```typescript
const feeBumpTx = TransactionBuilder.buildFeeBumpTransaction(
  sponsorKeypair,
  (innerTxFee * 2).toString(), // Double the fee for higher priority
  transaction,
  stellarNetwork
);
```

## Rate Limiting

The DeFindex API may return **429 (Too Many Requests)** responses. Use exponential backoff to handle this:

```typescript
async function withRateLimit<T>(
  fn: () => Promise<T>,
  maxRetries: number = 5,
  initialDelay: number = 1000
): Promise<T> {
  let lastError: any;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error: any) {
      lastError = error;

      if (error?.statusCode === 429 || error?.error === "Too Many Requests") {
        const retryAfter = error?.retryAfter || 1;
        const delayMs = Math.max(
          retryAfter * 1000,
          initialDelay * Math.pow(2, attempt)
        );

        if (attempt < maxRetries) {
          await new Promise((resolve) => setTimeout(resolve, delayMs));
          continue;
        }
      }

      throw error;
    }
  }

  throw lastError;
}
```

Then wrap your SDK calls:

```typescript
const depositResponse = await withRateLimit(() =>
  defindexSdk.depositToVault(vaultAddress, depositData, supportedNetwork)
);
```

## Common Issues

| Issue | Cause | Solution |
|---|---|---|
| `tx_insufficient_fee` | Fee-bump fee is lower than inner transaction fee | Use `parseInt(transaction.fee)` or a higher value |
| `tx_bad_auth` | Inner transaction not signed by caller | Ensure `transaction.sign(callerKeypair)` is called before building the fee-bump |
| Wrong signing order | Fee-bump created before signing inner tx | Always sign the inner transaction first, then build the fee-bump |
| `429 Too Many Requests` | API rate limit exceeded | Implement exponential backoff (see Rate Limiting section) |
| `tx_bad_seq` | Stale sequence number | Re-fetch the unsigned transaction from the API and retry |

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

* [Stellar Fee-Bump Transactions](https://developers.stellar.org/docs/learn/encyclopedia/transactions/fee-bump)
* [DeFindex SDK (TypeScript)](../../advanced-documentation/sdks/02-defindex-sdk.md)
* [Deposit](../smart-contracts/deposit.md)
* [Withdraw](../smart-contracts/withdraw.md)
* [DeFindex API Documentation](https://api.defindex.io/docs)
