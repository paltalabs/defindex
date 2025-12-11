# Getting Started with API

First generate API Key:

1. Register -> [https://api.defindex.io/register](https://api.defindex.io/register)
2. Login ->[ https://api.defindex.io/login](https://api.defindex.io/login)
3. Create api\_key and refresh

For more details, refer to the [DeFindex API documentation](https://api.defindex.io/docs).

Postman collection json [here](../wallet-developer/postman_collection.json)

This guide will walk you through integrating DeFindex into your app using the provided API. We'll use TypeScript for the examples, but the concepts apply to any language.

## 🚀 TypeScript SDK Available!

If you're developing in TypeScript, we highly recommend using our official SDK instead of direct API integration. The SDK provides:

* Type safety and comprehensive TypeScript definitions
* Simplified authentication with API keys
* Built-in error handling and validation
* Complete coverage of all API endpoints
* Working examples and detailed documentation

[**Check out the DeFindex TypeScript SDK documentation**](../advanced-documentation/sdks/02-defindex-sdk.md) **for the easiest integration experience.**

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

```typescript
```

Go to[ interact with vault](smart-contracts/), see the implementations of the functions:

* Deposit
* Withdraw
* Balance
* APY
