import '../defindex_sdk.dart';
import '../graph_ql_server.dart';

/// Utilidades para el manejo de eventos de Vault

/// Constante para el número de ledgers por día
const dayInLedgers = 17280;

/// Convierte los datos en bruto a una lista de objetos VaultEvent
List<VaultEvent> parseEventsFromData(Map<String, dynamic> data) {
  if (!data.containsKey('deFindexVaults')) return [];
  
  final nodes = data['deFindexVaults']['nodes'] as List;
  return nodes
      .map((node) => VaultEvent.fromMap(node as Map<String, dynamic>))
      .toList();
}

/// Obtiene un par de eventos de vault (actual y pasado) para cálculos de APY
Future<VaultEventPair?> fetchVaultEventPair(String contractId) async {
  final graphQLClient = DeFindexGraphQLClient();
  
  final lastEvent = await graphQLClient.query(
    DeFindexQueries.getVaultEvent,
    variables: {
      'vaultAddress': contractId,
      'orderBy': 'LEDGER_DESC', 
      'first': 1, 
    },
  );
  final lastEventData = graphQLClient.getResultData(lastEvent);
  
  if (lastEventData == null || 
      !lastEventData.containsKey('deFindexVaults') || 
      lastEventData['deFindexVaults']['nodes'].isEmpty) {
    return null;
  }
  
  final lastEventLedger = lastEventData['deFindexVaults']['nodes'][0]['ledger'] as int;
  final pastEventLedger = lastEventLedger - (dayInLedgers * 7); 

  final pastEvent = await graphQLClient.query(
    DeFindexQueries.getVaultEventByLedger,
    variables: {
      'vaultAddress': contractId,
      'orderBy': 'DATE_DESC', 
      'last': 1,
      'ledger': pastEventLedger,
    },
  );
  
  if (pastEvent.hasException) return null;
  
  final currentData = lastEventData;
  final pastData = graphQLClient.getResultData(pastEvent);

  if (pastData == null) return null;
  
  try {
    final List<VaultEvent> currentEvents = parseEventsFromData(currentData);
    final List<VaultEvent> pastEvents = parseEventsFromData(pastData);
    
    if (currentEvents.isEmpty || pastEvents.isEmpty) return null;
    
    return VaultEventPair(currentEvents.first, pastEvents.first);
  } catch (e) {
    return null;
  }
}
