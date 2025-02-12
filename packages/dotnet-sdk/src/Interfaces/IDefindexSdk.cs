using StellarDotnetSdk.Transactions;
using StellarDotnetSdk;
using StellarDotnetSdk.Accounts;
using StellarDotnetSdk.Soroban;

namespace DeFindex.Sdk.Interfaces;

/// <summary>
/// Represents user's vault shares
/// </summary>
public sealed record VaultShares(string AccountId, decimal Shares);

/// <summary>
/// Represents vault's total managed funds per asset
/// </summary>
public sealed record VaultFunds(
    decimal TotalFunds,
    decimal IdleFunds,
    decimal TotalInvestedFunds,
    Dictionary<string, decimal> PerStrategyFunds);

/// <summary>
/// Represents the result of submitting a transaction
/// </summary>
public sealed record TransactionResult(
    bool IsSuccess,
    string? TransactionHash,
    List<long> Amounts,
    decimal SharesChanged);

public interface IDefindexSdk
{
    /// <summary>
    /// Gets the contract address
    /// </summary>
    string ContractId { get; }

    /// <summary>
    /// Gets the Soroban server instance
    /// </summary>
    SorobanServer Server { get; }

    /// <summary>
    /// Retrieves the number of vault shares a user owns
    /// </summary>
    Task<VaultShares> GetUserShares(string accountId);

    /// <summary>
    /// Retrieves vault's total funds, idle funds, and invested funds per strategy for each underlying asset
    /// </summary>
    Task<Dictionary<string, VaultFunds>> FetchTotalManagedFunds();

    /// <summary>
    /// Retrieves the total number of vault shares issued
    /// </summary>
    Task<decimal> GetVaultTotalShares();

    /// <summary>
    /// Creates an unsigned transaction to deposit into a vault
    /// </summary>
    Task<Transaction> CreateDepositTransaction(
        List<long> amountsDesired,
        List<long> amountsMin,
        string from,
        bool invest);

    /// <summary>
    /// Creates an unsigned transaction to withdraw from a vault
    /// </summary>
    Task<Transaction> CreateWithdrawTransaction(
        long withdrawShares,
        string from);

    /// <summary>
    /// Submits a signed transaction to the network
    /// </summary>
    Task<TransactionResult> SubmitTransaction(Transaction transaction);
} 