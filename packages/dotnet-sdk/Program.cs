using StellarDotnetSdk.Accounts;
using StellarDotnetSdk;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Xdr;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;

class Program
{
    // var server = new Server("https://horizon-testnet.stellar.org");
    // var soroban_server = new Server("https://soroban-testnet.stellar.org/");

    async static Task Main(string[] args)
    {
        if(args.Length == 0 || args.Length > 1 || (args[0] != "testnet" && args[0] != "publicnet")){
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Please provide a network: testnet or publicnet");
            return;
        }
        // var account = GetAccountInfo("GCEZQZLDSZQYHJ7ZDWRHTK6V4RR4W2QZ5DKK2UY62VVQZJ53L2UQ");
        // Console.WriteLine(account);
        var network = args[0];
        switch (network) {
            case "testnet":
                Console.WriteLine("Using testnet");
                Network.UseTestNetwork();
                break;
            case "mainnet":
                Console.WriteLine("Using mainnet");
                Network.UsePublicNetwork();
                Console.ForegroundColor = ConsoleColor.Red;
                Console.WriteLine("Mainnet is not yet supported");
                return;
        }
        Console.ResetColor();

        var keypair = KeyPair.Random();
        Console.WriteLine("Generated public key: " + keypair.AccountId);

        var server = new Server("https://horizon-testnet.stellar.org");
        var friendbot = server.TestNetFriendBot;
        try {
            var response = await friendbot.FundAccount(keypair.AccountId).Execute();
            Console.ForegroundColor = ConsoleColor.Green;
            Console.WriteLine(response.Hash);
            Console.ResetColor();
        } catch (Exception ex) {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.Error.WriteLine("Error while funding account: " + ex.Message);
            Console.ResetColor();
        }

        //View account balances
        var account = await server.Accounts.Account(keypair.AccountId);
        if(account.Balances[0].BalanceString != "10000.0000000"){
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Account not funded");
            Console.ResetColor();
        } else {
            Console.ForegroundColor = ConsoleColor.Green;
            Console.WriteLine("Ok.");
            Console.ResetColor();
        }

        var soroban_server = new SorobanServer("https://soroban-testnet.stellar.org/");
  /*       var contract_address = new SCContractId("CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
        var method = new StellarDotnetSdk.Soroban.SCSymbol("Balance");
        var call_args = new StellarDotnetSdk.Soroban.SCVal[] {};

        var invokeContract = new InvokeContractHostFunction(contract_address, method, call_args);
        var xdr = new StellarDotnetSdk.Xdr.Operation { 
            Body = new StellarDotnetSdk.Xdr.Operation.OperationBody { 
                
            }
        };
        var operation = (InvokeHostFunctionOperation.FromXdr(xdr));
    
        var transaction = new TransactionBuilder(account)
            .AddOperation(operation)
            .Build(); */

        //var contractAddress = new StellarDotnetSdk.Soroban.SCAddress("CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC");
        var contract_address = new SCContractId("CARDT45FED2I3FKESPMHDFV3ZMR6VH5ZHCFIOPH6TPSU35GPB6QBBCSU");

        var method_symbol = new StellarDotnetSdk.Soroban.SCSymbol("balance");
        var sc_tx_args = new StellarDotnetSdk.Soroban.SCVal[] { }; // Argumentos de ejemplo

        var invokeContractOperation = new InvokeContractOperation(contract_address, method_symbol, sc_tx_args, keypair);
        var transaction = new TransactionBuilder(account)
            .AddOperation(invokeContractOperation)
            .Build();
        transaction.Sign(keypair);

        Console.WriteLine(transaction.ToEnvelopeXdrBase64());

        var res = await soroban_server.SendTransaction(transaction);
        Console.WriteLine(res.Status);
        Console.WriteLine(res.Hash);
        Console.WriteLine(res.ErrorResultXdr);
        return;
    }

    // // Example: Get account information
    // public async Task<AccountResponse> GetAccountInfo(string accountId)
    // {
    //     try
    //     {
    //     var account = await server.Accounts.Account(accountId);
    //     return account;
    // }
    // catch (Exception ex)
    // {
    //         Console.WriteLine($"Error getting account info: {ex.Message}");
    //         throw;
    //     }
    // }
} 