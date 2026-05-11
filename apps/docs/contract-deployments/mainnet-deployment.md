---
cover: ../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
description: ⏱️ 2 min read
---

# Mainnet Deployment

This page contains the current mainnet contract addresses for the DeFindex protocol.

For new and updated deployments, check\
[https://github.com/paltalabs/defindex/blob/main/public/mainnet.contracts.json](../../../public/mainnet.contracts.json)\


## Core Contracts

### Factory Contract

* **Contract ID**: `CDKFHFJIET3A73A2YN4KV7NSV32S6YGQMUFH3DNJXLBWL4SKEGVRNFKI`
* **Hash**: `b0fe36b2b294d0af86846ccc4036279418907b60f6f74dae752847ae9d3bca0e`

### Vault Contract

* **Hash**: `ae3409a4090bc087b86b4e9b444d2b8017ccd97b90b069d44d005ab9f8e1468b`

## Strategy Contracts

### Fixed Pool Strategies (with Autocompound)

#### USDC Strategy

* **Contract ID**: `CDB2WMKQQNVZMEBY7Q7GZ5C7E7IAFSNMZ7GGVD6WKTCEWK7XOIAVZSAP`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### EURC Strategy

* **Contract ID**: `CC5CE6MWISDXT3MLNQ7R3FVILFVFEIH3COWGH45GJKL6BD2ZHF7F7JVI`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### XLM Strategy

* **Contract ID**: `CDPWNUW7UMCSVO36VAJSQHQECISPJLCVPDASKHRC5SEROAAZDUQ5DG2Z`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

### Etherfuse Pool v2 Strategies (with Autocompound)

#### USDC Strategy

* **Contract ID**: `CCBTSHPUVNKCT5V675AAVYNANHXBU26PTZK2QLS7ZLFNYRJZT5HW3VL6`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### CETES Strategy

* **Contract ID**: `CAZ3LLLKPWEOVK6K4G5NCQ2VXWABLFIPKKNMN5GLKMZKEN7JSKTEMIKN`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### USTRY Strategy

* **Contract ID**: `CA3SO5RRKOONAPWVR5XY6CMOYZGN4M4QKVIGX5DFRIIJUJW2SFSELBXL`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### TESOURO Strategy

* **Contract ID**: `CDSCVJHJWUZQMR64FVK3XMND5NKSN7Z23KPRCHKFHVGOEJBWPVH5B5XA`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

## Important Notes

* All strategy contracts share the same hash as they are instances of the same contract template
* The factory contract is used to deploy new vaults
* The vault contract hash is used to verify vault deployments
* Always verify contract addresses before interacting with them
