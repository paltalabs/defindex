using StellarDotnetSdk.Transactions;
using StellarDotnetSdk.Soroban;
using StellarDotnetSdk.Responses.SorobanRpc;

namespace DeFindex.Sdk.Interfaces;

/// <summary>
/// Represents the number of vault shares a user owns
/// </summary>
/// <param name="AccountId"></param>
/// <param name="Shares"></param>
public sealed record VaultShares
(
    string AccountId, 
    ulong Shares
);

/// <summary>
/// Represents the result of a total managed funds query for a vault
/// </summary>
/// <param name="Asset"></param>
/// <param name="IdleAmount"></param>
/// <param name="InvestedAmount"></param>
/// <param name="TotalAmount"></param>
/// <param name="StrategyAllocations"></param>
public sealed record ManagedFundsResult
(
    string? Asset,
    ulong IdleAmount,
    ulong InvestedAmount,
    ulong TotalAmount,
    List<StrategyAllocation> StrategyAllocations
);

/// <summary>
/// Represents the allocation of funds to a strategy
/// </summary>
/// <param name="Amount"></param>
/// <param name="Paused"></param>
/// <param name="StrategyAddress"></param>
public sealed record StrategyAllocation
(
    ulong Amount,
    bool Paused,
    string? StrategyAddress
);


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
    List<ulong> Amounts,
    ulong SharesChanged);

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
    Task<List<ManagedFundsResult>> FetchTotalManagedFunds();

    /// <summary>
    /// Retrieves the total number of vault shares issued
    /// </summary>
    Task<ulong> GetVaultTotalShares();

    /// <summary>
    /// Creates an unsigned transaction to deposit into a vault
    /// </summary>
    Task<Transaction> CreateDepositTransaction(
        List<ulong> amountsDesired,
        List<ulong> amountsMin,
        string from,
        bool invest);

    /// <summary>
    /// Creates an unsigned transaction to withdraw from a vault
    /// </summary>
    Task<Transaction> CreateWithdrawTransaction(
        ulong withdrawShares,
        List<ulong> amountsMinOut,
        string from);


    /// <summary>
    /// Parse a successful transaction response
    /// </summary>
    Task<List<TransactionResult>> ParseTransactionResponse(GetTransactionResponse response);
}