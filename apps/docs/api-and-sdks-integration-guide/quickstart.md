# Quick Start

Get your DeFindex API integration running in **under 5 minutes**. This guide is for developers who want to integrate yield-generating vaults quickly.

## 📚 Documentation Hierarchy

**Choose your path:**

* 🐶 **New to blockchain/Stellar?** → Start with [`beginner-guide.md`](guides-and-tutorials/beginner-guide.md)
* 🎮 **Want to see it working?** → Try [`beginner-example.html`](../wallet-developer/beginner-example.html)
* ⚡ **Experienced developer?** → Continue with this Quick Start
* 📖 **Full API reference?** → See [api.defindex.io/docs](https://api.defindex.io/docs)

## Prerequisites

* **JavaScript/TypeScript** experience
* **Wallet integration** knowledge (Freighter/StellarWalletsKit)
* **API integration** experience
* **3-5 minutes** of your time

## 🚀 Step 1: Get Your API Key (1 minute)

1. Go to [Generate API Key](generate-api-key.md)
2. You'll receive an API key (starts with `sk_`)
3. Copy and store securely

### ⚠️ Authentication Format

```javascript
// ✅ CORRECT - Use Bearer authentication
const headers = {
  'Authorization': 'Bearer sk_test_1234567890abcdef',
  'Content-Type': 'application/json'
}

// ❌ WRONG - These will result in 403 Forbidden
const wrongHeaders = {
  'X-API-Key': 'sk_test_1234567890abcdef',    // Wrong header name
  'Authorization': 'sk_test_1234567890abcdef', // Missing 'Bearer'
  'Authorization': 'Bearer sk_expiredApiKey', // Expired key
}
```

## 🔧 Step 2: Environment Setup (2 minutes)

```javascript
const CONFIG = {
    API_BASE_URL: 'https://api.defindex.io',  // Production
    API_KEY: 'sk_your_api_key_here',
    NETWORK: 'testnet', // or 'mainnet'

    // Example vault address (replace with actual vault)
    VAULT_ADDRESS: 'CAQEPGA3XDBZSWHYLBUSH2UIP2SHHTEMXMHFPLIEN6RYH7G6GEGJWHGN',

    // Testnet token addresses
    TOKENS: {
        XLM: 'CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC',
        USDC: 'CBBHRKEP5M3NUDRISGLJKGHDHX3DA2CN2AZBQY6WLVUJ7VNLGSKBDUCM'
    }
};
```

### Quick Testnet Setup

```bash
# Get testnet XLM
curl "https://friendbot.stellar.org?addr=YOUR_ADDRESS"

# Get test tokens at:
# https://app.soroswap.finance (testnet mode)
```

## 💱 Step 3: Core Integration (2 minutes)

### Essential API Flow

**4-step process:** Get Vault Info → Build Deposit → Sign → Send

```javascript
// Minimal DeFindex vault client implementation
class DeFindexClient {
    constructor(apiKey, network = 'testnet') {
        this.apiKey = apiKey;
        this.network = network;
        this.baseUrl = 'https://api.defindex.io';
    }

    async apiRequest(endpoint, data, method = 'POST') {
        const url = `${this.baseUrl}${endpoint}?network=${this.network}`;
        const options = {
            method,
            headers: {
                'Authorization': `Bearer ${this.apiKey}`,
                'Content-Type': 'application/json'
            }
        };

        if (method === 'POST' && data) {
            options.body = JSON.stringify(data);
        }

        const response = await fetch(url, options);

        if (!response.ok) {
            const error = await response.json();
            throw new Error(`API Error: ${error.message}`);
        }

        return response.json();
    }

    // 1. Get vault information
    async getVaultInfo(vaultAddress) {
        return this.apiRequest(`/vault/${vaultAddress}`, null, 'GET');
    }

    // 2. Get user's vault balance
    async getVaultBalance(vaultAddress, userAddress) {
        return this.apiRequest(`/vault/${vaultAddress}/balance?from=${userAddress}`, null, 'GET');
    }

    // 3. Build deposit transaction
    async deposit(vaultAddress, amounts, callerAddress, invest = true, slippageBps = 0) {
        return this.apiRequest(`/vault/${vaultAddress}/deposit`, {
            amounts,
            caller: callerAddress,
            invest,
            slippageBps
        });
    }

    // 4. Build withdraw transaction
    async withdraw(vaultAddress, amounts, callerAddress, slippageBps = 0) {
        return this.apiRequest(`/vault/${vaultAddress}/withdraw`, {
            amounts,
            caller: callerAddress,
            slippageBps
        });
    }

    // 5. Submit signed transaction
    async sendTransaction(signedXdr, launchtube = false) {
        return this.apiRequest('/send', {
            xdr: signedXdr,
            launchtube
        });
    }
}

// Usage Example
const client = new DeFindexClient('sk_your_api_key');

async function executeDeposit() {
    try {
        // 1. Get Vault Info
        const vaultInfo = await client.getVaultInfo(CONFIG.VAULT_ADDRESS);
        console.log('Vault info:', vaultInfo);

        // 2. Check current balance
        const balance = await client.getVaultBalance(CONFIG.VAULT_ADDRESS, userAddress);
        console.log('Current vault shares:', balance.dfTokens);

        // 3. Build deposit transaction (1 XLM)
        const { xdr } = await client.deposit(
            CONFIG.VAULT_ADDRESS,
            [10000000], // 1 XLM (7 decimals)
            userAddress,
            true, // auto-invest
            50 // 0.5% slippage
        );

        // 4. Sign (using your preferred wallet)
        const signedXdr = await signWithWallet(xdr);

        // 5. Send
        const result = await client.sendTransaction(signedXdr);

        console.log('Deposit completed!', result.txHash);
    } catch (error) {
        console.error('Deposit failed:', error);
    }
}
```

### Or you can just use the [DeFindex SDK](sdks/) for a more streamlined experience.

### Working Examples

📂 **Complete examples available:**

* [**`beginner-example.html`**](../wallet-developer/beginner-example.html) - Full interactive tutorial with vault deposit/withdraw
* **Typescript SDK examples** - Available in the [DeFindex SDK repository](sdks/)

## 📤 Advanced Options

### Gasless Transactions

```javascript
// For users without XLM for fees (using Launchtube)
const result = await client.sendTransaction(signedXdr, true);
```

### Custom Transaction Submission

```javascript
// Submit through your own infrastructure
import { Server } from '@stellar/stellar-sdk';

const server = new Server('https://horizon-testnet.stellar.org');
const result = await server.submitTransaction(signedTransaction);
```

### Request Parameters

#### Deposit Request

```javascript
{
    amounts: [10000000],     // Array of amounts for each vault asset (7 decimals for XLM)
    caller: userAddress,     // User's wallet address
    invest: true,           // Auto-invest into strategies (recommended: true)
    slippageBps: 50         // 0.5% slippage tolerance (optional, default: 0)
}
```

#### Withdraw Request

```javascript
{
    amounts: [5000000],      // Array of amounts to withdraw from each asset
    caller: userAddress,     // User's wallet address
    slippageBps: 50         // 0.5% slippage tolerance (optional, default: 0)
}
```

#### Send Request

```javascript
{
    xdr: signedXdr,         // Signed transaction XDR
    launchtube: false       // Set true for gasless transactions
}
```

## ❓ Common Issues & Solutions

### "403 Forbidden" errors

1. API key starts with `sk_`
2. Using `Authorization: Bearer <key>` header
3. Using correct base URL
4. API key hasn't been revoked

### "Insufficient balance"

1. Ensure wallet has enough tokens for deposit
2. Check minimum deposit requirements
3. Verify vault is active and accepting deposits

### Network mismatch

```javascript
// Ensure vault exists on the correct network
const vaultUrl = `${API_BASE_URL}/vault/${vaultAddress}?network=testnet`;
```

### Expected Response Times

* `/vault/{address}`: 1-2 seconds
* `/vault/{address}/deposit`: 2-5 seconds
* `/send`: 3-10 seconds

## 🚨 Production Considerations

### Security

* Never expose API keys in frontend code for production
* Use environment variables for sensitive data
* Implement proper error handling and retry logic

### Performance

* Add request timeouts (30s recommended)
* Implement exponential backoff for retries
* Cache quotes for better UX (but respect freshness)

### Monitoring

* Track transaction success rates
* Monitor API response times
* Log error patterns for debugging

## 🎯 Next Steps

1. **Explore full API**: [api.defindex.io/docs](https://api.defindex.io/docs)
2. **Add error handling**: Implement retry logic and user feedback
3. **Production optimization**: Proper state management and caching
4. **Multi-vault support**: Integrate multiple vaults and strategies
5. **APY tracking**: Use vault APY endpoints for performance metrics

## 📚 Additional Resources

* **🔗 API Documentation**: [api.defindex.io/docs](https://api.defindex.io/docs)
* **🌍 Stellar Expert (Testnet)**: https://stellar.expert/explorer/testnet
* **🏦 DeFindex Interface**: https://app.defindex.io
* **💬 Discord Support**: https://discord.gg/ftPKMPm38f
* **📚 SDK Documentation**: [DeFindex SDK](sdks/)

🎉 **Ready to build?** You now have everything needed for a production-ready DeFindex vault integration!
