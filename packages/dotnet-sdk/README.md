**DeFindex .NET SDK Specification**

## **Overview**

This document outlines the required methods and data structures for the DeFindex .NET SDK. The SDK is designed to interact with the DeFindex vault system, allowing users to retrieve vault shares and managed funds while maintaining clear distinctions between vault shares and underlying assets.

## **Terminology**

- **Shares**: Represents ownership of a vault.
- **Funds**: Represents the underlying assets managed by the vault (e.g., USDC, XLM).
- **Vault**: The contract managing investments and distributions.

## **SDK Initialization**

The `Defindex` class requires a contract address and a `SorobanServer` instance from the dotnet-sdk. Clients must instantiate the `SorobanServer` with their RPC URL and bearer tokens before passing it to `Defindex` for initialization.

## **SDK Usage**
### 1. Create vault instance:
```` C#
 var sorobanServer = new SorobanServer("https://soroban-testnet.stellar.org/")
 var vaultAddress = "CXX...XXX" //(Smart contract address)
 var vaultInstance = new DefindexSdk(vaultAddress, sorobanServer);
````
### 2. Use vault instance to create a transaction:
```` C#
var account = await horizonServer.Accounts.Account(keypair.AccountId);
var adminAccount = new Account(account.AccountId, account.SequenceNumber);
var userWithShares =  "GXX...XX"; //Soroban G account
var transaction = await vaultInstance.CreateBalanceTransaction(adminAccount, userWithShares);
````
### 3. Setup the transaction data & submit:
```` C#
var simulatedDepositTransaction = await sorobanServer.SimulateTransaction(transaction);
transaction.SetSorobanTransactionData(simulatedDepositTransaction.SorobanTransactionData);
transaction.SetSorobanAuthorization(simulatedDepositTransaction.SorobanAuthorization);
transaction.AddResourceFee(simulatedDepositTransaction.MinResourceFee.Value + 100000);
transaction.Sign(keypair);
var submittedTx = await sorobanServer.SendTransaction(transaction);

// Now you should await for the blockchain confirmation...
````
### 4. Parse transaction result:
````C#
Task.Delay(5000).Wait(); //We will use a timeout for demonstration
// After the blockchain confirmation we can get the transaction by the hash using sorobanServer.GetTransaction(txhash)
var checkedTx = await sorobanServer.GetTransaction(submittedTx.Hash);

var parsedtx = vaultInstance.ParseTransactionResponse(checkedTx);
// Now you can print the results using:
Console.WriteLine($"Parsed transaction: {JsonConvert.SerializeObject(parsedtx, Formatting.Indented)}");
````
## **SDK Methods**

### **1. GetUserShares**

Retrieves the number of vault shares a user owns.

**Method Signature:**

```C#
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

```C#
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

```json
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

```C#
public async Task<ulong> GetVaultTotalShares()
```

**Returns:**

- `ulong`: Total number of shares in circulation.

---

### **4. CreateDepositTransaction**

Creates an unsigned transaction to deposit into a vault.

**Method Signature:**

```C#
public async Task<Transaction> CreateDepositTransaction(
    List<ulong> amountsDesired,
    List<ulong> amountsMin,
    string from,
    bool invest)
```

**Inputs:**
- `amountsDesired`: List of desired deposit amounts.
- `amountsMin`: List of minimum acceptable deposit amounts.
- `from`: Address (string) of the depositor.
- `invest`: Whether the deposit invests immediately into strategies.

**Returns:**
- `Transaction`: The unsigned deposit transaction.

---

### **5. CreateWithdrawTransaction**

Creates an unsigned transaction to withdraw from a vault.

**Method Signature:**

```C#
public async Task<Transaction> CreateWithdrawTransaction(
    ulong withdrawShares,
    List<ulong> amountsMinOut,
    string from)
```

**Inputs:**
- `withdrawShares`: Amount of vault shares to withdraw.
- `amountsMinOut`: List of minimum acceptable withdrawal amounts per asset.
- `from`: Address (string) of the withdrawer.

**Returns:**
- `Transaction`: The unsigned withdrawal transaction.

---

### **6. ParseTransactionResponse**

Parses the transaction response from the network.

**Method Signature:**

```C#
public async Task<List<TransactionResult>> ParseTransactionResponse(GetTransactionResponse response)
```

**Inputs:**
- `response`: A previously validated transaction response from the network.

**Returns:**
- `List<TransactionResult>`: List of transaction results.
    - `IsSuccess`: Boolean indicating if the transaction succeeded.
    - `TransactionHash`: The hash of the submitted transaction (if successful).
    - `Amounts`: An array of amounts deposited or withdrawn.
    - `SharesChanged`: The amount of shares minted or burned.

---

### **7. GetVaultAPY**

Retrieves the current estimated APY for the vault.

**Method Signature:**

```C#
public async Task<decimal?> GetVaultAPY()
```

**Returns:**
- `decimal?`: Estimated APY for the vault, or null if not available.

---

### **8. GetAssetAmountsPerShares**

Converts a given number of vault shares to the corresponding underlying asset amounts.

**Method Signature:**

```C#
public async Task<List<BigInteger>> GetAssetAmountsPerShares(BigInteger vaultShares)
```

**Inputs:**
- `vaultShares`: The number of vault shares to convert.

**Returns:**
- `List<BigInteger>`: List of asset amounts corresponding to the given vault shares.

---

### **9. CreateWithdrawUnderlyingTx**

Creates an unsigned transaction to withdraw a specific amount of underlying asset from the vault, with a basis points tolerance.

**Method Signature:**

```C#
public async Task<Transaction> CreateWithdrawUnderlyingTx(
    BigInteger withdrawAmount,
    int bpsTolerance,
    string from
)
```

**Inputs:**
- `withdrawAmount`: The amount of underlying asset to withdraw.
- `bpsTolerance`: The basis points tolerance for the withdrawal.
- `from`: The account to withdraw from.

**Returns:**
- `Transaction`: The unsigned withdrawal transaction for underlying assets.

---

## **Data Models**

### VaultShares
```C#
public sealed record VaultShares(
    string AccountId,
    ulong Shares
);
```

### ManagedFundsResult
```C#
public sealed record ManagedFundsResult(
    string? Asset,
    BigInteger IdleAmount,
    BigInteger InvestedAmount,
    BigInteger TotalAmount,
    List<StrategyAllocation> StrategyAllocations
);
```

### StrategyAllocation
```C#
public sealed record StrategyAllocation(
    BigInteger Amount,
    bool Paused,
    string? StrategyAddress
);
```

### TransactionResult
```C#
public sealed record TransactionResult(
    bool IsSuccess,
    string? TransactionHash,
    List<BigInteger> Amounts,
    BigInteger SharesChanged
);
```

---

## **Summary**

| Method | Purpose |
| --- | --- |
| `GetUserShares(string accountId)` | Retrieves user’s vault shares |
| `FetchTotalManagedFunds()` | Gets total vault funds, idle funds, invested funds, and per-strategy breakdown for each asset |
| `GetVaultTotalShares()` | Fetches total vault shares issued |
| `CreateDepositTransaction(List<ulong> amountsDesired, List<ulong> amountsMin, string from, bool invest)` | Creates an unsigned transaction to deposit into a vault |
| `CreateWithdrawTransaction(ulong withdrawShares, List<ulong> amountsMinOut, string from)` | Creates an unsigned transaction to withdraw from a vault |
| `ParseTransactionResponse(GetTransactionResponse response)` | Parses a transaction response from the network |
| `GetVaultAPY()` | Retrieves the current estimated APY for the vault |
| `GetAssetAmountsPerShares(BigInteger vaultShares)` | Converts vault shares to underlying asset amounts |
| `CreateWithdrawUnderlyingTx(BigInteger withdrawAmount, int bpsTolerance, string from)` | Creates an unsigned transaction to withdraw underlying assets from a vault |

Made with ❤️ by [PaltaLabs🥑](https://github.com/paltalabs)

