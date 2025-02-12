using StellarDotnetSdk;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;
using StellarDotnetSdk.Accounts;

namespace DeFindex.Sdk.Services;

using System;
using DeFindex.Sdk.Interfaces;

public class DefindexSdk : IDefindexSdk
{
    private readonly SCContractId _address;

    public DefindexSdk(string address)
    {
        _address = new SCContractId(address);
    }

    public Transaction CreateBalanceTransaction(Account account, string accountIdToCheck)
    {
        var getBalanceArgs = new SCVal[] {
            new SCAccountId(accountIdToCheck),
        };
        var balanceSymbol = new SCSymbol("balance");

        var invokeContractOperation = new InvokeContractOperation(_address, balanceSymbol, getBalanceArgs);
        
        return new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
    }

    public Transaction FetchTotalManagedFunds(Account account)
    {
        var getManagedFundsArgs = new SCVal[] {};
        var managedFundsSymbol = new SCSymbol("fetch_total_managed_funds");

        var invokeContractOperation = new InvokeContractOperation(_address, managedFundsSymbol, getManagedFundsArgs);
        
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
} 