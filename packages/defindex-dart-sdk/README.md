# Defindex Smart Contract Interaction Package

This package provides a Dart interface to interact with the Defindex Smart Contract on the Soroban network. It simplifies the process of sending transactions and querying contract state, making it easier for developers to integrate Defindex into their Dart or Flutter applications.

## Features

- Easy interaction with the Defindex Smart Contract
- Supports both testnet and mainnet
- Provides a high-level API for sending transactions and querying contract state

## Getting started

To use this package, add `defindex` as a [dependency in your pubspec.yaml file](https://flutter.dev/docs/development/packages-and-plugins/using-packages).


## Usage

Here's a simple example of how to use the `DefiIndex` class to deposit an amount into an account:

```dart
import 'package:defindex/defindex.dart';

void main() async {
  var vault = Vault(
    sorobanRPCUrl: 'your_rpc_url',
    network: SorobanNetwork.TESTNET,
    contractId: 'contract_id'
  );

  String? transactionHash = await vault.deposit(
    'your_account_id',
    100.0,
    (transaction) async => 'your_signed_transaction',
  );

  print('Transaction hash: $transactionHash');
}
```

## Example Configuration

In order to run the example, you can use `.env` files to store sensitive information such as keys and URLs. This allows you to keep your credentials secure and separate from your codebase.

### Example `.env` file

Here is an example of a `.env` file:

```properties
USER_SECRET=your_secret_key
USER_PUBLIC_KEY=your_public_key
SOROBAN_RPC_URL=https://your_rpc_url
```