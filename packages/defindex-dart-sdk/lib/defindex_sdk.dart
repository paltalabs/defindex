library defindex_sdk;
import 'package:defindex_sdk/custom_soroban_server.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';
enum SorobanNetwork {
  PUBLIC,
  TESTNET,
}

class Vault {
  String sorobanRPCUrl;
  late CustomSorobanServer sorobanServer;
  late final StellarSDK sdk;
  late final SorobanNetwork network;
  late String contractId;

  Vault({
    required this.sorobanRPCUrl,
    this.network = SorobanNetwork.TESTNET,
    required this.contractId,
  }) {
    sorobanServer = CustomSorobanServer(sorobanRPCUrl);
    sdk = network == SorobanNetwork.TESTNET
        ? StellarSDK.TESTNET
        : StellarSDK.PUBLIC;
  }
  // Load the USER_PUBLIC_KEY from environment variables
  late final String caller = dotenv.env['USER_PUBLIC_KEY'] ?? '';
  // poll until success or error
  Future<GetTransactionResponse> pollStatus(String transactionId) async {
    var status = GetTransactionResponse.STATUS_NOT_FOUND;
    GetTransactionResponse? transactionResponse;

    while (status == GetTransactionResponse.STATUS_NOT_FOUND) {
      await Future.delayed(const Duration(seconds: 5), () {});
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

  Future<String?> deposit(String accountId, double amount,
      Future<String> Function(String) signer) async {
    sorobanServer.enableLogging = true;

    GetHealthResponse healthResponse = await sorobanServer.getHealth();

    if (GetHealthResponse.HEALTHY == healthResponse.status) {
      AccountResponse account = await sdk.accounts.account(accountId);
      // Name of the function to be invoked
      String functionName = "deposit";

      // Determine the number of digits to multiply to achieve at least 7 digits in the decimal place
      int factor = 10000000;

      // Multiply the value by the factor and convert to int
      BigInt bigIntValue = BigInt.from(amount * factor);

      int transformedValue = bigIntValue.toInt();

      // Prepare the argument (Symbol)
      XdrSCVal amountSCVal = XdrSCVal.forI128(
          XdrInt128Parts(XdrInt64(0), XdrUint64(transformedValue)));

      XdrSCVal minAmountScVal = XdrSCVal.forI128(
          XdrInt128Parts(XdrInt64(0), XdrUint64(0))
      );

      XdrSCVal arg1 = XdrSCVal.forVec([amountSCVal]);
      XdrSCVal arg2 = XdrSCVal.forVec([minAmountScVal]);
      XdrSCVal arg3 = XdrSCVal.forAddress(XdrSCAddress.forAccountId(accountId));
      XdrSCVal arg4 = XdrSCVal.forBool(true);

      // Prepare the "invoke" operation
      InvokeContractHostFunction hostFunction = InvokeContractHostFunction(
          contractId, functionName,
          arguments: [arg1, arg2, arg3, arg4]);

      InvokeHostFunctionOperation operation =
          InvokeHostFuncOpBuilder(hostFunction).build();

      Transaction transaction =
          TransactionBuilder(account).addOperation(operation).build();

      var request = SimulateTransactionRequest(transaction);

      SimulateTransactionResponse simulateResponse =
          await sorobanServer.simulateTransaction(request);

      // simulateResponse.transactionData!.resourceFee =
      //     XdrInt64(((simulateResponse.minResourceFee ?? 0) * 120) ~/ 100);
      // simulateResponse.minResourceFee =
      //     ((simulateResponse.minResourceFee ?? 0) * 120) ~/ 100;

      // set transaction data, add resource fee and sign transaction
      transaction.sorobanTransactionData = simulateResponse.transactionData;
      transaction.addResourceFee(simulateResponse.minResourceFee!);
      transaction.setSorobanAuth(simulateResponse.sorobanAuth);

      String transactionString =
          transaction.toEnvelopeXdr().toEnvelopeXdrBase64();

      String transactionSigned = await signer(transactionString);

      SendTransactionResponse sendResponse =
          await sorobanServer.sendRawTransaction(transactionSigned);

      assert(!sendResponse.isErrorResponse);

      assert(sendResponse.status != SendTransactionResponse.STATUS_ERROR);

      GetTransactionResponse statusResponse =
          await pollStatus(sendResponse.hash!);

      String status = statusResponse.status!;
      assert(status == GetTransactionResponse.STATUS_SUCCESS);

      return sendResponse.hash;
    }

    return null;
  }

  Future<String?> withdraw(
      double amount,
      int toleranceBPS,
      String accountId,
      Future<String> Function(String) signer) async {
    sorobanServer.enableLogging = true;

    GetHealthResponse healthResponse = await sorobanServer.getHealth();

    if (GetHealthResponse.HEALTHY == healthResponse.status) {
      AccountResponse account = await sdk.accounts.account(accountId);
      // Name of the function to be invoked
      String functionName = "withdraw";

      // Determine the number of digits to multiply to achieve at least 7 digits in the decimal place
      int factor = 10000000;

      // Multiply the value by the factor and convert to int
      BigInt bigIntValue = BigInt.from(amount * factor);

      int transformedValue = bigIntValue.toInt();

      // Prepare the argument (Symbol)
      XdrSCVal arg1 = XdrSCVal.forI128(
          XdrInt128Parts(XdrInt64(0), XdrUint64(transformedValue)));

      XdrSCVal minAmountsOut = XdrSCVal.forVec([
        XdrSCVal.forI128(XdrInt128Parts(XdrInt64(0), XdrUint64(transformedValue * toleranceBPS ~/ 10000)))
      ]);

      XdrSCVal arg2 = XdrSCVal.forAddress(XdrSCAddress.forAccountId(accountId));
      // Prepare the "invoke" operation
      InvokeContractHostFunction hostFunction = InvokeContractHostFunction(
          contractId, functionName,
          arguments: [arg1, minAmountsOut, arg2]);

      InvokeHostFunctionOperation operation =
          InvokeHostFuncOpBuilder(hostFunction).build();

      Transaction transaction =
          TransactionBuilder(account).addOperation(operation).build();

      var request = SimulateTransactionRequest(transaction);

      SimulateTransactionResponse simulateResponse =
          await sorobanServer.simulateTransaction(request);

      transaction.sorobanTransactionData = simulateResponse.transactionData;
      transaction.addResourceFee(simulateResponse.minResourceFee!);
      transaction.setSorobanAuth(simulateResponse.sorobanAuth);

      String transactionString =
          transaction.toEnvelopeXdr().toEnvelopeXdrBase64();

      String transactionSigned = await signer(transactionString);

      SendTransactionResponse sendResponse =
          await sorobanServer.sendRawTransaction(transactionSigned);

      assert(!sendResponse.isErrorResponse);

      assert(sendResponse.status != SendTransactionResponse.STATUS_ERROR);

      GetTransactionResponse statusResponse =
          await pollStatus(sendResponse.hash!);

      String status = statusResponse.status!;
      assert(status == GetTransactionResponse.STATUS_SUCCESS);

      return sendResponse.hash;
    }

    return null;
  }

  Future<double?> balance(String accountId) async {
    sorobanServer.enableLogging = false;

    GetHealthResponse healthResponse = await sorobanServer.getHealth();

    double dfBalance = 0;

    if (GetHealthResponse.HEALTHY == healthResponse.status) {
      AccountResponse account = await sdk.accounts.account(accountId);
      String functionName = "balance";

      XdrSCVal arg1 = XdrSCVal.forAddress(XdrSCAddress.forAccountId(accountId));

      InvokeContractHostFunction hostFunction = InvokeContractHostFunction(
          contractId, functionName,
          arguments: [arg1]);

      InvokeHostFunctionOperation operation =
          InvokeHostFuncOpBuilder(hostFunction).build();

      Transaction transaction =
          TransactionBuilder(account).addOperation(operation).build();

      var request = SimulateTransactionRequest(transaction);

      SimulateTransactionResponse simulateResponse =
          await sorobanServer.simulateTransaction(request);

      if (simulateResponse.results != null && simulateResponse.results!.isNotEmpty) {
        String? xdrValue = simulateResponse.results![0].xdr;
        XdrSCVal xdrSCVal = XdrSCVal.fromBase64EncodedXdrString(xdrValue);
        dfBalance = BigInt.from(xdrSCVal.i128!.lo.uint64).toDouble() / 10000000; 
      }
      dynamic totalManagedFunds = await fetchTotalManagedFunds();
      if (totalManagedFunds == null || totalManagedFunds.isEmpty) {
        return 0;
      }
      double totalAmount = totalManagedFunds.values.first[0];
      double? totalSupplySim = await totalSupply();

      return dfBalance*totalAmount/totalSupplySim!;
    }
    return 0;
  }

  Future<double?> totalSupply() async {
    sorobanServer.enableLogging = false;

    GetHealthResponse healthResponse = await sorobanServer.getHealth();
    
    if (GetHealthResponse.HEALTHY == healthResponse.status) {
      // We'll use the contract's address as the account for this read-only operation
      AccountResponse account = await sdk.accounts.account(caller);
      
      // Name of the function to be invoked
      String functionName = "total_supply";

      // Prepare the "invoke" operation - no arguments needed for total_supply
      InvokeContractHostFunction hostFunction = InvokeContractHostFunction(
          contractId, functionName,
          arguments: []);

      InvokeHostFunctionOperation operation =
          InvokeHostFuncOpBuilder(hostFunction).build();

      Transaction transaction =
          TransactionBuilder(account).addOperation(operation).build();

      var request = SimulateTransactionRequest(transaction);

      SimulateTransactionResponse simulateResponse =
          await sorobanServer.simulateTransaction(request);

      if (simulateResponse.results != null && simulateResponse.results!.isNotEmpty) {
        String? xdrValue = simulateResponse.results![0].xdr;
        
        XdrSCVal xdrSCVal = XdrSCVal.fromBase64EncodedXdrString(xdrValue);
        
        return BigInt.from(xdrSCVal.i128!.lo.uint64).toDouble();
      }
    }
    
    return null;
  }

  Future<Map<String, dynamic>?> fetchTotalManagedFunds() async {
    sorobanServer.enableLogging = false;

    GetHealthResponse healthResponse = await sorobanServer.getHealth();
    
    if (GetHealthResponse.HEALTHY == healthResponse.status) {
      AccountResponse account = await sdk.accounts.account(caller);
      String functionName = "fetch_total_managed_funds";

      InvokeContractHostFunction hostFunction = InvokeContractHostFunction(
          contractId, functionName,
          arguments: []);

      InvokeHostFunctionOperation operation =
          InvokeHostFuncOpBuilder(hostFunction).build();

      Transaction transaction =
          TransactionBuilder(account).addOperation(operation).build();

      var request = SimulateTransactionRequest(transaction);

      SimulateTransactionResponse simulateResponse =
          await sorobanServer.simulateTransaction(request);

      if (simulateResponse.results != null && simulateResponse.results!.isNotEmpty) {
        String? xdrValue = simulateResponse.results![0].xdr;
        XdrSCVal xdrSCVal = XdrSCVal.fromBase64EncodedXdrString(xdrValue);
        try {
            var parsedMap = parseScVal(xdrSCVal);
          return parsedMap;
        } catch (e) {
          throw Exception('Unsupported type: ${xdrSCVal.discriminant}');
        }
      }
    }
    
    return null;
  }
}

Map<String, dynamic> parseMap(XdrSCVal scval) {
  if (scval.discriminant != XdrSCValType.SCV_MAP) {
    throw Exception('Expected Map type, got ${scval.discriminant}');
  }
  Map<String, dynamic> result = {};
  for (var entry in scval.map!) {
    String key = parseScVal(entry.key);
    dynamic value = parseScVal(entry.val);

    result[key] = value;
  }
  return result;
}

dynamic parseScVal(XdrSCVal val) {
  switch (val.discriminant) {
    case XdrSCValType.SCV_BOOL:
      return val.b;
    case XdrSCValType.SCV_U32:
      return BigInt.from(val.u32!.uint32).toDouble();
    case XdrSCValType.SCV_I32:
      return BigInt.from(val.i32!.int32).toDouble();
    case XdrSCValType.SCV_U64:
      return BigInt.from(val.u64!.uint64).toDouble();
    case XdrSCValType.SCV_I64:
      return BigInt.from(val.i64!.int64).toDouble();
    case XdrSCValType.SCV_U128:
      return BigInt.from(val.u128!.lo.uint64).toDouble();
    case XdrSCValType.SCV_I128:
      return BigInt.from(val.i128!.lo.uint64).toDouble();
    case XdrSCValType.SCV_STRING:
      return val.str;
    case XdrSCValType.SCV_BYTES:
      return val.bytes;
    case XdrSCValType.SCV_ADDRESS:
      try {
        var address = val.address!;
        var contractId = Address.fromXdr(address).contractId!;
        var parsedAddress = Address.forContractId(contractId);
        return parsedAddress;
      } catch (e) {
        throw Exception('Invalid address format');
      }
    case XdrSCValType.SCV_SYMBOL:
      return val.sym;
    case XdrSCValType.SCV_VEC:
      return val.vec?.map((e) => parseScVal(e)).toList();
    case XdrSCValType.SCV_MAP:
      return parseMap(val);
    default:
      throw Exception('Unsupported type: ${val.discriminant}');
  }
}
