import 'dart:convert';

/// Modelo para representar la asignación de una estrategia
class StrategyAllocation {
  final BigInt amount;
  final bool paused;
  final String strategyAddress;

  StrategyAllocation({
    required this.amount,
    required this.paused,
    required this.strategyAddress,
  });

  factory StrategyAllocation.fromMap(Map<String, dynamic> map) {
    return StrategyAllocation(
      amount: map['amount'] is num ? BigInt.from(map['amount'] as num) : BigInt.zero,
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

/// Modelo para representar los fondos totales gestionados
class TotalManagedFunds {
  final String asset;
  final BigInt idleAmount;
  final BigInt investedAmount;
  final List<StrategyAllocation> strategyAllocations;
  final BigInt totalAmount;

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
      idleAmount: map['idle_amount'] is num ? BigInt.from(map['idle_amount'] as num) : BigInt.zero,
      investedAmount: map['invested_amount'] is num ? BigInt.from(map['invested_amount'] as num) : BigInt.zero,
      strategyAllocations: (map['strategy_allocations'] as List?)
          ?.map((allocation) => StrategyAllocation.fromMap(allocation as Map<String, dynamic>))
          .toList() ?? [],
      totalAmount: map['total_amount'] is num ? BigInt.from(map['total_amount'] as num) : BigInt.zero,
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

/// Modelo para representar un evento de vault
class VaultEvent {
  final BigInt amounts;
  final BigInt dfTokens;
  final BigInt previousPricePerShare;
  final BigInt totalManagedFunds;
  final BigInt totalSupplyBefore;
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

  /// Convierte diferentes formatos de datos a BigInt preservando la máxima precisión
  /// 
  /// Esta función maneja varios tipos de entrada (números, strings, listas, y mapas)
  /// y extrae el valor numérico como BigInt para preservar todos los decimales.
  /// 
  /// Para valores decimales, multiplica por un factor de 10^7 para preservar hasta 7 decimales.
  static BigInt extractBigIntValue(dynamic value) {
    const int precisionFactor = 10000000; // 10^7 para preservar 7 decimales
    
    if (value == null) {
      return BigInt.zero;
    } else if (value is BigInt) {
      return value;
    } else if (value is int) {
      return BigInt.from(value) * BigInt.from(precisionFactor);
    } else if (value is double) {
      // Multiplicar por el factor de precisión y redondear para evitar errores de punto flotante
      return BigInt.from((value * precisionFactor).round());
    } else if (value is String) {
      try {
        if (value.contains('.')) {
          // Manejar decimales en strings
          final parts = value.split('.');
          final intPart = BigInt.parse(parts[0]);
          
          // Calcular los decimales con la precisión correcta
          String decimalPart = parts[1];
          if (decimalPart.length > 7) {
            decimalPart = decimalPart.substring(0, 7);
          } else {
            decimalPart = decimalPart.padRight(7, '0');
          }
          
          final decimalValue = BigInt.parse(decimalPart);
          return (intPart * BigInt.from(precisionFactor)) + decimalValue;
        } else {
          // Sin decimales
          return BigInt.parse(value) * BigInt.from(precisionFactor);
        }
      } catch (e) {
        return BigInt.zero;
      }
    } else if (value is List && value.isNotEmpty) {
      // Si es una lista, procesar el primer elemento
      return extractBigIntValue(value[0]);
    } else if (value is Map<String, dynamic>) {
      // Si es un objeto complejo como totalManagedFundsBefore
      if (value.containsKey('total_amount')) {
        return extractBigIntValue(value['total_amount']);
      }
    }
    
    return BigInt.zero;
  }

  factory VaultEvent.fromMap(Map<String, dynamic> map) {
    // Procesar valores numéricos que pueden estar en formato [valor] o como objetos complejos
    

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
      amounts: extractBigIntValue(map['amounts']),
      dfTokens: extractBigIntValue(map['dfTokens']),
      previousPricePerShare: extractBigIntValue(map['previousPricePerShare']),
      totalManagedFunds: extractBigIntValue(totalManagedFundsBefore['total_amount']),
      totalSupplyBefore: extractBigIntValue(map['totalSupplyBefore']),
      eventType: map['eventType'] as String? ?? '',
      ledger: map['ledger'] is num ? (map['ledger'] as num).toInt() : 0,
      date: extractDate(map['date']),
    );
  }

  // Calcular el precio por acción (PPS)
  double calculatePricePerShare() {
    const int precisionFactor = 10000000; // Debe coincidir con el factor en extractBigIntValue
    
    try {
      if (eventType.toLowerCase() == 'deposit') {
        // (totalManagedFunds + amounts) / (totalSupplyBefore + dfTokens)
        BigInt numerator = totalManagedFunds + amounts;
        BigInt denominator = totalSupplyBefore + dfTokens;
        
        if (denominator == BigInt.zero) return 0.0;
        
        // Multiplicar por precisionFactor adicional para conservar precisión en la división
        BigInt result = (numerator * BigInt.from(precisionFactor)) ~/ denominator;
        return result.toDouble() / precisionFactor;
      } else if (eventType.toLowerCase() == 'withdraw') {
        // (totalManagedFunds - amounts) / (totalSupplyBefore - dfTokens)
        BigInt numerator = totalManagedFunds - amounts;
        BigInt denominator = totalSupplyBefore - dfTokens;
        
        if (denominator == BigInt.zero) return 0.0;
        
        BigInt result = (numerator * BigInt.from(precisionFactor)) ~/ denominator;
        return result.toDouble() / precisionFactor;
      } else {
        // Default fallback
        if (totalSupplyBefore != BigInt.zero) {
          BigInt result = (totalManagedFunds * BigInt.from(precisionFactor)) ~/ totalSupplyBefore;
          return result.toDouble() / precisionFactor;
        }
      }
    } catch (e) {
      print('Error calculando PPS: $e');
    }
    
    return 0.0;
  }
}

/// Enum para la red de Soroban
enum SorobanNetwork {
  PUBLIC,
  TESTNET,
}
