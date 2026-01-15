# Integration Requirements

Before integrating DeFindex into your application, you'll need to configure three essential parameters. This page provides all the information you need to get started.

## Required Configuration Parameters

### 1. DEFINDEX_API_URL

**Value:** `https://api.defindex.io/`

This is the base URL for all DeFindex API endpoints. All API requests should be made to this base URL.

**Example:**
```javascript
const DEFINDEX_API_URL = 'https://api.defindex.io/';
```

**Full API Documentation:** [https://api.defindex.io/docs](https://api.defindex.io/docs)

---

### 2. DEFINDEX_API_KEY

**How to get your API key:**

1. **Register** an account at [https://api.defindex.io/register](https://api.defindex.io/register)
2. **Login** at [https://api.defindex.io/login](https://api.defindex.io/login)
3. **Create** your API key from your account dashboard
4. **Store** your API key securely (it starts with `sk_`)

**Authentication Format:**
```javascript
// ✅ CORRECT - Use Bearer authentication
const headers = {
  'Authorization': 'Bearer sk_your_api_key_here',
  'Content-Type': 'application/json'
}
```

**⚠️ Important:**
- API keys must be sent in the `Authorization` header with the `Bearer` prefix
- Never expose API keys in frontend code for production use
- Store API keys as environment variables

**For detailed instructions:** See [Generate API Key Guide](generate-api-key.md)

---

### 3. DEFINDEX_VAULT_ADDRESS

The vault address is the Stellar account address of the DeFindex vault you want to interact with. The address you use depends on whether you're testing or deploying to production.

#### For Testing/Development

Use the provided test vault address:

```
CCFWKCD52JNSQLN5OS4F7EG6BPDT4IRJV6KODIEIZLWPM35IKHOKT6S2
```

This is a USDC test vault (`usdc_palta_vault`) that you can use for development and testing purposes.

#### For Production

**You need to create a vault for your production environment.**

1. **Option 1: Use the Factory (Advanced)**
   - Follow the guide: [Using the Factory (Advanced)](../creating-a-defindex-vault/using-the-factory-advanced.md)
   - This allows you to create a custom vault with your own configuration

2. **Option 2: Request Vault Creation**
   - Contact the DeFindex team to create a production vault for you
   - Reach out via [Discord](https://discord.gg/ftPKMPm38f) or email dev@paltalabs.io
   - Provide your requirements (assets, strategies, network, etc.)

**⚠️ Important:**
- Testnet and Mainnet vaults have different addresses
- Always verify the vault address matches your target network
- Ensure the vault is active and accepting deposits before production use

---

## Complete Configuration Example

Here's a complete configuration example you can use in your application:

```javascript
// Environment Configuration
const CONFIG = {
    // API Configuration
    DEFINDEX_API_URL: 'https://api.defindex.io/',
    DEFINDEX_API_KEY: process.env.DEFINDEX_API_KEY, // Store in environment variables
    
    // Vault Configuration
    DEFINDEX_VAULT_ADDRESS: process.env.NODE_ENV === 'production' 
        ? 'YOUR_PRODUCTION_VAULT_ADDRESS'  // Replace with your production vault
        : 'CCFWKCD52JNSQLN5OS4F7EG6BPDT4IRJV6KODIEIZLWPM35IKHOKT6S2', // Test vault
    
    // Network Configuration
    NETWORK: process.env.NODE_ENV === 'production' ? 'mainnet' : 'testnet'
};
```

### Environment Variables Setup

Create a `.env` file (or your environment configuration):

```bash
# .env
DEFINDEX_API_URL=https://api.defindex.io/
DEFINDEX_API_KEY=sk_your_api_key_here
DEFINDEX_VAULT_ADDRESS=CCFWKCD52JNSQLN5OS4F7EG6BPDT4IRJV6KODIEIZLWPM35IKHOKT6S2
```

---

## Quick Start Checklist

- [ ] ✅ Obtained API key from [https://api.defindex.io/register](https://api.defindex.io/register)
- [ ] ✅ Configured `DEFINDEX_API_URL` to `https://api.defindex.io/`
- [ ] ✅ Set up `DEFINDEX_API_KEY` in environment variables
- [ ] ✅ Selected appropriate `DEFINDEX_VAULT_ADDRESS` (test or production)
- [ ] ✅ Verified API authentication works (Bearer token format)
- [ ] ✅ Tested connection to the vault

---

## Next Steps

Once you have all three parameters configured:

1. **Read the API Integration Tutorial:** [API Integration Tutorial](api.md)
2. **Try the Quick Start Guide:** [Quick Start Guide](../guides-and-tutorials/quickstart.md)
3. **Explore the SDKs:** [SDKs Documentation](../sdks/README.md)

---

## Support

If you need help with any of these requirements:

- **API Key Issues:** Check [Generate API Key Guide](generate-api-key.md)
- **Vault Creation:** See [Creating a DeFindex Vault](../creating-a-defindex-vault/README.md)
- **General Support:** Join [Discord](https://discord.gg/ftPKMPm38f) or email dev@paltalabs.io


