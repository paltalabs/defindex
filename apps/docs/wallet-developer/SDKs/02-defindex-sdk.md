---
cover: ../.gitbook/assets/image 31.png
coverY: 0
---

# DeFindex TypeScript SDK

Welcome to the DeFindex TypeScript SDK documentation! This SDK provides server-side access to DeFindex's vault management system through a comprehensive TypeScript interface. With this SDK, you can:

1. Create and manage decentralized vaults
2. Perform vault operations (deposit, withdraw, balance queries)
3. Access real-time APY data
4. Execute administrative operations
5. Integrate secure API key authentication

## Prerequisites

Before integrating the SDK, ensure you have:
- Node.js environment (version 16 or higher)
- TypeScript knowledge for optimal development experience
- API key from DeFindex (contact paltalabs team for access)
- Understanding of Stellar/Soroban blockchain concepts

## Integration Guide

### 1. Install the SDK

Add the SDK to your project using your preferred package manager:

```bash
npm install @defindex/sdk
# or
pnpm install @defindex/sdk
# or
yarn add @defindex/sdk
```

### 2. Import and Initialize

Import the SDK and configure it with your API key:

```typescript
import { DefindexSDK, SupportedNetworks } from '@defindex/sdk';

// Initialize with API key (recommended for server-side use)
const sdk = new DefindexSDK({
  apiKey: process.env.DEFINDEX_API_KEY, // Store securely in environment variables
  baseUrl: 'https://api.defindex.io',   // Optional: defaults to production API
  timeout: 30000                        // Optional: request timeout in milliseconds
});
```

## Quick Start

Here's a minimal example to get you started with vault operations:

```typescript
import { DefindexSDK, SupportedNetworks } from '@defindex/sdk';

// Initialize the SDK
const sdk = new DefindexSDK({
  apiKey: 'sk_your_api_key_here'
});

async function quickStart() {
  try {
    // Check API health
    const health = await sdk.healthCheck();
    console.log('API Status:', health.status.reachable);
    
    // Get factory address
    const factory = await sdk.getFactoryAddress(SupportedNetworks.TESTNET);
    console.log('Factory Address:', factory.address);
    
    // Get vault information
    const vaultAddress = 'CVAULT_CONTRACT_ADDRESS...';
    const vaultInfo = await sdk.getVaultInfo(vaultAddress, SupportedNetworks.TESTNET);
    console.log(`Vault: ${vaultInfo.name} (${vaultInfo.symbol})`);
    
    // Check user balance
    const userAddress = 'GUSER_ADDRESS...';
    const balance = await sdk.getVaultBalance(vaultAddress, userAddress, SupportedNetworks.TESTNET);
    console.log(`Vault Shares: ${balance.dfTokens}`);
    
  } catch (error) {
    console.error('Operation failed:', error.message);
  }
}

quickStart();
```

## Implementation Example

### Complete Vault Operations Flow

Here's a comprehensive example demonstrating vault creation, deposits, and withdrawals:

```typescript
import { 
  DefindexSDK, 
  SupportedNetworks, 
  CreateDefindexVault,
  DepositToVaultParams,
  WithdrawFromVaultParams 
} from '@defindex/sdk';

const sdk = new DefindexSDK({
  apiKey: process.env.DEFINDEX_API_KEY
});

async function completeVaultFlow() {
  try {
    // 1. Create a new vault
    const vaultConfig: CreateDefindexVault = {
      roles: {
        0: "GEMERGENCY_MANAGER_ADDRESS...", // Emergency Manager
        1: "GFEE_RECEIVER_ADDRESS...",      // Fee Receiver  
        2: "GVAULT_MANAGER_ADDRESS...",     // Vault Manager
        3: "GREBALANCE_MANAGER_ADDRESS..."  // Rebalance Manager
      },
      vault_fee_bps: 100, // 1% fee (100 basis points)
      assets: [{
        address: "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC", // XLM asset
        strategies: [{
          address: "CBO77JLVAT54YBRHBY4PSITLILWAAXX5JHPXGBFRW2XUFQKXZ3ZLJ7MJ", // Strategy contract
          name: "XLM HODL Strategy",
          paused: false
        }]
      }],
      name_symbol: { 
        name: "My DeFi Vault", //Max 20 characters
        symbol: "MDV" 
      },
      upgradable: true,
      caller: "GCREATOR_ADDRESS..." // Public key of the signer account
    };

    const createResponse = await sdk.createVault(vaultConfig, SupportedNetworks.TESTNET);
    console.log('Vault XDR for signing:', createResponse.xdr);
    
    // Sign the XDR with your wallet here
    // const signedXDR = await yourWallet.sign(createResponse.xdr);
    // const txResult = await sdk.sendTransaction(signedXDR, SupportedNetworks.TESTNET);
    
    // 2. Deposit to vault
    const vaultAddress = 'CVAULT_CONTRACT_ADDRESS...';
    const depositData: DepositToVaultParams = {
      amounts: [1000000], // 1 XLM (7 decimals)
      caller: 'GUSER_ADDRESS...', // User's public key from which to sign and deposit
      invest: true, // Auto-invest after deposit
      slippageBps: 100 // 1% slippage tolerance
    };

    const depositResponse = await sdk.depositToVault(vaultAddress, depositData, SupportedNetworks.TESTNET);
    console.log('Deposit XDR for signing:', depositResponse.xdr);
    
    // Sign the deposit XDR with your wallet here
    // const signedDepositXDR = await yourWallet.sign(depositResponse.xdr);
    // const depositResult = await sdk.sendTransaction(signedDepositXDR, SupportedNetworks.TESTNET);
    
    // 3. Check balance after deposit
    const balance = await sdk.getVaultBalance(
      vaultAddress, 
      'GUSER_ADDRESS...', 
      SupportedNetworks.TESTNET
    );
    console.log(`New vault shares: ${balance.dfTokens}`);
    
    // 4. Withdraw from vault
    const withdrawData: WithdrawFromVaultParams = {
      amounts: [500000], // 0.5 XLM
      caller: 'GUSER_ADDRESS...',
      slippageBps: 100
    };

    const withdrawResponse = await sdk.withdrawFromVault(vaultAddress, withdrawData, SupportedNetworks.TESTNET);
    console.log('Withdrawal XDR for signing:', withdrawResponse.xdr);
    
  } catch (error) {
    console.error('Vault operation failed:', error.message);
  }
}
```

---

## Core Functions

### System Operations

#### Health Check
Monitor API availability and system status:

```typescript
const health = await sdk.healthCheck();
if (health.status.reachable) {
  console.log('API is healthy and operational');
} else {
  console.log('API health issues detected');
}
```

### Factory Operations

#### Get Factory Address
Retrieve the factory contract address for vault creation:

```typescript
const factory = await sdk.getFactoryAddress(SupportedNetworks.TESTNET);
console.log('Factory contract:', factory.address);
```

#### Create Vault
Deploy a new vault with custom configuration:

```typescript
const vaultConfig: CreateDefindexVault = {
  roles: {
    0: "GEMERGENCY_MANAGER...",  // Emergency Manager role
    1: "GFEE_RECEIVER...",       // Fee Receiver role
    2: "GVAULT_MANAGER...",      // Vault Manager role
    3: "GREBALANCE_MANAGER..."   // Rebalance Manager role
  },
  vault_fee_bps: 100,            // 1% vault fee
  assets: [{
    address: "CASSET_ADDRESS...", // Asset contract address
    strategies: [{
      address: "CSTRATEGY_ADDR...", // Strategy contract address
      name: "Strategy Name",
      paused: false
    }]
  }],
  name_symbol: { 
    name: "Vault Name", 
    symbol: "VLT" 
  },
  upgradable: true,
  caller: "GCALLER_ADDRESS..."
};

const response = await sdk.createVault(vaultConfig, SupportedNetworks.TESTNET);
// Sign response.xdr with your wallet and submit via sendTransaction()
```

### Vault Operations

#### Get Vault Information
Query comprehensive vault details:

```typescript
const vaultInfo = await sdk.getVaultInfo(vaultAddress, SupportedNetworks.TESTNET);
console.log(`Vault: ${vaultInfo.name} (${vaultInfo.symbol})`);
console.log(`Total Assets: ${vaultInfo.totalAssets}`);
console.log(`Vault Fee: ${vaultInfo.feesBps.vaultFee / 100}%`);

// Display strategies
vaultInfo.assets.forEach((asset, index) => {
  console.log(`Asset ${index + 1}: ${asset.address}`);
  asset.strategies.forEach(strategy => {
    console.log(`  - ${strategy.name}: ${strategy.paused ? 'PAUSED' : 'ACTIVE'}`);
  });
});
```

#### Get User Balance
Check user's vault position:

```typescript
const balance = await sdk.getVaultBalance(
  vaultAddress, 
  userAddress, 
  SupportedNetworks.TESTNET
);
console.log(`Vault Shares: ${balance.dfTokens}`);
console.log(`Underlying Value: ${balance.underlyingBalance}`);
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

#### Get Vault APY
Query current Annual Percentage Yield:

```typescript
const apy = await sdk.getVaultAPY(vaultAddress, SupportedNetworks.TESTNET);
console.log(`Current APY: ${apy.apyPercent}%`);
console.log(`Calculation period: ${apy.period}`);
```

### Administrative Operations

#### Emergency Rescue
Execute emergency asset recovery and pauses strategy (requires Emergency Manager role):

```typescript
const rescueData: RescueFromVaultParams = {
  strategy_address: 'CSTRATEGY_TO_RESCUE...',
  caller: 'GEMERGENCY_MANAGER_ADDRESS...'
};

const response = await sdk.emergencyRescue(vaultAddress, rescueData, SupportedNetworks.TESTNET);
console.log('Emergency rescue XDR:', response.transactionXDR);
// Sign and submit the transaction
```

#### Pause/Unpause Strategy
Control strategy operations (requires appropriate role):

```typescript
// Note: Ensure the caller has the necessary role to perform this operation
// Pause a strategy
await sdk.pauseStrategy(vaultAddress, {
  strategy_address: 'CSTRATEGY_ADDRESS...',
  caller: 'GMANAGER_ADDRESS...'
}, SupportedNetworks.TESTNET);

// Unpause a strategy
await sdk.unpauseStrategy(vaultAddress, {
  strategy_address: 'CSTRATEGY_ADDRESS...',
  caller: 'GMANAGER_ADDRESS...'
}, SupportedNetworks.TESTNET);
```

### Transaction Management

#### Submit Signed Transactions
Send signed XDR to the Stellar network:

```typescript
// Submit via Stellar directly
const response = await sdk.sendTransaction(
  signedXDR, 
  SupportedNetworks.TESTNET,
  false // Don't use LaunchTube
);

// Submit via LaunchTube
const response = await sdk.sendTransaction(
  signedXDR, 
  SupportedNetworks.TESTNET,
  true // Use LaunchTube
);

console.log('Transaction hash:', response.hash);
console.log('Status:', response.status);
```

---

## Error Handling

The SDK provides comprehensive error handling with specific error types:

```typescript
import { 
  isApiError, 
  isAuthError, 
  isValidationError, 
  isNetworkError 
} from '@defindex/sdk';

try {
  const vaultInfo = await sdk.getVaultInfo(vaultAddress, network);
} catch (error) {
  if (isAuthError(error)) {
    console.error('Authentication failed:', error.message);
    // Check API key configuration
  } else if (isValidationError(error)) {
    console.error('Validation error:', error.message);
    // Check input parameters
  } else if (isNetworkError(error)) {
    console.error('Network error:', error.message);
    // Handle blockchain/network issues
  } else {
    console.error('Unknown error:', error.message);
  }
}
```

## Security Best Practices

1. **Environment Variables**: Always store API keys in environment variables
```typescript
const sdk = new DefindexSDK({
  apiKey: process.env.DEFINDEX_API_KEY // Never hardcode credentials
});
```

2. **Error Handling**: Always wrap API calls in try-catch blocks
```typescript
try {
  const result = await sdk.someOperation();
  // Handle success
} catch (error) {
  // Handle error appropriately
  console.error('Operation failed:', error.message);
}
```

3. **Server-Side Only**: This SDK is designed for server-side use only
4. **Role Management**: Understand vault roles and permissions before administrative operations

---

## Running Examples

The SDK includes a comprehensive functional example demonstrating all features:

```bash
# Navigate to SDK directory
cd /path/to/defindex-sdk

# Install dependencies
pnpm install

# Copy environment configuration
cp .env.example .env

# Edit .env with your API key
# DEFINDEX_API_KEY=sk_your_api_key_here

# Run the complete example
pnpm run example
```

The example demonstrates:
- SDK initialization and authentication
- API health checking
- Factory operations and vault creation
- Vault deposits and withdrawals
- Administrative vault management
- Error handling patterns

## TypeScript Support

The SDK provides full TypeScript support with comprehensive type definitions:

```typescript
import {
  DefindexSDK,
  DefindexSDKConfig,
  SupportedNetworks,
  CreateDefindexVault,
  DepositToVaultParams,
  WithdrawFromVaultParams,
  VaultInfo,
  VaultBalance,
  VaultAPY
} from '@defindex/sdk';
```

## Support and Resources

- **API Documentation**: [https://api.defindex.io/docs](https://api.defindex.io/docs)
- **GitHub Repository / SDK documentation**: [https://github.com/paltalabs/defindex-sdk](https://github.com/paltalabs/defindex-sdk)
- **Developer Support / Discord Community**: [Join our Discord](https://discord.gg/ftPKMPm38f)

For additional questions or integration support, please reach out to our developer support team.