using System;
using System.Threading;
using StellarDotnetSdk;

class Program
{
    // var server = new Server("https://horizon-testnet.stellar.org");

    static void Main(string[] args)
    {
        // var account = GetAccountInfo("GCEZQZLDSZQYHJ7ZDWRHTK6V4RR4W2QZ5DKK2UY62VVQZJ53L2UQ");
        // Console.WriteLine(account);

        for (int i = 0; i < 5; i++)
        {
            Console.WriteLine($"Test output {i}");
            Thread.Sleep(1000); // Wait 1 second between outputs
        }
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