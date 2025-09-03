# Manage your Vault

#### Rebalance

To adjust asset allocation within the vault, use the `rebalance` method. Here are the steps to create the transaction:

1. **Prepare parameters**:
   * `caller`: The address of the user performing the rebalance.
2.  **Example transaction**:

    ```json
    {
      "method": "rebalance",
      "params": {
        "caller": "GCINP..."
      }
    }
    ```

***

#### Rescue

To recover assets in critical situations, use the `rescue` method. Here are the steps to create the transaction:

1. **Prepare parameters**:
   * `strategy_address`: The address of the strategy from which you want to recover assets. This must be a valid address of a strategy linked to the vault.
   * `caller`: The address of the user performing the rescue operation.
2.  **Example transaction**:

    ```json
    {
      "method": "rescue",
      "params": {
        "strategy_address": "GCINP...",
        "caller": "GCINP..."
      }
    }
    ```

***

#### Pause / Unpause

To pause or unpause a strategy, use the `pause_strategy` and `unpause_strategy` methods. Here are the steps to create the transactions:

1. **Prepare parameters**:
   * `strategy_address`: The address of the strategy you want to pause or unpause.
   * `caller`: The address of the user performing the operation.
2.  **Example transaction for pausing**:

    ```json
    {
      "method": "pause_strategy",
      "params": {
        "strategy_address": "GCINP...",
        "caller": "GCINP..."
      }
    }
    ```
3.  **Example transaction for unpausing**:

    ```json
    {
      "method": "unpause_strategy",
      "params": {
        "strategy_address": "GCINP...",
        "caller": "GCINP..."
      }
    }
    ```

***

#### Upgrade

To update the vault's WASM code, use the `upgrade` method. Here are the steps to create the transaction:

1. **Prepare parameters**:
   * `new_wasm_hash`: The hash of the new WASM code.
   * `caller`: The address of the user performing the upgrade.
2.  **Example transaction**:

    ```json
    {
      "method": "upgrade",
      "params": {
        "new_wasm_hash": "HASH...",
        "caller": "GCINP..."
      }
    }
    ```
