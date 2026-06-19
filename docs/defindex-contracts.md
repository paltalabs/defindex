# DeFindex Smart Contracts — Knowledge Base

A reference for understanding the DeFindex contract suite (Factory, Vault, Strategy interface, Blend strategy, and other strategies). Audience: protocol auditors and integrators who need to reason about DeFindex's externally observable behavior and its internal accounting.

All file paths are relative to `apps/contracts/`.

---

## Table of contents

1. [Architecture overview](#1-architecture-overview)
2. [Factory](#2-factory)
3. [Vault](#3-vault) (the deepest section)
4. [Strategy interface](#4-strategy-interface)
5. [Blend strategy](#5-blend-strategy)
6. [Other strategies](#6-other-strategies)
7. [Math reference (consolidated)](#7-math-reference-consolidated)
8. [Roles, lifecycle, upgrade](#8-roles-lifecycle-upgrade)
9. [Errors](#9-errors)
10. [Events](#10-events)
11. [Known limitations and trust assumptions](#11-known-limitations-and-trust-assumptions)

---

## 1. Architecture overview

DeFindex is a Soroban-based vault protocol. Three contract layers cooperate:

```
┌──────────────────────────────────────────────────────┐
│  Factory          (one per network)                  │
│   - admin-set vault WASM hash                        │
│   - admin-set protocol fee receiver + rate           │
│   - deploys new Vault contracts                      │
└──────────────────────────────────────────────────────┘
                       │ deploy_v2(vault_wasm_hash, ...)
                       ▼
┌──────────────────────────────────────────────────────┐
│  Vault            (one per vault product)            │
│   - is itself a Soroban token (the share token)      │
│   - owns user-deposited assets                       │
│   - routes idle funds into Strategies                │
│   - tracks Reports per (vault, strategy) pair        │
│   - charges vault fee + protocol fee                 │
└──────────────────────────────────────────────────────┘
                       │ deposit / withdraw / harvest
                       ▼
┌──────────────────────────────────────────────────────┐
│  Strategy         (one per (asset, protocol) pair)   │
│   - all implement DeFindexStrategyTrait              │
│   - hold/forward the underlying asset                │
│   - report back `balance(vault_addr)` in underlying  │
└──────────────────────────────────────────────────────┘
```

### Conceptual model

- A **Vault** holds one or more **assets** (multi-asset vaults are supported). Each asset can be assigned to multiple **strategies**.
- A user deposits one or more of the vault's assets and receives **vault shares** (a Stellar token, 7 decimals). The vault shares are themselves a fungible token (allowance/transfer/burn supported).
- The vault holds **idle funds** (the raw asset balance of the contract) and **invested funds** (the strategies' balances of the vault). The sum is **total managed funds**.
- Yield accrues inside strategies. The vault tracks the difference between past and present strategy balance via per-strategy **Reports**.
- Fees are **locked** out of reported balances on every state-changing operation, then later **distributed** between the vault fee receiver and the DeFindex protocol fee receiver.

### Contract directory layout

| Path | Purpose |
| --- | --- |
| `factory/` | Factory contract. Deploys vaults from a known WASM hash. |
| `vault/` | Vault contract. Most logic lives here. |
| `strategies/core/` | The `DeFindexStrategyTrait` plus shared `event` and `StrategyError` definitions. Every strategy depends on this crate. |
| `strategies/blend/` | Blend (lending) strategy with BLND reward auto-compounding. |
| `strategies/hodl/`, `strategies/unsafe_hodl/` | Pure custody, no yield (used as sentinels / tests). |
| `strategies/fixed_apr/` | Synthetic fixed-APR yield, mints "yield" balance over time. |
| `strategies/soroswap/`, `strategies/xycloans/` | Soroswap LP and Xycloans flash-loan-pool strategies. |
| `common/` | Cross-crate types (`Strategy`, `AssetStrategySet`) and string helpers. |

---

## 2. Factory

File: `factory/src/lib.rs`

The factory is a thin deployer. Its only state is:

- `Admin` — can update everything below.
- `DeFindexWasmHash` — the bytecode hash of the vault contract used for new deployments.
- `DeFindexReceiver` — the protocol-side fee receiver passed into newly created vaults.
- `FeeRate` — the protocol fee rate (basis points) passed into newly created vaults.
- `TotalVaults` — counter of vaults deployed.
- `VaultAddressNIndexed(u32)` — vault address indexed by deployment order.

(`factory/src/storage.rs:6` lists every storage key.)

### 2.1 Constructor

`factory/src/lib.rs:271`

```rust
fn __constructor(
    e: Env,
    admin: Address,
    defindex_receiver: Address,
    defindex_fee: u32,           // bps, capped at MAX_DEFINDEX_FEE = 9000
    vault_wasm_hash: BytesN<32>,
);
```

- `defindex_fee` is enforced ≤ `MAX_DEFINDEX_FEE = 9000` bps (i.e. ≤ 90%) — see `factory/src/constants.rs:1` and the check in `put_defindex_fee` at `factory/src/storage.rs:111`.
- No `init` re-entry guard; the factory relies on the fact that Soroban runs `__constructor` exactly once at deploy time.

### 2.2 Creating a vault

Two entrypoints:

| Function | Behavior |
| --- | --- |
| `create_defindex_vault` (`factory/src/lib.rs:300`) | Deploys a vault with no initial deposit. |
| `create_defindex_vault_deposit` (`factory/src/lib.rs:341`) | Deploys a vault **and** immediately calls `deposit` with `invest=false` on behalf of `caller`. Requires `caller.require_auth()`. |

Deployment uses `deploy_v2` with a deterministic salt equal to the current `total_vaults` counter (`factory/src/vault.rs:7`):

```rust
let salt = {
    let mut salt_bytes = [0u8; 32];
    let total_vaults_bytes = total_vaults.to_be_bytes();
    salt_bytes[..total_vaults_bytes.len()].copy_from_slice(&total_vaults_bytes);
    BytesN::from_array(e, &salt_bytes)
};
e.deployer().with_current_contract(salt).deploy_v2(wasm_hash, constructor_args);
```

The factory passes the current `defindex_receiver` and `defindex_fee` into the vault constructor. This means:

> **Audit note.** Updating `defindex_fee` or `defindex_receiver` on the factory **only affects future vaults**. Existing vaults retain whatever values were active at their deploy time.

`create_defindex_vault_deposit` constructs the deposit args manually and calls the vault via `invoke_contract` (`factory/src/lib.rs:234`). It uses `amounts_min` = vector of zeros and `invest=false`; the deposit can therefore be sandwiched in the same tx. Watch out: the call sets `amounts_min = 0`, so the user does not get slippage protection for this first deposit.

### 2.3 Admin functions

All four admin setters require the current admin's auth (`factory/src/lib.rs:384–438`):

- `set_new_admin(new_admin)`
- `set_defindex_receiver(new_fee_receiver)`
- `set_defindex_fee(defindex_fee)` — capped at 9000 bps.
- `set_vault_wasm_hash(new_vault_wasm_hash)` — note again, this affects only **future** vaults.

There is **no two-step admin transfer**; setting a new admin is atomic and irreversible.

### 2.4 Read functions

`admin()`, `defindex_receiver()`, `total_vaults()`, `get_vault_by_index(index)`, `defindex_fee()`, `vault_wasm_hash()`.

### 2.5 Factory errors

`factory/src/error.rs`:

| Code | Name | Meaning |
| --- | --- | --- |
| 401 | `NotInitialized` | Constructor never ran (should be impossible) or storage entry missing. |
| 404 | `AssetLengthMismatch` | `assets.len() != amounts.len()` in `create_defindex_vault_deposit`. |
| 405 | `IndexDoesNotExist` | `get_vault_by_index` for an index ≥ `total_vaults`. |
| 406 | `FeeTooHigh` | `defindex_fee > 9000`. |

### 2.6 Factory events (topic = `"DeFindexFactory"`)

`factory/src/events.rs`:

- `create` — `CreateDeFindexEvent { roles, vault_fee, assets }` on each vault deploy.
- `nadmin` — `NewAdminEvent`.
- `nreceiver` — `NewDeFindexReceiverEvent`.
- `n_fee` — `NewFeeRateEvent`.
- `n_wasm` — `NewVaultWasmHashEvent`.

---

## 3. Vault

File entry: `vault/src/lib.rs`. The vault implements three traits defined in `vault/src/interface.rs`:

- `VaultTrait` — user-facing deposit/withdraw + read methods.
- `AdminInterfaceTrait` — role getters/setters and the upgrade.
- `VaultManagementTrait` — rebalance and fee lifecycle.

It also implements `soroban_sdk::token::Interface` via the embedded `VaultToken` (`vault/src/token/contract.rs`). **The vault contract address is the share token's address.** A consumer can call standard token methods (`transfer`, `balance`, `approve`, `burn`, `total_supply` (custom)) on the vault.

### 3.1 Constructor

`vault/src/lib.rs:115`

```rust
fn __constructor(
    e: Env,
    assets: Vec<AssetStrategySet>,        // assets and their strategy lists
    roles: Map<u32, Address>,             // 0=EM, 1=FeeRecv, 2=Mgr, 3=RebMgr
    vault_fee: u32,                       // bps, capped at 9000 (in storage.rs:110)
    defindex_protocol_receiver: Address,
    defindex_protocol_rate: u32,          // bps, capped at 9000
    soroswap_router: Address,
    name_symbol: Map<String, String>,     // "name" -> ..., "symbol" -> ...
    upgradable: bool,
);
```

Sequence:

1. Roles are set from the map. **All four must be present** or the constructor panics with `RolesIncomplete (104)`.
2. The vault token's name is set to `"DeFindex-Vault-" ++ name_symbol["name"]`, capped at 35 chars (see `StringExtensions::concat` in `common/src/utils.rs:9`). Symbol is set verbatim.
3. `vault_fee` is stored. The setter at `vault/src/storage.rs:110` panics if `> 9000`.
4. `defindex_protocol_rate` is checked inline (`vault/src/lib.rs:142`) — must be ≤ 9000.
5. `upgradable` and `soroswap_router` are stored.
6. Each asset is validated:
   - `validate_assets` (`vault/src/utils.rs:29`) rejects empty `assets` and duplicate asset addresses.
   - For each strategy in each asset, the constructor calls `strategy.asset()` via the strategy client and checks it equals the asset address. Panics with `StrategyDoesNotSupportAsset (102)` otherwise (`vault/src/lib.rs:159`).
7. Token metadata is written with **decimals = 7** (`vault/src/lib.rs:168`).

> **Audit note.** There is no liveness check on `soroswap_router`: any address can be supplied, but it is only invoked during `rebalance` SwapExactIn/SwapExactOut (`vault/src/router.rs`). A vault with a fake router cannot swap, but normal deposit/withdraw still work.

### 3.2 The share token (`VaultToken`)

`vault/src/token/`:

- The vault implements `soroban_sdk::token::Interface` (`vault/src/token/contract.rs:57`): standard `allowance`, `approve`, `balance`, `transfer`, `transfer_from`, `burn`, `burn_from`, `decimals`, `name`, `symbol`.
- Custom: `total_supply()` (`vault/src/token/contract.rs:52`) reads from instance storage.
- `internal_mint` and `internal_burn` (`vault/src/token/contract.rs:19, 32`) are the only paths the vault itself uses; they atomically adjust `TotalSupply`.
- Balance storage is **persistent** with `BALANCE_BUMP_AMOUNT = 120 * DAY_IN_LEDGERS` (`vault/src/token/storage_types.rs:7`). TotalSupply is **instance** storage.
- `check_nonnegative_amount` in `token/contract.rs:13` panics (not error) on negative amounts.

> **Audit note.** Shares are fully fungible. A consumer protocol that takes vault shares as collateral can use the standard token interface, but must understand the share-price drift caveats in §3.5.

### 3.3 Total managed funds and idle vs invested

`vault/src/funds.rs` is the single source of truth for accounting:

- `fetch_idle_funds_for_asset(asset)` (`funds.rs:23`) — calls `asset_token.balance(self)` on the asset's token contract. This is the asset sitting on the vault that hasn't been pushed into a strategy.
- `fetch_strategy_invested_funds(strategy, lock_fees)` (`funds.rs:45`) — calls `strategy.balance(self)` via the `DeFindexStrategyClient`, then subtracts `locked_fee` from the report. If `lock_fees=true`, it first reports the new balance and locks new fees (calls `update_report_and_lock_fees`).
- `fetch_total_managed_funds(lock_fees)` (`funds.rs:116`) — iterates all assets:
  - `idle_amount` = on-vault balance,
  - `invested_amount` = Σ strategy balances net of locked fees,
  - `total_amount = idle + invested`,
  - returns `Vec<CurrentAssetInvestmentAllocation>` in the order assets were stored.

> **Critical invariant.** `total_amount` returned by `fetch_total_managed_funds` is **net of locked fees** but **gross of yet-to-be-locked gains** when `lock_fees=false`. Always use `lock_fees=true` (the deposit/withdraw paths do) when the value is used to mint or burn shares. The public read endpoint `fetch_total_managed_funds(&Env)` (`vault/src/lib.rs:567`) passes `false`, so its result is a slightly stale "before-fees" view.

`CurrentAssetInvestmentAllocation` (`vault/src/models.rs:14`):

```rust
struct CurrentAssetInvestmentAllocation {
    asset: Address,
    total_amount: i128,
    idle_amount: i128,
    invested_amount: i128,
    strategy_allocations: Vec<StrategyAllocation>,  // {strategy, amount, paused}
}
```

### 3.4 Share-price math

A vault holds one or more assets. The share price is conceptually:

```
price_per_share[asset_i] = total_managed_funds[asset_i].total_amount / total_supply
```

That is, holding 1 share entitles the holder to a proportional slice of every asset.

#### 3.4.1 Single-asset deposit

`vault/src/deposit.rs:64` (`calculate_single_asset_shares`):

```
if total_supply == 0:
    shares = amounts_desired[0]
else:
    shares = total_supply * amounts_desired[0] / total_amount[0]    // integer div, floor
```

This is the canonical ERC-4626-style formula:

```
Δshares = totalShares · Δassets / totalAssets
```

Rounding direction: **floor** (Rust `i128::checked_div`). This favors existing holders on deposits (`new_shares ≤ true_share_value`), which is the standard safe direction.

#### 3.4.2 Multi-asset deposit

`vault/src/utils.rs:134, 186`. The algorithm:

1. Iterate `i = 0..num_assets`. For each non-zero-balance asset `i`, treat it as the "enforced" asset (the one whose `amounts_desired[i]` is used verbatim).
2. Call `calculate_optimal_amounts_and_shares_with_enforced_asset(i)`:
   - For the enforced asset, `optimal_amounts[i] = amounts_desired[i]`.
   - For every other asset `j`:
     ```
     optimal_amounts[j] = ceil( reserve[j] * amounts_desired[i] / reserve[i] )
     ```
     using `divide_rounding_up` (`utils.rs:20`). The ceiling here protects existing holders by demanding **at least** the pro-rata required amount; the user pays the rounding cost.
   - Shares minted = `total_supply * amounts_desired[i] / reserve[i]` (floor).
3. Validate: for every `j ≠ i`, either `optimal[j] ≤ desired[j]` (acceptable; user pays slightly less than desired on those assets) OR skip. Also check `optimal[j] ≥ amounts_min[j]`.
4. The first enforced asset whose calculation passes all checks wins. Returns that allocation.
5. If no enforced-asset choice satisfies all constraints, returns `NoOptimalAmounts (118)`.

> **Audit note.** This is Uniswap-V2-style `quote`-and-pick-the-feasible-ratio logic. The user supplies `amounts_desired` (max willing to put in) and `amounts_min` (slippage floor). The chosen allocation is **the first feasible**, not necessarily the one minting the most shares. This is deterministic but order-dependent: assets are scanned in the order they were registered. The ordering is fixed at construction.

For an **empty vault** with multiple assets (`total_supply == 0`), the branch at `deposit.rs:33` short-circuits to:

```
amounts = amounts_desired  (verbatim)
shares  = Σ amounts_desired
```

The sum-of-amounts heuristic establishes the initial ratio. The first depositor effectively defines the ratio; subsequent depositors are constrained to it. There is no on-chain price oracle; consumers must initialize the vault carefully (or use the bootstrap deposit pattern documented in §11.2).

#### 3.4.3 MINIMUM_LIQUIDITY (anti-inflation)

`vault/src/lib.rs:51` defines `MINIMUM_LIQUIDITY = 1000`. In `mint_shares` (`vault/src/deposit.rs:88`):

```rust
if total_supply == 0 {
    if shares_to_mint <= MINIMUM_LIQUIDITY {
        panic_with_error!(InsufficientAmount);
    }
    internal_mint(self, MINIMUM_LIQUIDITY);                          // burned to vault address
    internal_mint(from, shares_to_mint - MINIMUM_LIQUIDITY);
}
```

The first depositor:

- Must mint > 1000 shares.
- Receives `shares_to_mint - 1000`; the 1000 shares are minted **to the vault contract itself** and never recovered (no path exists to burn or transfer them out).

This is Uniswap V2's defense against a donation-then-deposit inflation attack. See §11.1 for the residual attack surface that remains.

#### 3.4.4 Withdraw math

`vault/src/lib.rs:299`. Withdrawal is share-burn-first:

1. `withdraw_shares > 0` and `withdraw_shares ≤ total_supply` else error.
2. `total_managed_funds = fetch_total_managed_funds(lock_fees=true)`. **Locks fees on every withdrawal**.
3. `internal_burn(from, withdraw_shares)` — burns immediately. Insufficient balance reverts via `spend_balance`.
4. For each asset `i`:
   ```
   requested[i] = total_amount[i] * withdraw_shares / total_supply_before_burn  // floor
   ```
5. Check `requested[i] ≥ min_amounts_out[i]`.
6. Pay `requested[i]` to `from`:
   - If `idle_amount[i] ≥ requested[i]`, transfer from idle.
   - Otherwise, transfer all idle, then unwind the remainder pro-rata across that asset's strategies. The last strategy receives whatever is left to balance any rounding shortfall (`vault/src/lib.rs:376`).

> **Audit note.** `total_supply_before_burn` is captured **before** the burn (`vault/src/lib.rs:321`). Steps 4–6 use that captured supply; correct ERC-4626 semantics.

> **Audit note.** Rounding direction is **floor**: a withdrawer of `s` shares receives `floor(total · s / supply)` of each asset, leaving any rounded-off dust in the vault for remaining holders. This is the conventional safe direction.

The `min_amounts_out` parameter is a per-asset floor: it lets the caller assert "I want at least X of asset[i]" and revert otherwise. The check (`vault/src/lib.rs:347`) returns `InsufficientOutputAmount (160)`.

#### 3.4.5 `get_asset_amounts_per_shares`

`vault/src/utils.rs:98` — public, view-ish (it still mutates because it calls `fetch_total_managed_funds(true)` which reports + locks fees). Returns the per-asset amount that `vault_shares` shares would entitle the holder to:

```
amount[i] = total_amount[i] * vault_shares / total_supply        (floor)
```

> **Audit note.** This is **not a pure read**. Calling it locks new fees if any gains have accrued. For a stale, no-side-effects read use `fetch_total_managed_funds` (the public one), then compute the proportional amount off-chain.

### 3.5 Reports — the fee accounting heart

File: `vault/src/report.rs`. The vault keeps one `Report` per strategy address, persisted under `DataKey::Report(strategy_address)` (`vault/src/storage.rs:31`):

```rust
struct Report {
    prev_balance: i128,        // last observed strategy.balance(self)
    gains_or_losses: i128,     // accumulated since last lock_fee
    locked_fee: i128,          // fees already accrued, not yet distributed
}
```

#### 3.5.1 `report(current_balance)`

`vault/src/report.rs:115`:

```
prev = self.prev_balance if self.prev_balance != 0 else current_balance
delta = current_balance - prev
self.gains_or_losses += delta
self.prev_balance = current_balance
```

- The first report bootstraps `prev_balance = current_balance` so the first delta is 0 (no phantom gain). Look at `vault/src/report.rs:117`.
- A loss (negative delta) is also accumulated. `gains_or_losses` can go negative.

#### 3.5.2 `lock_fee(fee_rate)`

`vault/src/report.rs:51`:

```
if gains_or_losses <= 0: return Ok(())          // no fee on losses
total_fee   = gains_or_losses * fee_rate / SCALAR_BPS    // SCALAR_BPS = 10_000
locked_fee += total_fee
gains_or_losses = 0
```

- Fees are taken **only on positive accumulated gains**.
- `fee_rate` is `get_vault_fee(&e)` at call site — i.e. the vault-level fee bps (capped at 9000). **The DeFindex protocol fee is NOT taken at lock time**, only at distribute time (§3.5.4).
- After locking, `gains_or_losses` is reset to 0 — meaning the next gain cycle starts fresh. **Losses that occurred before a positive lock are not carried forward.** A strategy that gained, locked a fee, then lost the same amount, leaves the locked fee in place. See §11.3.

#### 3.5.3 `release_fee(amount)`

`vault/src/report.rs:89`:

```
if locked_fee < amount: panic InsufficientFeesToRelease (129)
locked_fee     -= amount
gains_or_losses += amount
```

Manager-only (see `release_fees` at `vault/src/lib.rs:1014`). Used to refund part of a previously locked fee back into the share-price pool — e.g. when the manager decides the APY was misreported.

#### 3.5.4 `distribute_strategy_fees`

`vault/src/report.rs:189`:

```
fees_to_distribute = report.locked_fee
if fees_to_distribute > 0:
    defindex_amount = fees_to_distribute * defindex_protocol_rate / 10_000
    vault_amount    = fees_to_distribute - defindex_amount

    # Withdraw the underlying from the strategy back to the vault
    remaining_balance = strategy.withdraw(fees_to_distribute, self, self)

    # Pay both receivers
    asset_token.transfer(self, vault_fee_receiver,        vault_amount)
    asset_token.transfer(self, defindex_protocol_receiver, defindex_amount)

    report.prev_balance = remaining_balance
    report.locked_fee   = 0
```

> **Audit note.** Two consequences:
> 1. `defindex_protocol_rate` (set on the **vault** at construction) governs the protocol's share of every locked fee, regardless of what the factory's current rate is.
> 2. `vault_amount = locked_fee - defindex_amount` — the protocol takes its slice **out of** the vault fee, not on top. From the vault fee receiver's POV the effective rate is `vault_fee_bps · (1 - defindex_protocol_rate/10_000)`.

> **Audit note.** `distribute_strategy_fees` is called:
> - From `distribute_fees` (vault/src/lib.rs:1039) by Manager or VaultFeeReceiver.
> - From `rebalance` Unwind and Invest paths (vault/src/lib.rs:869, 903) automatically — so any rebalance flushes fees.
> - From `rescue` (vault/src/lib.rs:450) before unwinding the strategy.

#### 3.5.5 When are fees locked?

- `deposit` (`vault/src/lib.rs:243`) → `fetch_total_managed_funds(true)` → for every strategy, calls `update_report_and_lock_fees`.
- `withdraw` (`vault/src/lib.rs:308`) → same.
- `get_asset_amounts_per_shares` (`vault/src/lib.rs:594`) → same.
- `lock_fees` (`vault/src/lib.rs:972`) → manager-only explicit lock, optionally with a new `fee_bps`.
- `rebalance` Unwind/Invest paths → `fetch_strategy_invested_funds(true)` (`vault/src/lib.rs:862, 894`).

The public `fetch_total_managed_funds()` (`vault/src/lib.rs:567`) passes `false`; it returns a "pre-lock" snapshot useful for read-only UIs.

`report()` (`vault/src/lib.rs:656`) updates `prev_balance` and accumulates `gains_or_losses` but does **not** call `lock_fee`. It's a pure report.

#### 3.5.6 Update-balance side-effect of invest

`vault/src/strategies.rs:142` (`invest_in_strategy`):

```rust
let strategy_funds = strategy_client.deposit(amount, self);     // returns new balance
report.gains_or_losses += strategy_funds - report.prev_balance - amount;
report.prev_balance     = strategy_funds;
```

The subtraction of `amount` is crucial: `strategy.deposit` returns the new strategy balance **including the just-deposited amount**, so the delta we want to attribute to yield is `(new - old - newly_deposited)`. This implicitly assumes the strategy returns balance immediately consistent with the deposit. Watch: this is **only** correct if `strategy.deposit` returns the true post-deposit balance, which the interface requires (see §4).

### 3.6 Rebalance

`vault/src/lib.rs:846` — Manager or RebalanceManager.

Input: a vector of `Instruction`s. Defined in `vault/src/models.rs:31`:

```rust
enum Instruction {
    Unwind(strategy_address, amount),
    Invest(strategy_address, amount),
    SwapExactIn(token_in, token_out, amount_in, amount_out_min, deadline),
    SwapExactOut(token_in, token_out, amount_out, amount_in_max, deadline),
}
```

Handling, in order:

- `Unwind`: pulls `amount` of underlying out of the strategy and back to the vault (idle). Distributes any locked fees first (§3.5.4). Errors if `amount > strategy_invested_funds (lock_fees=true)`.
- `Invest`: pushes `amount` of idle into the strategy. Errors if the strategy is `paused` or `amount <= 0`. Updates the report via `invest_in_strategy`.
- `SwapExactIn` / `SwapExactOut`: both legs must be supported vault assets (`router.rs:17 is_supported_asset`). Uses the configured Soroswap router. The vault pre-authorizes the token transfer to the pair address (`router.rs:51`) so the router can pull the input.

> **Audit note.** Rebalance does **not** limit the swap path beyond "both tokens supported by the vault". A misconfigured RebalanceManager could swap any vault asset to any other vault asset at the router's price (with `amount_out_min` and `deadline` being the user-supplied slippage controls). There is no MEV / slippage curve enforcement beyond what the router itself does.

> **Audit note.** `rebalance` does not enforce a target weight or an upper/lower bound — it executes the given instructions atomically. The "where to put the money" policy is entirely the RebalanceManager's responsibility.

### 3.7 Rescue (emergency withdraw)

`vault/src/lib.rs:431` — Emergency Manager or Manager.

```
1. Look up the strategy's asset.
2. Distribute any locked fees (might transfer to fee receivers).
3. Read the strategy's current balance.
4. Call strategy.withdraw(full_balance, self, self) via unwind_from_strategy.
5. Reset the report (prev_balance, gains_or_losses, locked_fee all = 0).
6. Pause the strategy.
```

> **Audit note.** After rescue, the asset is **idle in the vault**. Users withdrawing get their share of idle just like normal. The strategy is paused; subsequent `rebalance Invest` to this strategy is blocked until `unpause_strategy` is called.

> **Audit note.** `rescue` calls `pause_strategy` (`strategies.rs:68`) **after** the withdraw. The pause is purely advisory — it gates the `Invest` instruction in rebalance and the `invest=true` path in deposit. **It does not block withdrawals from a paused strategy** if there is still balance there (e.g., dust). This is intentional but worth noting.

### 3.8 Pause / Unpause

`vault/src/lib.rs:497, 527` — Emergency Manager or Manager.

Toggles the `paused` flag on the `Strategy` struct stored within `AssetStrategySet`. Mutates the in-storage `AssetStrategySet` via `set_asset` (`strategies.rs:84`). A paused strategy is:

- Skipped by `generate_investment_allocations` (`investment.rs:116`) — auto-invest after deposit will not touch it.
- Rejected by `rebalance Invest` (`vault/src/lib.rs:889`).
- Still readable, balance still counted in `total_amount`.
- Still withdrawable during user `withdraw` (proportional unwind) and via `rebalance Unwind`.

### 3.9 Fee lifecycle (manager-facing)

`vault/src/lib.rs:972, 1014, 1039`:

- `lock_fees(new_fee_bps: Option<u32>)` — Manager-only. Optionally updates the vault fee rate, then iterates every strategy and calls `report.lock_fee(current_vault_fee)`. Returns the list of reports. Does **not** distribute.
- `release_fees(strategy, amount)` — Manager-only. Calls `report.release_fee` for the strategy.
- `distribute_fees(caller)` — Manager or VaultFeeReceiver. For every strategy, calls `distribute_strategy_fees` (which actually unwinds the underlying and pays it out). Emits `dfees`.

### 3.10 Upgrade

`vault/src/lib.rs:822` — Manager only, gated by the `Upgradable` flag set at construction (`storage::is_upgradable`, defaults to `true` if missing — `storage.rs:177`).

```rust
if !is_upgradable: return Err(NotUpgradable);
manager.require_auth();
env.deployer().update_current_contract_wasm(new_wasm_hash);
```

> **Audit note.** Constructor `__constructor` is **not** re-executed on upgrade. There is no `migrate()` function. Any storage schema change in a new WASM must be backward-compatible or migration must be done manually by the upgrader.

### 3.11 Storage layout

`vault/src/storage.rs:22`:

```rust
enum DataKey {
    TotalAssets,                // u32 — instance
    AssetStrategySet(u32),      // AssetStrategySet by index — instance
    DeFindexProtocolFeeReceiver, // Address — instance
    Upgradable,                 // bool — instance
    VaultFee,                   // u32 (bps) — instance
    SoroswapRouter,             // Address — instance
    DeFindexProtocolFeeRate,    // u32 (bps) — instance
    Factory,                    // (unused field, declared but never set/get)
    Report(Address),            // Report — persistent
}
```

Roles use a separate key enum (`vault/src/access.rs:6`), also instance storage:

```rust
enum RolesDataKey {
    EmergencyManager = 0,
    VaultFeeReceiver = 1,
    Manager          = 2,
    RebalanceManager = 3,
}
```

TTL constants (`vault/src/storage.rs:6`):

- Instance bump: `30 * 17_280 = 518_400` ledgers (~30 days). Lifetime threshold: 29 days.
- Persistent bump: `120 * 17_280 = 2_073_600` ledgers (~120 days). Lifetime threshold: 100 days.

All state-changing entrypoints call `extend_instance_ttl(&e)` at the top (`vault/src/storage.rs:14`).

> **Audit note.** Reports are persistent (per strategy address). If a strategy is removed or replaced in a future deployment, its old Report becomes orphaned but stays in storage until TTL.

### 3.12 Read-only endpoints

| Method | Notes |
| --- | --- |
| `get_assets()` | List of `AssetStrategySet`. |
| `fetch_total_managed_funds()` | `lock_fees=false`. Stale (pre-lock). |
| `get_asset_amounts_per_shares(vault_shares)` | `lock_fees=true`. Mutates fees. Not pure. |
| `get_fees()` | `(vault_fee_bps, defindex_protocol_fee_bps)`. |
| `report()` | Updates and returns reports without locking fees. |
| `get_fee_receiver()`, `get_manager()`, `get_emergency_manager()`, `get_rebalance_manager()` | Role getters. |
| `total_supply()` (token interface) | Vault shares total supply. |
| `balance(addr)`, `allowance(...)`, etc. | Standard token interface. |

---

## 4. Strategy interface

File: `strategies/core/src/lib.rs`.

```rust
pub trait DeFindexStrategyTrait {
    fn __constructor(env: Env, asset: Address, init_args: Vec<Val>);
    fn asset(env: Env)                                      -> Result<Address, StrategyError>;
    fn deposit(env: Env, amount: i128, from: Address)       -> Result<i128, StrategyError>;
    fn harvest(env: Env, from: Address, data: Option<Bytes>) -> Result<(), StrategyError>;
    fn balance(env: Env, from: Address)                     -> Result<i128, StrategyError>;
    fn withdraw(env: Env, amount: i128, from: Address, to: Address) -> Result<i128, StrategyError>;
}
```

The vault interacts with strategies exclusively through this trait via `DeFindexStrategyClient`. The vault's expectations are explicit in the trait documentation and reinforced by how `funds.rs` and `strategies.rs` use the return values:

### 4.1 Behavioral contract that every strategy must honor

| Method | What the vault assumes about the return value |
| --- | --- |
| `asset` | Returns the **underlying** asset address the strategy holds. Checked at vault construction (`vault/src/lib.rs:161`). |
| `deposit(amount, from)` | After transferring `amount` of `asset` from `from` to the strategy, deposits into the underlying protocol. **Returns `balance(from)` in underlying units** — that is, the new total amount of `asset` that `from` can claim back. The vault uses this to compute `gains_or_losses` (`vault/src/strategies.rs:172`). |
| `withdraw(amount, from, to)` | Withdraws `amount` of underlying to `to`. **Returns `balance(from)` in underlying units after the withdraw.** The vault uses this to set `report.prev_balance` (`vault/src/lib.rs:399`). |
| `balance(from)` | Pure read of `from`'s entitlement to underlying, including accrued yield. **Must NOT return shares or derivatives.** |
| `harvest(from, data)` | Trigger reward claim / re-investment. No return value. `from` must equal a strategy-defined keeper if applicable. `data` is an optional opaque `Bytes` blob — see §5.4 for how Blend uses it as `amount_out_min`. |

> **Audit note.** The trait does **not** mandate that `deposit` returns the new balance immediately — it just defines what "balance" means. If a strategy uses a delayed accrual model and `deposit` returns the pre-deposit balance, then `report.gains_or_losses` will be incorrect. Auditors of consumer-side code don't need to verify this on every strategy, but auditors of new strategies must.

### 4.2 Strategy errors

`strategies/core/src/error.rs`:

| Code | Name |
| --- | --- |
| 401 | `NotInitialized` |
| 410 | `NegativeNotAllowed` |
| 411 | `InvalidArgument` |
| 412 | `InsufficientBalance` |
| 413 | `UnderflowOverflow` |
| 414 | `ArithmeticError` |
| 415 | `DivisionByZero` |
| 416 | `InvalidSharesMinted` |
| 417 | `OnlyPositiveAmountAllowed` |
| 418 | `NotAuthorized` |
| 420 | `ProtocolAddressNotFound` |
| 421 | `DeadlineExpired` |
| 422 | `ExternalError` |
| 423 | `SoroswapPairError` |
| 451 | `AmountBelowMinDust` |
| 452 | `UnderlyingAmountBelowMin` |
| 453 | `BTokensAmountBelowMin` |
| 454 | `InternalSwapError` |
| 455 | `SupplyNotFound` |

### 4.3 Strategy events

`strategies/core/src/event.rs` — every strategy is expected to emit these:

- `deposit` topic on the strategy's name → `DepositEvent { amount, from }`
- `withdraw` topic → `WithdrawEvent { amount, from }`
- `harvest` topic → `HarvestEvent { amount, from, price_per_share }` — `price_per_share` is scaled to 12 decimals where applicable (see Blend §5.4).

The vault unwinds always call the strategy's `withdraw`, which emits a withdraw event from the strategy. The vault itself also emits its own `VaultWithdrawEvent`. Both are observable.

### 4.4 Why the vault treats strategies as a passthrough

In the vault's accounting, `strategy.balance(self)` is the source of truth for invested funds. The vault does **not** track strategy-internal shares. It only tracks the strategy's externally reported balance and computes deltas. This delegates the share-bookkeeping to each strategy. Consequences:

- The vault is **decoupled** from the underlying protocol's share mechanics.
- A buggy strategy (over-reporting balance, etc.) directly inflates vault share price, which is then realized by depositors/withdrawers.
- This is why §11.6 emphasizes strategy trust assumptions.

---

## 5. Blend strategy

The Blend strategy lends the underlying asset (e.g. XLM, USDC) into a Blend lending pool and auto-compounds BLND rewards. It's the most sophisticated strategy in the repo and the only one with non-trivial math.

File entry: `strategies/blend/src/lib.rs`.

### 5.1 Three layers of accounting

The Blend strategy juggles three units:

```
┌─────────────────────────────────────┐
│  underlying asset (e.g. USDC)       │  what the vault deposits and withdraws
├─────────────────────────────────────┤
│  Blend bTokens                      │  Blend pool's lending-share token
├─────────────────────────────────────┤
│  strategy shares                    │  the strategy's internal share token (per vault)
└─────────────────────────────────────┘
```

Conversions (all live in `reserves.rs:24` on `StrategyReserves`):

- `underlying → bTokens`: `bTokens = underlying * SCALAR_12 / b_rate` (`utils.rs:41`, `utils.rs:75`). `SCALAR_12 = 10^12`.
- `bTokens → underlying`: `underlying = bTokens * b_rate / SCALAR_12` (`reserves.rs:54`, floor).
- `bTokens → strategy shares`: `shares = bTokens * total_shares / total_b_tokens` (`reserves.rs:26`, floor; `reserves.rs:37`, ceil).
- `strategy shares → bTokens`: `bTokens = shares * total_b_tokens / total_shares` (`reserves.rs:47`, floor).
- `strategy shares → underlying`: composition `shares_to_b_tokens_down ∘ b_tokens_to_underlying_down` (`utils.rs:22`).

The `b_rate` is read from the Blend pool itself at every reserve-touching operation via `get_strategy_reserve_updated` (`reserves.rs:79`), which calls `blend_pool::reserve_b_rate` (`blend_pool.rs:264`). So `b_rate` is always the pool's latest. `b_rate` rises over time as the pool accrues interest — that is the strategy's source of yield (plus the BLND rewards).

`StrategyReserves` (`reserves.rs:11`) is stored in instance storage:

```rust
struct StrategyReserves {
    total_shares: i128,    // sum of all vaults' strategy-share balances
    total_b_tokens: i128,  // strategy's bToken holding in the Blend pool
    b_rate: i128,          // last-known b_rate
}
```

Per-vault share balance is stored separately, persistent, under `DataKey::VaultPos(vault_address)` (`storage.rs:18, 54`).

### 5.2 Constructor

`strategies/blend/src/lib.rs:76`:

```rust
init_args = [
    blend_pool_address: Address,
    blend_token:        Address,    // BLND token
    soroswap_router:    Address,
    reward_threshold:   i128,       // min BLND balance that triggers reinvest
    keeper:             Address,
];
```

The constructor:

1. Calls `blend_pool.get_reserve(asset)` to read the asset's `reserve_id` (`config.index`).
2. Derives `claim_id = reserve_id * 2 + 1` (the bToken claim ID — see `lib.rs:107` comment for the layout: dTokens are even, bTokens are odd).
3. Validates `reward_threshold > 0`.
4. Stores `Config` and `keeper`.

### 5.3 Deposit

`lib.rs:166`. `from` must be the vault (or whoever has the asset and wants strategy shares).

```
1. from.require_auth()
2. reserves = get_strategy_reserve_updated()       // pulls fresh b_rate
3. (optimal_deposit, optimal_b_tokens) = calculate_optimal_deposit_amount(amount, reserves)
4. token.transfer(from, self, amount)              // pull full amount
5. if amount != optimal_deposit:
        token.transfer(self, from, amount - optimal_deposit)  // refund excess
6. blend_pool::supply(from, optimal_deposit)       // supply to Blend
7. (vault_shares, reserves) = reserves::deposit(from, optimal_b_tokens, reserves)
8. underlying_balance = shares_to_underlying(vault_shares, reserves)
9. emit deposit; return underlying_balance
```

`calculate_optimal_deposit_amount` (`utils.rs:36`):

```
b_tokens_minted = floor(amount * SCALAR_12 / b_rate)
if total_shares == 0:
    optimal_b_tokens = b_tokens_minted
else:
    shares_minted    = b_tokens_to_shares_down(b_tokens_minted)
    if shares_minted == 0: error InvalidSharesMinted
    optimal_b_tokens = ceil(shares_minted * total_b_tokens / total_shares)
optimal_deposit = ceil(optimal_b_tokens * b_rate / SCALAR_12)
```

> **Audit note.** The "optimal" amount is the **smallest** deposit that mints at least the rounded-down number of shares the user is entitled to. Excess is refunded. This prevents users from being charged for share-rounding loss; the strategy absorbs at most 1 wei of rounding into `b_tokens` ↔ `shares` conversion.

`reserves::deposit` (`reserves.rs:135`):

```
new_minted_shares = b_tokens_to_shares_down(b_tokens_amount)
if new_minted_shares <= 0: panic InvalidSharesMinted

if total_shares == 0:                    // first depositor
    if new_minted_shares <= 1000: panic InvalidSharesMinted
    new_vault_minted_shares = new_minted_shares - 1000
else:
    new_vault_minted_shares = new_minted_shares

total_shares   += new_minted_shares      // full mint to global accounting
total_b_tokens += b_tokens_amount
vault_shares[from] += new_vault_minted_shares
```

> **Audit note.** Blend strategy has its **own** 1000-share burn (separate from the vault's 1000-share burn in §3.4.3). The 1000 shares are minted to `total_shares` but **not credited to the depositor** — they are effectively burned (no address owns them). This adds another layer of protection against share-price manipulation when a strategy is freshly deployed.

### 5.4 Harvest

`lib.rs:220`. Keeper-only.

```
1. keeper.require_auth(); from must equal keeper.
2. harvested_blend = blend_pool.claim(self, claim_ids, self)   // pulls BLND to strategy
3. amount_out_min = decode(data) as i128, default 0
4. perform_reinvest(config, amount_out_min):
   a. blnd_balance = BLND.balance(self)
   b. if blnd_balance < reward_threshold: return current reserves (no-op)
   c. path = [BLND, asset]
   d. deadline = ledger.timestamp() + 1
   e. swap_amounts = soroswap.swap_exact_tokens_for_tokens(blnd_balance, amount_out_min, path, self, deadline)
   f. amount_out = swap_amounts[1]
   g. b_tokens_minted = blend_pool::supply(self, amount_out, is_reinvest=true)
   h. reserves::harvest(b_tokens_minted): total_b_tokens += b_tokens_minted
5. emit harvest(amount=harvested_blend, price_per_share = shares_to_underlying(SCALAR_12, reserves))
```

> **Audit note.** Harvest **does not mint new strategy shares** — it grows `total_b_tokens` while leaving `total_shares` unchanged. The result is that the per-share underlying balance **increases**, distributing rewards pro-rata across existing share holders.

> **Audit note.** The `data: Option<Bytes>` argument is used here to pass an `amount_out_min` for the Soroswap leg. The harvest caller (the keeper) chooses it. If `data` is `None` or empty, `amount_out_min = 0`, which permits any output (including a sandwich attack on small reward amounts). Consumers building over DeFindex should not assume harvest is MEV-protected.

> **Audit note.** `deadline = ledger.timestamp() + 1`. This is effectively "execute this ledger or the next" — very tight, intentional, prevents replays past one ledger. Note Soroban ledger times are deterministic at simulation, so a real txn submitted with this deadline is unlikely to expire from RPC-side delay but might from cross-ledger contention.

### 5.5 Withdraw

`lib.rs:271`.

```
1. from.require_auth()
2. reserves = get_strategy_reserve_updated()
3. (optimal_withdraw, b_tokens_burnt) = calculate_optimal_withdraw_amount(amount, reserves)
4. blend_pool::withdraw(to, optimal_withdraw)
5. (vault_shares, reserves) = reserves::withdraw(from, b_tokens_burnt, reserves)
6. underlying_balance = shares_to_underlying(vault_shares, reserves)
7. emit withdraw; return underlying_balance
```

`calculate_optimal_withdraw_amount` (`utils.rs:71`):

```
b_tokens_burnt   = ceil(withdraw_amount * SCALAR_12 / b_rate)
shares_burnt    = b_tokens_to_shares_up(b_tokens_burnt)
optimal_b_tokens = floor(shares_burnt * total_b_tokens / total_shares)
optimal_withdraw = floor(optimal_b_tokens * b_rate / SCALAR_12)
```

> **Audit note.** Rounding here is **the opposite** of deposit (ceil on the way into shares, floor on the way out to underlying). This favors existing holders again — the withdrawer burns a possibly inflated share count and receives a possibly deflated underlying amount. The strategy retains at most 1 wei of dust per withdraw. This is consistent with conservative-rounding patterns.

`reserves::withdraw` (`reserves.rs:210`):

```
if total_shares < shares_burnt OR total_b_tokens < b_tokens_amount: error InsufficientBalance
total_shares      -= shares_burnt
total_b_tokens    -= b_tokens_amount
if shares_burnt > vault_shares[from]: error InsufficientBalance
vault_shares[from] -= shares_burnt
```

### 5.6 Balance

`lib.rs:309`:

```
vault_shares = storage::get_vault_shares(from)
if vault_shares == 0: return 0
reserves = get_strategy_reserve_updated()
return shares_to_underlying(vault_shares, reserves)
```

Returns the value of `from`'s shares in underlying units at the latest `b_rate`. The vault uses this in `fetch_strategy_invested_funds` to compute total managed funds.

### 5.7 Keeper management

`lib.rs:341, 364`:

- `set_keeper(new_keeper)` — current keeper auths. Emits `setkeeper` event. (`(old_keeper, new_keeper)` payload.)
- `get_keeper()` — getter.

There is **no** way to set the keeper from the vault. The strategy's keeper is a strategy-local role.

> **Audit note.** A compromised keeper can call `harvest` with `amount_out_min = 0` and sandwich the BLND→asset swap. Mitigation: keeper should be a multisig or a relay account that submits with a sensible `amount_out_min`. The reward_threshold further bounds the impact since harvest is no-op below threshold.

### 5.8 Blend strategy invariants and operational concerns

- The strategy holds the actual bTokens in the Blend pool. The vault never sees bTokens directly.
- The vault is the only depositor address in practice; `vault_shares[vault_addr]` is the only nonzero entry. (Multiple vaults can use the same strategy contract — though that's not the current deployment pattern. If they did, share allocations are isolated per `from` address.)
- The strategy depends on three external contracts: the Blend pool, the BLND token, and the Soroswap router. Misbehavior or upgrade of any of these can break the strategy.
- The `b_rate` is **read** from Blend on every operation; the strategy never assumes the cached rate is fresh.
- The `reward_threshold` is fixed at construction. Changing it requires redeploying the strategy.

---

## 6. Other strategies

All strategies implement `DeFindexStrategyTrait`. The non-Blend ones are largely pass-throughs.

### 6.1 Hodl & UnsafeHodl

`strategies/hodl/src/lib.rs`, `strategies/unsafe_hodl/src/lib.rs`.

Both contracts have **identical** strategy logic: the strategy just holds the underlying asset on its own contract address and tracks `from → balance` in persistent storage. `harvest` is a no-op that emits a zero-amount harvest event for indexer compatibility. `balance(from)` returns the user's tracked balance. No yield is generated; no external protocol is called.

Both list the same `STRATEGY_NAME = "HodlStrategy"` in their event metadata.

> **Audit note.** The differentiation between `hodl` and `unsafe_hodl` is not visible from `lib.rs` source — they look identical. If you're auditing a vault that mounts `unsafe_hodl`, treat it as plain custody; there is no in-code mechanism that distinguishes "unsafe" from "safe" custody.

### 6.2 Fixed APR

`strategies/fixed_apr/src/lib.rs`.

Simulates a fixed APR by accumulating a synthetic yield balance. On every state-changing call, `update_yield_balance(from)` computes:

```
time_elapsed = ledger.timestamp() - last_harvest_time[from]
reward       = user_balance * apr_bps * time_elapsed / (SECONDS_PER_YEAR * 10_000)
```

(`fixed_apr/src/lib.rs:121`.) `harvest` moves the accumulated yield from a "yield balance" into the user's principal balance. `withdraw` does **not** include unharvested yield — the caller must `harvest` first.

> **Audit note.** Fixed APR is a synthetic strategy. The strategy contract must already hold enough underlying to honor withdrawals + yield. There is no funding mechanism. This is typically used for testing or in scenarios where the strategy admin pre-funds it.

### 6.3 Soroswap

`strategies/soroswap/src/lib.rs`.

Provides liquidity to a USDC/XLM Soroswap pair. On deposit, half the USDC is swapped to XLM and both are added as liquidity. On withdraw, liquidity is removed and XLM is swapped back to USDC. Balance is computed by reading the user's LP token balance against the pair's reserves.

> **Audit note.** The USDC and XLM addresses are **hard-coded** in this strategy (`lib.rs:55, 59`). It's a single-pool LP adapter. The strategy is not generic across pools.

### 6.4 Xycloans

`strategies/xycloans/src/lib.rs`.

Wraps an Xycloans flash-loan liquidity pool. Deposit swaps `token_in` → `pool_token` via Soroswap, then deposits into Xycloans. Balance is the user's share + matured fees, valued in `token_in` via Soroswap quotes.

> **Audit note.** Balance valuation uses a **spot reserve quote** (`get_amount_out`) which is manipulable in the same transaction. Consumers reading `vault.fetch_total_managed_funds()` on a Xycloans-backed vault should treat the value as an approximate market price, not an oracle.

---

## 7. Math reference (consolidated)

A single place to find all the formulas with their rounding directions.

### 7.1 Constants

| Symbol | Value | Where |
| --- | --- | --- |
| `SCALAR_BPS` | `10_000` (100% = 10,000 bps) | `vault/src/constants.rs:2` |
| `SCALAR_12` | `1_000_000_000_000` (12 decimal places) | `strategies/blend/src/constants.rs:1` |
| `MINIMUM_LIQUIDITY` (vault) | `1000` shares | `vault/src/lib.rs:51` |
| `MINIMUM_LIQUIDITY` (Blend strategy) | `1000` shares (separate burn) | `strategies/blend/src/reserves.rs:158` |
| `MAX_DEFINDEX_FEE` (factory) | `9000` bps (90%) | `factory/src/constants.rs:1` |
| Vault fee cap | `9000` bps (90%) | `vault/src/storage.rs:111` |
| Protocol fee cap (vault constructor) | `9000` bps (90%) | `vault/src/lib.rs:142` |
| Decimals (vault shares) | `7` | `vault/src/lib.rs:168` |
| `SECONDS_PER_YEAR` (fixed APR) | `31_536_000` | `strategies/fixed_apr/src/constants.rs:2` |

### 7.2 Share mint on single-asset deposit

```
if total_supply == 0:
    Δshares = Δassets
else:
    Δshares = floor( total_supply · Δassets / total_amount )       // floor → existing holders favored
```

(`vault/src/deposit.rs:64`.)

### 7.3 Share mint on multi-asset deposit (enforced-asset method)

For an enforced asset `i`, with all other assets `j ≠ i`:

```
optimal_j = ceil( reserve_j · desired_i / reserve_i )              // ceil → user pays the rounding
Δshares   = floor( total_supply · desired_i / reserve_i )

constraints (must hold for the choice to be feasible):
  for all j ≠ i with reserve_j > 0:
    optimal_j ≤ desired_j  AND  optimal_j ≥ amounts_min_j

first feasible enforced index wins (iteration order = asset registration order)
```

(`vault/src/utils.rs:134–184, 186–242`.)

### 7.4 Asset amount per share (read)

```
amount_i(s) = floor( total_amount_i · s / total_supply )
```

(`vault/src/utils.rs:98–132`.)

### 7.5 Withdraw per asset

```
requested_i = floor( total_amount_i · withdraw_shares / total_supply_before_burn )
```

(`vault/src/lib.rs:340`.)

### 7.6 Fee lock

```
fee_locked = floor( gains_or_losses · vault_fee_bps / 10_000 )       // only if gains_or_losses > 0
locked_fee += fee_locked
gains_or_losses = 0
```

(`vault/src/report.rs:51`.)

### 7.7 Fee distribute

```
defindex_amount = floor( locked_fee · defindex_protocol_rate / 10_000 )
vault_amount    = locked_fee - defindex_amount
```

(`vault/src/report.rs:189`.)

### 7.8 Blend underlying ↔ bTokens

```
bTokens   = floor( underlying · SCALAR_12 / b_rate )      // on deposit (utils.rs:41)
bTokens   = ceil ( underlying · SCALAR_12 / b_rate )      // on withdraw (utils.rs:75)
underlying= floor( bTokens · b_rate / SCALAR_12 )         // on both (reserves.rs:54)
```

### 7.9 Blend bTokens ↔ strategy shares

```
shares    = floor( bTokens · total_shares / total_b_tokens )    // deposit (reserves.rs:26)
shares    = ceil ( bTokens · total_shares / total_b_tokens )    // withdraw (reserves.rs:37)
bTokens   = floor( shares · total_b_tokens / total_shares )     // both (reserves.rs:47)
```

### 7.10 Blend optimal deposit / withdraw (composite)

Deposit (`strategies/blend/src/utils.rs:36`):

```
b_tokens_minted    = floor( amount · SCALAR_12 / b_rate )
shares_minted      = floor( b_tokens_minted · total_shares / total_b_tokens )         (or = b_tokens_minted if first)
optimal_b_tokens   = ceil ( shares_minted · total_b_tokens / total_shares )
optimal_deposit    = ceil ( optimal_b_tokens · b_rate / SCALAR_12 )
```

Withdraw (`strategies/blend/src/utils.rs:71`):

```
b_tokens_burnt     = ceil ( withdraw · SCALAR_12 / b_rate )
shares_burnt       = ceil ( b_tokens_burnt · total_shares / total_b_tokens )
optimal_b_tokens   = floor( shares_burnt · total_b_tokens / total_shares )
optimal_withdraw   = floor( optimal_b_tokens · b_rate / SCALAR_12 )
```

### 7.11 Fixed APR yield

```
reward = floor( user_balance · apr_bps · time_elapsed / (SECONDS_PER_YEAR · 10_000) )
```

(`strategies/fixed_apr/src/lib.rs:121`.)

### 7.12 Worked example — vault share dilution from yield

**Setup.** Single-asset USDC vault, 1 strategy. State at t₀:

- `total_supply = 1_000_000` shares
- `total_amount = 1_000_000` USDC
- `share_price = 1.0`

A yield event credits 100,000 USDC of strategy gain. The strategy reports `balance(vault) = 1_100_000`.

**On the next deposit, withdraw, or `lock_fees`**, the vault:

1. Reads strategy balance: 1_100_000.
2. `report(1_100_000)`: `gains_or_losses += 1_100_000 - 1_000_000 = 100_000`.
3. `lock_fee(vault_fee_bps=2000 [20%])`: `locked_fee += 100_000 * 2000 / 10_000 = 20_000`, `gains_or_losses = 0`.
4. `invested_funds = balance - locked_fee = 1_100_000 - 20_000 = 1_080_000`.

A user holding 1 share now has `floor(1_080_000 · 1 / 1_000_000) = 1` USDC. The "yield" passes through as the share price rising from 1.0 to 1.08 effective.

If `defindex_protocol_rate = 1000` (10%), on `distribute_fees`:
- `defindex_amount = 20_000 · 1000 / 10_000 = 2_000` to protocol.
- `vault_amount   = 20_000 - 2_000 = 18_000` to vault fee receiver.

### 7.13 Worked example — first-depositor + MINIMUM_LIQUIDITY

User deposits 10,000 USDC to an empty single-asset vault.

- `shares_to_mint = 10_000` (since `total_supply == 0`, branch in `deposit.rs:32`).
- `mint_shares`: `total_supply == 0`, `shares_to_mint > 1000`, so:
  - Mint 1,000 shares to the vault contract address (locked forever).
  - Mint 9,000 shares to the depositor.
- After: `total_supply = 10_000`, depositor has 9,000, vault has 1,000.

Share price right after: `total_amount / total_supply = 10_000 / 10_000 = 1.0` USDC/share.

If the next depositor adds 1,000 USDC:

- `Δshares = floor(10_000 · 1_000 / 10_000) = 1_000`.
- `total_supply = 11_000`, second depositor has 1,000.
- `share_price = 11_000 / 11_000 = 1.0`. Consistent.

---

## 8. Roles, lifecycle, upgrade

### 8.1 Role matrix

| Role | Setter | Powers |
| --- | --- | --- |
| **Factory Admin** | `factory.set_new_admin` (current admin) | Update protocol fee, fee receiver, vault WASM hash, transfer admin. |
| **Manager** (vault) | `vault.set_manager` (current Manager) | All admin-style vault calls: change emergency manager, change rebalance manager, change fee receiver, upgrade, distribute fees, lock fees, release fees, rebalance, pause/unpause/rescue. |
| **Emergency Manager** (vault) | `vault.set_emergency_manager` (Manager) | `rescue`, `pause_strategy`, `unpause_strategy`. |
| **Rebalance Manager** (vault) | `vault.set_rebalance_manager` (Manager) | `rebalance` only. |
| **Vault Fee Receiver** | `vault.set_fee_receiver` (Manager OR current Fee Receiver) | Receive vault-portion fees on `distribute_fees`. Can transfer the role to itself or to any address. |
| **DeFindex Protocol Receiver** | (set on the vault at construction; only changeable via upgrade) | Receive protocol-portion fees. |
| **Strategy Keeper** (Blend strategy only) | `BlendStrategy.set_keeper` (current keeper) | Call `harvest`. |

### 8.2 Lifecycle (per vault)

```
   1. Factory.create_defindex_vault[_deposit](...)
        → __constructor on the vault
        → roles set, assets validated, vault token initialized
   2. Users call vault.deposit (invest=false → idle, or invest=true → auto-allocate)
   3. Manager / RebalanceManager call vault.rebalance to move funds
   4. Strategies accrue yield over time
   5. On every deposit/withdraw/rebalance, fees are locked into Reports
   6. Manager or VaultFeeReceiver call vault.distribute_fees periodically
   7. Users call vault.withdraw to redeem shares
   8. Optional: EmergencyManager calls vault.rescue on a misbehaving strategy
   9. Optional: Manager calls vault.upgrade (if upgradable=true at constructor)
```

### 8.3 Upgrade semantics

- The vault upgrade calls `update_current_contract_wasm` — same WASM hash format the factory uses.
- **The constructor is NOT re-executed.** All instance/persistent storage carries over.
- There is no `migrate()` hook. Any storage layout change must be backward compatible.
- `is_upgradable()` returns `true` if the key is missing (`storage.rs:177`). For maximum safety, a vault deployer should explicitly set `upgradable=false` at construction. Once `Upgradable=false`, it cannot be turned back on (no setter exists).
- The factory itself is **not** upgradable. Replacing the factory means redeploying it.

### 8.4 Initialization fragility

- A vault deployed via `create_defindex_vault` (no initial deposit) starts with `total_supply = 0`. The first depositor faces the multi-asset-empty-vault branch where shares = sum of amounts; this defines the ratio (§3.4.2).
- A vault deployed via `create_defindex_vault_deposit` makes that first deposit atomically with the deployer's funds and `amounts_min = 0`. This pattern is documented as the "bootstrap deposit" defense against inflation attacks; the deployer should burn or hold these shares deliberately.

---

## 9. Errors

### 9.1 Factory errors (`factory/src/error.rs`)

| Code | Name |
| --- | --- |
| 401 | NotInitialized |
| 404 | AssetLengthMismatch |
| 405 | IndexDoesNotExist |
| 406 | FeeTooHigh |

### 9.2 Vault errors (`vault/src/error.rs`)

| Code | Name | Notes |
| --- | --- | --- |
| 100 | NotInitialized | Storage missing. |
| 101 | InvalidRatio | (declared, used in older paths) |
| 102 | StrategyDoesNotSupportAsset | Constructor's strategy.asset() != asset.address. |
| 103 | NoAssetAllocation | Empty assets vector. |
| 104 | RolesIncomplete | Constructor roles map missing a key. |
| 105 | MetadataIncomplete | name or symbol missing in `name_symbol`. |
| 106 | MaximumFeeExceeded | vault fee or protocol rate > 9000 bps. |
| 107 | DuplicatedAsset | Same asset address appears twice. |
| 108 | DuplicatedStrategy | Same strategy address appears twice within one asset. |
| 110 | AmountNotAllowed | Negative amounts, zero withdraw_shares. |
| 111 | InsufficientBalance | Token spend exceeds vault token balance. |
| 112 | WrongAmountsLength | `amounts_desired` or `amounts_min` length ≠ assets length. |
| 113 | WrongLockedFees | `locked_fee > strategy_balance` (corrupted state). |
| 114 | InsufficientManagedFunds | (declared) |
| 115 | MissingInstructionData | (declared) |
| 116 | UnsupportedAsset | rebalance swap touches non-vault asset. |
| 117 | InsufficientAmount | shares_to_mint ≤ MINIMUM_LIQUIDITY on first deposit, or 0 in mint. |
| 118 | NoOptimalAmounts | Multi-asset deposit has no feasible allocation. |
| 119 | WrongInvestmentLength | (declared) |
| 120 | ArithmeticError | Overflow/underflow on checked op. |
| 121 | Overflow | (specific overflow path) |
| 122 | WrongAssetAddress | (declared) |
| 123 | WrongStrategiesLength | (declared) |
| 124 | AmountOverTotalSupply | withdraw_shares > total_supply. |
| 125 | NoInstructions | rebalance called with empty instructions vector. |
| 126 | NotUpgradable | upgrade attempted on a non-upgradable vault. |
| 127 | Underflow | (specific underflow path) |
| 128 | UnwindMoreThanAvailable | rebalance Unwind > strategy invested. |
| 129 | InsufficientFeesToRelease | release_fee amount > locked_fee. |
| 130 | Unauthorized | Caller is not in any required role. |
| 131 | RoleNotFound | Role key not set in storage. |
| 132 | ManagerNotInQueue | (declared, two-step manager not implemented) |
| 133 | SetManagerBeforeTime | (declared) |
| 134 | QueueEmpty | (declared) |
| 140 | StrategyNotFound | Strategy address not in any asset's strategy list. |
| 141 | StrategyPausedOrNotFound | (declared) |
| 142 | StrategyWithdrawError | strategy.withdraw try-failure. |
| 143 | StrategyInvestError | strategy.deposit try-failure. |
| 144 | StrategyPaused | Cannot Invest into a paused strategy. |
| 150 | AssetNotFound | (declared) |
| 151 | NoAssetsProvided | (declared) |
| 160 | InsufficientOutputAmount | withdraw `requested[i] < min_amounts_out[i]`. |
| 161 | ExcessiveInputAmount | swap `amount_in > amount_in_max`. |
| 162 | InvalidFeeBps | (declared) |
| 190 | LibrarySortIdenticalTokens | Soroswap library error pass-through. |
| 200 | SoroswapRouterError | router_pair_for failure. |
| 201 | SwapExactInError | swap_exact_tokens_for_tokens failure. |
| 202 | SwapExactOutError | swap_tokens_for_exact_tokens failure. |

### 9.3 Strategy errors (`strategies/core/src/error.rs`) — see §4.2 above.

---

## 10. Events

### 10.1 Factory events

Topic = `"DeFindexFactory"`, then a symbol:

| Symbol | Payload |
| --- | --- |
| `create` | `CreateDeFindexEvent { roles, vault_fee, assets }` |
| `nadmin` | `NewAdminEvent { new_admin }` |
| `nreceiver` | `NewDeFindexReceiverEvent { new_defindex_receiver }` |
| `n_fee` | `NewFeeRateEvent { new_defindex_fee }` |
| `n_wasm` | `NewVaultWasmHashEvent { new_vault_wasm_hash }` |

### 10.2 Vault events

Topic = `"DeFindexVault"`:

| Symbol | Payload | Emitted by |
| --- | --- | --- |
| `deposit` | `VaultDepositEvent { depositor, amounts, df_tokens_minted, total_supply_before, total_managed_funds_before }` | `vault.deposit` |
| `withdraw` | `VaultWithdrawEvent { withdrawer, df_tokens_burned, amounts_withdrawn, total_supply_before, total_managed_funds_before }` | `vault.withdraw` |
| `rescue` | `EmergencyWithdrawEvent { caller, strategy_address, amount_withdrawn }` | `vault.rescue` |
| `paused` | `StrategyPausedEvent { strategy_address, caller }` | `vault.pause_strategy`, `vault.rescue` |
| `unpaused` | `StrategyUnpausedEvent { strategy_address, caller }` | `vault.unpause_strategy` |
| `nreceiver` | `FeeReceiverChangedEvent { new_fee_receiver, caller }` | `vault.set_fee_receiver` |
| `nmanager` | `ManagerChangedEvent { new_manager }` | `vault.set_manager` |
| `nemanager` | `EmergencyManagerChangedEvent { new_emergency_manager }` | `vault.set_emergency_manager` |
| `rbmanager` | `RebalanceManagerChangedEvent { new_rebalance_manager }` | `vault.set_rebalance_manager` |
| `dfees` | `FeesDistributedEvent { distributed_fees: Vec<(asset, amount)> }` | `vault.distribute_fees`, `vault.rescue` |
| `rebalance` (sub-method = `unwind`) | `UnwindEvent { call_params, rebalance_method, report }` | rebalance Unwind |
| `rebalance` (sub-method = `invest`) | `InvestEvent { asset_investments, rebalance_method, report }` | rebalance Invest |
| `rebalance` (sub-method = `SwapEIn`) | `SwapExactInEvent { swap_args, rebalance_method }` | rebalance SwapExactIn |
| `rebalance` (sub-method = `SwapEOut`) | `SwapExactOutEvent { swap_args, rebalance_method }` | rebalance SwapExactOut |

The vault token also emits the standard SEP-41 token events: `mint`, `burn`, `transfer`, `approve` — fired from `TokenUtils::events()` calls in `vault/src/token/contract.rs`.

### 10.3 Strategy events

Topic = strategy name (e.g. `"BlendStrategy"`, `"HodlStrategy"`, `"FixAprStrategy"`):

| Symbol | Payload |
| --- | --- |
| `deposit` | `DepositEvent { amount, from }` |
| `withdraw` | `WithdrawEvent { amount, from }` |
| `harvest` | `HarvestEvent { amount, from, price_per_share }` — `price_per_share` is `shares_to_underlying(SCALAR_12, reserves)` in Blend (i.e. the price for 1.0 share scaled by `10^12`). For non-Blend strategies it is `0`. |

Blend additionally emits `setkeeper` with `(old_keeper, new_keeper)` on `set_keeper`.

---

## 11. Known limitations and trust assumptions

### 11.1 First-depositor / share-price inflation attack

The vault burns `MINIMUM_LIQUIDITY = 1000` shares on the first deposit (§3.4.3). For a 7-decimal share token, this is a `0.0001` share floor — enough to make the classic donation attack (deposit 1 wei, donate Y assets, dilute next depositor) unprofitable for large donations but **not** impossible for tiny ones.

The deployment scripts at `apps/contracts/src/strategies/deploy_blend.ts` make an automatic bootstrap deposit of ~1001 stroops (`README.md:204`). Combined with the in-strategy 1000-share burn (§5.3) this gives two layers of dilution defense, but the **economic** safety still depends on the deployer making a non-trivial first deposit.

> **Audit guidance.** Confirm that any vault the consumer protocol depends on was deployed with a meaningful bootstrap deposit. A "fresh" vault (total_supply close to MINIMUM_LIQUIDITY) is vulnerable.

### 11.2 Idle-funds drag and share-price discontinuity

Funds sitting idle in the vault don't earn yield. If the manager doesn't promptly `rebalance Invest`, depositors share a lower effective APY than the underlying strategies. Conversely, a manager that batches deposits idle and then floods a strategy in one tx might create a momentary share-price tick when fees lock.

`deposit(invest=true)` mitigates this by auto-allocating funds at the current strategy ratio (`vault/src/investment.rs:82`). However:

- The ratio is preserved, not optimized.
- Paused strategies are skipped — funds intended for a paused strategy stay idle.

### 11.3 Loss accounting is one-directional in `lock_fee`

`Report.lock_fee` only locks fees on positive `gains_or_losses` and resets to 0 (§3.5.2). A strategy that gains, locks a fee, then loses the same amount leaves a "phantom" locked fee. The `release_fee` flow exists for the Manager to manually return that fee, but it requires off-chain intervention. There is no automatic loss netting.

> **Audit guidance.** If the consumer's accounting depends on `invested_amount` matching `strategy.balance(vault) - locked_fee` reflecting true economic value, be aware that `locked_fee` may overstate fees in loss scenarios.

### 11.4 Reading `fetch_total_managed_funds` is not free

The public read endpoint (`vault.fetch_total_managed_funds`) calls `lock_fees=false` (`vault/src/lib.rs:569`) but **still** does cross-contract calls into every strategy to read `strategy.balance(vault)`. This is the cost of accurate quoting. The trade-off:

- `get_asset_amounts_per_shares` calls `lock_fees=true` — mutating state. Calling this in a view path will write Reports.
- For a pure preview, build it from `fetch_total_managed_funds()` (pre-lock) yourself.

### 11.5 Soroswap router is a trusted dependency

The vault stores a `SoroswapRouter` address at construction and uses it for rebalance swaps. The Blend strategy stores its own `router` in `Config`. There is no validation of the router's authenticity beyond "the address exists and the calls don't panic". A malicious or upgraded router could:

- Misprice swaps during rebalance — bounded by user-supplied `amount_out_min` / `amount_in_max` / `deadline`.
- Steal the input during harvest reinvest — bounded by Blend strategy's `amount_out_min` (from the `data` argument).

### 11.6 Strategy trust = vault trust

The vault reads `strategy.balance(vault)` and trusts the answer. A bug or hostile upgrade in a strategy that:

- Over-reports balance → inflates vault share price → depositors lose, early withdrawers gain.
- Under-reports → deflates share price → depositors gain, locked withdrawers lose.
- Reverts on `withdraw` → vault rebalance/withdraw paths fail (recoverable via `rescue`).

The `rescue` path lets EmergencyManager extract remaining funds and pause the strategy, but cannot undo accounting changes already realized through deposit/withdraw.

### 11.7 Centralization vectors

- **Manager** can change all other roles, upgrade the vault, swap arbitrarily between vault assets, and unwind any strategy. A compromised Manager can drain the vault into idle and then... well, can't pay it out directly (no admin-withdraw exists), but can unwind, swap to a single asset, and let users withdraw on a skewed ratio.
- **RebalanceManager** alone can swap and rebalance — same surface but without the role-changing powers.
- **Emergency Manager** can rescue (pull strategy → idle) and pause, but cannot move funds out of the vault directly.
- **Factory admin** cannot affect existing vaults' fee or receiver, only future deploys. (See §2.2.)
- **DeFindex protocol receiver** is set at vault construction and **cannot be changed** post-deploy (no setter; only re-settable via WASM upgrade if the storage key is rewritten — which would be a non-standard migration). Worth re-verifying in any specific deployment.

### 11.8 Reentrancy

The vault is single-threaded (Soroban's execution model is sequential), but it makes cross-contract calls to strategies and the Soroswap router during deposit, withdraw, and rebalance. Strategies in turn call into Blend / Soroswap. There is no explicit reentrancy lock in the vault. Theoretical reentrancy via a malicious strategy is possible but bounded by:

- Soroban's host-side authorization: a strategy can only call back into the vault with the auth it was granted (none, by default).
- The vault's state mutations are well-ordered (e.g. `internal_burn` runs before `strategy.withdraw` in `vault.withdraw` — see `vault/src/lib.rs:330` then `lib.rs:390`).

### 11.9 Floor rounding of withdraw can underpay the last withdrawer

Standard ERC-4626 behavior: the last withdrawer of an asset gets `floor(total_amount · 1 / 1) = total_amount`, but any rounding dust accumulated over previous withdraws remains in the vault. For a vault that drains to zero shares but holds tiny dust, the dust is unrecoverable.

### 11.10 No oracle / no price-based math

The vault does not consult any price oracle. All share-price math is in terms of the assets' own units. The vault is asset-agnostic for math purposes. Consumers that want a USD-denominated NAV must derive it externally from each asset's `total_amount`.

### 11.11 Vault fee can be changed mid-flight

`lock_fees(new_fee_bps: Option<u32>)` (`vault/src/lib.rs:972`) lets the Manager change the vault fee and immediately lock at the new rate. This is the only place the vault fee can be changed. There is no advance notice / time delay. Depositors at t₀ may face a fee rate increase at t₁.

> **Audit guidance.** A consumer protocol whose users may have a "guaranteed APY" expectation should monitor the vault fee changes (event `dfees` is per-strategy; vault fee changes happen only via `lock_fees` which doesn't emit a fee-changed event explicitly — only the reports). Consider a notification path off-chain.

### 11.12 `get_asset_amounts_per_shares` and `fetch_total_managed_funds(true)` are side-effecting

Reading these endpoints from a consumer's read path can:

- Update Reports (cost the consumer extra TX fees).
- Lock fees (irreversibly move value from `gains_or_losses` to `locked_fee`).
- Extend persistent TTL on the Report keys.

If the consumer must integrate one of these in a high-frequency read path, it should batch or cache off-chain.

---

## Appendix A — File map (where to look)

```
apps/contracts/
├── common/src/
│   ├── lib.rs                — module index
│   ├── models.rs             — Strategy, AssetStrategySet
│   └── utils.rs              — StringExtensions::concat
├── factory/src/
│   ├── lib.rs                — entrypoints, __constructor, create_defindex_vault*
│   ├── constants.rs          — MAX_DEFINDEX_FEE = 9000
│   ├── error.rs              — FactoryError
│   ├── events.rs             — Factory events
│   ├── storage.rs            — DataKey + getters/setters
│   └── vault.rs              — create_contract (deploy_v2 wrapper)
├── vault/src/
│   ├── lib.rs                — three traits + impls (the entrypoints)
│   ├── interface.rs          — VaultTrait, AdminInterfaceTrait, VaultManagementTrait
│   ├── constants.rs          — SCALAR_BPS = 10_000
│   ├── access.rs             — RolesDataKey, AccessControl
│   ├── deposit.rs            — process_deposit, mint_shares
│   ├── error.rs              — ContractError (all vault errors)
│   ├── events.rs             — Vault events
│   ├── funds.rs              — idle / invested / total managed funds
│   ├── investment.rs         — generate_investment_allocations
│   ├── models.rs             — Instruction, AssetInvestmentAllocation, etc.
│   ├── report.rs             — Report, lock_fee, release_fee, distribute_strategy_fees
│   ├── router.rs             — Soroswap integration for rebalance swaps
│   ├── storage.rs            — DataKey + helpers + Report storage
│   ├── strategies.rs         — strategy client helpers, invest/unwind, pause/unpause
│   ├── token/
│   │   ├── contract.rs       — VaultToken impl (SEP-41 + total_supply)
│   │   ├── balance.rs        — balance read/write (persistent)
│   │   ├── total_supply.rs   — total supply (instance)
│   │   ├── allowance.rs      — allowance
│   │   ├── metadata.rs       — name/symbol/decimal
│   │   └── storage_types.rs  — TTLs
│   └── utils.rs              — multi-asset share math, validate_assets
├── strategies/
│   ├── core/src/
│   │   ├── lib.rs            — DeFindexStrategyTrait
│   │   ├── error.rs          — StrategyError
│   │   └── event.rs          — deposit/withdraw/harvest events
│   ├── blend/src/
│   │   ├── lib.rs            — BlendStrategy impl
│   │   ├── blend_pool.rs     — Blend pool client + supply/withdraw/claim/perform_reinvest
│   │   ├── reserves.rs       — StrategyReserves + share math
│   │   ├── soroswap.rs       — internal_swap_exact_tokens_for_tokens
│   │   ├── storage.rs        — Config, vault shares, keeper
│   │   ├── utils.rs          — calculate_optimal_deposit/withdraw_amount, shares_to_underlying
│   │   └── constants.rs      — SCALAR_12 = 10^12
│   ├── hodl/src/lib.rs       — pure custody strategy
│   ├── unsafe_hodl/src/lib.rs — same code as hodl
│   ├── fixed_apr/src/lib.rs  — synthetic APR strategy
│   ├── soroswap/src/lib.rs   — Soroswap LP strategy (hard-coded USDC/XLM)
│   └── xycloans/src/lib.rs   — Xycloans flash-loan-pool strategy
```

---

## Appendix B — Quick-reference: deposit and withdraw sequence diagrams

### B.1 `deposit(amounts_desired, amounts_min, from, invest=true)` — multi-asset

```
caller ──require_auth──► vault.deposit
                            │
                            ├─► fetch_total_managed_funds(lock_fees=true)
                            │     ├─ for every strategy:
                            │     │   ├─ strategy.balance(vault)        ← cross-contract
                            │     │   ├─ Report.report(balance)
                            │     │   ├─ Report.lock_fee(vault_fee_bps)
                            │     │   └─ invested -= locked_fee
                            │     └─ returns Vec<CurrentAssetInvestmentAllocation>
                            │
                            ├─► process_deposit
                            │     ├─ calculate_deposit_amounts_and_shares_to_mint (multi-asset)
                            │     ├─ for each asset i: token.transfer(from → vault, amounts[i])
                            │     └─ mint_shares(from, shares_to_mint)
                            │           └─ (if first deposit) mint 1000 to vault, rest to from
                            │
                            ├─► emit VaultDepositEvent
                            │
                            └─ if invest:
                                  ├─ generate_investment_allocations(total_managed_funds, amounts)
                                  └─ for each strategy in each non-paused allocation:
                                       └─► invest_in_strategy
                                             ├─ authorize_as_current_contract(asset.transfer)
                                             ├─ strategy.deposit(amount, vault) ← cross-contract
                                             └─ Report.gains_or_losses, prev_balance updated
```

### B.2 `withdraw(withdraw_shares, min_amounts_out, from)`

```
caller ──require_auth──► vault.withdraw
                            │
                            ├─► fetch_total_managed_funds(lock_fees=true)   [locks fees]
                            │
                            ├─► validate min_amounts_out length & non-negative
                            ├─► require withdraw_shares ≤ total_supply
                            ├─► capture total_shares_supply (before burn)
                            │
                            ├─► internal_burn(from, withdraw_shares)        [BURN FIRST]
                            │
                            └─ for each asset i:
                                  ├─ requested[i] = floor(total[i] · shares / supply_before)
                                  ├─ require requested[i] ≥ min_amounts_out[i]
                                  ├─ if idle[i] ≥ requested[i]:
                                  │     └─ token.transfer(vault → from, requested[i])
                                  │
                                  └─ else:
                                       ├─ token.transfer(vault → from, idle[i])
                                       ├─ remaining = requested[i] - idle[i]
                                       └─ for each strategy j (last one absorbs remainder):
                                            ├─ amount = (j is last) ? remaining_for_asset : pro_rata
                                            ├─ strategy.withdraw(amount, vault, from)  ← cross-contract
                                            └─ Report.prev_balance updated
                            │
                            └─► emit VaultWithdrawEvent
```

---

*Doc generated from contract source at the repository root. For up-to-date behavior, always verify against the deployed WASM hash — the factory's `vault_wasm_hash()` getter returns the canonical hash currently used for new deploys, but existing vaults use the hash that was active when they were created.*
