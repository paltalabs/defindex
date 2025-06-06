import 'package:flutter/material.dart';
import 'package:graphql_flutter/graphql_flutter.dart';

const indexerURL = "https://index-api.onfinality.io/sq/soroswap/testnet-indexer";

class DeFindexGraphQLClient {
  late final GraphQLClient _client;
  
  late final GraphQLCache _cache;

  final String _serverUrl;

  DeFindexGraphQLClient({
    String? serverUrl,
    Map<String, String>? headers,
  }) : _serverUrl = serverUrl ?? indexerURL {
    _cache = GraphQLCache();
    
    final HttpLink httpLink = HttpLink(
      _serverUrl,
      defaultHeaders: headers ?? {},
    );

    // Create graphql client with cache and link
    _client = GraphQLClient(
      cache: _cache,
      link: httpLink,
    );
  }

  // Constructor with authentication token
  factory DeFindexGraphQLClient.withAuth({
    String? serverUrl,
    required String token,
  }) {
    return DeFindexGraphQLClient(
      serverUrl: serverUrl,
      headers: {'Authorization': 'Bearer $token'},
    );
  }

  // Calls to the GraphQL server
  Future<QueryResult> query(
    String query, {
    Map<String, dynamic>? variables,
    FetchPolicy? fetchPolicy,
  }) async {
    final QueryOptions options = QueryOptions(
      document: gql(query),
      variables: variables ?? {},
      fetchPolicy: fetchPolicy ?? FetchPolicy.noCache,
    );

    return await _client.query(options);
  }

  // Execute mutation on the GraphQL server (Update, Insert, Delete)
  Future<QueryResult> mutate(
    String mutation, {
    Map<String, dynamic>? variables,
  }) async {
    final MutationOptions options = MutationOptions(
      document: gql(mutation),
      variables: variables ?? {},
    );

    return await _client.mutate(options);
  }

  /// Subscribe to a GraphQL server to receive real-time updates
  Stream<QueryResult> subscribe(
    String subscription, {
    Map<String, dynamic>? variables,
  }) {
    final SubscriptionOptions options = SubscriptionOptions(
      document: gql(subscription),
      variables: variables ?? {},
    );

    return _client.subscribe(options);
  }

  // Search for a query result or fail
  Map<String, dynamic>? getResultData(QueryResult queryResult) {
    if (queryResult.hasException) {
      print('Error: ${queryResult.exception}');
      return null;
    }

    return queryResult.data;
  }

  // Clear the cache of the GraphQL client
  void clearCache() {
    _cache.store.reset();
  }
}

/// Widget provider for GraphQL client
class DeFindexGraphQLProvider extends StatelessWidget {
  final String? serverUrl;

  // Optional HTTP headers
  final Map<String, String>? headers;
  
  // Optional authentication token for JWT
  final String? authToken;
  
  // Child widget to provide the GraphQL client
  final Widget child;

  /// Constructor
  ///
  /// [serverUrl] - GraphQL server URL.
  /// [headers] - Optional HTTP headers.
  /// [authToken] - Optional authentication token for JWT.
  /// [child] - Child widget.
  const DeFindexGraphQLProvider({
    super.key,
    this.serverUrl,
    this.headers,
    this.authToken,
    required this.child,
  });

  @override
  Widget build(BuildContext context) {
    // Create the HTTP link with the server URL and headers
    final HttpLink httpLink = HttpLink(
      serverUrl ?? indexerURL,
      defaultHeaders: _buildHeaders(),
    );

    final ValueNotifier<GraphQLClient> client = ValueNotifier(
      GraphQLClient(
        link: httpLink,
        cache: GraphQLCache(),
      ),
    );

    return GraphQLProvider(
      client: client,
      child: child,
    );
  }

  // Builds the headers for the GraphQL client
  Map<String, String> _buildHeaders() {
    final Map<String, String> defaultHeaders = headers ?? {};
    
    if (authToken != null) {
      defaultHeaders['Authorization'] = 'Bearer $authToken';
    }
    
    return defaultHeaders;
  }
}

/* ------------------------------------------- 🚧 Work in progress ------------------------------------------- */
class DeFindexQueries {
  // Sample query, might not work
  static String getVaultInfo = r'''
    query GetVaultInfo($contractId: String!) {
      vault(address: $contractId) {
        address
        totalValue
        assetAddress
        strategies {
          address
          allocation
          paused
        }
      }
    }
  ''';

  // create here more queries as needed
  static String getVaultEventsForAPYQuery = r'''
    query getVaultEvents($vaultAddress: String!, $ledger: Int!) {
      deFindexVaults(filter: {vault: {equalTo: $vaultAddress}, ledger:  {
        lessThanOrEqualTo: $ledger
      }}) {
        nodes {
          amounts
          dfTokens
          previousPricePerShare
          totalManagedFundsBefore
          totalSupplyBefore
          eventType
          ledger
          date
        }
      }
    }
  ''';
  
  static String getVaultEvent = r'''
    query defindex($vaultAddress: String, $orderBy: [DeFindexVaultsOrderBy!], $last: Int) {
    deFindexVaults(filter: {vault: {equalTo: $vaultAddress}}, orderBy: $orderBy, last: $last) {
      nodes {
        amounts
        dfTokens
        previousPricePerShare
        totalManagedFundsBefore
        totalSupplyBefore
        eventType
        ledger
        date
      }
    }
  }
  ''';
  }



class VaultQueryResponse {
  final String address;
  final double totalValue;
  final String assetAddress;
  final List<StrategyInfo> strategies;

  VaultQueryResponse({
    required this.address,
    required this.totalValue,
    required this.assetAddress,
    required this.strategies,
  });

  factory VaultQueryResponse.fromMap(Map<String, dynamic> map) {
    return VaultQueryResponse(
      address: map['address'] as String,
      totalValue: map['totalValue'] is num ? (map['totalValue'] as num).toDouble() : 0.0,
      assetAddress: map['assetAddress'] as String,
      strategies: (map['strategies'] as List?)
          ?.map((strategy) => StrategyInfo.fromMap(strategy as Map<String, dynamic>))
          .toList() ?? [],
    );
  }
}

/// Modelo para información de estrategias
class StrategyInfo {
  final String address;
  final double allocation;
  final bool paused;

  StrategyInfo({
    required this.address,
    required this.allocation,
    required this.paused,
  });

  factory StrategyInfo.fromMap(Map<String, dynamic> map) {
    return StrategyInfo(
      address: map['address'] as String,
      allocation: map['allocation'] is num ? (map['allocation'] as num).toDouble() : 0.0,
      paused: map['paused'] as bool,
    );
  }
}

/// Modelo para transacciones de Vault
class VaultTransaction {
  final String id;
  final String type;
  final double amount;
  final DateTime date;
  final String userAddress;

  VaultTransaction({
    required this.id,
    required this.type,
    required this.amount,
    required this.date,
    required this.userAddress,
  });

  factory VaultTransaction.fromMap(Map<String, dynamic> map) {
    return VaultTransaction(
      id: map['id'] as String,
      type: map['type'] as String,
      amount: map['amount'] is num ? (map['amount'] as num).toDouble() : 0.0,
      date: DateTime.fromMillisecondsSinceEpoch(map['date'] as int),
      userAddress: map['userAddress'] as String,
    );
  }
}

/// Modelo para eventos de Vault utilizados para el cálculo de APY
class VaultEvent {
  final double amounts;
  final double dfTokens;
  final double previousPricePerShare;
  final double totalManagedFundsBefore;
  final double totalSupplyBefore;
  final String eventType;
  final int ledger;
  final DateTime date;

  VaultEvent({
    required this.amounts,
    required this.dfTokens,
    required this.previousPricePerShare,
    required this.totalManagedFundsBefore,
    required this.totalSupplyBefore,
    required this.eventType,
    required this.ledger,
    required this.date,
  });

  factory VaultEvent.fromMap(Map<String, dynamic> map) {
    return VaultEvent(
      amounts: map['amounts'] is num ? (map['amounts'] as num).toDouble() : 0.0,
      dfTokens: map['dfTokens'] is num ? (map['dfTokens'] as num).toDouble() : 0.0,
      previousPricePerShare: map['previousPricePerShare'] is num ? (map['previousPricePerShare'] as num).toDouble() : 0.0,
      totalManagedFundsBefore: map['totalManagedFundsBefore'] is num ? (map['totalManagedFundsBefore'] as num).toDouble() : 0.0,
      totalSupplyBefore: map['totalSupplyBefore'] is num ? (map['totalSupplyBefore'] as num).toDouble() : 0.0,
      eventType: map['eventType'] as String,
      ledger: map['ledger'] is num ? (map['ledger'] as num).toInt() : 0,
      date: map['date'] is DateTime 
          ? map['date'] 
          : map['date'] is int 
              ? DateTime.fromMillisecondsSinceEpoch(map['date'] as int) 
              : DateTime.now(),
    );
  }

  // Calcular el precio por acción (PPS)
  double calculatePricePerShare() {
    if (totalSupplyBefore <= 0) return 0;
    return totalManagedFundsBefore / totalSupplyBefore;
  }
}

/// Ejemplo de uso:
///
/// ```dart
/// // Crear un cliente GraphQL
/// final client = DeFindexGraphQLClient();
///
/// // Realizar una consulta
/// final result = await client.query(
///   DeFindexQueries.getVaultInfo,
///   variables: {'contractId': 'CB64D...'},
/// );
///
/// // Obtener datos formateados
/// final data = client.getResultData(result);
/// if (data != null) {
///   final vaultInfo = VaultQueryResponse.fromMap(data['vault']);
///   print('Vault total value: ${vaultInfo.totalValue}');
/// }
/// ```
