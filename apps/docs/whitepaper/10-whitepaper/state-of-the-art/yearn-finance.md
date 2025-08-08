---
cover: ../../../.gitbook/assets/image 31.png
coverY: 0
---

# Yearn Finance

### Yearn Finance V3

[Yearn Finance V3 ](https://yearn.fi/v3)represents the latest evolution of the Yearn Finance protocol, a decentralized platform focused on optimizing yield farming strategies. Leveraging advanced automation and smart contract technology, Yearn V3 introduces enhanced modularity, allowing for more flexible and efficient strategy deployments across various DeFi protocols. This iteration aims to provide users with higher returns, reduced risks, and greater customization in managing their assets. By continuously aggregating yields from different lending protocols, Yearn V3 ensures that users' assets are dynamically allocated to the most profitable opportunities, streamlining the complex process of yield farming.

### The Yearn Finance Concepts

* Yearn Vaults (yVaults): A Vault is a Smart Contract that manages users funds and allocates them in different strategies. Funds can be allocated in a single strategy or in a collection of multiple strategies. From the [documentation page](https://docs.yearn.fi/getting-started/products/yvaults/overview), yVaults are like savings accounts for your crypto assets. They accept your deposit, then route it through strategies which seek out the highest yield available in DeFi. With YearnV3, yVaults are ERC-4626 compatible. See more [here](https://docs.yearn.fi/getting-started/products/yvaults/v3).
* A vault or "Allocator Vault" in V3 refers to an [ERC-4626 compliant](https://github.com/yearn/yearn-vaults-v3/blob/master/contracts/VaultV3.vy#L40) contract that takes in user deposits, mints shares corresponding to the user's share of the underlying assets held in that vault, and then allocates the underlying asset to an array of different "strategies" that earn yield on that asset.
* **Stategy:** A strategy or Opportunioty in V3 refers to a yield-generating contract added to a vault that has the needed ERC-4626 interface. The strategy takes the underlying asset and deploys it to a single source, generating yield on that asset.
* **Shares or Vault Tokens:** Tokens that will represents an user participation in a specific Yearn Vault. Depositors receive shares proportional to their deposit amount

## Yearn V3 Main Smart Contracts

### Factory Contract

The factory contract is designed to deploy new vaults using a specific `VAULT_ORIGINAL` as a blueprint. The deployment process ensures that each vault has unique parameters and cannot be duplicated.

### Key Function

```css
def deploy_new_vault(
    asset: address,
    name: String[64],
    symbol: String[32],
    role_manager: address,
    profit_max_unlock_time: uint256
) -> address
```

This function creates a new vault with the following parameters:

* **asset:** The underlying token the vault will use (e.g., USDC).
* **name:** The name of the vault token (e.g., DeFindex Blend USDC Pool) that will be issued to investors.
* **symbol:** The symbol of the vault token (e.g., dfBlUSDC) that will be issued to investors.\
  role\_manager: The admin responsible for managing the vault's roles.
* **profit\_max\_unlock\_time:** The time over which the profits will unlock (in seconds).

### Events

* **NewVault:** Emitted when a new vault is deployed. Provides the vault address
* **UpdateProtocolFeeBps:** Emitted when the protocol fee basis points are updated.
* **UpdateProtocolFeeRecipient:** Emitted when the protocol fee recipient is updated.

NOTE: The vault factory utilizes create2 opcode to deploy vaults to deterministic addresses. This means the same address can not deploy two vaults with the same default parameters for 'asset', 'name' and 'symbol'.

### Vault Contract

The vault contract manages user deposits, handles idle assets, and interacts with various strategies to generate yield. It issues vault tokens to users based on their share of the vault's total assets.

### Key Concepts

* **vault (allocator):** ERC-4626 compliant contract that accepts deposits, issues shares, and allocates funds to different strategies to earn yield.&#x20;
* **vault shares:** A tokenized representation of a depositor's share of the underlying balance of a vault. strategy: Any ERC-4626 compliant contract that can be added to an allocator vault that earns yield on an underlying asset.
* **debt:** The amount of the underlying asset that an allocator vault has sent to a strategy to earn yield.
* **report:** The function where a vault accounts for any profits or losses a strategy has accrued, charges applicable fees, and locks profit to be distributed to depositors.
* **Idle Amount:** The portion of underlying assets kept liquid within the vault for quick withdrawals.
* **Min Idle Amount:** Minimum liquid amount of underlying assets the vault must maintain.
* **Update Debt:** A function executed by administrators to allocate funds to different strategies, setting target debt levels.
* **Debt Manager:** A role responsible for managing the vault's debt (strategy allocations).
* **Default Queue:** A queue of strategies to take funds from when the vault needs to free up assets. It defines the priority order for liquidating strategy positions.

### Important Functions

* **Deposit:** Users deposit funds into the vault, receiving vault shares in return.
* **Withdraw:** Users burn their vault shares and withdraw their funds, which may involve liquidating strategy positions if the idle amount is insufficient.

### Issuing Shares

[From the Github Repo:](https://github.com/yearn/yearn-vaults-v3/blob/9fbc614bbce9d7cbad42e284a15f0f43cf1a673f/contracts/VaultV3.vy#L503)

```solidity
def _total_assets() -> uint256:
    return self.total_idle + self.total_debt

def _deposit(sender: address, recipient: address, assets: uint256) -> uint256:
    ...
    self.total_idle += assets
    shares: uint256 = self._issue_shares_for_amount(assets, recipient)
    ...

def _issue_shares_for_amount(amount: uint256, recipient: address) -> uint256:
    total_supply: uint256 = self._total_supply()
    total_assets: uint256 = self._total_assets()
    new_shares: uint256 = 0

    if total_supply == 0:
        new_shares = amount
    elif total_assets > amount:
        new_shares = amount * total_supply / (total_assets - amount)

    if new_shares == 0:
        return 0

    self._issue_shares(new_shares, recipient)
    return new_shares
```

This function calculates and issues new shares based on the amount of assets deposited. What these functions do is to maintain the relation between shares and assets invested. In fact, if S<sub>t</sub> is the Total Share Supply at time t, A<sub>t</sub> is the total amount of Assets at time t, s is the new amount of shares to be minted and a is the amount of assets being invested, what this code is doing is to maintain the following relationship

$$
\frac{S_t}{A_t} = \frac{s}{a}
$$

Because, when `_issue_shares_for_amount` is being called, `total_assets` is already A<sub>0</sub> + a = A<sub>1</sub>, but `total_supply` is still S<sub>0</sub> then the relationship will be

$$
\frac{S_1}{A_1} = \frac{S_0 + s}{A_1} = \frac{s}{a}
$$

$$
(S_0 + s) \cdot a = S_0 \cdot a + s \cdot a = s \cdot A_1
$$

$$
s = \frac{a \cdot S_0}{A_1 - a}
$$

**Links:**

[Tech Specs for YearnV3 Vaults](https://github.com/yearn/yearn-vaults-v3/blob/master/TECH_SPEC.md)

[Vault Management](https://docs.yearn.fi/developers/v3/vault_management)

**Strategy Contract**

The strategy contract in Yearn V3 focuses on specific yield-generating tasks, delegating standardized ERC-4626 and vault logic to a central `TokenizedStrategy` contract.

**Key Components**

* **BaseStrategy:** Inherited by strategies to handle communication with the `TokenizedStrategy.`
* **TokenizedStrategy:** Implements all ERC-4626 and vault-specific logic.
* **Modifiers:** Ensure only authorized addresses can call certain functions, enhancing security.

**Functions**

* \_deployFunds: Deploys assets into yield sources.
* \_freeFunds: Frees assets when needed.
* \_harvestAndReport: Harvests rewards, redeploys idle funds, and reports the strategy's total assets.

**Fee and Price Per Share (PPS) Management**

**Fee Management**

Fees are a percentage charged each time a V3 vault or strategy "reports". In Yearn V3 there are also Protocol Fees, which are a percentage of the total performance fees, that will go to Yearn for providing the infrastucture. Yearn Governande is responsible to set this percentage. It can be set for all Vaults or for individual vaults and strategies. Allowing full customization of the system.

* **Default and Custom Protocol Fees:** The factory contract allows setting default and custom protocol fees for vaults and strategies.
* **Fee Recipient:** Protocol fees are sent to the designated fee recipient, with the remaining fees going to the vault or strategy-specific recipient (vaults managers).

**Price Per Share (PPS) Calculation**

The PPS is calculated based on the total assets and total supply of shares within the vault.

```python
@view
@internal
def _convert_to_assets(shares: uint256, rounding: Rounding) -> uint256:
    """ 
    assets = shares * (total_assets / total_supply) --- (== price_per_share * shares)
    """
    if shares == max_value(uint256) or shares == 0:
        return shares

    total_supply: uint256 = self._total_supply()
    # if total_supply is 0, price_per_share is 1
    if total_supply == 0: 
        return shares

    numerator: uint256 = shares * self._total_assets()
    amount: uint256 = numerator / total_supply
    if rounding == Rounding.ROUND_UP and numerator % total_supply != 0:
        amount += 1

    return amount

@view
@external
def pricePerShare() -> uint256:
    return self._convert_to_assets(10 ** convert(self.decimals, uint256), Rounding.ROUND_DOWN)
e
```

This function provides the PPS, ensuring precise share-to-asset confeversion.

In pricePerShare, we are converting 10\*\*decimals units of shares into asset, meaning, an exact unit of share. Meaning `pricePerShare() === convert_to_assets(1)`

Calculating Price Per Share (PPS):

The PPS is a crucial metric for ensuring users receive the correct value for their dfTokens. It is calculated as follows:

$$
\text{PPS} = \frac{\text{Total Assets}}{\text{Total Supply of dfTokens}}
$$

### Limitations of Yearn Finance V3

* Does not support multi-asset strategies: For example you can't invest on a vault composed by a strategy of USDC on Aave and another strategy of USDC-WETH on Uniswap.&#x20;
* Price Per Shares (PPS), ConvertToShares and ConvertToAssets functions need to be described in a single asset.
* Fees and Revenues: Every DeFindex Vault generates revenue through protocol fees, which include:

&#x20;      \- Streaming Fees: Charged on assets under management, collected over time.

&#x20;     \-  Performance Fees: Based on the returns generated by the investment strategies.

&#x20;      \-  Protocol Fees: For transactions such as trading and borrowing.

These fees are designed to incentivize protocol development and cover operational costs.

TODO: CHeck who decides the amounts of these fees

### Fees Collection

All info is in the this website; https://docs.yearn.fi/developers/v3/protocol\_fees

Yearn collects fees through a performance-based system defined by governance, which controls the percentage of protocol fees and allows customization for each vault and strategy. This ensures flexibility and precise tuning of the fee structure. Yearn Governance dictates the amount of the Protocol fee and can be set anywhere between 0 - 50%. Yearn governance also holds the ability to set custom protocol fees for individual vaults and strategies. Allowing full customization of the system.

Example

```
profit = 100
performance_fee = 20%
protocol_fee = 10%

total_fees = profit * performance_fee = 20
protocol_fees = total_fees * protocol_fee = 2
performance_fees = total_fees - protocol_fees = 18

18 would get paid to the vault managers performance_fee_recipient.
2 would get paid to the Yearn Treasury.

```

### When Fees Are Collected

Fees are collected when a strategy reports gains or losses via the report() function. During the report, the strategy will calculate the gains since the last report and then calculate the fees based on the gains. This fees are then distributed as shares of the vault. Then, fees are collected per strategy.

Accountant reports the fees or refunds to the vault, from the gains or losses of the strategy. Then, the vault will calculate the fees and the protocol fees and then distribute the fees to the vault manager and the protocol fee recipient. This accountant is an interface and apparently it depends on the vault.

Yearn burns shares when there is fees or losses. When there is a loss and there is still fees not paid, the vault will burn shares to pay the fees.

The Vaults utilizes several mechanisms to mitigate price per share (pps) fluctuations and manipulation:

1. Internal accounting is used instead of balanceOf() to keep track of the vault's debt and idle.
2. A profit locking machenism designed by V3 Vaults locks profits or accountant's refunds by issuing new shares to the vault itself that are slowly burnt over the unlock perior.
3. In the event of losses or fees, the vault will always try to offset them by butning locked shares it owns. the price per share is expected to decrease only when excess losses or fees occur upon processing a report, or a loss occurs upon force revoking a strategy. reference
