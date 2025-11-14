# Troubleshooting Guide

This guide provides solutions and explanations for common issues encountered when using the DeFindex .NET SDK and interacting with DeFindex smart contracts on Soroban. It covers transaction failures, contract error codes, environment setup, and frequently asked questions.


## Error Log Debbugging Example

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



## Common Transaction Errors and Their Meanings

The DeFindex Vault contract may return specific error codes when a transaction fails. Below are some of the most common errors, their meanings, and suggested solutions:

| Error Code | Error Name                | Description                                                                 | Solution/Tip                                                                 |
|-----------|--------------------------|-----------------------------------------------------------------------------|------------------------------------------------------------------------------|
| 110       | AmountNotAllowed          | The provided amount is not allowed (e.g., zero or negative).                | Check that all amounts are positive and within allowed limits.               |
| 112       | WrongAmountsLength        | The number of amounts provided does not match the expected number of assets. | Ensure your input arrays match the vault's asset count.                      |
| 115       | MissingInstructionData    | Required instruction data is missing.                                       | Verify all required parameters are provided for the operation.               |
| 119       | WrongInvestmentLength     | The investment allocation array length is incorrect.                        | Check that investment allocations match the number of strategies/assets.      |
| 160       | InsufficientOutputAmount  | The output amount is less than the required minimum.                        | Lower your minimums or check vault liquidity before retrying.                |
| 161       | ExcessiveInputAmount      | The input amount exceeds allowed limits.                                    | Reduce the input amount to within allowed limits.                            |

For a full list of contract errors, see the `error.rs` file in the contract source code.



## Step-by-Step Debugging Guide

### 1. Check Environment Variables
- Ensure all required environment variables (e.g., `MAINNET_RPC_URL`) are set correctly.
- Example (.env):
  ```dotenv
  MAINNET_RPC_URL="https://soroban-mainnet.stellar.org"
  ```

### 2. Validate Network and Contract Deployment
- Confirm you are connected to the correct network (testnet/mainnet).
- Verify the contract address is correct and the contract is deployed.

### 3. Simulate Transactions Before Sending
- Use the SDK's simulation methods to check for errors before submitting transactions.
- Review simulation results for error codes or failed preconditions.

### 4. Handle Transaction Failures
- If a transaction fails, inspect the error code returned.
- Refer to the table above to interpret the error and apply the suggested fix.

### 5. Check Parameter Types and Lengths
- Ensure all parameters (amounts, addresses, etc.) are of the correct type and length.
- For multi-asset vaults, input arrays must match the number of assets.

### 6. Review Contract and SDK Versions
- Make sure you are using compatible versions of the SDK and smart contracts.


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



## Additional Resources
- [DeFindex Protocol Documentation](https://github.com/paltalabs)
- [Smart Contract Error Codes](../contracts/vault/src/error.rs)

If you encounter an issue not covered here, please open an issue on the project's GitHub repository.
