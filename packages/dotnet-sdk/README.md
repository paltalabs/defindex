# DeFindex .NET SDK

.NET SDK for the DeFindex protocol.

## Installation

## Usage
# DeFindex .NET SDK Specification

## Overview

The SDK is designed to interact with the DeFindex vault system, allowing users to retrieve vault shares and managed funds while maintaining clear distinctions between vault shares and underlying assets.

## Terminology

- **Shares**: Represents ownership of a vault.
- **Funds**: Represents the underlying assets managed by the vault (e.g., USDC, XLM).
- **Vault**: The contract managing investments and distributions.

## SDK Initialization

The `Defindex` class requires a contract address and a `SorobanServer` instance from the dotnet-sdk. Clients must instantiate the `SorobanServer` with their RPC URL and bearer tokens before passing it to `Defindex` for initialization.

## SDK Methods

### 1. GetUserShares

Retrieves the number of vault shares a user owns.

#### Method Signature

```csharp
public async Task<VaultShares> GetUserShares(string accountId)
```

#### Returns

- `VaultShares` object containing:
  - `AccountId`: The user’s account ID.
  - `Shares`: The number of vault shares owned.

---

### 2. FetchTotalManagedFunds

Retrieves vault's total funds, idle funds, and invested funds per strategy for each underlying asset.

#### Method Signature

```csharp
public async Task<Dictionary<string, VaultFunds>> FetchTotalManagedFunds()
```

#### Returns

- `Dictionary<string, VaultFunds>` where:
  - The key (`string`) is the underlying asset code (e.g., "USDC").
  - The value (`VaultFunds`) contains:
    - `TotalFunds`: Total amount of this asset in the vault.
    - `IdleFunds`: Amount of this asset not currently invested.
    - `TotalInvestedFunds`: Amount of this asset currently invested.
    - `PerStrategyFunds`: A dictionary mapping **strategy contract IDs** to **total funds invested**.

#### Example

```csharp
{
  "USDC": {
    "TotalFunds": 300000,
    "IdleFunds": 50000,
    "TotalInvestedFunds": 250000,
    "PerStrategyFunds": {
      "STRATEGY_1": 150000,
      "STRATEGY_2": 100000
    }
  },
  "XLM": {
    "TotalFunds": 200000,
    "IdleFunds": 50000,
    "TotalInvestedFunds": 150000,
    "PerStrategyFunds": {
      "STRATEGY_3": 80000,
      "STRATEGY_4": 70000
    }
  }
}
```

---

### 3. GetVaultTotalShares

Retrieves the total number of vault shares issued.

#### Method Signature

```csharp
public async Task<decimal> GetVaultTotalShares()
```

#### Returns

- `decimal`: Total number of shares in circulation.

---

### 4. CreateDepositTransaction

Creates an unsigned transaction to deposit into a vault.

#### Method Signature

```csharp
public async Task<Transaction> CreateDepositTransaction(
    List<long> amountsDesired,
    List<long> amountsMin,
    Address from,
    bool invest)
```

#### Inputs

- `amountsDesired`: List of desired deposit amounts.
- `amountsMin`: List of minimum acceptable deposit amounts.
- `from`: Address of the depositor.
- `invest`: Whether the deposit invests immediately into strategies.

#### Returns

- `Transaction`: The unsigned deposit transaction.

---

### 5. CreateWithdrawTransaction

Creates an unsigned transaction to withdraw from a vault.

#### Method Signature

```csharp
public async Task<Transaction> CreateWithdrawTransaction(
    long withdrawShares,
    Address from)
```

#### Inputs

- `withdrawShares`: Amount of vault shares to withdraw.
- `from`: Address of the withdrawer.

#### Returns

- `Transaction`: The unsigned withdrawal transaction.

---

### 6. SubmitTransaction

Submits a signed transaction to the network.

#### Method Signature

```csharp
public async Task<TransactionResult> SubmitTransaction(Transaction transaction)
```

#### Inputs

- `transaction`: The signed transaction to be submitted.

#### Returns

- `TransactionResult`: A sealed record containing:
  - `IsSuccess`: Boolean indicating if the transaction succeeded.
  - `TransactionHash`: The hash of the submitted transaction (if successful).
  - `Amounts`: An array of amounts deposited or withdrawn.
  - `SharesChanged`: The amount of shares minted or burned.

---

## Data Models

### VaultShares (Represents user’s vault shares)

```csharp
public sealed record VaultShares(string AccountId, decimal Shares);
```

### VaultFunds (Represents vault's total managed funds per asset)

```csharp
public sealed record VaultFunds(
    decimal TotalFunds,
    decimal IdleFunds,
    decimal TotalInvestedFunds,
    Dictionary<string, decimal> PerStrategyFunds);
```

### TransactionResult (Represents the result of submitting a transaction)

```csharp
public sealed record TransactionResult(
    bool IsSuccess,
    string? TransactionHash,
    List<long> Amounts,
    decimal SharesChanged);
```

---

## Summary

| Method                                       | Purpose                                                                                       |
| -------------------------------------------- | --------------------------------------------------------------------------------------------- |
| `GetUserShares(string accountId)`            | Retrieves user’s vault shares                                                                 |
| `FetchTotalManagedFunds()`                   | Gets total vault funds, idle funds, invested funds, and per-strategy breakdown for each asset |
| `GetVaultTotalShares()`                      | Fetches total vault shares issued                                                             |
| `CreateDepositTransaction(...)`              | Creates an unsigned transaction to deposit into a vault                                       |
| `CreateWithdrawTransaction(...)`             | Creates an unsigned transaction to withdraw from a vault                                      |
| `SubmitTransaction(Transaction transaction)` | Submits a signed transaction and returns detailed success/failure information                 |

This SDK structure ensures clarity in handling **shares vs. funds**, maintains modularity, and provides a future-proof way to interact with DeFindex vaults.