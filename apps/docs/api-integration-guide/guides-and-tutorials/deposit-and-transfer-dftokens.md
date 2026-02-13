---
description: ⏱️ 7 min read
---

# Mint & Transfer dfTokens

## 📖 What You'll Learn

In this tutorial you'll build a TypeScript script that:

* **Deposits** assets into a DeFindex vault and receives dfTokens in return
* **Transfers** those dfTokens to another Stellar wallet

By the end, you'll have a working script that automates this entire flow — useful for vault managers who want to run **raffles**, **airdrops**, or distribute **incentives** to their users.

## 💡 Why Would I Want to Transfer dfTokens?

When you deposit into a DeFindex vault, the vault mints **dfTokens** — tokens that represent your proportional share of the vault's assets. These tokens:

* Increase in value as the vault earns yield
* Can be redeemed later for the underlying assets + profits
* **Are transferable**, just like any other Stellar token

This means a vault manager can deposit funds, receive dfTokens, and then **distribute them to other wallets**. The recipients now hold vault shares without needing to deposit themselves. Some practical use cases:

| Use Case | Description |
|---|---|
| 🎁 Raffles & Giveaways | Deposit and distribute dfTokens as prizes |
| 🏆 User Incentives | Reward active users with yield-bearing tokens |
| 💼 Team Distribution | Split vault ownership across team members |
| 🔄 OTC Transfers | Move vault positions between wallets |

## 🎯 Prerequisites

Before starting, make sure you have:

* **Node.js** (v18+) and a package manager (`pnpm`, `npm`, or `yarn`)
* A **Stellar wallet** with enough balance for the deposit + transaction fees
* A **DeFindex API key** (register at [api.defindex.io](https://api.defindex.io/register) or contact us on [Discord](https://discord.gg/ftPKMPm38f))
* Basic knowledge of **TypeScript** and the **Stellar network**

## 🧠 Key Concepts

Before diving into code, let's understand the two core operations we'll perform.

### Deposit → Mint dfTokens

When you call `deposit` on a DeFindex vault, the contract:

1. Transfers your assets (e.g., XLM) from your wallet into the vault
2. Calculates how many shares you should receive, proportional to your contribution
3. **Mints dfTokens** to your wallet representing those shares
4. Optionally invests the deposited funds into the vault's strategies

The vault contract's deposit function signature looks like this:

```rust
fn deposit(
    e: Env,
    amounts_desired: Vec<i128>,
    amounts_min: Vec<i128>,
    from: Address,
    invest: bool,
) -> Result<(Vec<i128>, i128, Option<Vec<Option<AssetInvestmentAllocation>>>), ContractError>
```

It returns a tuple with: the actual deposited amounts, the **number of dfTokens minted**, and the investment allocations.

### Transfer dfTokens

Here's something important: **the vault contract itself is the dfToken contract**. The vault implements the standard Stellar token interface, which means you can call `transfer` directly on the vault contract to move dfTokens between wallets:

```rust
fn transfer(e: Env, from: Address, to: Address, amount: i128)
```

No separate token contract needed — just call `transfer` on the vault address with the sender, receiver, and amount.

## 🏗️ Step-by-Step Implementation

### Step 1: Project Setup

Create a new project and install the required dependencies:

```bash
mkdir defindex-deposit-transfer
cd defindex-deposit-transfer
pnpm init
pnpm add @defindex/sdk @stellar/stellar-sdk dotenv
pnpm add -D typescript tsx @types/node
```

Add a run script to your `package.json`:

```json
{
  "scripts": {
    "start": "tsx src/index.ts"
  }
}
```

Create a `tsconfig.json`:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "module": "CommonJS",
    "rootDir": "src",
    "outDir": "dist",
    "strict": true,
    "esModuleInterop": true,
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "skipLibCheck": true
  },
  "include": ["src/**/*.ts"],
  "exclude": ["node_modules"]
}
```

Create a `.env` file with your configuration:

```bash
DEFINDEX_API_KEY=sk_your_api_key_here
STELLAR_SECRET_KEY=S_your_wallet_secret_key
RECEIVER_ADDRESS=G_receiver_wallet_public_key
SOROBAN_RPC=your testnet soroban rpc url (e.g., https://soroban-testnet.stellar.org)
```

> ⚠️ **Never commit your `.env` file to version control.** Add it to your `.gitignore`.
>
> **Note:** This tutorial uses **testnet** for learning purposes. For production, change `SOROBAN_RPC` to a mainnet endpoint (e.g., `https://soroban-rpc.mainnet.stellar.gateway.fm`) and update the network references in the code to `Networks.PUBLIC` / `SupportedNetworks.MAINNET`.

### Step 2: Load Configuration

Create `src/index.ts` and start by importing dependencies and loading environment variables:

```typescript
import { DefindexSDK, SupportedNetworks } from '@defindex/sdk';
import * as StellarSdk from '@stellar/stellar-sdk';
import {
  rpc,
  Keypair,
  Networks,
  TransactionBuilder,
  Contract,
  Address,
  BASE_FEE,
  xdr,
} from '@stellar/stellar-sdk';
import { config } from 'dotenv';

config();
```

Define your constants. You'll need to update `VAULT_ADDRESS` to match the vault you want to deposit into:

```typescript
// ─── Constants ───────────────────────────────────────────────
const VAULT_ADDRESS = 'YOUR_VAULT_ADDRESS_HERE'; // ← Replace with your vault
const DECIMALS = 7;
const DEPOSIT_AMOUNT = 10;  // Human-readable amount
const DEPOSIT_AMOUNT_RAW = DEPOSIT_AMOUNT * 10 ** DECIMALS; // In stroops
```

> **About decimals:** Stellar uses 7 decimal places for most assets. So `10 XLM` = `100,000,000` stroops. Always convert to raw amounts before calling contract functions.

Now load and validate the environment variables:

```typescript
// ─── Environment ─────────────────────────────────────────────
interface EnvConfig {
  apiKey: string;
  secretKey: string;
  receiverAddress: string;
  sorobanRpc: string;
}

function loadEnv(): EnvConfig {
  const apiKey = process.env.DEFINDEX_API_KEY;
  const secretKey = process.env.STELLAR_SECRET_KEY;
  const receiverAddress = process.env.RECEIVER_ADDRESS;
  const sorobanRpc = process.env.SOROBAN_RPC;

  const missing: string[] = [];
  if (!apiKey) missing.push('DEFINDEX_API_KEY');
  if (!secretKey) missing.push('STELLAR_SECRET_KEY');
  if (!receiverAddress) missing.push('RECEIVER_ADDRESS');
  if (!sorobanRpc) missing.push('SOROBAN_RPC');

  if (missing.length > 0) {
    console.error(`Missing environment variables: ${missing.join(', ')}`);
    process.exit(1);
  }

  return {
    apiKey: apiKey!,
    secretKey: secretKey!,
    receiverAddress: receiverAddress!,
    sorobanRpc: sorobanRpc!,
  };
}
```

### Step 3: Transaction Helper

Both deposit and transfer need to submit a transaction and wait for on-chain confirmation. Let's create a reusable helper:

```typescript
// ─── Transaction Helpers ─────────────────────────────────────
interface TxResult {
  txHash: string;
  returnValue?: xdr.ScVal;
}

async function sendAndConfirm(
  rpcServer: rpc.Server,
  transaction: StellarSdk.Transaction
): Promise<TxResult> {
  const response = await rpcServer.sendTransaction(transaction);

  if (response.status !== 'PENDING') {
    const errorXdr = response.errorResult?.toXDR('base64');
    if (errorXdr) {
      const errorName = xdr.TransactionResult.fromXDR(errorXdr, 'base64')
        .result()
        .switch().name;
      throw new Error(`Transaction rejected: ${errorName}`);
    }
    throw new Error(`Transaction rejected with status: ${response.status}`);
  }

  const txHash = response.hash;
  console.log(`  Submitted: ${txHash}`);
  console.log('  Waiting for confirmation...');

  // Poll until confirmed or failed
  while (true) {
    await new Promise((resolve) => setTimeout(resolve, 2000));
    const txResponse = await rpcServer.getTransaction(txHash);

    if (txResponse.status === 'SUCCESS') {
      const successResponse =
        txResponse as rpc.Api.GetSuccessfulTransactionResponse;
      return { txHash, returnValue: successResponse.returnValue };
    }

    if (txResponse.status === 'FAILED') {
      throw new Error(`Transaction failed on-chain: ${txHash}`);
    }
  }
}
```

**🔍 What this function does:**

1. **Submits** the signed transaction to the Soroban RPC
2. **Validates** the transaction was accepted (status `PENDING`)
3. **Polls** every 2 seconds until the transaction is confirmed or fails
4. **Returns** the transaction hash and the on-chain return value

### Step 4: The Deposit Function 💰

This function uses the DeFindex SDK to build a deposit transaction, signs it locally, and submits it. The key part is **extracting the minted dfTokens** from the return value:

```typescript
// ─── Deposit ─────────────────────────────────────────────────
async function deposit(
  sdk: DefindexSDK,
  rpcServer: rpc.Server,
  keypair: Keypair
): Promise<{ txHash: string; dfTokensMinted: bigint }> {
  const caller = keypair.publicKey();

  console.log('  Building deposit via SDK...');
  const depositResponse = await sdk.depositToVault(
    VAULT_ADDRESS,
    { amounts: [DEPOSIT_AMOUNT_RAW], caller, invest: true },
    SupportedNetworks.TESTNET
  );

  console.log('  Signing...');
  const tx = TransactionBuilder.fromXDR(
    depositResponse.xdr,
    Networks.TESTNET
  ) as StellarSdk.Transaction;
  tx.sign(keypair);

  console.log('  Sending deposit...');
  const { txHash, returnValue } = await sendAndConfirm(rpcServer, tx);

  if (!returnValue) {
    throw new Error('Deposit transaction returned no value');
  }

  // The deposit function returns a tuple:
  //   Index 0: Vec<i128> → actual amounts deposited
  //   Index 1: i128      → dfTokens minted  ← this is what we need
  //   Index 2: Option     → investment allocations
  const nativeResult = StellarSdk.scValToNative(returnValue) as unknown[];
  const dfTokensMinted = BigInt(nativeResult[1] as string | number | bigint);

  return { txHash, dfTokensMinted };
}
```

**🔍 What this function does:**

1. **Builds** an unsigned deposit transaction using the DeFindex SDK (`depositToVault`)
2. **Signs** the transaction with your keypair
3. **Submits** it to the network and waits for confirmation
4. **Parses** the return value to extract how many dfTokens were minted

> **Why `invest: true`?** When set to `true`, the vault automatically allocates your deposited funds into its yield-generating strategies. Set to `false` if you want the funds to remain idle in the vault.

### Step 5: The Transfer Function 🔄

Now the interesting part — transferring dfTokens to another wallet. Since the vault contract implements the token interface, we call `transfer` directly on the vault contract address:

```typescript
// ─── Transfer dfTokens ──────────────────────────────────────
async function transferDfTokens(
  rpcServer: rpc.Server,
  keypair: Keypair,
  toAddress: string,
  amount: bigint
): Promise<string> {
  const from = keypair.publicKey();
  const contract = new Contract(VAULT_ADDRESS);

  // Build the transfer operation on the vault contract
  const operation = contract.call(
    'transfer',
    new Address(from).toScVal(),
    new Address(toAddress).toScVal(),
    StellarSdk.nativeToScVal(amount, { type: 'i128' })
  );

  // Create the transaction
  const account = await rpcServer.getAccount(from);
  const tx = new TransactionBuilder(account, {
    fee: BASE_FEE,
    networkPassphrase: Networks.TESTNET,
  })
    .addOperation(operation)
    .setTimeout(30)
    .build();

  // Simulate to estimate resources
  console.log('  Simulating transfer...');
  const simulation = await rpcServer.simulateTransaction(tx);

  if (rpc.Api.isSimulationError(simulation)) {
    throw new Error(`Transfer simulation failed: ${simulation.error}`);
  }

  // Assemble with simulation results, sign, and send
  const preparedTx = rpc.assembleTransaction(tx, simulation).build();
  preparedTx.sign(keypair);

  console.log('  Sending transfer...');
  const { txHash } = await sendAndConfirm(rpcServer, preparedTx);

  return txHash;
}
```

**🔍 What this function does:**

1. **Creates** a `Contract` instance pointing to the vault address
2. **Builds** a `transfer(from, to, amount)` call — this is the standard Soroban token transfer
3. **Simulates** the transaction to estimate the required resources (CPU, memory, ledger I/O)
4. **Assembles** the final transaction with the simulation results
5. **Signs** and **submits** it to the network

> **Why simulate first?** Soroban smart contract calls require accurate resource estimation. The simulation step calculates CPU instructions, memory bytes, and ledger operations needed, so the transaction has the right resource limits to succeed on-chain.

### Step 6: Put It All Together 🚀

Finally, wire everything up in a `main` function:

```typescript
// ─── Main ────────────────────────────────────────────────────
async function main(): Promise<void> {
  const { apiKey, secretKey, receiverAddress, sorobanRpc } = loadEnv();

  const keypair = Keypair.fromSecret(secretKey);
  const walletA = keypair.publicKey();
  const walletB = receiverAddress;
  const rpcServer = new rpc.Server(sorobanRpc);
  const sdk = new DefindexSDK({ apiKey });

  console.log('='.repeat(60));
  console.log('DeFindex: Deposit & Transfer dfTokens');
  console.log('='.repeat(60));
  console.log(`  Wallet A (depositor): ${walletA}`);
  console.log(`  Wallet B (receiver):  ${walletB}`);
  console.log(`  Vault:                ${VAULT_ADDRESS}`);
  console.log(`  Deposit:              ${DEPOSIT_AMOUNT} XLM`);
  console.log('='.repeat(60));
  console.log('');

  // Step 1: Deposit into vault → receive dfTokens
  console.log('[Step 1] Depositing into vault...');
  const { txHash: depositTxHash, dfTokensMinted } = await deposit(
    sdk,
    rpcServer,
    keypair
  );
  console.log(`  ✅ Deposit confirmed: ${depositTxHash}`);
  console.log(`  dfTokens minted: ${dfTokensMinted}`);
  console.log('');

  // Step 2: Transfer dfTokens to Wallet B
  console.log(`[Step 2] Transferring ${dfTokensMinted} dfTokens to Wallet B...`);
  const transferTxHash = await transferDfTokens(
    rpcServer,
    keypair,
    walletB,
    dfTokensMinted
  );
  console.log(`  ✅ Transfer confirmed: ${transferTxHash}`);
  console.log('');

  // Summary
  console.log('='.repeat(60));
  console.log('DONE');
  console.log('='.repeat(60));
  console.log(`  Deposited:      ${DEPOSIT_AMOUNT} XLM`);
  console.log(`  dfTokens:       ${dfTokensMinted}`);
  console.log(`  Transferred to: ${walletB}`);
  console.log(`  Deposit TX:     ${depositTxHash}`);
  console.log(`  Transfer TX:    ${transferTxHash}`);
  console.log('='.repeat(60));
}

main().catch((error: unknown) => {
  console.error('Fatal error:', error);
  process.exit(1);
});
```

### Run It

```bash
pnpm start
```

You should see output similar to:

```text
============================================================
DeFindex: Deposit & Transfer dfTokens
============================================================
  Wallet A (depositor): GABC...
  Wallet B (receiver):  GXYZ...
  Vault:                CVAULT...
  Deposit:              10 XLM
============================================================

[Step 1] Depositing into vault...
  Building deposit via SDK...
  Signing...
  Sending deposit...
  Submitted: abc123...
  Waiting for confirmation...
  ✅ Deposit confirmed: abc123...
  dfTokens minted: 19850000

[Step 2] Transferring 19850000 dfTokens to Wallet B...
  Simulating transfer...
  Sending transfer...
  Submitted: def456...
  ✅ Transfer confirmed: def456...

============================================================
DONE
============================================================
```

## 🚨 Troubleshooting

| Problem | Solution |
|---|---|
| `Missing environment variables` | Check that your `.env` file has all four variables filled in |
| `Transaction rejected` | Verify Wallet A has enough asset balance and XLM for fees |
| `Transfer simulation failed` | Confirm the vault address is correct and Wallet A holds dfTokens |
| `Deposit returned no value` | May be a network issue — check your Soroban RPC endpoint |
| `Transaction failed on-chain` | Check the transaction on [Stellar Expert](https://stellar.expert) for details |

## 🎓 Next Steps

* **Withdraw** — Redeem dfTokens back for the underlying assets: [Withdraw Guide](../smart-contracts/withdraw.md)
* **Check Balances** — Query dfToken holdings: [Get Balance](../smart-contracts/get-balance.md)
* **Monitor APY** — Track vault performance: [Get APY](../smart-contracts/get-apy.md)
* **Explore the SDK** — More features available in the [TypeScript SDK](../../advanced-documentation/sdks/02-defindex-sdk.md)

---

## 🤖 Build It with AI

Want to build this script quickly? Copy the prompt below and paste it into a new Claude Code session. It contains all the technical details needed to generate the complete implementation:

````text
Build a TypeScript script that deposits assets into a DeFindex vault and transfers the minted
dfTokens to another wallet on the Stellar testnet.

## Technical Context

- DeFindex vaults are Soroban smart contracts on Stellar that accept asset deposits and mint
  dfTokens (vault share tokens) in return
- The vault contract itself implements the Soroban token interface, so dfTokens can be
  transferred by calling `transfer` directly on the vault contract address
- The deposit function returns a tuple where index [1] is the number of dfTokens minted (i128)

## Required Dependencies

```json
{
  "@defindex/sdk": "^0.1.2",
  "@stellar/stellar-sdk": "^14.5.0",
  "dotenv": "^17.2.4"
}
```

Dev dependencies: `typescript`, `tsx`, `@types/node`

## Environment Variables (.env)

```
DEFINDEX_API_KEY=     # DeFindex API key (sk_...)
STELLAR_SECRET_KEY=   # Wallet A secret key (depositor/signer)
RECEIVER_ADDRESS=     # Wallet B public key (receives dfTokens)
SOROBAN_RPC="your mainnet soroban rpc url",
```

## Implementation Requirements

Create `src/index.ts` with the following structure:

### 1. Constants
- `VAULT_ADDRESS`: The target vault contract address (user must configure)
- `DECIMALS = 7` (Stellar standard)
- `DEPOSIT_AMOUNT`: Human-readable amount, converted to raw (amount * 10^7)

### 2. `loadEnv()` function
- Load and validate all four environment variables
- Exit with error if any are missing
- Return typed config object

### 3. `sendAndConfirm(rpcServer, transaction)` helper
- Send transaction to Soroban RPC via `rpcServer.sendTransaction()`
- Check status is 'PENDING', otherwise parse error from `response.errorResult`
- Poll `rpcServer.getTransaction(hash)` every 2 seconds until SUCCESS or FAILED
- On SUCCESS, return `{ txHash, returnValue }` from `GetSuccessfulTransactionResponse`

### 4. `deposit(sdk, rpcServer, keypair)` function
- Use `sdk.depositToVault(VAULT_ADDRESS, { amounts: [RAW_AMOUNT], caller, invest: true },
  SupportedNetworks.TESTNET)` to build unsigned XDR
- Parse XDR with `TransactionBuilder.fromXDR(xdr, Networks.TESTNET)`
- Sign with keypair, send via `sendAndConfirm()`
- Parse return value: `StellarSdk.scValToNative(returnValue)` returns an array,
  index [1] is dfTokens minted as bigint
- Return `{ txHash, dfTokensMinted }`

### 5. `transferDfTokens(rpcServer, keypair, toAddress, amount)` function
- Create `new Contract(VAULT_ADDRESS)` instance
- Build operation: `contract.call('transfer', Address(from).toScVal(),
  Address(to).toScVal(), nativeToScVal(amount, { type: 'i128' }))`
- Get account via `rpcServer.getAccount(from)`
- Build transaction with `TransactionBuilder`, fee `BASE_FEE`,
  network `Networks.TESTNET`, timeout 30s
- Simulate with `rpcServer.simulateTransaction(tx)`, check for errors
  with `rpc.Api.isSimulationError()`
- Assemble with `rpc.assembleTransaction(tx, simulation).build()`
- Sign and send via `sendAndConfirm()`

### 6. `main()` function
- Load env, create Keypair, rpc.Server, and DefindexSDK instances
- Step 1: Call deposit, log the deposit tx hash and dfTokens minted
- Step 2: Call transferDfTokens with the minted amount, log the transfer tx hash
- Print summary with all transaction details

## Run Script
Add `"start": "tsx src/index.ts"` to package.json scripts.

## Important Notes
- Use `Networks.TESTNET` and `SupportedNetworks.TESTNET` for testnet
- For production, switch to `Networks.PUBLIC` / `SupportedNetworks.MAINNET`
  and a mainnet RPC endpoint
- All amounts use 7 decimals (stroops)
- The vault contract address is used both for deposit (via SDK) and for the
  transfer call (as a Contract instance)
- Handle errors with try/catch and provide meaningful error messages
````
