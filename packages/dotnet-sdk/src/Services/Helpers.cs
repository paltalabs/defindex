using System.Text.Json.Nodes;
using DeFindex.Sdk.Interfaces;
using StellarDotnetSdk.Accounts;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Responses.SorobanRpc;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Transactions;

public class DefindexHelpers
{
  public async Task<JsonObject?> FetchBlendDeployConfig()
    {
        using (var httpClient = new HttpClient())
        {
            try
            {
                var jsonUrl = "https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/apps/contracts/src/strategies/blend_deploy_config.json";
                var blendDeployConfigJson = await httpClient.GetStringAsync(jsonUrl);
                Console.WriteLine("blend_deploy_config.json fetched successfully.");
                return JsonNode.Parse(blendDeployConfigJson)?.AsObject();
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine($"Error fetching blend_deploy_config.json: {e.Message}");
                return null;
            }
        }
    }
    
    public async Task<JsonObject?> FetchDefindexDeployments(string networkName)
    {
        using (var httpClient = new HttpClient())
        {
            try
            {
                var jsonUrl = $"https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/{networkName}.contracts.json";
                var defindexDeploymentsJson = await httpClient.GetStringAsync(jsonUrl);
                return JsonNode.Parse(defindexDeploymentsJson)?.AsObject();
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine($"Error fetching {networkName}.contracts.json: {e.Message}");
                return null;
            }
        }
    }
    
    public List<string> FindBlendPoolAddresses(List<string> strategyIds, JsonArray blendStrategies)
    {
        var blendPoolAddresses = new List<string>();

        foreach (var strategyId in strategyIds)
        {
            foreach (var configEntryNode in blendStrategies)
            {
                if (configEntryNode is not JsonObject configEntry)
                    continue;

                var strategyName = configEntry["name"]?.GetValue<string>();
                var assetSymbol = configEntry["asset_symbol"]?.GetValue<string>();
                var blendPoolName = configEntry["blend_pool_name"]?.GetValue<string>();

                if (string.IsNullOrEmpty(strategyName) ||
                    string.IsNullOrEmpty(assetSymbol) ||
                    string.IsNullOrEmpty(blendPoolName))
                {
                    Console.WriteLine($"Invalid config entry: {configEntry}");
                    continue;
                }

                var expectedId = $"{assetSymbol.ToLowerInvariant()}_blend_{strategyName.ToLowerInvariant()}_{blendPoolName.ToLowerInvariant().Replace(" ", "_")}_strategy";

                if (!strategyId.Equals(expectedId, StringComparison.OrdinalIgnoreCase))
                    continue;

                var blendPoolAddress = configEntry["blend_pool_address"]?.GetValue<string>();
                if (!string.IsNullOrEmpty(blendPoolAddress))
                {
                    blendPoolAddresses.Add(blendPoolAddress);
                    break;
                }
                else
                {
                    Console.WriteLine($"Blend pool address not found for strategy ID: {strategyId}");
                }
            }
        }

        return blendPoolAddresses;
    }
    public List<string> ExtractStrategyIds(List<ManagedFundsResult> assetAllocation, JsonObject defindexDeploymentsJson)
    {
        if (defindexDeploymentsJson is null || !defindexDeploymentsJson.ContainsKey("ids"))
        return new List<string>();

        if (defindexDeploymentsJson["ids"] is not JsonObject idsObject)
        return new List<string>();

        var strategyIdLookup = idsObject
        .Where(kvp => kvp.Value is not null)
        .ToDictionary(
            kvp => kvp.Value!.ToString() ?? string.Empty,
            kvp => kvp.Key,
            StringComparer.OrdinalIgnoreCase
        );
        var strategiesIds = assetAllocation
        .SelectMany(asset => asset.StrategyAllocations)
        .Select(strategy => strategy.StrategyAddress)
        .Where(addr => addr is not null && strategyIdLookup.TryGetValue(addr, out _))
        .Select(addr => strategyIdLookup[addr!])
        .ToList();
        return strategiesIds;
    }

    public async Task<SimulateTransactionResponse?> CallContractMethod(string contractAddress, string methodName, SCVal[] args, SorobanServer server)
    {
        var keypair = KeyPair.Random();
        var loadedAccount = new Account(keypair.AccountId, 0);
        var invokeContractOperation = new InvokeContractOperation(new SCContractId(contractAddress), new SCSymbol(methodName), args);
        var transaction = new TransactionBuilder(loadedAccount)
            .AddOperation(invokeContractOperation)
            .Build();
        var simulatedTransaction = (SimulateTransactionResponse) await server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error calling contract method: {simulatedTransaction.Error}");
        }
        return simulatedTransaction;
    }
    
}