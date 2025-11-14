---
cover: ../.gitbook/assets/Captura de pantalla 2025-04-30 a las 15.21.10.png
coverY: 0
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

### Yieldblox Pool Strategies (with Autocompound)

#### USDC Strategy

* **Contract ID**: `CCSRX5E4337QMCMC3KO3RDFYI57T5NZV5XB3W3TWE4USCASKGL5URKJL`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### EURC Strategy

* **Contract ID**: `CA33NXYN7H3EBDSA3U2FPSULGJTTL3FQRHD2ADAAPTKS3FUJOE73735A`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### XLM Strategy

* **Contract ID**: `CBDOIGFO2QOOZTWQZ7AFPH5JOUS2SBN5CTTXR665NHV6GOCM6OUGI5KP`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### CETES Strategy

* **Contract ID**: `CBTSRJLN5CVVOWLTH2FY5KNQ47KW5KKU3VWGASDN72STGMXLRRNHPRIL`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### AQUA Strategy

* **Contract ID**: `CCMJUJW6Z7I3TYDCJFGTI3A7QA3ASMYAZ5PSRRWBBIJQPKI2GXL5DW5D`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### USTRY Strategy

* **Contract ID**: `CDDXPBOF727FDVTNV4I3G4LL4BHTJHE5BBC4W6WZAHMUPFDPBQBL6K7Y`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

#### USDGLO Strategy

* **Contract ID**: `CCTLQXYSIUN3OSZLZ7O7MIJC6YCU3QLLS6TUM3P2CD6DAVELMWC3QV4E`
* **Hash**: `11329c2469455f5a3815af1383c0cdddb69215b1668a17ef097516cde85da988`

## Important Notes

* All strategy contracts share the same hash as they are instances of the same contract template
* The factory contract is used to deploy new vaults
* The vault contract hash is used to verify vault deployments
* Always verify contract addresses before interacting with them
