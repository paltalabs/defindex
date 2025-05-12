---
cover: ../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
---

# Creating a DeFindex Vault

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

**Fixed Pool Strategies (with Autocompound)**

* USDC
* EURC
* XLM

**Yieldblox Pool Strategies (with Autocompound)**

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
