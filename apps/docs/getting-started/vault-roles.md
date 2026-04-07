---
cover: ../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
description: ⏱️ 2 min read
---

# Vault Roles

**Roles** are unique identifiers that assign specific responsibilities within the vault and are the only entities with privileges to perform critical actions. Each role is associated with an `Address` that represents the entity responsible for that function. None of these roles can withdraw funds from the users.

Since each role is just an `Address`, any role can be assigned to a **smart contract** instead of a regular wallet. This enables policy-based or role-access control patterns — for example, a contract acting as the Manager could define its own internal rules, conditions, or sub-roles to govern who is allowed to trigger actions on behalf of that role.

Also, when deploying a vault, the deploying address can be any address — it doesn't need to be tied to the Manager or any other role. In other words, a vault can be set up on behalf of someone else.

The roles are:

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
  * Receives fees collected by the vault
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
| Receive fees | — | — | — | ✅ |
| Distribute fees | ✅ | — | — | ✅ |
| Lock / release fees | ✅ | — | — | — |
| Upgrade contract code | ✅ | — | — | — |
| Change Manager | ✅ | — | — | — |
| Change Emergency Manager | ✅ | — | — | — |
| Change Rebalance Manager | ✅ | — | — | — |
| Change Fee Receiver | ✅ | — | — | ✅ |

## Protocol Fee Receiver

In addition to the vault roles above, each vault also stores a **DeFindex Protocol Fee Receiver** (`DeFindexProtocolFeeReceiver`). This is not a vault role — it cannot call any function on the vault. It is a passive recipient address set at vault initialization that automatically receives a portion of the fees whenever `distribute_fees` is called. The split between the Protocol Fee Receiver and the Vault Fee Receiver is determined by the `DeFindexProtocolFeeRate` (in basis points), also set at initialization and fixed thereafter.

## Role Assignment and Updates

Roles are set at deployment and can be updated afterward by calling the corresponding setter function. Only the Manager can change most roles — the Fee Receiver address is the only one that either the Manager or the current Fee Receiver can update.

| Role | Who can change it |
|------|------------------|
| Manager | Manager |
| Emergency Manager | Manager |
| Rebalance Manager | Manager |
| Fee Receiver | Manager or Fee Receiver |