# Interact with your Vault

To integrate DeFindex into your wallet, you can choose between two approaches:

1. **SDKs**: Utilize the SDKs provided by DeFindex, which facilitate interaction with the protocol and are faster to implement.
2. **Smart Contracts**: Interact directly with DeFindex's smart contracts, giving you greater control over transactions but requiring a deeper understanding of the protocol's structure.

DeFindex is a protocol that allows users to interact with various investment strategies and liquidity pools. To integrate it into your wallet, you need to understand how transactions and the smart contracts that make up the protocol are structured.

You can review the contract addresses in the [`~/public/`](https://github.com/paltalabs/defindex/tree/main/public) folder, where you'll find information about the contract addresses, or deploy your own custom vault and strategies using our Factory contract.

The first thing you need to do is to deploy a vault instance.

***

***

### Interacting with the Vault

Within vault interactions, there are several methods you can use to manage and query the vault's state. Here are the most relevant ones:

**User-facing relevant methods:**

* **Deposit**: Allows users to deposit assets into the vault.
* **Withdraw**: Allows users to withdraw assets from the vault.
* **Balance**: Allows users to query their vault balance.

**Management methods:**

* **rebalance**: Allows adjusting asset allocation within the vault.
* **rescue**: Allows recovering assets in critical situations.
* **set\_fees**: Allows managing the fees associated with the vault.
* **pause / unpause**: Allows pausing or resuming vault operations.

**Methods available only to the `Manager` role:**

* **set\_fee\_receiver**: Allows changing the fee receiver of the vault.
* **set\_manager**: Allows changing the manager of the vault.
* **set\_emergency\_manager**: Allows changing the emergency manager of the vault.
* **set\_rebalance\_manager**: Allows changing the rebalance manager of the vault.
* **upgrade**: Allows changing the WASM code without requiring users signatures.
* **lock\_fees**: Allows locking the fees in the vault, preventing them from being withdrawn until the lock is released.
* **release\_fees**: Allows releasing the locked fees in the vault, making them available for withdrawal.

You can find the complete list of methods and their parameters in the [Vault contract](../../../contracts/vault/src/interface.rs)

### Creating Transactions to Interact with the Vault

}

## Using the Example Script (`vault_usage_example.ts`)

You can interact with your DeFindex Vault directly from the command line using the provided example script: `Contracts/src/vault_usage_example.ts`. This script demonstrates how to perform key vault operations such as deposit, withdraw, invest, unwind, and harvest.

### Prerequisites

* Node.js and yarn installed
* All dependencies installed (`yarn install` in the project root)
* Properly configured environment (setup a `.env` file, see `Contracts/src/utils/env_config.js` for user/secret setup)
* The vault and strategy contracts deployed and addresses set in your address book

### How to Use

1.  **Navigate to the Contracts directory:**

    ```bash
    cd Contracts
    ```
2. **Edit the script if needed:** Uncomment the function call(s) you want to run at the bottom of `src/vault_usage_example.ts` (e.g., `await deposit();`).
3.  **Run the script:**

    ```bash
    yarn vault-example <network>
    ```

Replace `<network>` with your target network (e.g., `testnet`, `mainnet`, or your custom config).

### Available Operations

*   **Deposit:**

    Deposits assets into the vault for the configured user.

    ```typescript
    await deposit();
    ```
*   **Withdraw:** Withdraws assets from the vault for the configured user.

    ```typescript
    await withdraw();
    ```
*   **Invest:**

    Allocates vault funds into a strategy (admin only).

    ```typescript
    await invest();
    ```
*   **Unwind:**

    Withdraws funds from a strategy back to the vault (admin only).

    ```typescript
    await unwind();
    ```
*   **Harvest:** Triggers a strategy harvest (keeper only).

    ```typescript
    await harvest();
    ```

**Note:** Only uncomment and run one operation at a time to avoid transaction conflicts. Make sure your environment variables and address book are set up for the network you are targeting.

For more details, review the comments and code in `vault_usage_example.ts`.

***

***

If you need a solution out of the box, you can use the DeFindex SDKs, which provide a set of functions to interact with the vault and strategies without having to manually create transactions. The SDKs handle the underlying complexities and allow you to focus on building your wallet's user interface and experience.
