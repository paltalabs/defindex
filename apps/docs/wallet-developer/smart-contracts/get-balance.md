# Get Balance

#### Balance

To query the vault's balance, use the `balance` method. Here are the steps to create the transaction:

1. **Prepare parameters**:
   * `from`: The address of the user who wants to query the balance. Represents a Soroban address.
2.  **Example transaction**:

    ```json
    {
      "method": "balance",
      "params": {
        "from": "GCINP..."
      }
    }
    ```

### Balance

Fetches the balance for a user.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function balance(user: string, apiClient: ApiClient): bigint {
    const {underlyingBalance: balance} = await apiClient.getData("balance", vault, {
        from: user
    });
    return BigInt(balance[0]);
}
```
