# Contracts

There are 3 main contracts: Zapper, DeFindex and Strategy (formerly known as Adapter). 

## Zapper
This contract enables users to invest in and withdraw from a DeFindex using a single asset. For instance, if a DeFindex requires both USDC and XLM, the Zapper contract allows users to deposit USDC, automatically swapping the USDC for XLM and depositing both assets into the DeFindex according to a predefined ratio. Similarly, the Zapper contract facilitates withdrawals by swapping the XLM back to USDC before returning the funds to the user.

The specific paths used for asset swaps, as well as the proportion of the output assets, are determined off-chain.

## DeFindex
This contract serves as the core of the DeFindex platform, responsible for managing assets, executing strategies, and ensuring proper asset rebalancing. It operates with three primary roles: Deployer, Manager, and Emergency Manager. Additionally, the contract functions as a token, similar to a liquidity pool token, referred to as the dfToken.

While anyone can invest in a DeFindex, only the Manager and Emergency Manager have the authority to move funds between the DeFindex and its associated strategies.

The contract also holds funds not currently invested in any strategy, known as IDLE funds. These funds act as a safety buffer, allowing the Emergency Manager to withdraw assets from underperforming or unhealthy strategies and store them as IDLE funds.

### Initialization
The DeFindex contract is initialized with a predefined proportion of assets. Let's say 1 token A, 2 token B, and 3 token C. The contract will hold these assets in the right proportion. When a user deposits assets into the DeFindex, they receive dfTokens in return, representing their share of the DeFindex's assets. In the following documents we talk of dfTokens and shares as the same thing.

This is proportion is used for the first deposit made to the DeFindex. The Manager can later modify these proportions in response to changing conditions. Also, the performance of the Strategies may change the proportion of the assets.

Strategies are stored in instance storage, since we expect to have DeFindex with a small number of strategies. 

### Investing: Deposit
When a user deposits assets into the DeFindex, they receive dfTokens in return. These tokens represent the user's share of the DeFindex's assets. The user can later burn these tokens to withdraw their assets.

When calling the `deposit` function, the assets are transfered to the DeFindex, in the right proportion, and the user receives dfTokens. The user can later call the `withdraw` function to burn the dfTokens and receive the assets in the right proportion.

Then, the price per share is a multi-asset price. Let's take the example form before: you need to have 1 token A, 2 tokens B and 3 tokens C to mint 1 dfToken (share)

### Withdrawals
When a user wants to withdraw her funds, she will burn an amount of shares in order to get her funds. 
If there is enough IDLE funds to withdraw, the funds are got from the IDLE funds. However, if more assets are required, she will trigger a liquidation process that will release enough funds.
The amount of the withdrawal for every asset $a_i$ can be got by:
$$
a_i = \frac{m_s}{M_s} \cdot A_i \quad \forall i \in \text{Underlying Asset}
$$
where:
- $a_i$: Amount of asset $i$ to receive
- $m_s$: Amount of shares to burn
- $M_s$: Total supply of dfTokens (shares)
- $A_i$: Total amount of asset $i$ held by the **DeFindex**

Then, the withdrawal function will liquidate the positions in the strategies to get the remaining assets. As in the following equation:
$$
a_i = a_{i, \text{IDLE}} + a_{i, \text{Strategy}} \quad \forall a_i>a_{i, \text{IDLE}}
$$

Where:
- $a_{i, \text{IDLE}}$: Amount of asset $i$ to get from the IDLE funds
- $a_{i, \text{Strategy}}$: Amount of asset $i$ to get from the strategies

### Rebalancing
Rebalancing is managed by the Manager, who can shift funds from one strategy to another, thereby adjusting the proportions of the underlying assets. For example, the Deployer might initially set a ratio of 2 USDC to 1 XLM for a DeFindex, but this ratio can change based on strategy yields or rebalancing actions by the Manager.

When a DeFindex is deployed, the Deployer sets the strategies and the initial proportion of underlying assets. The Manager can later modify these proportions in response to changing conditions.

Strategies are required to report the amount of underlying assets they hold, ensuring that when shares of the DeFindex liquidity pool token are minted, they reflect the correct asset proportions.
#### Functions
- `set_strategy`: Allows the Manager to add/remove the strategies.
- `withdraw_from_strategies`: Allows the Manager to withdraw assets from a strategy, letting them as IDLE funds.
- `invest_in_strategies`: Allows the Manager to invest IDLE fund assets in a strategy.
- `internal_swap`: Allows the Manager to swap IDLE assets. As arguments, it receives an array of Soroswap's Aggregator Swap transactions.
- `rebalance`: Allows the Manager to rebalance the DeFindex. It executes `withdraw_from_strategies`, `internal_swap`, and `invest_in_strategies` functions.

Then, a rebalance execution will withdraw assets from the strategies, swap them, and invest them back in the strategies.

### Emergency Management
The Emergency Manager has the authority to withdraw assets from the DeFindex in case of an emergency. This role is designed to protect users' assets in the event of a critical situation, such as a hack or a strategy gets unhealthy. The Emergency Manager can withdraw assets from the DeFindex and store them as IDLE funds until the situation is resolved. It also turns off a strategy if it is unhealthy.

#### Functions
- `emergency_withdraw`: Allows the Emergency Manager to withdraw assets from the DeFindex. As arguments, it receives the amount to withdraw and the address of a Strategy. It also turns off the strategy.


### Fee Collection
The revenues generated by the strategies are collected as shares of the DeFindex. The initial setup recommends a fee of 1%-2% APR on these shares. For instance, if a DeFindex has 100 shares and the fee is set at 1% APR, the fees collected would be 1 share annually.

The distribution of the collected shares is as follows: 50% to **palta**labs, 30% to the Manager, and 20% to the Strategy developers, proportionally allocated based on the underlying assets. These allocations are recalculated whenever a user deposits or withdraws from the DeFindex or when rebalancing occurs.

### Multi-transaction Actions _(TODO)_

In the ideal escenario, once the user deposits the assets, the DeFindex will invest this assets in their strategies. However, if there is too many strategies, the amount of CPU instructions required to execute a deposit and allocation of the assets to the strategies could be too high.

In this case, the DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `invest_idle_funds` to allocate the assets to the strategies.

<!-- ### Functions
- `deposit`: Allows users to deposit assets into the DeFindex.
- `withdraw`: Allows users to withdraw assets from the DeFindex.
- `balance`: Returns the balance of the DeFindex.
- `initialize`: Initializes the DeFindex with the initial strategies and proportions.
- `emergencyWithdraw`: Allows the Emergency Manager to withdraw assets from the DeFindex.

### Variables
- `strategies`: An array of strategies.
- `strategyBalances`: A mapping of the strategy balances. -->

## Strategy
This contract is responsible for generating yields for the DeFindex. It provides key functions such as deposit, withdraw, and balance.

The strategy itself is a `struct` that can be implemented by other developers. This can include DeFi protocol developers or anyone with a compelling strategy designed to generate yields.