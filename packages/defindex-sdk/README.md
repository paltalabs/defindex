# DeFindex SDK

A TypeScript SDK for interacting with DeFindex Vaults on the Stellar Soroban network. DeFindex is a protocol for creating and managing multi-asset investment vaults with customizable strategies.

## Overview

DeFindex Vaults serve as the core of the DeFindex platform, responsible for:
- Managing multiple assets
- Executing investment strategies
- Ensuring proper asset rebalancing
- Minting/burning dfTokens (vault shares)

## Installation

```bash
npm install defindex-sdk
# or
yarn add defindex-sdk

```

## Quick Start

```typescript
import { Vault, SorobanNetwork } from 'defindex-sdk';

// Initialize a vault instance
const vault = new Vault({
    network: SorobanNetwork.TESTNET,
    contractId: 'YOUR_VAULT_CONTRACT_ID'
});

// Deposit assets
const txHash = await vault.deposit(
    accountAddress,
    100, // amount
    true, // signAndSend
    sorobanContext,
    secretKey
);
```

## Features

- **Asset Management**: Deposit and withdraw multiple assets
- **Balance Tracking**: Monitor your vault share balance
- **Transaction Handling**: Built-in support for Soroban transactions
- **Type Safety**: Full TypeScript support

## API Reference

### `Vault`

The main class for interacting with DeFindex vaults.

#### Constructor

```typescript
new Vault({
    network: SorobanNetwork,
    contractId: string
})
```

#### Methods

##### `deposit(account: string, amount: number, signAndSend: boolean, sorobanContext: SorobanContextType, secretKey?: string): Promise<string>`

Deposits assets into the vault and receives dfTokens in return.

- **Parameters:**
  - `account`: The depositor's address
  - `amount`: Amount to deposit
  - `signAndSend`: Whether to sign and submit the transaction
  - `sorobanContext`: Soroban context object
  - `secretKey`: Optional secret key for signing

- **Returns:** Transaction hash

##### `withdraw(account: string, amount: number, signAndSend: boolean, sorobanContext: SorobanContextType, secretKey?: string): Promise<string>`

Withdraws assets from the vault by burning dfTokens.

- **Parameters:**
  - `account`: The withdrawer's address
  - `amount`: Amount of dfTokens to burn
  - `signAndSend`: Whether to sign and submit the transaction
  - `sorobanContext`: Soroban context object
  - `secretKey`: Optional secret key for signing

- **Returns:** Transaction hash

##### `balance(account: string, sorobanContext: SorobanContextType): Promise<number>`

Checks the dfToken balance of an account.

- **Parameters:**
  - `account`: The account address
  - `sorobanContext`: Soroban context object

- **Returns:** Balance amount

## Example Usage

```typescript
import { Vault, SorobanNetwork } from 'defindex-sdk';

async function main() {
    // Initialize vault
    const vault = new Vault({
        network: SorobanNetwork.TESTNET,
        contractId: 'YOUR_CONTRACT_ID'
    });

    // Create Soroban context
    const sorobanContext = await createSorobanContext();
    
    // Check initial balance
    const balance = await vault.balance(accountAddress, sorobanContext);
    console.log('Balance:', balance);

    // Deposit assets
    const txHash = await vault.deposit(
        accountAddress,
        100,
        true,
        sorobanContext,
        secretKey
    );
    console.log('Deposit TX:', txHash);
}
```

For a complete example, see [vault-example.ts](./examples/vault-example.ts).


## Support

- Discord: [Join our community](https://discord.gg/ftPKMPm38f)
