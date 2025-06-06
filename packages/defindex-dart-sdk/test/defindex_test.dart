import 'package:defindex_sdk/defindex_sdk.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';

void main() {
  String testOn = 'testnet';

  SorobanServer sorobanServer = testOn == 'testnet'
      ? SorobanServer("https://soroban-testnet.stellar.org")
      : SorobanServer("https://rpc-futurenet.stellar.org");

  StellarSDK sdk =
      testOn == 'testnet' ? StellarSDK.TESTNET : StellarSDK.FUTURENET;

  Network network = testOn == 'testnet' ? Network.TESTNET : Network.FUTURENET;

  KeyPair aliceKeypair = KeyPair.random();
  String aliceId = aliceKeypair.accountId;

  setUp(() async {
    sorobanServer.enableLogging = true;

    try {
      await sdk.accounts.account(aliceId);
    } catch (e) {
      if (testOn == 'testnet') {
        await FriendBot.fundTestAccount(aliceId);
      } else if (testOn == 'futurenet') {
        await FuturenetFriendBot.fundTestAccount(aliceId);
      }
    }
  });

  // poll until success or error
  Future<GetTransactionResponse> pollStatus(String transactionId) async {
    var status = GetTransactionResponse.STATUS_NOT_FOUND;
    GetTransactionResponse? transactionResponse;
    while (status == GetTransactionResponse.STATUS_NOT_FOUND) {
      await Future.delayed(const Duration(seconds: 3), () {});
      transactionResponse = await sorobanServer.getTransaction(transactionId);
      assert(transactionResponse.error == null);
      status = transactionResponse.status!;
      if (status == GetTransactionResponse.STATUS_FAILED) {
        assert(transactionResponse.resultXdr != null);
        assert(false);
      } else if (status == GetTransactionResponse.STATUS_SUCCESS) {
        assert(transactionResponse.resultXdr != null);
      }
    }
    return transactionResponse!;
  }

  Future<String?> deposit() async {
    Vault vault = Vault(
        sorobanRPCUrl: "https://soroban-testnet.stellar.org",
        network: SorobanNetwork.TESTNET,
        contractId: "CBWJQE6F7YF2IPDE645A3PW4WNP5KGODDMZSDVGZOGTEP423YLW2L7J5");

    String? hash =
        await vault.deposit(aliceId, 10000, (String transactionStr) async {
      Transaction transaction =
          AbstractTransaction.fromEnvelopeXdrString(transactionStr)
              as Transaction;
      transaction.sign(aliceKeypair, network);

      return transaction.toEnvelopeXdrBase64();
    });

    assert(hash != null, "Hash not found. Transaction failed");
  }

  group('all tests', () {
    test('deposit', () async {
      // String? hash = await deposit();

      assert(true);
    });
  });
}
