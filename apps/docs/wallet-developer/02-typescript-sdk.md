---
cover: ../.gitbook/assets/image 31.png
coverY: 0
---

# Typescript SDK

The TypeScript SDK provides a simple way to interact with DeFindex vaults in your web applications. You can easily integrate vault functionality with just **a few lines of code**. The SDK handles all the complexities of Soroban contract interactions while providing a type-safe interface.

## Getting Started

1. **Install the SDK**

```bash
npm install defindex-sdk
# or
yarn add defindex-sdk
```

2. **Import and Initialize**

```typescript
import { Vault, SorobanNetwork } from 'defindex-sdk';

const vault = new Vault({
    network: SorobanNetwork.PUBLIC,
    contractId: 'YOUR_VAULT_CONTRACT_ID'
});
```

3. **Use Vault Functions**

```typescript
// Check balance
const balance = await vault.balance(accountAddress, sorobanContext);

// Make a deposit
const txHash = await vault.deposit(
    accountAddress,
    100,
    true,
    sorobanContext,
    secretKey // Optional secret key for signing, if you are using a connected wallet it's not needed
);

// Withdraw funds
const withdrawTxHash = await vault.withdraw(
    accountAddress,
    50,
    true,
    sorobanContext,
    secretKey // Optional secret key for signing, if you are using a connected wallet it's not needed
);
```
