---
description: ⏱️ 15 min read
---
# Troubleshooting Guide

This guide provides solutions and explanations for common issues encountered when using the DeFindex protocol. It covers API errors, environment setup, contract error codes, transaction failures, and frequently asked questions.

## Table of Contents

- [API](#api)
  - [API Error Reference](#api-error-reference)
  - [API Rate Limits](#rate-limiting)
- [Environment & Configuration](#environment--configuration)
  - [Step-by-Step Debugging Guide](#step-by-step-debugging-guide)
- [Smart Contracts](#smart-contracts)
  - [Error Log Debugging Example](#error-log-debbugging-example)
  - [Common Contract Errors](#common-contract-errors)
  - [Withdrawing All — Dust Left Behind](#withdrawing-all--dust-left-behind)
- [Soroban Transaction Errors](#soroban-transaction-errors)
- [Frequently Asked Questions (FAQ)](#frequently-asked-questions-faq)
- [Additional Resources](#additional-resources)


## API

### API Error Reference

HTTP status codes returned by the DeFindex API:

| Status | Error | Common Causes | Solution |
|--------|-------|---------------|----------|
| 400 | Bad Request | Invalid address format, wrong amounts length, missing parameters, slippage out of range, contract simulation failure | Check request body against API docs; for simulation failures inspect the `errorCode` field for the contract error code |
| 401 | Unauthorized | Missing or expired JWT / API key | Refresh the token or provide a valid API key |
| 403 | Forbidden | Insufficient vault role permissions | Verify API key has role |
| 404 | Not Found | Vault address not found, account has no transactions | Verify the address and network (testnet or mainnet) |
| 409 | Conflict | Duplicate email on registration | Use a different email address |
| 429 | Too Many Requests | Rate limit exceeded | Implement backoff (see [API Rate Limits](#rate-limiting)); check `retryAfter` in response |
| 503 | Service Unavailable | Stellar network unreachable, external service failure | Retry after a delay; check Stellar network status |


<a id="rate-limiting"></a>

### API Rate Limits

The DeFindex API uses a rate limiter with a 5 minute window. Limits are applied per API key (authenticated requests) or per IP address (unauthenticated requests).

#### Rate Limit Tiers

| Tier | Burst Capacity | Sustained Rate |
|------|---------------|----------------|
| Free | 5 requests | 1 req/s |
| Starter | 20 requests | 10 req/s |
| Professional | 100 requests | 50 req/s |
| Business | 200 requests | 100 req/s |

Contact PaltaLabs🥑 team on [discord](https://discord.gg/MABd5JXmPN) to upgrade tier.

#### Response Headers

Every API response includes rate limit headers:

| Header | Description |
|--------|-------------|
| `X-RateLimit-Limit` | Maximum requests allowed in the current window |
| `X-RateLimit-Remaining` | Requests remaining in the current window |
| `X-RateLimit-Reset` | Unix timestamp when the window resets |

> **Tip:** Use the `GET /rate-limits/tiers` endpoint to retrieve the current rate limit configuration for your API key.

#### Handling 429 Responses

When you exceed the rate limit, the API returns **429 (Too Many Requests)**. Use exponential backoff to handle this:

```typescript
async function withRateLimit<T>(
  fn: () => Promise<T>,
  maxRetries: number = 5,
  initialDelay: number = 1000
): Promise<T> {
  let lastError: any;

  for (let attempt = 0; attempt <= maxRetries; attempt++) {
    try {
      return await fn();
    } catch (error: any) {
      lastError = error;

      if (error?.statusCode === 429 || error?.error === "Too Many Requests") {
        const retryAfter = error?.retryAfter || 1;
        const delayMs = Math.max(
          retryAfter * 1000,
          initialDelay * Math.pow(2, attempt)
        );

        if (attempt < maxRetries) {
          await new Promise((resolve) => setTimeout(resolve, delayMs));
          continue;
        }
      }

      throw error;
    }
  }

  throw lastError;
}
```

Usage example:

```typescript
const depositResponse = await withRateLimit(() =>
  defindexSdk.depositToVault(vaultAddress, depositData, supportedNetwork)
);
```


## Environment & Configuration

### Step-by-Step Debugging Guide

#### 1. Check Environment Variables
- Ensure all required environment variables (e.g., `MAINNET_RPC_URL`) are set correctly.
- Example (.env):
  ```dotenv
  MAINNET_RPC_URL=your_rpc_url
  ```

#### 2. Validate Network and Contract Deployment
- Confirm you are connected to the correct network (testnet/mainnet).
- Verify the contract address is correct and the contract is deployed.

#### 3. Simulate Transactions Before Sending
- Use the SDK's simulation methods to check for errors before submitting transactions.
- Review simulation results for error codes or failed preconditions.

#### 4. Handle Transaction Failures
- If a transaction fails, inspect the error code returned.
- Refer to the tables in this page to interpret the error and apply the suggested fix.

#### 5. Check Parameter Types and Lengths
- Ensure all parameters (amounts, addresses, etc.) are of the correct type and length.
- For multi-asset vaults, input arrays must match the number of assets.

#### 6. Review Contract and SDK Versions
- Make sure you are using compatible versions of the SDK and smart contracts.

#### 7. Debug XDR manually
- Debug XDR transaction in [Stellar Lab](https://lab.stellar.org)


## Smart Contracts

### Error Log Debbugging Example

Consider the following error log from a `withdraw` transaction:

```
Event log (newest first):
  0: [Diagnostic Event] contract:CBDZYJVQJQT7QJ7ZTMGNGZ7RR3DF32LERLZ26A2HLW5FNJ4OOZCLI3OG, topics:[error, Error(Contract, #160)], data:"escalating error to VM trap from failed host function call: fail_with_error"
  1: [Diagnostic Event] contract:CBDZYJVQJQT7QJ7ZTMGNGZ7RR3DF32LERLZ26A2HLW5FNJ4OOZCLI3OG, topics:[error, Error(Contract, #160)], data:["failing with contract error", 160]
  2: [Contract Event] contract:CBDZYJVQJQT7QJ7ZTMGNGZ7RR3DF32LERLZ26A2HLW5FNJ4OOZCLI3OG, topics:[burn, GBI6SIGPSKXTBLXGSAFT2TN5DYFBHIJXKO7IGGQTR7DKO2ANWILGXIDA], data:9999563
  11: [Diagnostic Event] topics:[fn_call, CBDZYJVQJQT7QJ7ZTMGNGZ7RR3DF32LERLZ26A2HLW5FNJ4OOZCLI3OG, withdraw], data:[9999563, [10000065], GBI6SIGPSKXTBLXGSAFT2TN5DYFBHIJXKO7IGGQTR7DKO2ANWILGXIDA]

```

**Breakdown:**

1.  **Identify the Error:**

    -   Events `0` and `1` indicate the error: `Error(Contract, #160)`.
    -   Error code: `160` (InsufficientOutputAmount)
2.  **Understand the Context:**

    -   Event `0`: The contract explicitly triggered an error.
    -   Event `1`: Confirms the contract error code is 160.
3.  **Determine Function Arguments:**

    -   Event `11` shows the `withdraw` function call and its arguments:
        -   `9999563`: `withdraw_shares` (number of shares to burn)
        -   `[10000065]`: `min_amounts_out` (minimum expected output amount)
        -   `GBI6SIGPSKXTBLXGSAFT2TN5DYFBHIJXKO7IGGQTR7DKO2ANWILGXIDA`: `to` (recipient address)
4.  **Interpret the Error in Context:**

    -   The `withdraw` transaction failed because the vault could not provide at least 10000065 stroops of the underlying asset when burning 9999563 shares.



### Common Contract Errors

The DeFindex contracts return specific error codes when a transaction fails. Below is a comprehensive reference grouped by contract and category.

### Vault Errors

#### Initialization Errors (100–108)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 100 | NotInitialized | Vault has not been initialized | Call the vault's `initialize` function before any other operation |
| 101 | InvalidRatio | Asset allocation ratios are invalid (e.g., don't sum to expected total) | Ensure allocation ratios are valid and sum correctly |
| 102 | StrategyDoesNotSupportAsset | A strategy was assigned an asset it cannot handle | Verify the strategy supports the asset before assigning |
| 103 | NoAssetAllocation | No asset allocation was provided during initialization | Provide at least one asset allocation |
| 104 | RolesIncomplete | Required roles were not fully assigned | Assign all required roles (manager, emergency manager, etc.) |
| 105 | MetadataIncomplete | Vault metadata (name, symbol, etc.) is missing or incomplete | Provide all required metadata fields |
| 106 | MaximumFeeExceeded | The fee set exceeds the maximum allowed | Lower the fee to within the allowed range |
| 107 | DuplicatedAsset | The same asset was provided more than once | Remove duplicate assets from the allocation |
| 108 | DuplicatedStrategy | The same strategy was provided more than once | Remove duplicate strategies from the allocation |

#### Validation Errors (110–129)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 110 | AmountNotAllowed | The provided amount is not allowed (e.g., zero or negative) | Check that all amounts are positive and within allowed limits |
| 111 | InsufficientBalance | User does not have enough balance for the operation | Verify the user's token balance before submitting |
| 112 | WrongAmountsLength | The number of amounts does not match the number of assets | Ensure your input arrays match the vault's asset count |
| 113 | WrongLockedFees | Locked fees value is incorrect | Verify the locked fees parameter |
| 114 | InsufficientManagedFunds | The vault does not have enough managed funds | Check vault TVL before attempting the operation |
| 115 | MissingInstructionData | Required instruction data is missing | Verify all required parameters are provided for the operation |
| 116 | UnsupportedAsset | The provided asset is not supported by the vault | Use only assets that the vault was configured with |
| 117 | InsufficientAmount | The amount provided is too small | Increase the amount to meet the minimum requirement |
| 118 | NoOptimalAmounts | Could not calculate optimal deposit amounts | This is an internal error; check vault state and asset ratios |
| 119 | WrongInvestmentLength | The investment allocation array length is incorrect | Check that investment allocations match the number of strategies/assets |
| 122 | WrongAssetAddress | The provided asset address does not match expected | Verify the asset contract address is correct |
| 123 | WrongStrategiesLength | The strategies array length does not match expected | Ensure the strategies array length matches the vault configuration |
| 124 | AmountOverTotalSupply | The requested amount exceeds the total supply of shares | Reduce the amount to at most the total supply |
| 125 | NoInstructions | No instructions were provided for the operation | Provide the required instructions for the operation |
| 126 | NotUpgradable | The vault contract is not upgradable | This vault was deployed as non-upgradable; deploy a new vault if needed |
| 128 | UnwindMoreThanAvailable | Attempting to unwind more than is available in the strategy | Reduce the unwind amount or check available strategy balance |
| 129 | InsufficientFeesToRelease | Not enough accrued fees to release | Wait for more fees to accumulate before releasing |

#### Arithmetic Errors (120–127)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 120 | ArithmeticError | A general arithmetic error occurred | Check for very large or very small values that may cause overflow/underflow |
| 121 | Overflow | An arithmetic overflow occurred | Reduce input values to prevent overflow |
| 127 | Underflow | An arithmetic underflow occurred | Ensure values are large enough to avoid underflow |

#### Authorization Errors (130–134)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 130 | Unauthorized | The caller is not authorized to perform this action | Ensure the caller has the required role (manager, emergency manager, etc.) |
| 131 | RoleNotFound | The specified role does not exist | Check available roles in the vault configuration |
| 132 | ManagerNotInQueue | The manager address is not in the pending queue | Add the manager to the queue first using the appropriate function |
| 133 | SetManagerBeforeTime | Attempted to set manager before the timelock expires | Wait for the timelock period to pass before confirming the manager change |
| 134 | QueueEmpty | The manager queue is empty | Add a manager to the queue before attempting to confirm |

#### Strategy Operation Errors (140–144)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 140 | StrategyNotFound | The specified strategy was not found in the vault | Verify the strategy address is registered with the vault |
| 141 | StrategyPausedOrNotFound | The strategy is paused or does not exist | Check strategy status; if paused, contact the vault manager |
| 142 | StrategyWithdrawError | An error occurred while withdrawing from a strategy | Check the strategy's state and available balance |
| 143 | StrategyInvestError | An error occurred while investing into a strategy | Verify the investment amount and strategy availability |
| 144 | StrategyPaused | The strategy is currently paused | Wait for the strategy to be unpaused or contact the vault manager |

#### Asset Errors (150–151)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 150 | AssetNotFound | The specified asset was not found in the vault | Verify the asset address is correct and registered with the vault |
| 151 | NoAssetsProvided | No assets were provided for the operation | Provide at least one asset |

#### Input Errors (160–162)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 160 | InsufficientOutputAmount | The output amount is less than the required minimum | Lower your minimums or check vault liquidity before retrying |
| 161 | ExcessiveInputAmount | The input amount exceeds allowed limits | Reduce the input amount to within allowed limits |
| 162 | InvalidFeeBps | The fee basis points value is invalid | Provide a valid fee in basis points (0–10000) |

#### External / Swap Errors (190–202)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 190 | LibrarySortIdenticalTokens | Soroswap library received two identical token addresses | Ensure the two tokens in the swap are different |
| 200 | SoroswapRouterError | An error occurred in the Soroswap router | Check the Soroswap router status and input parameters |
| 201 | SwapExactInError | The exact-input swap failed | Verify swap parameters (token addresses, amounts, deadline) |
| 202 | SwapExactOutError | The exact-output swap failed | Verify swap parameters (token addresses, amounts, deadline) |

### Factory Errors

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 401 | NotInitialized | The factory contract has not been initialized | Call `initialize` on the factory before creating vaults |
| 404 | AssetLengthMismatch | The number of assets does not match the expected length | Ensure all asset arrays have consistent lengths |
| 405 | IndexDoesNotExist | The requested vault index does not exist | Verify the vault index; use the factory to list available vaults |
| 406 | FeeTooHigh | The specified fee exceeds the maximum allowed by the factory | Reduce the fee to within the factory's allowed range |

### Strategy Errors

#### Validation Errors (401–418)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 401 | NotInitialized | The strategy contract has not been initialized | Call `initialize` on the strategy before use |
| 410 | NegativeNotAllowed | A negative value was provided where only positive is allowed | Ensure all amounts are non-negative |
| 411 | InvalidArgument | An invalid argument was provided | Check all function arguments against expected types and ranges |
| 412 | InsufficientBalance | The strategy does not have enough balance | Verify the strategy's balance before withdrawing |
| 413 | UnderflowOverflow | An arithmetic underflow or overflow occurred | Reduce input values to prevent arithmetic errors |
| 414 | ArithmeticError | A general arithmetic error occurred | Check for edge cases in input values |
| 415 | DivisionByZero | A division by zero was attempted | Ensure divisor values are non-zero |
| 416 | InvalidSharesMinted | The calculated shares to mint are invalid (zero or negative) | Check that the deposit amount is large enough to produce valid shares |
| 417 | OnlyPositiveAmountAllowed | The amount must be strictly positive | Provide a positive (non-zero) amount |
| 418 | NotAuthorized | The caller is not authorized | Ensure the caller has permission (typically the vault contract) |

#### Protocol Errors (420–423)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 420 | ProtocolAddressNotFound | The external protocol address was not found | Verify the protocol address is correctly configured in the strategy |
| 421 | DeadlineExpired | The transaction deadline has passed | Resubmit the transaction with a fresh deadline |
| 422 | ExternalError | An error occurred in an external protocol call | Check the external protocol's status and parameters |
| 423 | SoroswapPairError | An error occurred with a Soroswap liquidity pair | Verify the pair exists and has sufficient liquidity |

#### Blend Strategy Errors (451–455)

| Code | Name | Cause | Solution/Tip |
|------|------|-------|--------------|
| 451 | AmountBelowMinDust | The amount is below the Blend protocol's minimum dust threshold | Increase the amount above the minimum dust requirement |
| 452 | UnderlyingAmountBelowMin | The underlying token amount is below the minimum | Increase the underlying amount |
| 453 | BTokensAmountBelowMin | The bToken amount is below the minimum | Increase the deposit to receive more bTokens |
| 454 | InternalSwapError | An error occurred during an internal swap within the strategy | Check the swap route and liquidity availability |
| 455 | SupplyNotFound | The Blend supply pool was not found | Verify the Blend pool is correctly configured and active |

### Withdrawing All — Dust Left Behind

#### Why Does This Happen?

When withdrawing all shares, a tiny residual balance (typically 1–3 stroops per asset) may remain. This is by design, not a bug.

The vault calculates each asset's withdrawal amount using **integer floor division**:

```
withdrawal_amount = (total_asset × user_shares) / total_shares
```

Since Soroban uses integer arithmetic (no decimals), the division **truncates** any fractional stroop. In multi-asset vaults, this truncation can happen once per asset per division step, compounding to a few stroops total.

This behavior is intentional: it prevents the vault from ever paying out more than it holds.

#### How Much Dust?

Typically **1–3 stroops per asset** (1 stroop = 0.0000001 XLM or the smallest unit of a Soroban token). This is economically negligible.

#### Workaround: Two-Step Withdraw

If you need to recover the dust, use a two-step approach:

```typescript
// Step 1: Withdraw all shares
const userShares = await defindexSdk.getUserShares(vaultAddress, userAddress);
const minAmountsOut = vaultAssets.map(() => 0n); // accept any amount
await defindexSdk.withdraw(vaultAddress, {
  withdrawShares: userShares,
  minAmountsOut,
  from: userAddress,
});

// Step 2: Check for remaining dust
const remainingShares = await defindexSdk.getUserShares(vaultAddress, userAddress);
if (remainingShares > 0n) {
  // Withdraw the remaining dust
  const dustMinAmounts = vaultAssets.map(() => 0n);
  await defindexSdk.withdraw(vaultAddress, {
    withdrawShares: remainingShares,
    minAmountsOut: dustMinAmounts,
    from: userAddress,
  });
}
```


## Soroban Transaction Errors

These are Stellar/Soroban transaction-level errors that occur before or outside of contract execution. They appear in the transaction result rather than in contract event logs.

| Error | Cause | Solution |
|-------|-------|----------|
| `tx_failed` | One or more operations failed | Check operation-level result codes and contract error codes (contract error tables above) |
| `tx_bad_seq` | Wrong sequence number | Re-fetch the transaction from the API; Soroban resource estimates are ledger-specific |
| `tx_insufficient_fee` | Fee-bump per-op rate too low | Increase base fee ≥ inner tx fee (see [Sponsored Transactions](guides-and-tutorials/sponsored-transactions.md#fee-considerations)) |
| `tx_too_late` | Timebounds expired before submission | Re-fetch fresh XDR and submit promptly |
| `tx_too_early` | Timebounds not yet valid | Wait for the valid time window or re-fetch the transaction |
| `tx_bad_auth` | Missing or invalid signature | Ensure all required signers have signed the transaction |
| `tx_insufficient_balance` | Source account lacks XLM for fee | Fund the fee-paying account with enough XLM |
| `tx_soroban_invalid` | Invalid Soroban transaction structure | Check that contract invocation arguments match the expected types |


## Frequently Asked Questions (FAQ)

**Q: My transaction fails with `WrongAmountsLength` (112). What does this mean?**
- A: The number of amounts you provided does not match the number of assets in the vault. Double-check your input arrays.

**Q: What should I do if I get `InsufficientOutputAmount` (160)?**
- A: The vault could not provide the minimum output you requested. Try lowering your minimums or check if the vault has enough liquidity.

**Q: How do I know which error code was returned?**
- A: Inspect the transaction response object. The error code will be included in the failure reason or logs.

**Q: How can I debug contract errors further?**
- A: Review the contract's `error.rs` file for detailed error definitions. Use simulation and logging to narrow down the cause.

**Q: Are there any environment setup issues I should be aware of?**
- A: Yes. Ensure all required environment variables are set, dependencies are installed, and you are using the correct network and contract addresses.

**Q: When I withdraw a specific amount, why might I receive slightly more than requested?**
- A:  Due to the fluctuating ratio between the underlying asset and vault shares (caused by Blend strategy gains) and the Soroban contract's handling of the smallest asset unit ("stroop"), the contract uses ceiling division to calculate the shares to burn. This ensures you receive *at least* the requested amount, but it can sometimes result in a slightly higher output.  The contract prioritizes fulfilling the minimum withdrawal amount.

**Q: When I deposit a specific amount, will I always receive the same number of shares?**
- A: No. Similar to withdrawals, the number of shares you receive when depositing a fixed amount of the underlying asset can vary. This is because the ratio between the asset and shares changes constantly. A deposit made moments apart can yield slightly different share amounts.

**Q: I withdrew all my shares but still have a tiny balance (1–3 stroops). Is this a bug?**
- A: No. This is expected behavior due to integer floor division in the contract. See [Withdrawing All — Dust Left Behind](#withdrawing-all--dust-left-behind) for a full explanation and workaround.



## Additional Resources
- [DeFindex Protocol Documentation](https://github.com/paltalabs)
- [Smart Contract Error Codes](../../contracts/vault/src/error.rs)

If you encounter an issue not covered here, please open an issue on the project's GitHub repository.
