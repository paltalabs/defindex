using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;
using StellarDotnetSdk.Accounts;
using DeFindex.Sdk.Interfaces;

namespace DeFindex.Sdk.Services;

using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using StellarDotnetSdk.Responses.SorobanRpc;
using System.Net.Http;
using System.Text.Json.Nodes;
using Newtonsoft.Json;

public class DefindexSdk : IDefindexSdk
{
    private readonly SCContractId _contractId;
    private readonly SorobanServer _server;

    private string? _blendDeployConfigJson;

    private string? _defindexDeploymentsJson;

    public DefindexSdk(string contractId, SorobanServer server)
    {
        _contractId = new SCContractId(contractId);
        _server = server ?? throw new ArgumentNullException(nameof(server));
    }

    public string ContractId => _contractId.InnerValue;
    public SorobanServer Server => _server;

    public async Task<SimulateTransactionResponse> CreateBalanceTransaction(Account account, string accountIdToCheck)
    {
        var getBalanceArgs = new SCVal[] {
            new SCAccountId(accountIdToCheck),
        };
        var balanceSymbol = new SCSymbol("balance");
        var invokeContractOperation = new InvokeContractOperation(_contractId, balanceSymbol, getBalanceArgs);

        var transaction = new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
        var simulatedTransaction = await this.Server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null)
        {
            Console.WriteLine($"Error while simulating transaction: {simulatedTransaction.Error}");
            throw new Exception($"Error while simulating transaction: {simulatedTransaction.Error}");
        }
        if (simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception("No results found in the simulated transaction.");
        }
        var xdrString = simulatedTransaction.Results[0].Xdr;
        if (string.IsNullOrEmpty(xdrString))
        {
            throw new Exception("XDR string is null or empty.");
        }
        var resultXdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(resultXdr);
        var parsedResult = (SCInt128)SCInt128.FromSCValXdr(xdr);
        Console.WriteLine($"get balance transaction: {parsedResult.Lo}");

        return simulatedTransaction;
    }

    public async Task<List<ManagedFundsResult>> FetchTotalManagedFunds()
    {
        var keypair = KeyPair.Random();
        var args = new SCVal[] { };
        var symbol = new SCSymbol("fetch_total_managed_funds");
        var invokeContractOperation = new InvokeContractOperation(_contractId, symbol, args);

        var loadedAccount = new Account(keypair.AccountId, 0);
        var transaction = new TransactionBuilder(loadedAccount)
            .AddOperation(invokeContractOperation)
            .Build();

        var simulatedTransaction = await _server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error fetching total managed funds: {simulatedTransaction.Error}");
        }
        var parsedResult = DefindexResponseParser.ParseManagedFundsResult(simulatedTransaction);
        return parsedResult;
    }

    public bool InitializeAsync()
    {
        Console.WriteLine("Starting SDK initialization...");
        // Implementation here
        Console.WriteLine("SDK initialization completed!");
        return true;
    }

    public async Task<VaultShares> GetUserShares(string accountId)
    {
        var keypair = KeyPair.Random();
        var args = new SCVal[] {
            new SCAccountId(accountId),
        };
        var symbol = new SCSymbol("balance");
        var invokeContractOperation = new InvokeContractOperation(_contractId, symbol, args);
        var loadedAccount = new Account(keypair.AccountId, 0);
        var transaction = new TransactionBuilder(loadedAccount)
            .AddOperation(invokeContractOperation)
            .Build();

        var simulatedTransaction = await _server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error getting user shares: {simulatedTransaction.Error}");
        }
        var xdrString = simulatedTransaction.Results[0].Xdr;
        var resultXdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString!));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(resultXdr);
        var parsedResult = (SCInt128)SCInt128.FromSCValXdr(xdr);
        var vaultShares = new VaultShares(accountId, parsedResult.Lo);
        return vaultShares;
    }

    public async Task<ulong> GetVaultTotalShares()
    {
        //Simular llamada a la función total_supply
        var keypair = KeyPair.Random();
        var args = new SCVal[] { };
        var symbol = new SCSymbol("total_supply");
        var invokeContractOperation = new InvokeContractOperation(_contractId, symbol, args);
        var loadedAccount = new Account(keypair.AccountId, 0);
        var transaction = new TransactionBuilder(loadedAccount)
            .AddOperation(invokeContractOperation)
            .Build();

        var simulatedTransaction = await _server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error getting vault total shares: {simulatedTransaction.Error}");
        }
        var xdrString = simulatedTransaction.Results[0].Xdr;
        var resultXdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString!));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(resultXdr);
        var parsedResult = (SCInt128)SCInt128.FromSCValXdr(xdr);
        return parsedResult.Lo;
    }

    public async Task<Transaction> CreateDepositTransaction(
        List<ulong> amountsDesired,
        List<ulong> amountsMin,
        string from,
        bool invest)
    {
        var account = await this._server.GetAccount(from);
        var args = new SCVal[] {
            new SCVec(amountsDesired.Select(a => new SCInt128(a.ToString())).ToArray()),
            new SCVec(amountsMin.Select(a => new SCInt128(a.ToString())).ToArray()),
            new SCAccountId(from),
            new SCBool(invest),
        };
        var symbol = new SCSymbol("deposit");
        var invokeContractOperation = new InvokeContractOperation(_contractId, symbol, args);
        var transaction = new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
        var simulatedTransaction = await this._server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error creating deposit transaction: {simulatedTransaction.Error}");
        }
        return transaction;
    }

    public async Task<Transaction> CreateWithdrawTransaction(
        ulong withdrawShares,
        List<ulong> amountsMinOut,
        string from)
    {
        var account = await this._server.GetAccount(from);
        var args = new SCVal[] {
            new SCInt128(withdrawShares.ToString()),
            new SCVec(amountsMinOut.Select(a => new SCInt128(a.ToString())).ToArray()),
            new SCAccountId(from),
        };
        var symbol = new SCSymbol("withdraw");
        var invokeContractOperation = new InvokeContractOperation(_contractId, symbol, args);
        var transaction = new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
        var simulatedTransaction = await this._server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error creating withdraw transaction: {simulatedTransaction.Error}");
        }
        return transaction;
    }

    public Task<List<TransactionResult>> ParseTransactionResponse(GetTransactionResponse txResponse)
    {
        if (txResponse.ResultValue == null || txResponse.TxHash == null)
        {
            throw new Exception("Transaction result value is null.");
        }
        var result = (SCVal)SCVal.FromXdrBase64(txResponse.ResultValue.ToXdrBase64());
        var parsedResponse = DefindexResponseParser.ParseSubmittedTransaction(result, txResponse.TxHash);
        return Task.FromResult(parsedResponse);
    }

    public async Task<JsonObject?> FetchBlendDeployConfig()
    {
        using (var httpClient = new HttpClient())
        {
            try
            {
                var jsonUrl = "https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/apps/contracts/src/strategies/blend_deploy_config.json";
                _blendDeployConfigJson = await httpClient.GetStringAsync(jsonUrl);
                Console.WriteLine("blend_deploy_config.json fetched successfully.");
                // Puedes procesar el _blendDeployConfigJson aquí o simplemente devolverlo/usarlo.
                return JsonNode.Parse(_blendDeployConfigJson)?.AsObject();
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine($"Error fetching blend_deploy_config.json: {e.Message}");
                return null;
            }
        }
    }
    
    public async Task<JsonObject?> FetchDefindexDeployments()
    {
        var network = (GetNetworkResponse)await this.Server.GetNetwork();
        var networkName = network.Passphrase.IndexOf("public", StringComparison.OrdinalIgnoreCase) >= 0 ? "mainnet" : "testnet";
        using (var httpClient = new HttpClient())
        {
            try
            {
                var jsonUrl = $"https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/{networkName}.contracts.json";
                _defindexDeploymentsJson = await httpClient.GetStringAsync(jsonUrl);

                Console.WriteLine($"{networkName}.contracts.json fetched successfully.");
                return JsonNode.Parse(_defindexDeploymentsJson)?.AsObject();
            }
            catch (HttpRequestException e)
            {
                Console.WriteLine($"Error fetching {networkName}.contracts.json: {e.Message}");
                return null;
            }
        }
    }
    
    List<string> FindBlendPoolAddresses(List<string> strategyIds, JsonArray blendStrategies)
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
    List<string> ExtractStrategyIds(List<ManagedFundsResult> assetAllocation, JsonObject defindexDeploymentsJson)
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

    public async Task<SimulateTransactionResponse?> CallContractMethod(string contractAddress, string methodName, SCVal[] args)
    {
        var keypair = KeyPair.Random();
        var loadedAccount = new Account(keypair.AccountId, 0);
        var invokeContractOperation = new InvokeContractOperation(new SCContractId(contractAddress), new SCSymbol(methodName), args);
        var transaction = new TransactionBuilder(loadedAccount)
            .AddOperation(invokeContractOperation)
            .Build();
        var simulatedTransaction = (SimulateTransactionResponse) await this.Server.SimulateTransaction(transaction);
        if (simulatedTransaction.Error != null || simulatedTransaction.Results == null || simulatedTransaction.Results.Count() == 0)
        {
            throw new Exception($"Error calling contract method: {simulatedTransaction.Error}");
        }
        return simulatedTransaction;
    }
    
    public async Task<decimal?> GetVaultAPY()
    {
        var assetAllocation = await this.FetchTotalManagedFunds();
        var defindexDeploymentsJson = await this.FetchDefindexDeployments();
        var blendDeployConfig = await this.FetchBlendDeployConfig();
        if (defindexDeploymentsJson is null || !defindexDeploymentsJson.ContainsKey("ids")) return null;
        var strategiesIds = ExtractStrategyIds(assetAllocation, defindexDeploymentsJson);
        if (strategiesIds is null || !strategiesIds.Any() || blendDeployConfig is null || !strategiesIds.Any())
            return null;
        var networkResponse = await this.Server.GetNetwork();
        var networkName = networkResponse.Passphrase.IndexOf("public", StringComparison.OrdinalIgnoreCase) >= 0 ? "mainnet" : "testnet";

        if (!blendDeployConfig.TryGetPropertyValue(networkName, out var networkConfigNode) || networkConfigNode is not JsonObject networkConfig)
            return null;

        if (!networkConfig.TryGetPropertyValue("strategies", out var strategiesNode) || strategiesNode is not JsonArray blendStrategiesArray)
            return null;

        var blendPoolAddressesFound = FindBlendPoolAddresses(strategiesIds, blendStrategiesArray);
        foreach (var pool in blendPoolAddressesFound)
        {
            var result = await CallContractMethod(pool, "get_config", new SCVal[] { });
            if (result is null || result.Error != null || result.Results == null || result.Results.Count() == 0)
            {
                Console.WriteLine($"Error calling get_config on pool {pool}: {result?.Error}");
                continue;
            }
            var parsedResponse = DefindexResponseParser.ParsePoolConfigResult(result);
            Console.WriteLine($"Parsed PoolConfig: {JsonConvert.SerializeObject(parsedResponse, Formatting.Indented)}");
        }

        return 0.0m;
    }

}