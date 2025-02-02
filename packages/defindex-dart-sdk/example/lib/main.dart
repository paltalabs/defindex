import 'package:flutter/material.dart';
import 'package:defindex_sdk/defindex_sdk.dart';
import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page 1'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  var vault = Vault(
    sorobanRPCUrl: 'https://soroban-testnet.stellar.org',
    network: SorobanNetwork.TESTNET,
    contractId: 'CC4J2YNRVGDUWEUVIFHTPGKDA4QMOM6RJAP4S4P7PTI3O4Q6RRHVXELH',
  );
  void _incrementCounter() {
    setState(() {
      _counter++;
    });
  }

  Future<void> _executeDeposit() async {
    try {
      String? transactionHash = await vault.deposit(
        'GCGKMP4VMPGECGWBMFTA5663QBNYFMO5QG7WPWKYTHWFEJVTNZNAVVR7',
        100.0,
        (transaction) async => signerFunction(transaction),
      );

      print('Transaction hash: $transactionHash');

      // You can also show a dialog or snackbar with the result
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Transaction hash: $transactionHash')),
      );
    } catch (error) {
      print('Error: $error');
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error during deposit: $error')),
      );
    }
  }

  Future<void> _executeWithdraw() async {
  try {
    String? transactionHash = await vault.withdraw(
      100.0,
      'GCGKMP4VMPGECGWBMFTA5663QBNYFMO5QG7WPWKYTHWFEJVTNZNAVVR7',
      (transaction) async => signerFunction(transaction),
    );

    print('Transaction hash: $transactionHash');

    // You can also show a dialog or snackbar with the result
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Transaction hash: $transactionHash')),
    );
  } catch (error) {
    print('Error: $error');
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text('Error during deposit: $error')),
    );
  }
}

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text('You have pushed the button this many times:'),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
            SizedBox(height: 20),
            FutureBuilder<double?>(
              future: vault.balance('GCGKMP4VMPGECGWBMFTA5663QBNYFMO5QG7WPWKYTHWFEJVTNZNAVVR7'),
              builder: (context, snapshot) {
                if (snapshot.connectionState == ConnectionState.waiting) {
                  return const CircularProgressIndicator();
                } else if (snapshot.hasError) {
                  return Text('Error: ${snapshot.error}');
                } else {
                  return Text(
                    'Balance: ${snapshot.data ?? 0.0} XLM',
                    style: Theme.of(context).textTheme.headlineSmall,
                  );
                }
              },
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: _executeDeposit,
              child: const Text('Execute Deposit'),
            ),
            SizedBox(height: 20),
            ElevatedButton(
              onPressed: _executeWithdraw,
              child: const Text('Execute Withdraw'),
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _incrementCounter,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}

// Future<String> signerFunction(String transactionString) async {
//   // Decode the transaction from the base64 string
//   final transaction = Transaction.fromEnvelopeXdrBase64(transactionString);

//   // Create KeyPair from the user's secret seed
//   KeyPair keyPair = KeyPair.fromSecretSeed('your_secret_key');

//   // Sign the transaction with the KeyPair
//   transaction.sign(keyPair, Network.TESTNET);

//   // Return the signed transaction as a base64 string
//   return transaction.toEnvelopeXdrBase64();
// }

String signerFunction(String transactionXdr) {
  // Create transaction from XDR
  AbstractTransaction transaction = AbstractTransaction.fromEnvelopeXdrString(
    transactionXdr,
  );
  
  // Create keypair and sign
  KeyPair keyPair = KeyPair.fromSecretSeed("SDI5ZSGJBJS2BD7PE7MPA6EXHUPJQM7I6TX5SB63HSSSZVD47OYE5X6X");
  transaction.sign(keyPair, Network.TESTNET);
  
  // Return signed XDR
  return transaction.toEnvelopeXdrBase64();
}