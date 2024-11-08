
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
## Fees

### Fee Receivers
The DeFindex protocol defines two distinct fee receivers to reward both the creators of the DeFindex Protocol and the deployers of individual Vaults:

1. **DeFindex Protocol Fee Receiver**: Receives a fixed protocol fee of 0.5% APR.
2. **Vault Fee Receiver**: Receives a fee set by the vault deployer, typically recommended between 0.5% and 2% APR.

The Total Management Fee consists of both the protocol fee and the vault fee. Thus, each Vault has a total APR fee rate $f_{\text{total}}$ such that:

$$
f_{\text{total}} = f_{\text{DeFindex}} + f_{\text{Vault}}
$$

where $f_{\text{DeFindex}} = 0.5\%$ is a fixed `defindex_fee` that goes to the DeFindex Protocol Fee Receiver address, and $f_{\text{Vault}}$ is a variable APR `vault_fee`, typically between 0.5% and 2%, that goes to the Vault Fee Receiver address.

### Fee Collection Methodology

The fee collection process mints new shares, or dfTokens, to cover the accrued management fees. These shares are calculated based on the elapsed time since the last fee assessment, ensuring fees are accrued based on the actual period of asset management. The fee collection is triggered whenever there is a vault interaction, such as a `deposit`, `withdrawal`, or even an explicit `fee_collection` call, with calculations based on the time elapsed since the last fee assessment.

### Mathematical Derivation of New Fees

Let:

- $V_0$  be the Total Value Locked (TVL) at the last assessment,
- $s_0$  be the Total Shares (dfTokens) at the last assessment,
- $f_{\text{total}}$  be the Total Management Fee (APR).

Over a time period $\Delta t$, the fees due for collection are derived as a value represented by newly minted shares.

To mint new shares for fee distribution, we calculate the required number of new shares, $s_f$, that correspond to the total management fee over the elapsed period.

After a period $\Delta t$ (expressed in seconds), and after the fee collection process the new total shares $s_1$ should be:

$$
s_1 = s_0 + s_f
$$

Since `fee_collection` is always called before any `deposit` or `withdrawal`, we assume that the Total Value $V_1$ remains equal to $V_0$.

We establish the following condition to ensure the number of minted shares accurately reflects the management fee accrued over $\Delta t$. The value of the new minted shares $s_f$ should equal the prorated APR fee share of the total value of the vault. In mathematical terms:

$$
\frac{V_0}{s_1} \times s_f = V_0 \times f_{\text{total}} \times \frac{\Delta t}{\text{SECONDS PER YEAR}}
$$

Rearranging terms, we get:

$$
s_f = \frac{f_{\text{total}} \times s_0 \times \Delta t}{\text{SECONDS PER YEAR} - f_{\text{total}} \times \Delta t}
$$

This equation gives the precise quantity of new shares $s_f$ to mint as dfTokens for the management fee over the period $\Delta t$.

### Distribution of Fees

Once the total fees, $s_f$, are calculated, the shares are split proportionally between the DeFindex Protocol Fee Receiver and the Vault Fee Receiver. This is done by calculating the ratio of each fee receiver’s APR to the total APR:

$$
s_{\text{DeFindex}} = \frac{s_f \times f_{\text{DeFindex}}}{f_{\text{total}}}
$$

$$
s_{\text{Vault}} = s_f - s_{\text{DeFindex}}
$$

This ensures that each fee receiver is allocated their respective share of dfTokens based on their fee contribution to $f_{\text{total}}$. The dfTokens are then minted to each receiver’s address as a direct representation of the fees collected.


### Example

Suppose a DeFindex vault begins with an initial value of 1 USDC per share and a total of 100 shares (dfTokens), representing an investment of 100 USDC. This investment is placed in a lending protocol with an 8% APY. The DeFindex protocol has a total management fee of 1% APR, split between a 0.5% protocol fee and a 0.5% vault fee.

After one year, the investment grows to 108 USDC due to the 8% APY.

#### Step 1: Calculate the Shares to Mint for Fees

Using the formula:

$
s_f = \frac{f_{\text{total}} \times s_0 \times \Delta t}{\text{SECONDS PER YEAR} - f_{\text{total}} \times \Delta t}
$

where:
- \( f_{\text{total}} = 0.01 \) (1% APR management fee),
- \( s_0 = 100 \) (initial shares),
- \( \Delta t = \text{SECONDS PER YEAR} \) (since this example spans a full year),

we calculate \( s_f \), the number of shares to mint for the fee collection.

Substituting values:

$
s_f = \frac{0.01 \times 100 \times \text{SECONDS PER YEAR}}{\text{SECONDS PER YEAR} - (0.01 \times \text{SECONDS PER YEAR})}
$

Simplifying:

$
s_f = \frac{1 \times \text{SECONDS PER YEAR}}{0.99 \times \text{SECONDS PER YEAR}} \approx 1.0101
$

Thus, approximately 1.01 dfTokens are minted as fees.

#### Step 2: Update Total Shares and Calculate Price per Share

With the fee tokens minted, the total dfTokens increase from 100 to 101.01.

The vault now holds 108 USDC backing 101.01 dfTokens, so the new price per share is:

$
\text{Price per Share} = \frac{108}{101.01} \approx 1.069 \, \text{USDC}
$

#### Step 3: Determine the Value for a User Holding 100 dfTokens

For a user holding 100 dfTokens, the value of their holdings after one year is approximately:

$
100 \, \text{dfTokens} \times 1.069 \, \text{USDC per share} = 106.9 \, \text{USDC}
$

The remaining 1.01 dfTokens represent the collected fee, backed by around:

$
1.01 \, \text{dfTokens} \times 1.069 \, \text{USDC per share} \approx 1.08 \, \text{USDC}
$

---

This breakdown clarifies how the investment grows and the management fee is deducted by minting new dfTokens, resulting in a proportional share value for both users and fee recipients.


It is expected that the Fee Receiver is associated with the manager, allowing the entity managing the Vault to be compensated through the Fee Receiver. In other words, the Fee Receiver could be the manager using the same address, or it could be a different entity such as a streaming contract, a DAO, or another party.

<!-- ### Multi-transaction Actions _(TODO)_

In the ideal escenario, once the user deposits the assets, the DeFindex will invest this assets in their strategies. However, if there is too many strategies, the amount of CPU instructions required to execute a deposit and allocation of the assets to the strategies could be too high.

In this case, the DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `invest_idle_funds` to allocate the assets to the strategies.

The same happens when a user withdraws the assets. The DeFindex will store the assets as IDLE funds, and the Manager will need to execute a function called `withdraw_idle_funds` to withdraw the assets from the strategies. Then, the user will be able to withdraw the assets. -->


## Storage Management
Strategies are stored in instance storage, as the DeFindex is expected to work with a limited number of strategies.


