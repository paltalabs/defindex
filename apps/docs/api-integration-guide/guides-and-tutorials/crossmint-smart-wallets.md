---
description: ⏱️ 3 min read
---

# Crossmint Smart Wallets Integration

## Overview

This guide points to a reference repository that shows you how to integrate DeFindex vaults using **Crossmint smart wallets** — enabling fully automated, server-side deposits, withdrawals, and cross-chain bridging with **zero user interaction**.

The pattern uses an **EVM private key registered as `adminSigner`** to control both an ERC-4337 smart wallet on Base and a Stellar smart wallet. All Defindex vault interactions go through Crossmint's REST API as Soroban `contract-call` transactions — no manual XDR construction required.

## Repository

[**defindex-io/crossmint-defindex-guide**](https://github.com/defindex-io/crossmint-defindex-guide)

## What the Repository Covers

| Topic | Description |
| --- | --- |
| Crossmint setup | Server API key (`sk_`), wallet email, staging vs production |
| EVM wallet | ERC-4337 smart wallet on Base with `external-wallet` adminSigner |
| Stellar wallet | Stellar smart wallet with auto-XLM funding, Soroban contract-call signing |
| Deposit | `contract-call` via Crossmint REST → base64 XDR approval → poll |
| Withdraw | Withdraw by amount or by shares (% redemption) |
| Bridge | Base USDC → Stellar → Defindex vault via Sodax |
| Gotchas | 9 documented edge cases with root causes and fixes |

## Architecture at a Glance

```
Your Server (EVM Private Key as adminSigner)
       │
       ▼
Crossmint REST API (api/2025-06-09)
  ├── EVM Smart Wallet (ERC-4337, Base)
  │     POST /transactions → sign hex bytes → POST /approvals → onChain.txId
  └── Stellar Smart Wallet
        POST /transactions (contract-call) → sign base64 XDR → POST /approvals → onChain.txId

Sodax Bridge
  └── Base USDC → Stellar USDC (via Sonic hub, no Horizon polling needed)

Defindex Vault (Soroban)
  ├── method: deposit         → amounts_desired, amounts_min, from, invest
  ├── method: withdraw        → amounts_to_withdraw, from
  └── method: withdraw_shares → shares_amount, from
```

All vault operations follow the same pattern:

1. `POST` to Crossmint REST → create `contract-call` transaction
2. Response is `awaiting-approval` with a base64-encoded XDR message
3. Sign with `keypair.sign(Buffer.from(message, "base64"))` using `STELLAR_SERVER_KEY`
4. `POST` signature to `/approvals`
5. Poll until `onChain.txId` is returned

## Quick Start

```bash
git clone https://github.com/defindex-io/crossmint-defindex-guide
cd crossmint-defindex-guide
pnpm install
cp .env.example .env
# Fill in: CROSSMINT_SERVER_API_KEY (sk_...), CROSSMINT_WALLET_EMAIL,
#          EVM_PRIVATE_KEY, STELLAR_SERVER_KEY
```

```bash
CROSSMINT_ENV=staging pnpm example:deposit          # Deposit into testnet vault
CROSSMINT_ENV=staging pnpm example:withdraw         # Withdraw by amount (testnet)
CROSSMINT_ENV=staging pnpm example:withdraw-shares  # Withdraw by shares (testnet)
CROSSMINT_ENV=production pnpm example:bridge        # Base USDC → Stellar → Defindex vault (mainnet)
```

## Prerequisites

* [Crossmint](https://crossmint.com) account with a **server API key** (must start with `sk_`, not `ck_`)
* `EVM_PRIVATE_KEY` — becomes the `adminSigner` of the EVM smart wallet on Base
* `STELLAR_SERVER_KEY` — Stellar ed25519 secret key, becomes the `adminSigner` of the Stellar wallet
* Defindex API key — request access on [Discord](https://discord.gg/e2qAhJCBmx)

## Additional Resources

* [Crossmint Documentation](https://docs.crossmint.com)
* [Defindex API Reference](https://api.defindex.io/docs)
* [Sodax Bridge Documentation](https://docs.sodax.io)