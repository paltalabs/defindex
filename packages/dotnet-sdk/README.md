**DeFindex .NET SDK Specification**

## **Overview**

This document outlines the required methods and data structures for the DeFindex .NET SDK. The SDK is designed to interact with the DeFindex vault system, allowing users to retrieve vault shares and managed funds while maintaining clear distinctions between vault shares and underlying assets.

## **Terminology**

- **Shares**: Represents ownership of a vault.
- **Funds**: Represents the underlying assets managed by the vault (e.g., USDC, XLM).
- **Vault**: The contract managing investments and distributions.

## **SDK Initialization**

The `Defindex` class requires a contract address and a `SorobanServer` instance from the dotnet-sdk. Clients must instantiate the `SorobanServer` with their RPC URL and bearer tokens before passing it to `Defindex` for initialization.

## **SDK Methods**

### **1. GetUserShares**

Retrieves the number of vault shares a user owns.

**Method Signature:**

```csharp
public async Task<VaultShares> GetUserShares(string accountId)

```

**Returns:**

- `VaultShares` object containing:
    - `AccountId`: The user’s account ID.
    - `Shares`: The number of vault shares owned.

---

### **2. FetchTotalManagedFunds**

Retrieves vault's total funds, idle funds, and invested funds per strategy for each underlying asset.

**Method Signature:**

```csharp
public async Task<List<ManagedFundsResult>> FetchTotalManagedFunds()

```

**Returns:**

- `List<ManagedFundsResult>` where:
- Every (ManagedFundsResult) object contains:
    - `Asset`: The underliying asset contractID
    - `TotalAmount`: Total amount of this asset in the vault.
    - `IdleAmount`: Amount of this asset not currently invested.
    - `InvestedAmount`: Amount of this asset currently invested.
    - `StrategyAllocations`: A List mapping **strategy contract IDs, Invested amounts and paused status**.

Example of a returned structure:

```csharp
[
  {
    "Asset": "CARDT45FED2I3FKESPMHDFV3ZMR6VH5ZHCFIOPH6TPSU35GPB6QBBCSU",
    "IdleAmount": 100000000,
    "InvestedAmount": 0,
    "TotalAmount": 100000000,
    "StrategyAllocations": [
      {
        "Amount": 0,
        "Paused": false,
        "StrategyAddress": "CAS6KDVEXMCB5BNXJYHWGWO2U6INQFC2SGFGIJLU27HLWIOF4XQE5ZMS"
      }
    ]
  },
  {
    "Asset": "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC",
    "IdleAmount": 50000000,
    "InvestedAmount": 50000000,
    "TotalAmount": 100000000,
    "StrategyAllocations": [
      {
        "Amount": 50000000,
        "Paused": false,
        "StrategyAddress": "CAS6KDVEXMCB5BNXJYHWGWO2U6INQFC2SGFGIJLU27HLWIOF4XQE5ZMS"
      }
    ]
  }
]

```

---

### **3. GetVaultTotalShares**

Retrieves the total number of vault shares issued.

**Method Signature:**

```csharp
public async Task<ulong> GetVaultTotalShares()

```

**Returns:**

- `ulong`: Total number of shares in circulation.

---

### **4. CreateDepositTransaction**

Creates an unsigned transaction to deposit into a vault.

**Method Signature:**

```csharp
public async Task<Transaction> CreateDepositTransaction(
    List<ulong> amountsDesired,
    List<ulong> amountsMin,
    Address from,
    bool invest)

```

**Inputs:**

- `amountsDesired`: List of desired deposit amounts.
- `amountsMin`: List of minimum acceptable deposit amounts.
- `from`: Address of the depositor.
- `invest`: Whether the deposit invests immediately into strategies.

**Returns:**

- `Transaction`: The unsigned deposit transaction.

---

### **5. CreateWithdrawTransaction**

Creates an unsigned transaction to withdraw from a vault.

**Method Signature:**

```csharp
public async Task<Transaction> CreateWithdrawTransaction(
    long withdrawShares,
    Address from)

```

**Inputs:**

- `withdrawShares`: Amount of vault shares to withdraw.
- `from`: Address of the withdrawer.

**Returns:**

- `Transaction`: The unsigned withdrawal transaction.

---
### **6. ParseTransactionResponse**

Parses the transaction response from the network.

**Method Signature:**

```csharp
public async Task<List<TransactionResult>> ParseTransactionResponse(GetTransactionResponse response)

```

**Inputs:**

- `response`: A previously validated transaction response from the network.

**Returns:**
  
- `TransactionResult`: A sealed record containing:
    - `IsSuccess`: Boolean indicating if the transaction succeeded.
    - `TransactionHash`: The hash of the submitted transaction (if successful).
    - `Amounts`: An array of amounts deposited or withdrawn.
    - `SharesChanged`: The amount of shares minted or burned.

### **7. SubmitTransaction**

Submits a signed transaction to the network.

**Method Signature:**

```csharp
public async Task<TransactionResult> SubmitTransaction(Transaction transaction)

```

**Inputs:**

- `transaction`: The signed transaction to be submitted.

**Returns:**

- `TransactionResult`: A sealed record containing:
    - `IsSuccess`: Boolean indicating if the transaction succeeded.
    - `TransactionHash`: The hash of the submitted transaction (if successful).
    - `Amounts`: An array of amounts deposited or withdrawn.
    - `SharesChanged`: The amount of shares minted or burned.

---

## **Data Models**

### **VaultShares** (Represents user’s vault shares)

```csharp
public sealed record VaultShares(
		string AccountId, 
		decimal Shares
);

```

### ManagedFundsResponse (Represents vault's total managed funds per asset)

```csharp
public sealed record VaultFunds(
		string Asset,
		ulong IdleFunds,
		ulong InvestedFunds,
		ulong TotalAmount,
		StrategyAllocation StrategyAllocations
    );
```

### StrategyAllocations (Represents Strategy info)

```csharp
public sealed record StrategyAllocation
{
    ulong Amount,
    bool Paused,
    string? StrategyAddress,
}
```

### **TransactionResult** (Represents the result of submitting a transaction)

```csharp
public sealed record TransactionResult(
    bool IsSuccess,
    string? TransactionHash,
    List<ulong> Amounts,
    ulong SharesChanged);

```

---

## **Summary**

| Method | Purpose |
| --- | --- |
| `GetUserShares(string accountId)` | Retrieves user’s vault shares |
| `FetchTotalManagedFunds()` | Gets total vault funds, idle funds, invested funds, and per-strategy breakdown for each asset |
| `GetVaultTotalShares()` | Fetches total vault shares issued |
| `CreateDepositTransaction(...)` | Creates an unsigned transaction to deposit into a vault |
| `CreateWithdrawTransaction(...)` | Creates an unsigned transaction to withdraw from a vault |
| `SubmitTransaction(Transaction transaction)` | Submits a signed transaction and returns detailed success/failure information |

This SDK structure ensures clarity in handling **shares vs. funds**, maintains modularity, and provides a future-proof way to interact with DeFindex vaults.

