# DeFindex Strategy Contract

This contract is responsible for generating yields for every DeFindex Vault that implements it. It provides key functions such as deposit, harvest, withdraw, and balance.

The strategy `struct` that can be implemented by other developers. This can include DeFi protocol developers or anyone with a compelling strategy designed to generate yields.

A Strategy has only one underlying asset. For instance, a Strategy could be designed to generate yields from a single asset, such as USDC or XLM. The Strategy contract is initialized with the asset it will manage.

In the case of Liquidity pools, the Strategy will manage the liquidity pool token. For instance, if the Strategy is managing a USDC/XLM pool, the Strategy will manage the liquidity pool token.

The main functions for a Strategy are:
- `initialize`: Initializes the Strategy with the required parameters.
- `asset`: Returns the asset of the Strategy.
- `deposit`: Allows the DeFindex to deposit assets into the Strategy.
- `harvest`: Allows the Strategy to generate yields. In case an action is required for the strategy to generate yields, the `harvest` function will execute it.
- `withdraw`: Allows the DeFindex to withdraw assets from the Strategy.
- `balance`: Returns the balance of the Strategy.