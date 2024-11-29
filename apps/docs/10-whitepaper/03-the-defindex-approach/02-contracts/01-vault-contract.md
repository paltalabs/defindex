
# DeFindex Vault Contract
This contract serves as the core of the DeFindex platform, responsible for managing assets, executing strategies, and ensuring proper asset rebalancing. It operates with four primary roles: **Deployer**, **Fee Receiver**, **Manager**, and **Emergency Manager**. Additionally, the contract functions as a token referred to as the *dfToken* that represents the shares of the vault.

While anyone can invest in a DeFindex, only the Manager and Emergency Manager have the authority to move funds between strategies or even outside strategies and into the Vault itself (see idle assets and emergency withdrawal).

The contract also holds funds not currently invested in any strategy, known as **IDLE funds**. These funds act as a safety buffer, allowing the Emergency Manager to withdraw assets from underperforming or unhealthy strategies and store them as IDLE funds. (also to enable fast small withdrawals)

## Underlying Assets
Each DeFindex Vault will use a defined set of underlying assets to be invested in one or more strategies. 

Because Strategies are the only one that know exactly the current balance of the asset, the Vault relies on the strategies in order to know the exact total balance for each underlying asset.??

Or if the Vault executes Strategies at its own name (auth), it should execute a speficic `get_assets_balance` function in the strategy contract to know exactely how many assets it has at a specific moment.

## Initialization
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


## Investing: Deposit

When a user deposits assets into the DeFindex Vault, they receive dfTokens, representing their proportional share of the Vault’s total assets. These dfTokens can later be burned to redeem the user’s share of assets.

Upon calling the `deposit()` function, assets are transferred to the DeFindex Vault and allocated based on the current asset ratios. For example, if the Vault maintains a 1:2:3 ratio for assets A, B, and C per dfToken, this ratio will be applied to new deposits. The user receives dfTokens reflecting their share of the Vault’s total assets.

To withdraw assets, users call the `withdraw` function to burn their dfTokens, releasing assets according to the current asset ratio.

Thus, the value per dfToken reflects a multi-asset backing. Using the above example, to mint 1 dfToken, a user would need to deposit 1 unit of asset A, 2 units of asset B, and 3 units of asset C. Therefore, the value of 1 dfToken can be represented as:

$$
p(\text{dfToken}) = (1 \text{A}, 2 \text{B}, 3 \text{C})
$$

### Depositing When Total Assets = 1    

When the Vault only holds one asset, the deposit process is straightforward: the amount deposited by the user will be directly used to mint shares proportional to the total funds in the Vault.

1. **First Deposit**:  
   For the initial deposit, `shares_to_deposit` is set equal to the `amount` sent by the user, simplifying the initial setup.

2. **When There Are Existing Funds**:  
   If the Vault already holds funds, `shares_to_deposit` are calculated based on the current `total_managed_funds` and `total_supply` (i.e., the current number of shares), according to the following formula:

Let’s denote the total supply at time 0 as $s_0$ and the total managed funds as $v_0$. At time 1, a user wants to deposit an additional amount $v'$, and new shares $s'$ are minted. The value of any share $val(s)$ at time $t$ is calculated as:

$$
val(s)_t = \frac{v_t}{s_t} \cdot s
$$

At time $t_1$, this must hold:

$$
val(s') = \frac{v_1}{s_1} \cdot s'
$$

Given that $v_1 = v_0 + v'$ and $s_1 = s_0 + s'$, we can rearrange terms to find the new shares:

$$
s' = \frac{v'}{v_0} \cdot s_0
$$

## Withdrawals
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

## Rebalancing
Rebalancing is overseen by the **Manager**, who adjusts the allocation of funds between different strategies to maintain or change the ratio of underlying assets. For example, a DeFindex might start with a ratio of 2 USDC to 1 XLM, as initially set by the Deployer. However, this ratio can be modified by the Manager based on strategy performance or market conditions.

Upon deployment, the Deployer establishes the initial strategies and asset ratios for the DeFindex. The Manager has the authority to adjust these ratios as needed to respond to evolving conditions or to optimize performance.

To ensure accurate representation of asset proportions, strategies are required to **report** the amount of each underlying asset they hold. This reporting ensures that when dfTokens are minted or redeemed, the DeFindex maintains the correct asset ratios in line with the current balance and strategy allocations.

### Functions
- `assets()`: Returns the assets addresses and amount of each  of them in the DeFindex (and hence its current ratio).
`[[adress0, amount0], [address1, amount1]]`. TODO: Separate in 2 functions.
- `withdraw_from_strategies`: Allows the Manager to withdraw assets from one or more strategies, letting them as IDLE funds.
- `invest_in_strategies`: Allows the Manager to invest IDLE fund assets in one or more strategies.
- `internal_swap`: Allows the Manager to swap one IDLE asset into another IDLE asset supported by the Vault. As arguments, it receives an array of Soroswap's Aggregator Swap arguments.
- `rebalance`: Allows the Manager to rebalance the DeFindex. It executes `withdraw_from_strategies`, `internal_swap`, and `invest_in_strategies` functions.

Then, a rebalance execution will withdraw assets from the strategies, swap them, and invest them back in the strategies.
- `emergency_withdraw`: Allows the Emergency Manager to withdraw all assets from a specific Strategy. As arguments, it receives the the address of a Strategy. It also turns off the strategy.

## Emergency Management
The Emergency Manager has the authority to withdraw assets from the DeFindex in case of an emergency. This role is designed to protect users' assets in the event of a critical situation, such as a hack of a underlying protocol or a if a strategy gets unhealthy. The Emergency Manager can withdraw assets from the Strategy and store them as IDLE funds inside the Vault until the situation is resolved. 

## Management
Every DeFindex has a manager, who is responsible for managing the DeFindex. The Manager can ebalance the Vault, and invest IDLE funds in strategies. 

## Fee Collection

### Fee Receivers
The DeFindex protocol defines two distinct fee receivers to reward both the creators of the DeFindex Protocol and the deployers of individual Vaults:

1. **DeFindex Protocol Fee Receiver**
2. **Vault Fee Receiver**

The fees collected are from the gains of the strategies. Thus, it is a performance-based fee.

### Fee Collection Methodology

The DeFindex fee collection process is designed to track fees in the vault until distribution, with fees originating from the strategy gains. This ensures an organized and accountable fee handling system.

#### General Overview
Fees are charged on a per-strategy basis, meaning each strategy independently calculates its gains and the corresponding fees. These fees are then collected and distributed to the protocol and manager. The fee percentages can vary, and the vault adjusts total assets accordingly by deducting fees from the strategy gains.

#### Detailed Workflow

1. **Fee Structure Example**:
   - Protocol Fee Receiver: 5%
   - Vault Fee Receiver: 15%

2. **Execution Example**:
   - A user deposits 100 USDC into a vault with one strategy.
   - The strategy earns 10 USDC in gains.
   - The vault collects 20% of the gains as fees (2 USDC).
   - Fees are distributed between the protocol (0.5 USDC) and the manager (1.5 USDC).
   - The total assets of the vault become \(100 + 10 - 2 = 108\) USDC.

#### Strategy Gains Tracking
Since fees depend on strategy performance, gains and losses must be tracked meticulously. To achieve this, a `report()` function is implemented to log the gains or losses since the last update.  

**Pseudocode for Tracking Gains and Losses**:
```rust
fn report(strategy: Address) -> (u256, u256) {
    let current_balance = get_current_balance(strategy);
    let prev_balance = get_prev_balance(strategy);
    let gains_or_losses = current_balance - prev_balance;

    store_gains_or_losses(strategy, gains_or_losses);
    store_prev_balance(strategy, current_balance);
}

fn report_all_strategies() {
    for strategy in strategies {
        report(strategy);
    }
}
```
- **Usage**: The `report_all_strategies()` function is invoked during key operations such as rebalancing, deposits, or withdrawals to ensure accurate gain tracking.

#### Fee Distribution
Once gains are tracked, fees are calculated and distributed accordingly. After distribution, the gains and losses for each strategy are reset to 0.

**Pseudocode for Fee Distribution**:
```rust
fn distribute_fees() {
    for strategy in strategies {
        let gains_or_losses = get_gains_or_losses(strategy);
        if gains_or_losses > 0 {
            let protocol_fee = gains_or_losses * protocol_fee_receiver / MAX_BPS;
            let vault_fee = gains_or_losses * vault_fee_receiver / MAX_BPS;
            transfer_from_strategy(strategy.asset, protocol_fee_receiver, protocol_fee);
            transfer_from_strategy(strategy.asset, vault_fee_receiver, vault_fee);
            reset_gains_or_losses(strategy);
        }
    }
}
```
This function is public and can be called by anyone.

#### Displaying User Balances
To provide users with an accurate view of their balances, the vault deducts any outstanding fees from the total assets when reporting current balances.

By following this structured methodology, DeFindex ensures transparent and fair fee collection, tracking, and distribution processes.


It is expected that the Fee Receiver is associated with the manager, allowing the entity managing the Vault to be compensated through the Fee Receiver. In other words, the Fee Receiver could be the manager using the same address, or it could be a different entity such as a streaming contract, a DAO, or another party.

<!-- ### Multi-transaction Actions _(TODO)_

In the ideal escenario, once the user deposits the assets, the DeFindex will invest this assets in their strategies. However, if there is too many strategies, the amount of CPU instructions required to execute a deposit and allocation of the assets to the strategies could be too high.

In this case, the DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `invest_idle_funds` to allocate the assets to the strategies.

The same happens when a user withdraws the assets. The DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `withdraw_idle_funds` to withdraw the assets from the strategies. Then, the user will be able to withdraw the assets. -->


## Storage Management
Strategies are stored in instance storage, as the DeFindex is expected to work with a limited number of strategies.


