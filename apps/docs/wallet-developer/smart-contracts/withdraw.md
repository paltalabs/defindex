# Withdraw

#### Withdraw

To withdraw assets from the vault, use the `withdraw` method. Here are the steps to create the transaction:

1. **Prepare parameters**:
   * `withdraw_shares`: The amount of vault shares you wish to withdraw.
   * `min_amounts_out`: A vector specifying the minimum amounts required to receive before the transaction fails (tolerance). This amount is represented in underlying assets.
   * `from`: The address of the user performing the withdrawal, who will receive the funds. Represents a Soroban address.
2.  **Example transaction**:

    ```json
    {
      "method": "withdraw",
      "params": {
        "withdraw_shares": 500,
        "min_amounts_out": [450],
        "from": "GCINP..."
      }
    }
    ```

***

### Withdraw

Withdraws funds from the DeFindex vault.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function withdraw(amount: number, user: string, apiClient: ApiClient, signerFunction: (tx: string) => string) {
    const { xdr: unsignedTx } = await apiClient.postData("withdraw", vault, {
        amounts: [amount],
        from: user
    });

    // This should be done by implementer
    const signedTx = signerFunction(unsignedTx);

    const response = await apiClient.postData("send", vault, {
        xdr: signedTx
    });

    return response;
}
```

#### Withdraw from Vault

Remove funds by specifying amounts:

```typescript
const withdrawData: WithdrawFromVaultParams = {
  amounts: [500000], // Specific amounts to withdraw
  caller: userAddress,
  slippageBps: 100 // 1% slippage tolerance
};

const response = await sdk.withdrawFromVault(vaultAddress, withdrawData, SupportedNetworks.TESTNET);
// Sign response.xdr with the caller account and submit transaction

```

#### Withdraw by Shares

Remove funds by burning vault shares:

```typescript
const shareData: WithdrawSharesParams = {
  shares: 1000000, // Number of vault shares to burn
  caller: userAddress,
  slippageBps: 100
};

const response = await sdk.withdrawShares(vaultAddress, shareData, SupportedNetworks.TESTNET);
// Sign response.xdr with the caller account and submit transaction
```

####
