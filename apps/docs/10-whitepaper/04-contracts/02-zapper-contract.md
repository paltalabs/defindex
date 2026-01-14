---
cover: ../../../.gitbook/assets/image 31.png
coverY: 0
---

# DeFindex Zapper

This contract enables users to invest in and withdraw from a DeFindex Vault withouyt needing to hold the exact required set of assets of the vault.

For instance, if a DeFindex Vault requires both USDC and XLM in a defined ratio, the Zapper contract allows users to inpu USDC, automatically swapping the USDC for XLM and depositing both assets into the DeFindex Vault according to a predefined ratio. Similarly, the Zapper contract facilitates withdrawals by swapping the XLM back to USDC before returning the funds to the user.

The specific paths used for asset swaps, as well as the proportion of the output assets, are determined off-chain.

### Functions

* `deposit`: Allows users to deposit assets into the DeFindex Vault.
* `zap`: Allows users to deposit assets into the DeFindex using a single asset. This function receives the amount of one asset and an array of Soroswap's Aggregator Swap transactions. This array is computed offchain using the best path and the proportion of the output assets.
* `zap_deposit`: It executes a zap and a deposit in a single transaction.
* `withdraw`: Allows users to withdraw assets from the DeFindex.
* `zap_withdraw`: Allows users to withdraw assets from the DeFindex and receive a single asset. This function receives the amount of one asset and an array of Soroswap's Aggregator Swap transactions. This array is computed offchain using the best path and the proportion of the output assets.
