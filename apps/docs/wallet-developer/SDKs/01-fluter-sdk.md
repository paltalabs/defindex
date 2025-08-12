---
cover: ../.gitbook/assets/image 31.png
coverY: 0
---

# DeFindex Flutter SDK

Welcome to the DeFindex Flutter SDK documentation! This SDK enables you to integrate DeFindex's savings account functionality into your Flutter application. With this SDK, your users can:

1. Deposit funds into a DeFindex vault
2. Check their vault balance
3. Withdraw funds from their vault
4. View the current APY (Annual Percentage Yield) of their vault

## Prerequisites

Before integrating the SDK, you'll need to deploy a vault contract for your application. You can do this through our [DeFindex DApp](https://app.defindex.io/). Make sure to thoroughly understand vault management and operations before proceeding.

For detailed instructions on creating, deploying, and managing vaults, please refer to our [Creating a DeFindex Vault](../getting-started/creating-a-defindex-vault.md) guide.

## Integration Guide

### 1. Add the SDK to Your Project

Add the following dependency to your `pubspec.yaml` file:

```yaml
dependencies:
  defindex_sdk: ^1.0.1
```

### 2. Import the SDK

Import the SDK in your `main.dart` file:

```dart
import 'package:defindex_sdk/defindex_sdk.dart';
```

## Quick Start

The Flutter SDK makes it incredibly simple to integrate DeFindex vault functionality into your app. With just three lines of code, you can set up a vault and enable deposits! Here's what you need to do:

1. **Get Your Vault Contract Address:** Retrieve the contract address for your vault from the DeFindex DApp
2. **Initialize the Vault:** Create a vault instance in your code
3. **Implement Vault Functions:** Use `vault.deposit`, `vault.balance`, or `vault.withdraw` as needed

## Implementation Example

Here's a practical example demonstrating how to create a vault instance and implement a deposit function:

```dart
import 'package:defindex_sdk/defindex_sdk.dart';

// Initialize the vault
var vault = Vault(
  sorobanRPCUrl: 'https://soroban-testnet.stellar.org', // Your RPC URL
  network: SorobanNetwork.TESTNET, // Your network
  contractId: 'CD76H2IVRMRMLE4KZXLAVK3L3CO7PENUB3X4VB2FQVUAFVAJMQYQIFDE', // Your vault contract address
);

// Execute a deposit
String? transactionHash = await vault.deposit(
  'GCW36WQUHJASZVNFIIL7VZQWL6Q72XT6TAU6N3XMFGTLSNE2L7LMJNWT', // User's Stellar address
  100.0, // Deposit amount
  (transaction) async => signerFunction(transaction),
);

print('Transaction hash: $transactionHash');

// Display transaction result to user
ScaffoldMessenger.of(context).showSnackBar(
  SnackBar(content: Text('Transaction hash: $transactionHash')),
);
```
