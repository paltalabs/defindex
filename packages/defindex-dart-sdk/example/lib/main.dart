import 'package:defindex_sdk/defindex_sdk.dart';
import 'package:flutter/material.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';

void main() async {
  await dotenv.load(fileName: ".env");
  String userSecret = dotenv.env['USER_SECRET'] ?? '';
  runApp(const MyApp());
}



class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Defindex SDK Example',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.green),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Defindex SDK Example Home Page'),
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
  String userAccount = dotenv.env['USER_PUBLIC_KEY'] ?? '';
  var vault = Vault(
    sorobanRPCUrl: dotenv.env['SOROBAN_RPC_URL'] ?? 'https://soroban-testnet.stellar.org',
    network: SorobanNetwork.PUBLIC,
    contractId: 'CAIZ3NMNPEN5SQISJV7PD2YY6NI6DIPFA4PCRUBOGDE4I7A3DXDLK5OI',
  );
  
  Future<void> _executeDeposit() async {
    try {
      String? transactionHash = await vault.deposit(
        userAccount,
        100.0,
        (transaction) async => signerFunction(transaction),
      );

      print('Transaction hash: $transactionHash');

      // You can also show a dialog or snackbar with the result
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Transaction hash: $transactionHash')),
      );
    } catch (error) {
      print('Error: $error');
      if (!mounted) return;
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error during deposit: $error')),
      );
    }
  }

  Future<void> _executeWithdraw() async {
    try {
      String? transactionHash = await vault.withdraw(
        100.0,
        100,
        userAccount,
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

  Future<void> _getAPY() async {
    try {
      double apy = await vault.getAPY();
      print('APY: $apy');
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('APY: ${(apy * 100).toStringAsFixed(2)}%')),
      );
    } catch (error) {
      print('Error: $error');
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text('Error getting APY: $error')),
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
            FutureBuilder<double?>(
              future: vault.balance(userAccount),
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
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: _executeDeposit,
              child: const Text('Execute Deposit'),
            ),
            const SizedBox(height: 20),
            ElevatedButton(
              onPressed: _executeWithdraw,
              child: const Text('Execute Withdraw'),
            ),
             ElevatedButton(
              onPressed: _getAPY,
              child: const Text('Get APY'),
            ),
          ],
        ),
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
  String userSecret = dotenv.env['USER_SECRET']!;
  if (userSecret.isEmpty) {
    throw Exception('USER_SECRET not found in .env file');
  }

  AbstractTransaction transaction = AbstractTransaction.fromEnvelopeXdrString(
    transactionXdr,
  );
  
  // Create keypair and sign
  KeyPair keyPair = KeyPair.fromSecretSeed(userSecret);
  transaction.sign(keyPair, Network.TESTNET);
  
  // Return signed XDR
  return transaction.toEnvelopeXdrBase64();
}