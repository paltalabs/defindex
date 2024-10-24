
# DeFindex Vault Contract
This contract serves as the core of the DeFindex platform, responsible for managing assets, executing strategies, and ensuring proper asset rebalancing. It operates with four primary roles: **Deployer**, **Fee Receiver**, **Manager**, and **Emergency Manager**. Additionally, the contract functions as a token referred to as the *dfToken* that represents the shares of the vault.

While anyone can invest in a DeFindex, only the Manager and Emergency Manager have the authority to move funds between strategies or even outside strategies and into the Vault itself (see idle assets and emergency withdrawal).

The contract also holds funds not currently invested in any strategy, known as **IDLE funds**. These funds act as a safety buffer, allowing the Emergency Manager to withdraw assets from underperforming or unhealthy strategies and store them as IDLE funds. (also to enable fast small withdrawals)

### Underlying Assets
Each DeFindex Vault will use a defined set of underlying assets to be invested in one or more strategies. 

Because Strategies are the only one that know exactly the current balance of the asset, the Vault relies on the strategies in order to know the exact total balance for each underlying asset.??

Or if the Vault executes Strategies at its own name (auth), it should execute a speficic `get_assets_balance` function in the strategy contract to know exactely how many assets it has at a specific moment.

### Initialization
The DeFindex Vault contract is structured with specific roles and strategies for managing assets effectively. The key roles include the **Fee Receiver**, **Manager**, and **Emergency Manager**, each responsible for different tasks in managing the Vault. Additionally, a predefined set of strategies determines how assets will be allocated within the Vault. A management fee is also established at the time of initialization, which can later be adjusted by the Fee Receiver. Further details on fee handling are explained later in the document.

The allocation ratios for these strategies are not set during the deployment but are defined during the first deposit made into the Vault. For example, imagine a scenario where the Vault is set up to allocate 20% of its assets to a USDC lending pool (like Blend), 30% to another USDC lending pool (such as YieldBlox), and 50% to a USDC-XLM liquidity pool on an Automated Market Maker (AMM) platform (like Soroswap). 

To establish this allocation, the deployer must make a first deposit into the Vault, even if the amount is small. This initial deposit sets the ratio for all future deposits. The deployer is required to hold USDC and the liquidity pool tokens, such as LP-USDC-XLM, to start this process. However, a **zapper contract** simplifies this by automating asset conversion and liquidity pooling. The zapper takes the deployer’s USDC, swaps 25% of it into XLM, and then uses both USDC and XLM to add liquidity to the Soroswap pool. This process generates LP tokens, which is required to complete the first deposit, ensuring the allocation ratios are correctly set. It's worth noting that the first deposit is made within the same transaction that creates and initializes the vault, so the deployer must have at least a minimal amount of assets ready when creating a vault.

Once the contract is initialized and the first deposit is made, the **Manager** has the authority to adjust the allocation ratios over time. For example, if market conditions change or certain strategies perform better, the Manager can rebalance the allocations between the strategies to optimize performance. However, the Manager is limited to reallocating funds only between the existing strategies. They cannot introduce new strategies, which ensures the safety of user funds by minimizing potential security risks.

This restriction on adding new strategies is a deliberate security feature. Allowing new strategies could increase the attack surface, potentially exposing the Vault to more vulnerabilities. By keeping the strategies fixed, the contract provides a stable and secure environment for users’ assets while still allowing flexibility in reallocating funds between existing strategies.

In summary:
1. **Roles and strategies are predefined** in the contract.
2. **Allocation ratios** for these strategies are set during the **first deposit**.
3. A **zapper contract** helps convert assets and establish the correct ratios.
4. The **Manager** can adjust allocations but cannot add new strategies, ensuring security and stability.



### Investing: Deposit
When a user deposits assets into the DeFindex, they receive dfTokens that represent their proportional share of the DeFindex's assets. These dfTokens can later be burned to withdraw the corresponding assets.

Upon calling the `deposit()` function, **the assets are transferred to the DeFindex in accordance with the current asset ratio**. For example, if the current ratio is 1 token A, 2 tokens B, and 3 tokens C for each dfToken, this ratio is maintained when assets are deposited. In return, the user receives dfTokens that represent their participation in the DeFindex Vault. 

When the user wishes to withdraw their assets, they call the `withdraw` function to burn their dfTokens. The **withdrawn assets will be dispensed according to the asset ratio at the time of withdrawal**.

Thus, the price per dfToken reflects a multi-asset price. For instance, using the earlier example, because in order to mint 1 dfToken, the user needs to deposit 1 token A, 2 tokens B, and 3 tokens C, the price per 1 dfToken will be `p(dfToken)=(1A, 2B, 3C)`.



### Withdrawals
When a user wishes to withdraw funds, they must burn a corresponding amount of dfTokens (shares) to receive their **assets at the ratio of the time of withdrawal**.

If there are sufficient **IDLE funds** available, the withdrawal is fulfilled directly from these IDLE funds. If additional assets are needed beyond what is available in the IDLE funds, a liquidation process is triggered to release the required assets.

**To Discuss: (TODO) Do we need minimum idle funds?**

To calculate the amount of each asset $a_i$ to be withdrawn, use the following formula:

$$
a_i = \frac{m_s}{M_s} \cdot A_i \quad \forall i \in \text{Underlying Asset}
$$
where:
- $a_i$: Amount of asset $i$ to receive
- $m_s$: Amount of shares to burn
- $M_s$: Total supply of dfTokens (shares)
- $A_i$: Total amount of asset $i$ held by the **DeFindex**

As discussed in the [Underlying Assets](#underlying-assets) section, $A_i$ is the sum of balances held by every strategy that works with asset $i$, plus total amount of iddle assets $i$.


$$
A_i = a_{i, \text{IDLE}} + \sum^{j \in S_i} a_{i,s^i_j}  
$$

Here $a_{i,s^i_j} $ represents the amount of assets $i$ held by any strategy $s^i_j$, and  $S_i$ is the set os stretegies that works with asset $i$ that are supported by the Vault.

#### Liquidation on Withdrawal
For every time that the amount to assets to withdraw $a_i$ is greater than IDLE assets, the `withdraw()` function will liquidate the positions in the strategies to get the remaining assets, allways mantaining the following relationship:
$$
a_i = a_{i, \text{IDLE}} + a_{i, \text{Strategy}} \quad \forall a_i>a_{i, \text{IDLE}}
$$

Where:
- $a_{i}$: Amount of asset $i$ withdraw.
- $a_{i, \text{IDLE}}$: Amount of asset $i$ to get from the IDLE funds
- $a_{i, \text{Strategy}}$: Amount of asset $i$ to get from the strategies

### Rebalancing
Rebalancing is overseen by the **Manager**, who adjusts the allocation of funds between different strategies to maintain or change the ratio of underlying assets. For example, a DeFindex might start with a ratio of 2 USDC to 1 XLM, as initially set by the Deployer. However, this ratio can be modified by the Manager based on strategy performance or market conditions.

Upon deployment, the Deployer establishes the initial strategies and asset ratios for the DeFindex. The Manager has the authority to adjust these ratios as needed to respond to evolving conditions or to optimize performance.

To ensure accurate representation of asset proportions, strategies are required to **report** the amount of each underlying asset they hold. This reporting ensures that when dfTokens are minted or redeemed, the DeFindex maintains the correct asset ratios in line with the current balance and strategy allocations.

#### Functions
- `assets()`: Returns the assets addresses and amount of each  of them in the DeFindex (and hence its current ratio).
`[[adress0, amount0], [address1, amount1]]`. TODO: Separate in 2 functions.
- `withdraw_from_strategies`: Allows the Manager to withdraw assets from one or more strategies, letting them as IDLE funds.
- `invest_in_strategies`: Allows the Manager to invest IDLE fund assets in one or more strategies.
- `internal_swap`: Allows the Manager to swap one IDLE asset into another IDLE asset supported by the Vault. As arguments, it receives an array of Soroswap's Aggregator Swap arguments.
- `rebalance`: Allows the Manager to rebalance the DeFindex. It executes `withdraw_from_strategies`, `internal_swap`, and `invest_in_strategies` functions.

Then, a rebalance execution will withdraw assets from the strategies, swap them, and invest them back in the strategies.
- `emergency_withdraw`: Allows the Emergency Manager to withdraw all assets from a specific Strategy. As arguments, it receives the the address of a Strategy. It also turns off the strategy.

### Emergency Management
The Emergency Manager has the authority to withdraw assets from the DeFindex in case of an emergency. This role is designed to protect users' assets in the event of a critical situation, such as a hack of a underlying protocol or a if a strategy gets unhealthy. The Emergency Manager can withdraw assets from the Strategy and store them as IDLE funds inside the Vault until the situation is resolved. 

### Management
Every DeFindex has a manager, who is responsible for managing the DeFindex. The Manager can ebalance the Vault, and invest IDLE funds in strategies. 

### Fee Collection
The revenues generated by the strategies are collected as shares of the DeFindex. These shares, or dfTokens, are minted whenever a deposit or withdrawal takes place.

The deployer can set a management fee, which can later be adjusted by the Fee Receiver. Additionally, **palta**labs charges a 0.5% APR on the vault shares.

The recommended initial setup suggests a fee of **0.5%-2% APR on these shares**. For example, if a DeFindex has 100 shares and the fee is set at 0.5% APR, the fees collected annually would be 1 share, split as 0.5 for the Fee Receiver and 0.5 for **palta**labs. The **palta**labs fee is defined in the Factory contract.

These allocations are recalculated and minted whenever a user deposits, withdraws from the DeFindex, or when rebalancing occurs.

For instance, let's say a DeFindex starts with an initial value of 1 USDC per share and 100 shares (dfTokens). These 100 USDC are invested in a lending protocol offering an 8% APY. The DeFindex also has a 0.5% APR fee. After one year, the investment grows to 108 USDC. Additionally, 1 dfToken is minted as a fee. This means the DeFindex now holds 101 dfTokens backed by 108 USDC, making the price per share approximately 1.07 USDC. As a result, a user holding 100 dfTokens will have a value equivalent to around 107 USDC, while the collected fee will be backed by about 1.07 USDC.

It is expected that the Fee Receiver is associated with the manager, allowing the entity managing the Vault to be compensated through the Fee Receiver. In other words, the Fee Receiver could be the manager using the same address, or it could be a different entity such as a streaming contract, a DAO, or another party.

<!-- ### Multi-transaction Actions _(TODO)_

In the ideal escenario, once the user deposits the assets, the DeFindex will invest this assets in their strategies. However, if there is too many strategies, the amount of CPU instructions required to execute a deposit and allocation of the assets to the strategies could be too high.

In this case, the DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `invest_idle_funds` to allocate the assets to the strategies.

The same happens when a user withdraws the assets. The DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `withdraw_idle_funds` to withdraw the assets from the strategies. Then, the user will be able to withdraw the assets. -->


## Storage Management
Strategies are stored in instance storage, as the DeFindex is expected to work with a limited number of strategies.


