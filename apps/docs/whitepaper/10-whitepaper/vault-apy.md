# Vault APY

The **Vault APY** shows how much the value of a vault shares grows over time — similar to earning interest on  savings.

This value depends on:

* The **assets** supported in the vault,
* The **strategies** the vault uses,
* The **rebalancing actions** taken by the Vault Manager.

Even though there are many moving parts, **DeFindex Vaults make it easy** to track this performance through the **Vault Price Per Share (VPPS)**.

***

💡 What is Vault Price Per Share?

Just like strategies have a **price per share**, the **vault itself** has a **price per share** that shows how much 1 share of the vault is worth.

This includes:

* All the strategies the vault uses,
* How much is allocated to each strategy,
* And how well each strategy is performing.

So, when the vault earns yield or its strategies grow, the **vault price per share increases**.

***

### 🧮 How to Get the Vault Price Per Share

There are two main ways to calculate it. Both give the same result.

#### ✅ Method 1: Use the Contract Function `get_asset_amounts_per_shares`

```rust
fn get_asset_amounts_per_shares(
        e: Env,
        vault_shares: i128,
    ) -> Result<Vec<i128>, ContractError>;
```

You can get the real-time vault PPS by calling&#x20;

```rust
get_asset_amounts_per_shares(1_000_000_000_000) // 1 Vault Share is SCALAR_12
```

This function returns a `Vec`of asset amounts per share. Each amount matches the asset at the same index in the vault’s asset list.

To calculate the vault share price in a specific pricing currency (e.g. USD):

$$
\text{Vault PPS} = \sum_{i} \left( \text{Asset Price}_i \times \text{VAmount}_i \right)
$$

Where:

* `VAmount_i` = amount of asset `i` per share fasdf
* `Asset Price_i` = price of asset `i` (from an oracle or external source)

**If the vault has only one asset** and you're pricing in that same asset, just use the first value from `get_asset_amounts_per_shares`.

***

#### ✅ Method 2: Use Vault Events

Each time someone deposits or withdraws from the vault, deposit and withdraw events are emitted:

```rust

pub struct VaultWithdrawEvent {
    pub withdrawer: Address,
    pub df_tokens_burned: i128,
    pub amounts_withdrawn: Vec<i128>,
    pub total_supply_before: i128,
    pub total_managed_funds_before: Vec<CurrentAssetInvestmentAllocation>,
}
pub struct VaultDepositEvent {
    pub depositor: Address,
    pub amounts: Vec<i128>,
    pub df_tokens_minted: i128,
    pub total_supply_before: i128,
    pub total_managed_funds_before: Vec<CurrentAssetInvestmentAllocation>,
} 
```

Each event includes:

* `total_supply_before` — the number of vault shares before the action
* `total_managed_funds_before` — a list of all asset allocations

Each asset allocation looks like this:

```rust
pub struct CurrentAssetInvestmentAllocation {
    pub asset: Address,
    pub total_amount: i128,
    pub idle_amount: i128,
    pub invested_amount: i128,
    pub strategy_allocations: Vec<StrategyAllocation>,
}
```

To calculate the vault price per share:

$$
\text{Vault PPS} = \frac{\sum \left( \text{Asset Price}_i \times \text{Total Asset Amount}_i \right)}{\text{Total Vault Shares}}
$$

If the vault only holds **one asset**, then:

$$
\text{Vault PPS} = \frac{\text{Total Asset Amount}}{\text{Total Supply}}
$$

Where:

* **`Total Asset Amount`** = sum of all units of that one asset held by the vault (from `total_managed_funds_before[0].total_amount`)
* **`Total Supply`** = number of shares before the action (from `total_supply_before`)

### 📈 How to Calculate Vault APY

Once you have the Vault PPS at two different points in time, you can calculate **APY** using the same method as when is calculated for strategies:

$$
\text{pps\_delta} = \frac{\text{PPS}_{\text{now}}}{\text{PPS}_{\text{then}}} - 1
$$



Then annualize it:

$$
\text{Vault APY} = \left(1 + \text{pps\_delta} \right)^{\left( \frac{365.2425}{\text{days}} \right)} - 1
$$



Where `days` is the number of days between the two PPS values.
