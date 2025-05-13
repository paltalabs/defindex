---
cover: ../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
---

# Creating a DeFindex Vault

Webapp: https://app.defindex.io/
Vault Example: https://app.defindex.io/vault/CCFWKCD52JNSQLN5OS4F7EG6BPDT4IRJV6KODIEIZLWPM35IKHOKT6S2

### Prerequisites: Vault Roles

Before creating a vault, you'll need to establish the following key roles with their respective addresses:

#### Core Roles

1. **Vault Manager**
   * Primary owner of the vault
   * Controls vault settings and role assignments
   * Manages contract upgrades (if enabled)
   * Recommended: Use a multisig wallet for enhanced security
2. **Rebalance Manager**
   * Responsible for fund allocation between strategies
   * Optimizes strategy distribution
   * Recommended: Implement as an automated bot
3. **Fee Receiver**
   * Designated address for collecting strategy performance fees
   * Should be a secure, dedicated wallet
4. **Emergency Manager**
   * Handles emergency fund recovery from strategies
   * Critical for risk management
   * Recommended: Implement as an automated bot

### Deployment Process

#### Initial Setup

* Any address with XLM can deploy the vault
* No multisig required for deployment
* Recommended: Use a fresh address for deployment

#### Strategy Selection

Choose from our curated and audited strategies:

**Blend Fixed Pool Strategies (with Autocompound)**

* USDC
* EURC
* XLM

**Blend Yieldblox Pool Strategies (with Autocompound)**

* USDC
* EURC
* XLM

#### Best Practices

* Recommended configuration: One asset with two strategies
* This provides optimal balance between yield and risk management

#### Security Initialization

* Required: Initial deposit of 1001 stroops of the chosen underlying asset
* Purpose: Security verification
* Timing: Complete before user implementation
* This ensures the vault is properly initialized and secure

### Implementation Timeline

1. Set up all role addresses
2. Deploy vault
3. Select strategies
4. Make initial security deposit
5. Begin user implementation

This structured approach ensures a secure and efficient vault creation process while maintaining best practices for DeFi operations.


### Interacting with the Vault

## Using the Example Script (`vault_usage_example.ts`)

You can interact with your DeFindex Vault directly from the command line using the provided example script: `Contracts/src/vault_usage_example.ts`. This script demonstrates how to perform key vault operations such as deposit, withdraw, invest, unwind, and harvest.

### Prerequisites

- Node.js and yarn installed
- All dependencies installed (`yarn install` in the project root)
- Properly configured environment (setup a `.env` file, see `Contracts/src/utils/env_config.js` for user/secret setup)
- The vault and strategy contracts deployed and addresses set in your address book

### How to Use

1. **Navigate to the Contracts directory:**
   ```bash
   cd Contracts
   ```
2. **Edit the script if needed:**
   Uncomment the function call(s) you want to run at the bottom of `src/vault_usage_example.ts` (e.g., `await deposit();`).
3. **Run the script:**
   ```bash
   yarn vault-example <network>
   ```
   Replace `<network>` with your target network (e.g., `testnet`, `mainnet`, or your custom config).

### Available Operations

- **Deposit:**
  Deposits assets into the vault for the configured user.
  ```typescript
  await deposit();
  ```
- **Withdraw:**
  Withdraws assets from the vault for the configured user.
  ```typescript
  await withdraw();
  ```
- **Invest:**
  Allocates vault funds into a strategy (admin only).
  ```typescript
  await invest();
  ```
- **Unwind:**
  Withdraws funds from a strategy back to the vault (admin only).
  ```typescript
  await unwind();
  ```
- **Harvest:**
  Triggers a strategy harvest (keeper only).
  ```typescript
  await harvest();
  ```

**Note:** Only uncomment and run one operation at a time to avoid transaction conflicts. Make sure your environment variables and address book are set up for the network you are targeting.

For more details, review the comments and code in `vault_usage_example.ts`.

