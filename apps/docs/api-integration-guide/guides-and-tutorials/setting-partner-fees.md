---
description: ⏱️ 7 min read
---

# Setting Partner Fees

## 📖 What You'll Learn

This guide walks you through the complete fee management lifecycle for a deployed DeFindex vault:

* **Check** the current fee configuration of your vault
* **Update** the vault fee rate (in basis points)
* **Change** the fee receiver address
* **Distribute** accumulated fees to the appropriate parties

---

## 🎯 Prerequisites

Before starting, make sure you have:

* **Manager role** on the vault (or Fee Receiver role for specific operations)
* An **API key** from DeFindex (see [Getting Started with API](../api.md))
* Your **vault address** on mainnet or testnet
* A way to **sign transactions** (Freighter Wallet, Stellar Laboratory, etc.)

---

## 📐 Understanding BPS (Basis Points)

DeFindex uses **basis points (BPS)** to express fee percentages. One basis point equals 0.01%.

| BPS Value | Percentage | Description          |
|-----------|------------|----------------------|
| 100       | 1%         | Low fee              |
| 500       | 5%         | Moderate fee         |
| 1000      | 10%        | Common fee           |
| 3000      | 30%        | Standard partner fee |
| 5000      | 50%        | High fee             |
| 9000      | 90%        | Maximum allowed      |

**Key constants:**

* `SCALAR_BPS = 10,000` → 10,000 BPS = 100%
* **Maximum vault fee**: 9,000 BPS (90%)
* Fees are only charged on **yield generated**, never on deposited capital

---

## Step 1: Check Current Fee Configuration 🔍

Before making changes, verify the current fee settings on your vault.

### Get Vault Info (includes fee rates)

```bash
curl -X GET "https://api.defindex.io/vault/YOUR_VAULT_ADDRESS?network=mainnet" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

The response includes fee configuration:

```json
{
  "name": "My Yield Vault",
  "address": "CABC...XYZ",
  "feesBps": {
    "vaultFee": 3000,
    "defindexFee": 500
  }
}
```

**🔍 What this tells you:**

* `vaultFee`: The partner's fee rate in BPS (3000 = 30%)
* `defindexFee`: The DeFindex protocol fee rate in BPS (500 = 5%)

### Get Current Fee Receiver

```bash
curl -X GET "https://api.defindex.io/vault/YOUR_VAULT_ADDRESS/get/fee-receiver?network=mainnet" \
  -H "Authorization: Bearer YOUR_API_KEY"
```

Response:

```json
{
  "address": "GFEE_RECEIVER_ADDRESS..."
}
```

---

## Step 2: Update the Vault Fee (BPS) 💰

To change the fee rate on your vault, use the `lock-fees` endpoint. This endpoint serves a **dual purpose**: it locks any accrued fees AND updates the fee rate.

### Build the Transaction

```bash
curl -X POST "https://api.defindex.io/vault/YOUR_VAULT_ADDRESS/lock-fees?network=mainnet" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "new_fee_bps": 3000,
    "caller": "GMANAGER_ADDRESS..."
  }'
```

**Parameters:**

| Parameter     | Type   | Description                  |
|---------------|--------|------------------------------|
| `new_fee_bps` | number | New fee rate in BPS (0–9000) |
| `caller`      | string | Manager wallet address       |

**🔍 What this does:**

1. **Locks** any currently accrued fees at the previous rate
2. **Updates** the vault fee rate to the new value
3. Returns an **unsigned XDR** transaction

### Sign and Submit

The response contains an unsigned XDR transaction:

```json
{
  "xdr": "AAAAAgAAAA..."
}
```

**Sign the transaction** using Freighter, Stellar Laboratory, or your preferred method, then submit:

```bash
curl -X POST "https://api.defindex.io/send" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "xdr": "SIGNED_XDR_HERE..."
  }'
```

**🚨 Important notes:**

* Only the **Manager** role can update fees
* Maximum allowed value is **9000 BPS** (90%)
* Setting `new_fee_bps: 0` effectively disables partner fees

---

## Step 3: Change the Fee Receiver Address 🔄

To redirect fee payments to a different address, use the `set/fee-receiver` endpoint.

### Build the Transaction

```bash
curl -X POST "https://api.defindex.io/vault/YOUR_VAULT_ADDRESS/set/fee-receiver?network=mainnet" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "new_address": "GNEW_RECEIVER_ADDRESS...",
    "caller": "GMANAGER_ADDRESS..."
  }'
```

**Parameters:**

| Parameter     | Type   | Description                             |
|---------------|--------|-----------------------------------------|
| `new_address` | string | New fee receiver Stellar address        |
| `caller`      | string | Manager OR current Fee Receiver address |

**🔍 What this does:**

1. Updates the vault's fee receiver to the new address
2. Returns an **unsigned XDR** transaction

### Sign and Submit

Sign the returned XDR and submit it using the `/send` endpoint (same flow as Step 2).

**🚨 Important notes:**

* Can be called by **Manager** OR the **current Fee Receiver**
* The new address must be a valid Stellar address
* Always verify the new address before submitting — this action is irreversible without another update

---

## Step 4: Distribute Accumulated Fees 📤

Fees accumulate in the vault as yield is generated. To distribute them, use the `distribute-fees` endpoint.

### Build the Transaction

```bash
curl -X POST "https://api.defindex.io/vault/YOUR_VAULT_ADDRESS/distribute-fees?network=mainnet" \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "caller": "GMANAGER_ADDRESS..."
  }'
```

**Parameters:**

| Parameter | Type   | Description                     |
|-----------|--------|---------------------------------|
| `caller`  | string | Manager OR Fee Receiver address |

**🔍 What this does:**

1. Calculates the locked fee amount
2. **Splits** the fees between the vault fee receiver and the DeFindex protocol receiver based on the `defindexFee` rate
3. Sends each party their share
4. Returns an **unsigned XDR** transaction

### Sign and Submit

Sign the returned XDR and submit it using the `/send` endpoint (same flow as Step 2).

**🚨 Important notes:**

* Can be called by **Manager** OR **Fee Receiver**
* Fees must be locked before they can be distributed (Step 2 locks fees automatically)
* Distribution sends actual tokens to the receiver addresses

---

## 🔄 Complete Fee Management Workflow

```md
📐 CHECK CURRENT FEES
GET /vault/{address}?network=mainnet
GET /vault/{address}/get/fee-receiver?network=mainnet
            ↓

💰 UPDATE FEE RATE (Optional)
POST /vault/{address}/lock-fees
   → Locks accrued fees + sets new rate
   → Sign & submit unsigned XDR
            ↓

🔄 CHANGE FEE RECEIVER (Optional)
POST /vault/{address}/set/fee-receiver
   → Updates receiver address
   → Sign & submit unsigned XDR
            ↓

📤 DISTRIBUTE FEES
POST /vault/{address}/distribute-fees
   → Splits fees: partner share + DeFindex share
   → Sign & submit unsigned XDR
            ↓

✅ FEES DISTRIBUTED!
   → Fee Receiver gets vault fee share
   → DeFindex gets protocol fee share
```

---

## 📊 BPS Quick Reference Table

| BPS   | Percentage | Annual fee on $10,000 yield |
|-------|------------|-----------------------------|
| 100   | 1%         | $100                        |
| 250   | 2.5%       | $250                        |
| 500   | 5%         | $500                        |
| 1000  | 10%        | $1,000                      |
| 2000  | 20%        | $2,000                      |
| 3000  | 30%        | $3,000                      |
| 5000  | 50%        | $5,000                      |
| 9000  | 90%        | $9,000                      |

Remember: fees are charged on **yield only**, not on deposited capital.

---

## 🔒 Security Best Practices

### ✅ DO

* Use a **multisig wallet** for the Manager role
* Use a **dedicated, secure wallet** for the Fee Receiver
* **Verify fee values** before signing transactions (double-check BPS math)
* **Test on testnet** before making mainnet changes
* Distribute fees **regularly** to avoid large accumulated amounts

### ❌ DON'T

* Set fees above **9000 BPS** (the transaction will fail)
* Share your **API key** or expose it in client-side code
* Change the fee receiver to an **uncontrolled address**
* Skip **transaction verification** before signing

---

## 🔧 Common Troubleshooting

### Problem: "Permission denied" or "Unauthorized"

**Solution**: Only the Manager role can update fees. Verify you're using the correct caller address. For `set/fee-receiver` and `distribute-fees`, the current Fee Receiver can also call these.

### Problem: "Fee exceeds maximum"

**Solution**: The maximum allowed vault fee is 9000 BPS (90%). Reduce the `new_fee_bps` value.

### Problem: "No fees to distribute"

**Solution**: Fees accumulate as the vault generates yield. If the vault hasn't generated yield since the last distribution, there may be nothing to distribute. Also ensure fees have been locked first.

### Problem: "403 Forbidden"

**Solution**: Check your API key is correct and not expired. See [Getting Started with API](../api.md) for key generation.

### Problem: Transaction fails after signing

**Solution**: The unsigned XDR may have expired. Rebuild the transaction and sign again promptly. Also ensure your wallet has enough XLM for network fees.

---

## 📚 Related Resources

* [Partner Fees](../../getting-started/partner-fees.md) — Conceptual overview of the fee model
* [Vault Roles](../../getting-started/vault-roles.md) — Understanding Manager and Fee Receiver roles
* [Beginner Guide](beginner-guide.md) — Full walkthrough of the build → sign → submit flow
* [API Documentation](https://api.defindex.io/docs) — Complete API reference
* [Getting Started with API](../api.md) — API key setup and client configuration
