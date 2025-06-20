# DeFindex API Integration Tutorial

This guide will walk you through integrating DeFindex into your app using the provided API. We'll use TypeScript for the examples, but the concepts apply to any language.

## Prerequisites

- Basic knowledge of TypeScript or JavaScript
- Node.js environment
- [Stellar SDK](https://www.stellar.org/developers/reference/) installed (`npm install stellar-sdk`)

---

## 1. Setting Up the API Client

First, create an `ApiClient` class to handle authentication and API requests.

```typescript
import StellarSdk from 'stellar-sdk';

class ApiClient {
    private accessToken: string | null = null;
    private readonly apiEndpoint = "api.defindex.io";

    constructor(private username: string, private password: string) {}

    // Authenticate and store the access token
    async login(): Promise<void> {
        const response = await fetch(`https://${this.apiEndpoint}/login`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ username: this.username, password: this.password }),
        });

        if (!response.ok) throw new Error("Login failed");

        const data = await response.json();
        this.accessToken = data.token;
    }

    // Helper for POST requests
    async postData(endpoint: string, vault: string, params: Record<string, any>): Promise<any> {
        if (!this.accessToken) throw new Error("Not authenticated");

        const response = await fetch(`https://${this.apiEndpoint}/${vault}/${endpoint}`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${this.accessToken}`
            },
            body: JSON.stringify(params)
        });
        return await response.json();
    }

    // Helper for GET requests
    async getData(endpoint: string, vault: string, params?: Record<string, any>): Promise<any> {
        if (!this.accessToken) throw new Error("Not authenticated");

        const url = params
            ? `https://${this.apiEndpoint}/${endpoint}?${new URLSearchParams(params).toString()}`
            : `https://${this.apiEndpoint}/${endpoint}`;

        const response = await fetch(url, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${this.accessToken}`
            }
        });
        return await response.json();
    }
}
```

---

## 2. Implementing Core Functions

Below are the main functions you'll need: `deposit`, `withdraw`, `balance`, and `apy`.

### Deposit

Deposits funds into the DeFindex vault.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function deposit(amount: number, user: string, apiClient: ApiClient, signerFunction: (tx: string) => string) {
    // Step 1: Request an unsigned transaction from the API
    const { transaction: unsignedTx } = await apiClient.postData("deposit", vault, {
        amount,
        user
    });

    // Step 2: Sign the transaction (implement your own signer)
    const signedTx = signerFunction(unsignedTx);

    // Step 3: Send the signed transaction back to the API
    const response = await apiClient.postData("send", vault, {
        signedTx
    });
    return response;
}
```

### Withdraw

Withdraws funds from the DeFindex vault.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function withdraw(amount: number, user: string, apiClient: ApiClient, signerFunction: (tx: string) => string) {
    const { transaction: unsignedTx } = await apiClient.postData("withdraw", vault, {
        amount,
        user
    });

    // This should be done by implementer
    const signedTx = signerFunction(unsignedTx);

    const response = await apiClient.postData("send", vault, {
        signedTx
    });

    return response;
}
```

### Balance

Fetches the balance for a user.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function balance(user: string, apiClient: ApiClient) {
    const data = await apiClient.postData("balance", vault, {
        user
    });
    return data;
}
```

### APY

Fetches the current APY for the vault.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function apy(apiClient: ApiClient) {
    const data = await apiClient.getData("apy", vault);
    return data;
}
```

---

## 3. Usage Example

```typescript
const apiClient = new ApiClient('your-username', 'your-password');

async function main() {
    await apiClient.login();

    // Implement your own signer function
    const signerFunction = (unsignedTx: string) => {
        // Use StellarSdk or your wallet to sign the transaction
        // Example: return StellarSdk.TransactionBuilder.fromXDR(unsignedTx, StellarSdk.Networks.TESTNET).sign(...).toXDR();
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

---

## 4. Notes

- **Signer Function:** You must implement the `signerFunction` to sign transactions using your wallet or key management system.
- **Error Handling:** Add appropriate error handling for production use.
- **Security:** Never expose your private keys or sensitive credentials.

---

For more details, refer to the [DeFindex API documentation](https://api.defindex.io/docs).
