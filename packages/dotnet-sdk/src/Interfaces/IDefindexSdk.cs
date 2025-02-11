using StellarDotnetSdk.Transactions;
using StellarDotnetSdk;
using StellarDotnetSdk.Accounts;

namespace DeFindex.Sdk.Interfaces;

public interface IDefindexSdk
{
    Transaction CreateBalanceTransaction(Account account, string accountIdToCheck);
    // Add more transaction creation methods as needed
} 