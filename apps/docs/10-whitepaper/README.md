---
cover: ../.gitbook/assets/image 31.png
coverY: 0
---

# Whitepaper

This protocol has been designed and developed by PaltaLabs

Francisco Catrileo | Joaquin Soza | Esteban Iglesias

### Abstract

DeFindex is a suite of smart contracts designed to facilitate interaction with various Decentralized Finance (DeFi) protocols on the Stellar/Soroban Blockchain. It enables users to create custom strategies, allowing investments to be distributed across multiple DeFi protocols in a streamlined manner. The protocol serves two primary audiences:

1. **Wallet Users (including Web2 users):** DeFindex provides a simplified interface that wallet developers can integrate into their platforms, enabling users to access DeFi investment services effortlessly.
2. **Expert Users:** For experienced investors, DeFindex offers an efficient way to diversify investments without the complexity of building and managing their own strategies.

Inspired by projects such as Yearn, Set Protocol, Compound, and YieldYak, DeFindex adapts their core principles to the Stellar ecosystem.

The protocol comprises three main components:

1. **Factory:** A smart contract responsible for creating new Vaults.
2. **Vaults:** The primary contracts through which users interact, enabling deposits, withdrawals, and position adjustments.
3. **Strategies:** Contracts that allocate Vault assets across various DeFi protocols.

To ensure robust functionality and security, DeFindex implements a role-based management system:

* **Manager:** Oversees strategies and the assets within Vaults.
* **Emergency Manager:** Handles rescues.
* **Fee Receiver:** Collects and manages strategy-related fees.

By combining simplicity for newcomers with advanced features for seasoned users, DeFindex aims to make DeFi more accessible and efficient on the Stellar Blockchain.

### Introduction

* [Introduction](../whitepaper/10-whitepaper/01-introduction.md)
* [Core Concepts](../whitepaper/10-whitepaper/02-core-concepts.md)

### The DeFindex Approach

* [Overview](03-the-defindex-approach/)
* [Design Decisions](03-the-defindex-approach/01-design-decisions.md)

### Contracts

* [Vault Contract](../whitepaper/10-whitepaper/02-contracts/01-vault-contract.md)
* [Strategy Contract](../whitepaper/10-whitepaper/02-contracts/02-strategy-contract.md)
* [Zapper Contract](../whitepaper/10-whitepaper/02-contracts/02-zapper-contract.md)

### State of the Art

* [State of the Art](04-state-of-the-art/)

### Appendix

* [Appendix](05-appendix/)
