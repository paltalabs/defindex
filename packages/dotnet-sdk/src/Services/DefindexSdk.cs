using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;
using StellarDotnetSdk.Accounts;
using DeFindex.Sdk.Interfaces;
using DeFindex.Sdk.Services;
namespace DeFindex.Sdk.Services;

using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using StellarDotnetSdk.Responses.SorobanRpc;
using System.Text.Json.Nodes;
using Newtonsoft.Json;
using Paltalabs.Defindex.Services;
using System.Numerics;

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
    public DefindexHelpers Helpers = new DefindexHelpers();
    public SoroswapRouter Router = new SoroswapRouter();
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

    public async Task<List<uint>> GetVaultFee()
    {
        var simulatedTx = await Helpers.CallContractMethod(
            this._contractId.InnerValue,
            "get_fees",
            new SCVal[] { },
            this.Server
        );
        var result = simulatedTx?.Results?.FirstOrDefault()?.Xdr;
        if (result == null)
        {
            throw new Exception("Failed to get vault fee: result is null.");
        }
        var xdrDataInputStream = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(result));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(xdrDataInputStream).Vec.InnerValue;
        var results = new List<uint>();
        foreach (var item in xdr)
        {
            if (item.U32 is StellarDotnetSdk.Xdr.Uint32 uint32Value)
            {
                results.Add(uint32Value.InnerValue);
            }
            else
            {
                throw new Exception($"Unexpected type in results: {item.GetType()}");
            }
        }
        return results;
    }

    
    public async Task<decimal?> GetVaultAPY()
    {
        var assetAllocation = await this.FetchTotalManagedFunds();
        var network = await this.Server.GetNetwork();
        var networkName = network.Passphrase.IndexOf("public", StringComparison.OrdinalIgnoreCase) >= 0 ? "mainnet" : "testnet";

        var defindexDeploymentsJson = await Helpers.FetchDefindexDeployments(networkName);
        var blendDeployConfig = await Helpers.FetchBlendDeployConfig();
        if (defindexDeploymentsJson is null || !defindexDeploymentsJson.ContainsKey("ids")) return null;
        var strategiesIds = Helpers.ExtractStrategyIds(assetAllocation, defindexDeploymentsJson);
        if (strategiesIds is null || !strategiesIds.Any() || blendDeployConfig is null || !strategiesIds.Any())
            return null;

        if (!blendDeployConfig.TryGetPropertyValue(networkName, out var networkConfigNode) || networkConfigNode is not JsonObject networkConfig)
            return null;

        if (!networkConfig.TryGetPropertyValue("strategies", out var strategiesNode) || strategiesNode is not JsonArray blendStrategiesArray)
            return null;

        var blendPoolAddressesFound = Helpers.FindBlendPoolAddresses(strategiesIds, blendStrategiesArray);
        var reserves = await Router.GetPairReserves(assetAllocation[0].Asset!, Utils.BLND, this.Server);

        // Uncomment the following lines to fetch pool configurations
        var poolConfigDict = new Dictionary<string, PoolConfig>();
        foreach (var (strategyId, poolAddress) in blendPoolAddressesFound)
        {
            var strategyAddress = Helpers.GetStrategyAddressFromId(strategyId, defindexDeploymentsJson);
            if (strategyAddress == null)
            {
                Console.WriteLine($"Could not find strategy address for ID: {strategyId}");
                continue;
            }

            var blendPoolConfig = await Helpers.CallContractMethod(poolAddress, "get_config", new SCVal[] { }, this.Server);
            if (blendPoolConfig is null || blendPoolConfig.Error != null || blendPoolConfig.Results == null || blendPoolConfig.Results.Count() == 0)
            {
                Console.WriteLine($"Error calling get_config on pool {poolAddress}: {blendPoolConfig?.Error}");
                continue;
            }
            var parsedResponse = DefindexResponseParser.ParsePoolConfigResult(blendPoolConfig);
            poolConfigDict[strategyAddress] = parsedResponse;
        }

        // Uncomment the following lines to fetch pool reserves
        var reserveDataDict = new Dictionary<string, Reserve>();
        foreach (var (strategyId, poolAddress) in blendPoolAddressesFound)
        {
            var strategyAddress = Helpers.GetStrategyAddressFromId(strategyId, defindexDeploymentsJson);
            if (strategyAddress == null)
            {
                Console.WriteLine($"Could not find strategy address for ID: {strategyId}");
                continue;
            }

            var args = new SCVal[] {
                new SCContractId(assetAllocation[0].Asset!),
            };
            var blendPoolReserves = await Helpers.CallContractMethod(poolAddress, "get_reserve", args, this.Server);
            if (blendPoolReserves is null || blendPoolReserves.Error != null || blendPoolReserves.Results == null || blendPoolReserves.Results.Count() == 0)
            {
                Console.WriteLine($"Error calling get_reserves on pool {poolAddress}: {blendPoolReserves?.Error}");
                continue;
            }
            var parsedResponse = DefindexResponseParser.ParseReserveResult(blendPoolReserves);
            reserveDataDict[strategyAddress] = parsedResponse;
        }

        // Uncomment the following lines to fetch reserve emissions
        var reserveEmissionsDict = new Dictionary<string, ReserveEmissionData>();
        foreach (var (strategyId, poolAddress) in blendPoolAddressesFound)
        {   

            var strategyAddress = Helpers.GetStrategyAddressFromId(strategyId, defindexDeploymentsJson);
            if (strategyAddress == null)
            {
                Console.WriteLine($"Could not find strategy address for ID: {strategyId}");
                continue;
            }
            var id = reserveDataDict[strategyAddress].Config.Index * 2 + 1;
            var args = new SCVal[] {
                new SCUint32(id),
            };
            try
            {
                var bpReserveEmissions = await Helpers.CallContractMethod(poolAddress, "get_reserve_emissions", args, this.Server);
                if (bpReserveEmissions is null || bpReserveEmissions.Error != null || bpReserveEmissions.Results == null || bpReserveEmissions.Results.Count() == 0)
                {
                    Console.WriteLine($"Error calling get_reserve_emissions on pool {poolAddress}: {bpReserveEmissions?.Error}");
                    continue;
                }
                var parsedResponse = DefindexResponseParser.ParseReserveEmissionData(bpReserveEmissions);
                if (parsedResponse == null)
                {
                    throw new Exception($"Parsed response is null for strategy address: {strategyAddress}");
                }
                reserveEmissionsDict[strategyAddress] = parsedResponse;
            }
            catch (Exception ex)
            {
                reserveEmissionsDict[strategyAddress] = new ReserveEmissionData
                {
                    Eps = 0,
                    Expiration = 0,
                    Index = BigInteger.Zero,
                    LastTime = 0
                };
                continue;
            }
        }

        var defindexVaultFees = await GetVaultFee();
        var vaultFee = defindexVaultFees.Count > 0 ? defindexVaultFees[0] : 0; // Default to 0 if no fees are found

        var apy = Utils.calculateAssetAPY(
            poolConfigDict,
            reserveEmissionsDict,
            reserveDataDict,
            assetAllocation[0],
            reserves,
            vaultFee
        );

        return apy;
    }

}