using StellarDotnetSdk.Accounts;
using StellarDotnetSdk;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Operations;
using StellarDotnetSdk.Transactions;
using DeFindex.Sdk.Services;
using Newtonsoft.Json;

class Program
{
    // var server = new Server("https://horizon-testnet.stellar.org");
    // var soroban_server = new Server("https://soroban-testnet.stellar.org/");

    async static Task Main(string[] args)
    {
        if(args.Length == 0 || args.Length > 1 || (args[0] != "testnet" && args[0] != "mainnet")){
            Console.ForegroundColor = ConsoleColor.Red;
            Console.WriteLine("Please provide a network: testnet or mainnet");
            return;
        }
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
        var sourceAccount = new Account(account.AccountId, account.SequenceNumber);

        var vault_string = "CDOQGZLNTWDQSYPSLYQ3R7LDUETUDZFYWJBLYNEGQLJLQKXTTC573LVW";
        var vaultInstance = new DefindexSdk(vault_string, soroban_server);
        var user_with_shares =  "GBI3XNPOBMTX5KUYOY742JVCSW4AWPR462IOBJZF3BM7IDAVTN5HHLM3";
        var simulatedTransactionResult = await vaultInstance.CreateBalanceTransaction(sourceAccount, user_with_shares);

        if (simulatedTransactionResult.Results != null)
        {
            foreach (var result in simulatedTransactionResult.Results)
            {
                Console.WriteLine($"🟢Result: {result.Xdr}");
            }
        }
        var xdrString = simulatedTransactionResult.Results?[0].Xdr;
        if (xdrString == null){

            Console.WriteLine("XDR string is null.");
            return;
        }

        var result_xdr = new StellarDotnetSdk.Xdr.XdrDataInputStream(Convert.FromBase64String(xdrString));
        var xdr = StellarDotnetSdk.Xdr.SCVal.Decode(result_xdr);
        Console.WriteLine($"Balance result: {xdr.I128.Lo.InnerValue}");
        
        var totalManagedFunds = await vaultInstance.FetchTotalManagedFunds();
        
        Console.WriteLine($"Total Managed Funds: {JsonConvert.SerializeObject(totalManagedFunds, Formatting.Indented)}");

        var userShares = await vaultInstance.GetUserShares(user_with_shares);
        Console.WriteLine($"User Shares: {userShares}");

        var vaultTotalShares = await vaultInstance.GetVaultTotalShares();
        Console.WriteLine($"Vault Total Shares: {vaultTotalShares}");

        var amountsDesired = new List<ulong> { 10000000 };
        var amountsMin = new List<ulong> { 10000000 };
        var from = keypair.AccountId;
        Console.WriteLine($"Creating deposit transaction for {from}");
        var depositTransaction = await vaultInstance.CreateDepositTransaction(amountsDesired, amountsMin, from, true);
        Console.WriteLine($"Simulating transaction...");
        var simulatedDepositTransaction = await soroban_server.SimulateTransaction(depositTransaction);
        Console.WriteLine($"XDR value: {simulatedDepositTransaction.Results?[0].Xdr}");
        depositTransaction.SetSorobanTransactionData(simulatedDepositTransaction.SorobanTransactionData);
        depositTransaction.SetSorobanAuthorization(simulatedDepositTransaction.SorobanAuthorization);
        depositTransaction.AddResourceFee(simulatedDepositTransaction.MinResourceFee.Value + 100000);
        depositTransaction.Sign(keypair);
        var depositResult = await vaultInstance.SubmitTransaction(depositTransaction);

        Console.WriteLine($"{JsonConvert.SerializeObject(depositResult, Formatting.Indented)}");

        ulong withdrawShares = 10000000;
        Console.WriteLine($"Creating withdraw transaction for {from}");
        var withdrawTransaction = await vaultInstance.CreateWithdrawTransaction(withdrawShares, from);

        Console.WriteLine($"Simulating withdraw transaction...");
        var simulatedWithdrawTransaction = await soroban_server.SimulateTransaction(withdrawTransaction);
        Console.WriteLine($"XDR value: {simulatedWithdrawTransaction.Results?[0].Xdr}");
        withdrawTransaction.SetSorobanTransactionData(simulatedWithdrawTransaction.SorobanTransactionData);
        withdrawTransaction.SetSorobanAuthorization(simulatedWithdrawTransaction.SorobanAuthorization);
        withdrawTransaction.AddResourceFee(simulatedWithdrawTransaction.MinResourceFee.Value + 100000);
        withdrawTransaction.Sign(keypair);
        var withdrawResult = await vaultInstance.SubmitTransaction(withdrawTransaction);

        return;
    }
}