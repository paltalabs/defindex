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
        // var privateKey = "SC4NWRMSMK6CY4ZUYOLVWSL76GZQVK5FLKRT6JUZQKV224BK3SCFHBC4";
        // var publicKey = "GCH6YKNJ3KPESGSAIGBNHRNCIYXXXSRVU7OC552RDGQFHZ4SYRI26DQE";
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
        var contract_address = new SCContractId("CARDT45FED2I3FKESPMHDFV3ZMR6VH5ZHCFIOPH6TPSU35GPB6QBBCSU");
        var keypair_1 = KeyPair.FromSecretSeed("SC4NWRMSMK6CY4ZUYOLVWSL76GZQVK5FLKRT6JUZQKV224BK3SCFHBC4");
        var account_1 = await server.Accounts.Account(keypair_1.AccountId);
        var address = new StellarDotnetSdk.Soroban.SCAccountId("GCH6YKNJ3KPESGSAIGBNHRNCIYXXXSRVU7OC552RDGQFHZ4SYRI26DQE");
        var method_symbol = new StellarDotnetSdk.Soroban.SCSymbol("balance");
        var sc_tx_args = new StellarDotnetSdk.Soroban.SCVal[] {
            address
        };

        var invokeContractOperation = new InvokeContractOperation(contract_address, method_symbol, sc_tx_args);
        var transaction = new TransactionBuilder(account_1)
            .AddOperation(invokeContractOperation)
            .Build();

        Console.WriteLine("unsigned tx: " + transaction.ToUnsignedEnvelopeXdrBase64());
        var simulated_res = await soroban_server.SimulateTransaction(transaction);
        Console.WriteLine("Simulated transaction: " + simulated_res);
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