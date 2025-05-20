using DeFindex.Sdk.Services;
using Newtonsoft.Json;
using StellarDotnetSdk;
using StellarDotnetSdk.Accounts;
using StellarDotnetSdk.Responses.SorobanRpc;
using StellarDotnetSdk.Soroban;
using DotNetEnv;

class Program
{
    // var server = new Server("https://horizon-testnet.stellar.org");
    // var soroban_server = new Server("https://soroban-testnet.stellar.org/");

    async static Task Main(string[] args)
    {
        // Load environment variables from .env file
        Env.Load();

        if (args.Length == 0 || args.Length > 1 || (args[0] != "testnet" && args[0] != "mainnet"))
        {
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Please provide a network: testnet or mainnet");
            return;
        }
        var network = args[0];
        switch (network)
        {
            case "testnet":
                Console.WriteLine("Using testnet");
                Network.UseTestNetwork();
                break;
            case "mainnet":
                Console.WriteLine("Using mainnet");
                Network.UsePublicNetwork();
                break;
        }
        Console.ResetColor();

        var keypair = KeyPair.Random();
        Console.WriteLine("Generated public key: " + keypair.AccountId);

        var server = new Server("https://horizon-testnet.stellar.org");

        // var friendbot = server.TestNetFriendBot;
        // try {
        //     var response = await friendbot.FundAccount(keypair.AccountId).Execute();
        //     Console.ForegroundColor = ConsoleColor.Green;
        //     Console.WriteLine(response.Hash);
        //     Console.ResetColor();
        // } catch (Exception ex) {
        //     Console.ForegroundColor = ConsoleColor.Red;
        //     Console.Error.WriteLine("Error while funding account: " + ex.Message);
        //     Console.ResetColor();
        // }

        //View account balances
        // var account = await server.Accounts.Account(keypair.AccountId);
        // if(account.Balances[0].BalanceString != "10000.0000000"){
        //     Console.ForegroundColor = ConsoleColor.Red;
        //     Console.WriteLine("Account not funded");
        //     Console.ResetColor();
        // }
        // else
        // {
        //     Console.ForegroundColor = ConsoleColor.Green;
        //     Console.WriteLine("Ok.");
        //     Console.ResetColor();
        // }


        // Get SorobanServer URL from environment variable
        var sorobanServerUrl = Environment.GetEnvironmentVariable("MAINNET_RPC_URL") ?? "https://soroban-testnet.stellar.org/";
        Console.WriteLine("🚀 ~ sorobanServerUrl:", sorobanServerUrl);
        var soroban_server = new SorobanServer(sorobanServerUrl);

        var vault_string = "CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7";
        var vaultInstance = new DefindexSdk(vault_string, soroban_server);

        var vaultStrategies = await vaultInstance.GetVaultAPY();
        Console.WriteLine($"Vault APY: {vaultStrategies}");

        /* var vaultTotalShares = await vaultInstance.GetVaultTotalShares();
        Console.WriteLine($"Vault Total Shares: {vaultTotalShares}");

        var amountsDesired = new List<ulong> { 10000000 };
        var amountsMin = new List<ulong> { 10000000 };
        var from = keypair.AccountId;
        ConsoleInfo($"Creating deposit transaction for {from}.");
        
        var depositTransaction = await vaultInstance.CreateDepositTransaction(amountsDesired, amountsMin, from, false);

        ConsoleInfo("Simulating deposit transaction.");
        var simulatedDepositTransaction = await soroban_server.SimulateTransaction(depositTransaction);
        if(simulatedDepositTransaction.SorobanTransactionData == null || simulatedDepositTransaction.SorobanAuthorization == null || simulatedDepositTransaction.MinResourceFee == null){
            Console.WriteLine("Simulated transaction data is null.");
            return;
        }

        depositTransaction.SetSorobanTransactionData(simulatedDepositTransaction.SorobanTransactionData);
        depositTransaction.SetSorobanAuthorization(simulatedDepositTransaction.SorobanAuthorization);
        depositTransaction.AddResourceFee(simulatedDepositTransaction.MinResourceFee.Value + 100000);
        depositTransaction.Sign(keypair);
        
        ConsoleInfo("Submitting deposit transaction.");
        var submittedTx = await soroban_server.SendTransaction(depositTransaction);
        PrintTxResult("Deposit tx result", submittedTx);

        var depositTxResult = await CheckTransactionStatus(soroban_server, submittedTx.Hash);
        var parsedDepositTx = vaultInstance.ParseTransactionResponse(depositTxResult);
        DisplayParsedTransactionResponse(parsedDepositTx.Result);
        
        ConsoleInfo("Creating withdraw transaction.");
        ulong withdrawShares = 10000000;
        var amountsMinOut = new List<ulong> { 10000000 };
        var withdrawFrom = keypair.AccountId;
        var withdrawTransaction = await vaultInstance.CreateWithdrawTransaction(withdrawShares, amountsMinOut, withdrawFrom);

        ConsoleInfo("Simulating withdraw transaction.");
        var simulatedWithdrawTx = await soroban_server.SimulateTransaction(withdrawTransaction);
        if(simulatedWithdrawTx.SorobanTransactionData == null || simulatedWithdrawTx.SorobanAuthorization == null || simulatedWithdrawTx.MinResourceFee == null){
            Console.WriteLine("Simulated transaction data is null.");
            return;
        }
        withdrawTransaction.SetSorobanTransactionData(simulatedWithdrawTx.SorobanTransactionData);
        withdrawTransaction.SetSorobanAuthorization(simulatedWithdrawTx.SorobanAuthorization);
        withdrawTransaction.AddResourceFee(simulatedWithdrawTx.MinResourceFee.Value + 100000);
        withdrawTransaction.Sign(keypair);

        ConsoleInfo("Submitting withdraw transaction.");
        var submittedWithdrawTx = await soroban_server.SendTransaction(withdrawTransaction);
        PrintTxResult("Withdraw tx result", submittedWithdrawTx);

        var withdrawCheckedTx = await CheckTransactionStatus(soroban_server, submittedWithdrawTx.Hash);
        var parsedWithdrawTx = vaultInstance.ParseTransactionResponse(withdrawCheckedTx);
        DisplayParsedTransactionResponse(parsedWithdrawTx.Result);
        return; */
    }

    private static void ConsoleInfo(string message)
    {
        Console.ForegroundColor = ConsoleColor.DarkGray;
        Console.WriteLine($"\n{message}");
        Console.ResetColor();
    }

    private static void PrintTxResult(string label, SendTransactionResponse obj)
    {
        if(obj.ErrorResultXdr != null){
            Console.ForegroundColor = ConsoleColor.Gray;
            Console.Write($"\n{label}: ");
            Console.ForegroundColor = ConsoleColor.Red;
            Console.Write($"{obj.ErrorResultXdr}");
            Console.ResetColor();
            return;
        }
        Console.ForegroundColor = ConsoleColor.Gray;
        Console.Write($"\n{label}: ");
        Console.ForegroundColor = ConsoleColor.Green;
        Console.Write($"{JsonConvert.SerializeObject(obj, Formatting.Indented)}");
        Console.ResetColor();
    }
    private static void DisplayParsedTransactionResponse(object parsedResponse)
    {
        Console.ForegroundColor = ConsoleColor.Gray;
        Console.Write("\nParsed Transaction Response: ");
        Console.ForegroundColor = ConsoleColor.Green;
        Console.Write($"{JsonConvert.SerializeObject(parsedResponse, Formatting.Indented)}");
        Console.ResetColor();
    }

    private static async Task<GetTransactionResponse> CheckTransactionStatus(SorobanServer sorobanServer, string transactionHash)
    {
        Console.ForegroundColor = ConsoleColor.Yellow;
        Console.Write("\nChecking transaction status...");
        Console.ResetColor();
        GetTransactionResponse checkedTx;
        while (true)
        {
            var transactionResponse = await sorobanServer.GetTransaction(transactionHash);
            if (transactionResponse.Status.ToString() == "FAILED" || transactionResponse.Status.ToString() == "ERROR")
            {
                Console.ForegroundColor = ConsoleColor.Gray;
                Console.Write($"\nTransaction status: ");
                Console.ForegroundColor = ConsoleColor.Red;
                Console.Write($"{transactionResponse.Status}");
                Console.ForegroundColor = ConsoleColor.Gray;
                Console.Write($"\nTransaction hash: ");
                Console.ForegroundColor = ConsoleColor.Red;
                Console.Write($"{transactionHash}");
                Console.ResetColor();
                throw new Exception("Transaction failed.");
            }
            else if (transactionResponse.Status.ToString() == "SUCCESS")
            {
                Console.ForegroundColor = ConsoleColor.Gray;
                Console.Write($"\nTransaction Status: ");
                Console.ForegroundColor = ConsoleColor.Green;
                Console.Write($"{transactionResponse.Status}");
                Console.ForegroundColor = ConsoleColor.Gray;
                Console.Write($"\nTransaction hash: ");
                Console.ForegroundColor = ConsoleColor.Green;
                Console.Write($"{transactionHash}");
                Console.ResetColor();
                if (transactionResponse.ResultValue == null) throw new Exception("Transaction result value is null.");
                checkedTx = transactionResponse;
                return checkedTx;
            }
            else
            {
                Console.ForegroundColor = ConsoleColor.Yellow;
                Console.Write(".");
                await Task.Delay(50);
            }
        }
    }
}