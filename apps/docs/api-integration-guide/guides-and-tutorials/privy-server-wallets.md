---
description: ⏱️ 3 min read
---

# Privy Server Wallets Integration

## Overview

This guide points to a reference repository that shows you how to integrate DeFindex vaults using **Privy server wallets** enabling fully automated, server-side deposits, withdrawals, and cross-chain bridging with **zero user interaction**.

The pattern relies on Privy's **Authorization Key** (TEE-backed) to sign Stellar transactions from your backend, making it ideal for custodial products, bots, and programmatic yield strategies.

## Repository

[**defindex-io/privy-defindex-guide**](https://github.com/defindex-io/privy-defindex-guide)

## What the Repository Covers

| Topic | Description |
| --- | --- |
| Privy setup | App ID, TEE activation, Authorization Key generation |
| Stellar wallet | Creation, XLM funding via Friendbot, USDC trustline |
| EVM wallet | Base EVM wallet with `sendTransaction` |
| Deposit | Full signing flow: XDR → hash → `rawSign` → broadcast |
| Withdraw | Withdraw by amount or by shares (% redemption) |
| Bridge | Base USDC → Stellar → Defindex vault via Sodax |
| Gotchas | 9 documented edge cases with root causes and fixes |

## Architecture at a Glance

```md
Your Server (P-256 Authorization Key)
       │  signs every request
       ▼
Privy TEE
  ├── Stellar wallet (Tier 2) — rawSign only
  └── EVM wallet    (Tier 3) — full sendTransaction

Defindex API (api.defindex.io)
  ├── POST /vault/{addr}/deposit         → unsigned Soroban XDR
  ├── POST /vault/{addr}/withdraw        → unsigned Soroban XDR
  ├── POST /vault/{addr}/withdraw_shares → unsigned Soroban XDR
  └── POST /send                         → { txHash }
```

All vault operations follow the same signing loop:

1. Authenticated `POST` to Defindex API → receive unsigned XDR
2. Parse XDR → hash it
3. `privy.rawSign(walletId, { hash })` → Ed25519 signature
4. Attach `DecoratedSignature` to the envelope
5. `POST` signed XDR to `/send`

## Quick Start

```bash
git clone https://github.com/paltalabs/privy-defindex-guide
cd privy-defindex-guide
pnpm install
cp .env.example .env
# Fill in: PRIVY_APP_ID, PRIVY_APP_SECRET, PRIVY_AUTHORIZATION_PRIVATE_KEY, DEFINDEX_API_KEY
```

```bash
pnpm example:deposit          # Deposit into Defindex XLM vault (testnet)
pnpm example:withdraw         # Withdraw by amount (testnet)
pnpm example:withdraw-shares  # Withdraw by shares / percentage (testnet)
pnpm example:bridge           # Base USDC → Stellar → Defindex vault (mainnet)
```

## Prerequisites

* [Privy](https://privy.io) app with TEE enabled and an Authorization Key configured
* Defindex API key — request access on [Discord](https://discord.gg/e2qAhJCBmx)

## Additional Resources

* [Privy Documentation](https://docs.privy.io)
* [Defindex API Reference](https://api.defindex.io/docs)
* [Sodax Bridge Documentation](https://docs.sodax.io)
