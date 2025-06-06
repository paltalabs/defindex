library defindex_sdk;
import 'dart:convert';
import 'dart:math' as Math;

import 'package:defindex_sdk/custom_soroban_server.dart';
import 'package:defindex_sdk/graph_ql_server.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';
import 'package:stellar_flutter_sdk/stellar_flutter_sdk.dart';

class StrategyAllocation {
  final double amount;
  final bool paused;
  final String strategyAddress;

  StrategyAllocation({
    required this.amount,
    required this.paused,
    required this.strategyAddress,
  });

  factory StrategyAllocation.fromMap(Map<String, dynamic> map) {
    return StrategyAllocation(
      amount: map['amount'] is num ? (map['amount'] as num).toDouble() : 0.0,
      paused: map['paused'] as bool,
      strategyAddress: map['strategy_address'] as String,
    );
  }

  Map<String, dynamic> toMap() {
    return {
      'amount': amount,
      'paused': paused,
      'strategy_address': strategyAddress,
    };
  }
}


class TotalManagedFunds {
  final String asset;
  final double idleAmount;
  final double investedAmount;
  final List<StrategyAllocation> strategyAllocations;
  final double totalAmount;

  TotalManagedFunds({
    required this.asset,
    required this.idleAmount,
    required this.investedAmount,
    required this.strategyAllocations,
    required this.totalAmount,
  });

  factory TotalManagedFunds.fromMap(Map<String, dynamic> map) {
    return TotalManagedFunds(
      asset: map['asset'] as String,
      idleAmount: map['idle_amount'] is num ? (map['idle_amount'] as num).toDouble() : 0.0,
      investedAmount: map['invested_amount'] is num ? (map['invested_amount'] as num).toDouble() : 0.0,
      strategyAllocations: (map['strategy_allocations'] as List?)
          ?.map((allocation) => StrategyAllocation.fromMap(allocation as Map<String, dynamic>))
          .toList() ?? [],
      totalAmount: map['total_amount'] is num ? (map['total_amount'] as num).toDouble() : 0.0,
    );
  }

  Map<String, dynamic> toMap() {
    return {
      'asset': asset,
      'idle_amount': idleAmount,
      'invested_amount': investedAmount,
      'strategy_allocations': strategyAllocations.map((allocation) => allocation.toMap()).toList(),
      'total_amount': totalAmount,
    };
  }
}

class VaultEvent {
  final double amounts;
  final double dfTokens;
  final double previousPricePerShare;
  final double totalManagedFunds;
  final double totalSupplyBefore;
  final String eventType;
  final int ledger;
  final DateTime date;

  VaultEvent({
    required this.amounts,
    required this.dfTokens,
    required this.previousPricePerShare,
    required this.totalManagedFunds,
    required this.totalSupplyBefore,
    required this.eventType,
    required this.ledger,
    required this.date,
  });

  factory VaultEvent.fromMap(Map<String, dynamic> map) {
    // Procesar valores numéricos que pueden estar en formato [valor] o como objetos complejos
    double extractDoubleValue(dynamic value) {
      if (value is num) {
        return value.toDouble();
      } else if (value is List && value.isNotEmpty) {
        if (value[0] is num) {
          return (value[0] as num).toDouble();
        } else if (value[0] is String) {
          return double.tryParse(value[0] as String) ?? 0.0;
        }
      } else if (value is String) {
        return double.tryParse(value) ?? 0.0;
      } else if (value is Map<String, dynamic>) {
        // Si es un objeto complejo como totalManagedFundsBefore
        if (value.containsKey('total_amount')) {
          var totalAmount = value['total_amount'];
          if (totalAmount is String) {
            return double.tryParse(totalAmount) ?? 0.0;
          } else if (totalAmount is num) {
            return totalAmount.toDouble();
          }
        }
      }
      return 0.0;
    }

    // Procesar fecha que puede venir en varios formatos
    DateTime extractDate(dynamic dateValue) {
      if (dateValue is DateTime) {
        return dateValue;
      } else if (dateValue is int) {
        return DateTime.fromMillisecondsSinceEpoch(dateValue);
      } else if (dateValue is String) {
        try {
          return DateTime.parse(dateValue);
        } catch (e) {
          return DateTime.now();
        }
      }
      return DateTime.now();
    }

    // Parsear totalManagedFundsBefore como JSON
    Map<String, dynamic> parseJson(dynamic value) {
      if (value is String) {
        try {
          return Map<String, dynamic>.from(jsonDecode(value));
        } catch (e) {
        }
      }
      return {};
    }

    final totalManagedFundsBefore = parseJson(map['totalManagedFundsBefore']);

    return VaultEvent(
      amounts: extractDoubleValue(map['amounts']),
      dfTokens: extractDoubleValue(map['dfTokens']),
      previousPricePerShare: extractDoubleValue(map['previousPricePerShare']),
      totalManagedFunds: extractDoubleValue(totalManagedFundsBefore['total_amount']),
      totalSupplyBefore: extractDoubleValue(map['totalSupplyBefore']),
      eventType: map['eventType'] as String? ?? '',
      ledger: map['ledger'] is num ? (map['ledger'] as num).toInt() : 0,
      date: extractDate(map['date']),
    );
  }

  // Calcular el precio por acción (PPS)
  double calculatePricePerShare() {
    if (eventType.toLowerCase() == 'deposit') {
      return (totalManagedFunds + amounts) / (totalSupplyBefore + dfTokens);
    } else if (eventType.toLowerCase() == 'withdraw') {
      return (totalManagedFunds - amounts) / (totalSupplyBefore - dfTokens);
    } else {
      // Default fallback: avoid division by zero
      if (totalSupplyBefore != 0) {
        return totalManagedFunds / totalSupplyBefore;
      }
      return 0.0;
    }
  }
}

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
      TotalManagedFunds? totalManagedFunds = await fetchTotalManagedFunds();
      if (totalManagedFunds == null) {
        return 0;
      }
      double totalAmount = totalManagedFunds.totalAmount;
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

  Future<TotalManagedFunds?> fetchTotalManagedFunds() async {
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
        List<XdrSCVal> xdrSCVal = XdrSCVal.fromBase64EncodedXdrString(xdrValue).vec!;
        try {
          Map<String, dynamic> resultMap = parseScVal(xdrSCVal[0]);
          return TotalManagedFunds.fromMap(resultMap);
        } catch (e) {
          throw Exception('Unsupported type: ${xdrSCVal[0].discriminant}');
        }
      }
    }
    return null;
  }

  Future<double> getAPY() async {
    /* 
    Para sacar APY hay que:
    1.- Tomar el evento mas cercano al date de hace 7 días (o un mes). y obtener total_managed_funds_before, total_supply_before, amounts, df_tokens_minted/burned.
    2.- Tomar el evento mas cercano al momento actual. y obtener total_managed_funds_after, total_supply_after, amounts, df_tokens_minted/burned.
    3.- Crear una función para calcular el PPS (Price Per Share) como total_managed_funds/total_supply, (total_managed_funds_before+amounts)/(total_supply_before+df_tokens_minted) or substracting amounts and df tokens burned.
    4.- Calcular el APR por día como (PPS_today-PPS_previous)/period_in_days-1, so you can use period_in_days as 7, or for more accurate it can be (date_closest_event_today-date_closest_event_previous)/seconds_in_day. (I dont remember if dates is in seconds or miliseconds),
    5.- anualizar con APY = (1+APR_daily)^365.2425 */
    
    try {
      // Crear cliente GraphQL
      final graphQLClient = DeFindexGraphQLClient();
      
      // Estimar el ledger actual basado en el tiempo
      // Suponiendo aproximadamente un ledger cada 5 segundos
      final lastEvent = await graphQLClient.query(
        DeFindexQueries.getVaultEvent,
        variables: {
          'vaultAddress': contractId,
          'orderBy': 'LEDGER_ASC', 
          'first': 1, 
        },
      );
      final lastEventData = graphQLClient.getResultData(lastEvent);
      
      if (lastEventData == null || 
          !lastEventData.containsKey('deFindexVaults') || 
          lastEventData['deFindexVaults']['nodes'].isEmpty) {
        return 0.0;
      }
      
      final firstEvent = await graphQLClient.query(
        DeFindexQueries.getVaultEvent,
        variables: {
          'vaultAddress': contractId,
          'orderBy': 'LEDGER_DESC', // Ordenar por ledger ascendente para obtener los más antiguos
          'last': 1,
        },
      );
      
      if (firstEvent.hasException) {
      }
      
      // Extraer datos de las respuestas
      final currentData = lastEventData;
      final pastData = graphQLClient.getResultData(firstEvent);
      
      if (currentData.isEmpty || pastData == null) {
        return 0.0;
      }
      
      // Convertir a eventos
      List<VaultEvent> currentEvents = [];
      List<VaultEvent> pastEvents = [];
      
      try {
        // Extraer y mapear los eventos actuales
        final currentNodes = currentData['deFindexVaults']['nodes'] as List;
        currentEvents = currentNodes
            .map((node) => VaultEvent.fromMap(node as Map<String, dynamic>))
            .toList();
        
        // Extraer y mapear los eventos pasados
        final pastNodes = pastData['deFindexVaults']['nodes'] as List;
        pastEvents = pastNodes
            .map((node) => VaultEvent.fromMap(node as Map<String, dynamic>))
            .toList();
      } catch (e) {
        // Mostrar más información de debug
        if (currentData.containsKey('deFindexVaults')) {
          print("Estructura de deFindexVaults: ${currentData['deFindexVaults']}");
        }
        return 0.0;
      }
      
      // Verificar que hay suficientes datos
      if (currentEvents.isEmpty || pastEvents.isEmpty) {
        return 0.0;
      }
      
      // Ordenar por ledger (de más reciente a más antiguo)
      currentEvents.sort((a, b) => b.ledger.compareTo(a.ledger));
      pastEvents.sort((a, b) => b.ledger.compareTo(a.ledger));
      
      // Tomar el evento más reciente y el evento más antiguo
      final mostRecentEvent = currentEvents.first;
      final oldestEvent = pastEvents.first;
      
      // Calcular el precio por acción (PPS) para ambos eventos
      final currentPPS = mostRecentEvent.calculatePricePerShare();
      final pastPPS = oldestEvent.calculatePricePerShare();
      
      if (currentPPS <= 0 || pastPPS <= 0) {
        return 0.0;
      }
      
      // Calcular días transcurridos
      final daysDifference = (mostRecentEvent.date.difference(oldestEvent.date).inSeconds / 86400);
      
      if (daysDifference <= 0) {
        return 0.0;
      }
      
      // Calcular APR diario
      final daily = (currentPPS / pastPPS - 1) / daysDifference;
      
      // Anualizar para obtener el APY
      final apy = Math.pow(1 + daily, 365.2425) - 1;
      
      // Limitar el valor a un rango razonable (0% a infinito)
      // Y formatear como porcentaje
      final result = (apy as double).clamp(0.0, double.infinity);
      
      // Almacenar el resultado calculado (podría ser útil para consultas futuras)
      final apyCalculado = result;
      
      return apyCalculado;
    } catch (e) {
      return 0.00; // Valor por defecto en caso de error
    }
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
        if (val.address?.contractId != null) {
          final contractIdHex = Util.bytesToHex(val.address!.contractId!.hash);
          final strKeyContractId = StrKey.encodeContractIdHex(contractIdHex);
          return strKeyContractId;
        } else {
          throw Exception('Invalid address format');
        }
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
