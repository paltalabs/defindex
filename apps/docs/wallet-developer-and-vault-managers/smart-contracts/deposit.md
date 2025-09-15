# Deposit

To make a deposit into the vault, use the `deposit` method. Here are the steps to create the transaction:

1. **Prepare parameters**:
   * `amounts_desired`: A vector specifying the desired quantities of each asset you wish to deposit.
   * `amounts_min`: A vector specifying the minimum quantities of each asset to be transferred from the source for each asset.
   * `from`: The address of the user making the deposit. Represents a Soroban address.
   * `invest`: A boolean indicating whether the deposited funds should be automatically invested in the vault's strategies (`true`) or remain as idle\_funds (`false`).
2.  **Example transaction**:

    ```json
    {
      "method": "deposit",
      "params": {
        "amounts_desired": [1000],
        "amounts_min": [900],
        "from": "GCINP...",
        "invest": true
      }
    }
    ```

***

#### Deposit Request

```javascript
{
    amounts: [10000000],     // Array of amounts for each vault asset (7 decimals for XLM)
    caller: userAddress,     // User's wallet address
    invest: true,           // Auto-invest into strategies (recommended: true)
    slippageBps: 50         // 0.5% slippage tolerance (optional, default: 0)
}
```

### Deposit

Deposits funds into the DeFindex vault.

```typescript
const vaultAddress = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function deposit(amount: number, user: string, apiClient: ApiClient, signerFunction: (tx: string) => string) {
    // Step 1: Request an unsigned transaction from the API
    const { xdr: unsignedTx } = await apiClient.postData("deposit", vaultAddress, {
        amounts: [amount],
        from: user
    });

    // Step 2: Sign the transaction (implement your own signer)
    const signedTx = signerFunction(unsignedTx);

    // Step 3: Send the signed transaction back to the API
    const response = await apiClient.postData("send", vaultAddress, {
        xdr: signedTx
    });
    return response;
}
```

#### Deposit to Vault

Add funds to a vault:

```typescript
const depositData: DepositToVaultParams = {
  amounts: [1000000, 2000000], // Amounts for each vault asset
  caller: userAddress,
  invest: true, // Automatically invest after deposit
  slippageBps: 100 // 1% slippage tolerance
};

const response = await sdk.depositToVault(vaultAddress, depositData, SupportedNetworks.TESTNET);
// Sign response.xdr with the caller account and submit transaction
```

####
