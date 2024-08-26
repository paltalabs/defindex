# Yearn Finance V3
Yearn Finance V3 represents the latest evolution of the Yearn Finance protocol, a decentralized platform focused on optimizing yield farming strategies. Leveraging advanced automation and smart contract technology, Yearn V3 introduces enhanced modularity, allowing for more flexible and efficient strategy deployments across various DeFi protocols. This iteration aims to provide users with higher returns, reduced risks, and greater customization in managing their assets. By continuously aggregating yields from different lending protocols, Yearn V3 ensures that users' assets are dynamically allocated to the most profitable opportunities, streamlining the complex process of yield farming.

## Yearn V3 Vault System Overview

### Factory Contract

The factory contract is designed to deploy new vaults using a specific `VAULT_ORIGINAL` as a blueprint. The deployment process ensures that each vault has unique parameters and cannot be duplicated.

### Key Function

```jsx
def deploy_new_vault(
    asset: address,
    name: String[64],
    symbol: String[32],
    role_manager: address,
    profit_max_unlock_time: uint256
) -> address
```

This function creates a new vault with the following parameters:

- **asset**: The underlying token the vault will use (e.g., USDC).
- **name**: The name of the vault token (e.g., Blend USDC Pool).
- **symbol**: The symbol of the vault token (e.g., dfBlUSDC).
- **role_manager**: The admin responsible for managing the vault's roles.
- **profit_max_unlock_time**: The time over which the profits will unlock.

### Events

- **NewVault**: Emitted when a new vault is deployed.
- **UpdateProtocolFeeBps**: Emitted when the protocol fee basis points are updated.
- **UpdateProtocolFeeRecipient**: Emitted when the protocol fee recipient is updated.

### Vault Contract

The vault contract manages user deposits, handles idle assets, and interacts with various strategies to generate yield. It issues vault tokens to users based on their share of the vault's total assets.

### Key Concepts

- **Idle Amount**: The portion of assets kept liquid within the vault for quick withdrawals.
- **Min Idle Amount**: Minimum liquid amount the vault must maintain.
- **Update Debt**: A function executed by administrators to allocate funds to different strategies, setting target debt levels.
- **Debt Manager**: A role responsible for managing the vault's debt and strategy allocations.
- **Default Queue**: A queue of strategies to take funds from when the vault needs to free up assets. It defines the priority order for liquidating strategy positions.

### Important Functions

- **Deposit**: Users deposit funds into the vault, receiving vault tokens in return.
- **Withdraw**: Users can withdraw their funds, which may involve liquidating strategy positions if the idle amount is insufficient.

### Issuing Shares

```jsx
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

This function calculates and issues new shares based on the amount of assets deposited.

### Strategy Contract

The strategy contract in Yearn V3 focuses on specific yield-generating tasks, delegating standardized ERC-4626 and vault logic to a central `TokenizedStrategy` contract.

### Key Components

- **BaseStrategy**: Inherited by strategies to handle communication with the `TokenizedStrategy`.
- **TokenizedStrategy**: Implements all ERC-4626 and vault-specific logic.
- **Modifiers**: Ensure only authorized addresses can call certain functions, enhancing security.

### Functions

- **_deployFunds**: Deploys assets into yield sources.
- **_freeFunds**: Frees assets when needed.
- **_harvestAndReport**: Harvests rewards, redeploys idle funds, and reports the strategy's total assets.

### Fee and Price Per Share (PPS) Management

### Fee Management

- **Default and Custom Protocol Fees**: The factory contract allows setting default and custom protocol fees for vaults and strategies.
- **Fee Recipient**: Protocol fees are sent to the designated fee recipient, with the remaining fees going to the vault or strategy-specific recipient.

### Price Per Share (PPS) Calculation

The PPS is calculated based on the total assets and total supply of shares within the vault.

```jsx
@view
@external
def pricePerShare() -> uint256:
    return self._convert_to_assets(10 ** convert(self.decimals, uint256), Rounding.ROUND_DOWN)

```

This function provides the PPS, ensuring precise asset-to-share conversion.

**Calculating Price Per Share (PPS)**:

- The PPS is a crucial metric for ensuring users receive the correct value for their dfTokens. It is calculated as follows:

$$
\text{PPS} = \frac{\text{Total Assets}}{\text{Total Supply of dfTokens}}
$$

Where:

- **Total Assets**: The sum of the value of assets managed by all adapters plus any idle assets held directly by the DeFindex contract.
- **Total Supply of dfTokens**: The total number of dfTokens issued to users.

To illustrate, consider the following scenario:

- DeFindex has three adapters managing different investments:
    - Adapter A manages $50,000 in a liquidity pool.
    - Adapter B manages $30,000 in a lending pool.
    - Adapter C manages $20,000 in a staking protocol.
- The DeFindex contract holds an additional $10,000 in idle assets.

The Total Assets would be:

$$
50,000 + 30,000 + 20,000 + 10,000 = 110,000 \text{ USDC} 
$$

If the Total Supply of dfTokens is 100,000, the PPS would be:

$$
\text{PPS} = \frac{110,000 \text{ USDC}}{100,000 \text{ dfTokens}} = 1.1 \text{ USDC per dfToken} 
$$

This calculation ensures users can accurately determine the value of their holdings in DeFindex, promoting transparency and trust.

## Problems of Yearn Finance V3

- Does not support multi-asset strategies: For example you can't invest on a vault composed by a strategy of USDC on Aave and another strategy of USDC-WETH on Uniswap.
- Price Per Shares (PPS), ConvertToShares and ConvertToAssets functions need to be described in a single asset.