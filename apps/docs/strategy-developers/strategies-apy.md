# Strategies APY

Each **strategy** will give a different **APY (Annual Percentage Yield)** depending on what the strategy does — for example lending, swapping, farming or even leverage lending, etc...

To calculate APY, you might need some extra data, like:

* The value of the harvested token (the token the strategy earns),
* The APY of other protocols the strategy interacts with,
* The emission rate of the harvested token (how fast it’s being distributed),
* And any other rewards or fees involved.



But instead of tracking all of that manually, the [**Strategy Crate**](../../contracts/strategies/core/src/event.rs#L32) makes things easier. It emits a `HarvestEvent` every time the strategy runs its logic (in the `harvest()` function). This event includes a very important value: the **Price Per Share** (`price_per_share` or **PPS**).

```rust
pub struct HarvestEvent {
    pub amount: i128,
    pub from: Address,
    pub price_per_share: i128,
}
```

#### 🪙 What is Price Per Share (PPS)?

Every time someone deposits into a strategy, they receive **shares**. As the strategy earns yield, the value of each share increases.

The **Price Per Share (PPS)** tells you how much one share is worth. You don’t need to track individual profits — just track the PPS over time.

#### 📅 How to Calculate APY

To calculate the APY, we compare the **PPS now** with the **PPS in the past** (e.g., 1 day, 7 days, or 30 days ago).

Let:

* PPS now​: the latest price per share
* PPS then: the price per share at a past time
* Δt: number of days between the two points

#### 🧮 Step 1: Calculate ROI (Return on Investment)

$$
\text{ROI} = \frac{\text{PPS}_\text{now}}{\text{PPS}_\text{then}} - 1
$$

This gives the percentage growth over that time period.

#### 📈 Step 2: Annualize It to Get the APY

$$
\text{APY} = \left(1 + \text{ROI} \right)^{\left(\frac{365.2425}{\Delta t}\right)} - 1
$$

Here, `365.2425` is the average number of days in a year (to account for leap years).



#### ✅ Example

* PPS now = `1.10`
* PPS 30 days ago = `1.00`
* Days = 30

$$
\text{ROI} = \frac{1.10}{1.00} - 1 = 0.10
$$

$$
\text{APY} = (1 + 0.10)^{\left(\frac{365.2425}{30}\right)} - 1 \approx 2.138 - 1 = \mathbf{113.8\%}
$$

This means if the strategy keeps performing the same way, the estimated yearly return is **113.8%**.
