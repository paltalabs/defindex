# API Integration Tutorial

First[ generate API Key](generate-api-key.md) 

For more details, refer to the [DeFindex API documentation](https://api.defindex.io/docs).

Postman collection json [here](../../wallet-developer/postman_collection.json)

This guide will walk you through integrating DeFindex into your app using the provided API. We'll use TypeScript for the examples, but the concepts apply to any language.

## 🚀 TypeScript SDK Available!

If you're developing in TypeScript, we highly recommend using our official SDK instead of direct API integration. The SDK provides:

* Type safety and comprehensive TypeScript definitions
* Simplified authentication with API keys
* Built-in error handling and validation
* Complete coverage of all API endpoints
* Working examples and detailed documentation

[**Check out the DeFindex TypeScript SDK documentation**](../sdks/02-defindex-sdk.md) **for the easiest integration experience.**

For non-TypeScript projects or custom integrations, continue with this direct API guide below.

***

Complete reference: [API Reference](https://api.defindex.io/docs)

## Prerequisites

* Basic knowledge of TypeScript or JavaScript
* Node.js environment
* [Stellar SDK](https://www.stellar.org/developers/reference/) installed (`npm install stellar-sdk`)
* DeFindex API key (contact PaltaLabs team for access)

***

## 1. Setting Up the API Client

First, create an `ApiClient` class to handle authentication and API requests.

```typescript
import StellarSdk from 'stellar-sdk';

class ApiClient {
    private readonly apiUrl = "api.defindex.io";
    private readonly apiKey: string;

    constructor(apiKey: string) {
        this.apiKey = apiKey;
    }

    // Helper for POST requests
    async postData(endpoint: string, vaultAddress: string, params: Record<string, any>): Promise<any> {
        const response = await fetch(`https://${this.apiUrl}/vault/${vaultAddress}/${endpoint}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.apiKey}`
            },
            body: JSON.stringify(params)
        });
        return await response.json();
    }

    // Helper for GET requests
    async getData(endpoint: string, vaultAddress: string, params?: Record<string, any>): Promise<any> {
        const url = params
            ? `https://${this.apiUrl}/vault/${vaultAddress}/${endpoint}?${new URLSearchParams(params).toString()}`
            : `https://${this.apiUrl}/vault/${vaultAddress}/${endpoint}`;

        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${this.apiKey}`
            }
        });
        return await response.json();
    }
}
```

***

## 2. Implementing Core Functions

Below are the main functions you'll need: `deposit`, `withdraw`, `balance`, and `apy`.

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

### APY

Fetches the current APY for the vault.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function apy(apiClient: ApiClient): number {
    const {apy} = await apiClient.getData("apy", vault);
    return apy;
}
```

***

## 3. Usage Example

```typescript
// Initialize with API key (get from PaltaLabs team)
const apiClient = new ApiClient('sk_your_api_key_here');

async function main() {
    // Implement your own signer function
    const signerFunction = (unsignedTx: string) => {
        // Use StellarSdk or your wallet to sign the transaction
        return unsignedTx; // Replace with actual signing logic
    };

    // Deposit example
    await deposit(100, 'user-address', apiClient, signerFunction);

    // Withdraw example
    await withdraw(50, 'user-address', apiClient, signerFunction);

    // Get balance
    const userBalance = await balance('user-address', apiClient);

    // Get APY
    const currentApy = await apy(apiClient);

    console.log({ userBalance, currentApy });
}

main();
```

***

## 4. Notes

* **API Key:** Contact the PaltaLabs team to obtain your API key. Store it securely as an environment variable.
* **Signer Function:** You must implement the `signerFunction` to sign transactions using your wallet or key management system.
* **Error Handling:** Add appropriate error handling for production use.
* **Security:** Never expose your private keys, API keys, or sensitive credentials in your code.

***
