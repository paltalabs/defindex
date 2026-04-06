---
cover: ../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
description: ⏱️ 2 min read
---

# Vault Roles

**Roles** are unique identifiers that assign specific responsibilities within the vault and are the only entities with privileges to perform critical actions. Each role is associated with an `Address` that represents the entity responsible for that function. None of these roles can withdraw funds from the users.

Also, when deploying a vault, the deploying address can be any address — it doesn't need to be tied to the Manager or any other role. In other words, a vault can be set up on behalf of someone else.

All four roles **must** be assigned at deployment — none are optional. If any role address is missing, the contract will reject the deployment with a `RolesIncomplete` error.

These are:

* **Vault Manager** (`Manager`)
  * Primary owner of the vault
  * Controls vault settings, role assignments, and contract upgrades
  * Is included in the authorization check for every role-restricted function — can perform any action that Emergency Manager, Rebalance Manager, or Fee Receiver can perform, without needing to hold those roles
  * The only role that can lock or release fees and upgrade the contract code
  * Can update any role address, including its own
  * _Recommendation_: Use a multisig wallet
* **Rebalance Manager** (`RebalanceManager`)
  * Executes rebalancing instructions that move funds across strategies
  * _Recommendation_: Implement as an automated bot or delegate it
* **Fee Receiver** (`VaultFeeReceiver`)
  * Triggers distribution of already-locked fees to vault and protocol receivers by calling `distribute_fees`
  * Can also update the fee receiver address (shared with Manager)
  * Cannot lock or release fees — that is Manager-only
  * _Recommendation_: Use a secure, dedicated wallet
* **Emergency Manager** (`EmergencyManager`)
  * Can withdraw all funds from a specific strategy and automatically pause it (`rescue`)
  * Can pause a specific strategy, blocking deposits and withdrawals to it
  * Can unpause a specific strategy
  * Cannot access the vault balance or withdraw user funds directly
  * _Recommendation_: Implement as an automated bot or delegate it

## Role Permissions

| Action | Manager | Emergency Manager | Rebalance Manager | Fee Receiver |
|--------|:-------:|:-----------------:|:-----------------:|:------------:|
| Rescue assets from strategy | ✅ | ✅ | — | — |
| Pause strategy | ✅ | ✅ | — | — |
| Unpause strategy | ✅ | ✅ | — | — |
| Rebalance across strategies | ✅ | — | ✅ | — |
| Distribute fees | ✅ | — | — | ✅ |
| Lock / release fees | ✅ | — | — | — |
| Upgrade contract code | ✅ | — | — | — |
| Change Manager | ✅ | — | — | — |
| Change Emergency Manager | ✅ | — | — | — |
| Change Rebalance Manager | ✅ | — | — | — |
| Change Fee Receiver | ✅ | — | — | ✅ |

## Role Assignment and Updates

Roles are set at deployment and can be updated afterward by calling the corresponding setter function. Only the Manager can change most roles — the Fee Receiver address is the only one that either the Manager or the current Fee Receiver can update.

| Role | Who can change it |
|------|------------------|
| Manager | Manager |
| Emergency Manager | Manager |
| Rebalance Manager | Manager |
| Fee Receiver | Manager or Fee Receiver |