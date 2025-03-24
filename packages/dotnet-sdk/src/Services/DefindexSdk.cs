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
        var args = new SCVal[] {};
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
        var args = new SCVal[] {};
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
        string from)
    {
        var account = await this._server.GetAccount(from);
        var args = new SCVal[] {
            new SCInt128(withdrawShares.ToString()),
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

    public async Task<List<TransactionResult>> SubmitTransaction(Transaction transaction)
    {
        //
        Console.WriteLine("Submitting transaction...");
        var submittedTx = await this._server.SendTransaction(transaction);

        if (submittedTx.ErrorResultXdr != null || submittedTx.Status.ToString() == "3")
        {
            Console.WriteLine($"Error submitting transaction: {submittedTx.ErrorResultXdr}");
            return new List<TransactionResult> { new TransactionResult(false, null, new List<ulong>(), 0) };
        } 
        else 
        while (true)
        {
            var checkedTx = await this._server.GetTransaction(submittedTx.Hash);
            if (checkedTx.Status.ToString() == "FAILED" || checkedTx.Status.ToString() == "ERROR")
            {
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine($"Transaction status: {checkedTx.Status}");
                Console.WriteLine($"Transaction hash: {submittedTx.Hash}");
                throw new Exception("Transaction failed.");
            }
            else if (checkedTx.Status.ToString() == "SUCCESS")
            {
                Console.ForegroundColor = ConsoleColor.Green;
                Console.WriteLine($"Transaction Status: {checkedTx.Status}");
                Console.WriteLine($"Transaction hash: {submittedTx.Hash}");
                Console.ResetColor();
                if (checkedTx.ResultValue == null) throw new Exception("Transaction result value is null.");
                var response = this.ParseTransactionResponse(checkedTx).Result;
                return response;
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Yellow;
                var timeout = 50;
                Console.Write($"...");
                await Task.Delay(timeout);
            }
        }
    }
} 