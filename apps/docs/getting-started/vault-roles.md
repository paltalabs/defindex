---
cover: ../../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
---

# Vault Roles

**Roles** are unique identifiers that assign specific responsibilities within the vault and are the only entities with privileges to perform critical actions. Each role is associated with an `Address` that represents the entity responsible for that function. None of these roles can withdraw funds from the users.

Also, when deploying a vault, the deploying address can be any address — it doesn’t need to be tied to the Manager or any other role. In other words, a vault can be set up on behalf of someone else.

These are:

* **Vault Manager**
  * Primary owner of the vault
  * Controls vault settings, role assignments, and upgrades (if enabled)
  * Can execute functions from other roles
  * _Recommendation_: Use a multisig wallet
* **Rebalance Manager**
  * Allocates funds across strategies
  * Optimizes distribution for performance
  * _Recommendation_: Implement as an automated bot or delegate it
* **Fee Receiver**
  * Collects strategy performance fees
  * _Recommendation_: Use a secure, dedicated wallet
* **Emergency Manager**
  * Handles emergency fund recovery
  * Can pause/unwind risky strategies
  * _Recommendation_: Implement as an automated bot or delegate it

[Go back to Wallet Dev and Vault Manager docs](broken-reference)
