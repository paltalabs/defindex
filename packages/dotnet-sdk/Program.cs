using StellarDotnetSdk.Accounts;
using StellarDotnetSdk;

class Program
{
    // var server = new Server("https://horizon-testnet.stellar.org");

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
      
        Console.ForegroundColor = ConsoleColor.Yellow;
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