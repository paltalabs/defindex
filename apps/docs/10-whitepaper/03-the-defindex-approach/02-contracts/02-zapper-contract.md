# DeFindex Zapper
This contract enables users to invest in and withdraw from a DeFindex using a single asset. For instance, if a DeFindex requires both USDC and XLM, the Zapper contract allows users to deposit USDC, automatically swapping the USDC for XLM and depositing both assets into the DeFindex according to a predefined ratio. Similarly, the Zapper contract facilitates withdrawals by swapping the XLM back to USDC before returning the funds to the user.

The specific paths used for asset swaps, as well as the proportion of the output assets, are determined off-chain.

### Functions
- `deposit`: Allows users to deposit assets into the DeFindex.
- `zap`: Allows users to deposit assets into the DeFindex using a single asset. This function receives the amount of one asset and an array of Soroswap's Aggregator Swap transactions. This array is computed offchain using the best path and the proportion of the output assets. 
- `zap_deposit`: It executes a zap and a deposit in a single transaction.
- `withdraw`: Allows users to withdraw assets from the DeFindex.
- `zap_withdraw`: Allows users to withdraw assets from the DeFindex and receive a single asset. This function receives the amount of one asset and an array of Soroswap's Aggregator Swap transactions. This array is computed offchain using the best path and the proportion of the output assets. 
