using StellarDotnetSdk;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;
using StellarDotnetSdk.Accounts;
using DeFindex.Sdk.Interfaces;

namespace DeFindex.Sdk.Services;

using System;
using System.Collections.Generic;
using System.Threading.Tasks;

public class DefindexSdk : IDefindexSdk
{
    private readonly SCContractId _contractId;
    private readonly SorobanServer _server;

    public DefindexSdk(string contractId, SorobanServer server)
    {
        _contractId = new SCContractId(contractId);
        _server = server ?? throw new ArgumentNullException(nameof(server));
    }

    public string ContractId => _contractId.InnerValue;
    public SorobanServer Server => _server;

    public Transaction CreateBalanceTransaction(Account account, string accountIdToCheck)
    {
        var getBalanceArgs = new SCVal[] {
            new SCAccountId(accountIdToCheck),
        };
        var balanceSymbol = new SCSymbol("balance");

        var invokeContractOperation = new InvokeContractOperation(_contractId, balanceSymbol, getBalanceArgs);
        
        return new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
    }

    public Transaction FetchTotalManagedFunds(Account account)
    {
        var getManagedFundsArgs = new SCVal[] {};
        var managedFundsSymbol = new SCSymbol("fetch_total_managed_funds");

        var invokeContractOperation = new InvokeContractOperation(_contractId, managedFundsSymbol, getManagedFundsArgs);
        
        return new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
    }

    public async Task<bool> InitializeAsync()
    {
        Console.WriteLine("Starting SDK initialization...");
        // Implementation here
        Console.WriteLine("SDK initialization completed!");
        return true;
    }

    public async Task<VaultShares> GetUserShares(string accountId)
    {
        return new VaultShares(accountId, 0);
    }

    public async Task<Dictionary<string, VaultFunds>> FetchTotalManagedFunds()
    {
        return new Dictionary<string, VaultFunds>();
    }

    public async Task<decimal> GetVaultTotalShares()
    {
        return 0;
    }

    public async Task<Transaction> CreateDepositTransaction(
        List<long> amountsDesired,
        List<long> amountsMin,
        string from,
        bool invest)
    {
        throw new NotImplementedException();
    }

    public async Task<Transaction> CreateWithdrawTransaction(
        long withdrawShares,
        string from)
    {
        throw new NotImplementedException();
    }

    public async Task<TransactionResult> SubmitTransaction(Transaction transaction)
    {
        return new TransactionResult(false, null, new List<long>(), 0);
    }
} 