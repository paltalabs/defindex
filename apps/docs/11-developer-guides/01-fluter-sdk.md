# Flutter SDK

With the Flutter SDK, you can add functionality to buttons in your app to interact with a vault contract. You only need **three lines of code** to set up a vault and perform actions like deposits! Here’s how to use it:

1. **Obtain the Vault Contract Address:** You need the contract address for the vault where you want users to deposit. This can be got from the Dapp

2. **Create a Vault Instance:** Set up an instance of the vault in your code.

3. **Use Vault Functions:** Call `vault.deposit`, `vault.balance`, or `vault.withdraw` as needed.

## Example Code

Below is an example showing how to create a vault instance and call the `deposit` function:

```dart
import 'package:defindex_sdk/defindex_sdk.dart';

// Create a vault instance
var vault = Vault(
  sorobanRPCUrl: 'https://soroban-testnet.stellar.org',
  network: SorobanNetwork.TESTNET,
  contractId: 'CD76H2IVRMRMLE4KZXLAVK3L3CO7PENUB3X4VB2FQVUAFVAJMQYQIFDE',
);

// Make a deposit
String? transactionHash = await vault.deposit(
  'GCW36WQUHJASZVNFIIL7VZQWL6Q72XT6TAU6N3XMFGTLSNE2L7LMJNWT', // User's Stellar address
  100.0, // Amount to deposit
  (transaction) async => signerFunction(transaction),
);

print('Transaction hash: $transactionHash');

// Optionally, show a dialog or snackbar with the result
ScaffoldMessenger.of(context).showSnackBar(
  SnackBar(content: Text('Transaction hash: $transactionHash')),
);
