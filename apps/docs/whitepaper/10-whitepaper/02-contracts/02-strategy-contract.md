---
cover: ../../../.gitbook/assets/image 31.png
coverY: 0
---

# DeFindex Strategy

The Strategy contract is the backbone of the DeFindex Protocol, responsible for generating yields for each DeFindex Vault that integrates it. By adhering to the standardized [DeFindexStrategyTrait](https://crates.io/crates/defindex-strategy-core), these contracts enable seamless interaction between the Vaults and external DeFi protocols.

***

**Key Features of a Strategy**

1. **Protocol-Specific Integration**:\
   Strategies act as a **proxy**, managing specific external protocols such as lending pools or liquidity providers. This design ensures:
   * DeFindex Vaults are decoupled from protocol-specific complexities.
   * Flexibility in introducing new strategies without modifying the core Vault contracts.
2. **Single Underlying Asset**:\
   Each Strategy manages one underlying asset, such as USDC or XLM. For instance:
   * A Strategy can manage a single token like USDC.
   * In liquidity pools, the Strategy manages the liquidity pool token (e.g., USDC/XLM LP tokens).
3. **Position and Balance Tracking**:
   * **Shares Management**: In some cases, like the Blend Strategy, the Strategy must issue **shares** internally to track positions and investments. This happens when the protocol requires specific authorizations or lacks direct support for managing positions at the Vault level.
   * **Proxy Use Cases**: In other cases, where the protocol supports directly adding positions on behalf of the Vault, the Strategy can act purely as a proxy, and shares tracking inside the Strategy might not be necessary.
   * **Consistency with Vaults**: Regardless of the internal tracking mechanism, the `deposit()`, `withdrawal()`, and `balance()` functions **must always return the depositor's balance in the underlying asset**. This ensures that the Vault can accurately track the status and health of its associated Strategy.
4. **Modularity and Extensibility**:\
   Developers can create custom Strategies tailored to specific use cases, DeFi protocols, or yield-generation techniques. This opens doors for innovation while maintaining compatibility with the DeFindex ecosystem.

***

**Core Functions of a Strategy**

Every Strategy implements the [DeFindexStrategyTrait](https://crates.io/crates/defindex-strategy-core), which defines the following core functions:

1. **Initialization** (`__constructor`):
   * Configures the Strategy with parameters such as the underlying asset, external protocol addresses, and custom settings.
2. **Asset Retrieval** (`asset`):
   * Returns the address of the underlying asset managed by the Strategy.
3. **Deposits** (`deposit`):
   * Allows the Vault to deposit assets into the Strategy for yield generation.
   * **Requirement**: Must return the depositor’s balance in the underlying asset.
4. **Harvesting Yields** (`harvest`):
   * Executes protocol-specific actions to claim or generate rewards.
   * Can trigger reinvestments for compounding yields.
5. **Withdrawals** (`withdraw`):
   * Enables the Vault to withdraw assets from the Strategy.
   * **Requirement**: Must return the depositor’s balance in the underlying asset.
6. **Balance Tracking** (`balance`):
   * Provides the current balance of the underlying asset held by the Strategy for a specific depositor.
   * **Requirement**: Must return the balance in the underlying asset, not shares or derivatives.

***

**Advantages of the DeFindexStrategyTrait**

1. **Standardization**:
   * Unified interface for interacting with any Strategy, reducing complexity for Vaults.
   * Facilitates third-party Strategy development while maintaining compatibility.
2. **Transparency**:
   * Vaults can track their balances and yields in terms of the underlying asset, avoiding ambiguity with derivatives or shares.
3. **Flexibility**:
   * Support for diverse protocols and yield-generation mechanisms.
   * Easy to introduce new Strategies or upgrade existing ones without disrupting the Vaults.
4. **Security**:
   * Decoupling Vaults from external protocols minimizes risk by limiting direct interactions.
