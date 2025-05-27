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
        try
        {
            var assetAllocation = await FetchTotalManagedFunds();
            var network = await Server.GetNetwork();
            var networkName = network.Passphrase.IndexOf("public", StringComparison.OrdinalIgnoreCase) >= 0 ? "mainnet" : "testnet";
            
            var defindexDeploymentsJson = await Helpers.FetchDefindexDeployments(networkName);
            var blendDeployConfig = await Helpers.FetchBlendDeployConfig();
            if (defindexDeploymentsJson is null || !defindexDeploymentsJson.ContainsKey("ids")) return null;
            var strategiesIds = Helpers.ExtractStrategyIds(assetAllocation, defindexDeploymentsJson);
            if (strategiesIds is null || !strategiesIds.Any() || blendDeployConfig is null || !strategiesIds.Any())
                return null;

            var networkConfig = Utils.GetNetworkConfig(blendDeployConfig, networkName);
            if (networkConfig == null) return null;
            var blendStrategiesArray = Utils.GetBlendStrategiesArray(networkConfig);
            if (blendStrategiesArray == null) return null;

            var blendPoolAddressesFound = Helpers.FindBlendPoolAddresses(strategiesIds, blendStrategiesArray);
            var reserves = await Router.GetPairReserves(assetAllocation[0].Asset!, Utils.BLND, Server);

            var poolConfigDict = await Utils.FetchPoolConfigs(blendPoolAddressesFound, defindexDeploymentsJson, Server);
            var reserveDataDict = await Utils.FetchReserveData(blendPoolAddressesFound, defindexDeploymentsJson, assetAllocation[0].Asset!, Server);
            var reserveEmissionsDict = await Utils.FetchReserveEmissions(blendPoolAddressesFound, defindexDeploymentsJson, reserveDataDict, Server);

            var defindexVaultFees = await GetVaultFee();
            var vaultFee = defindexVaultFees.Count > 0 ? defindexVaultFees[0] : 0;

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
        catch (Exception ex)
        {
            Console.WriteLine($"Error in GetVaultAPY: {ex.Message}");
            return null;
        }
    }

}