---
cover: ../.gitbook/assets/image 31.png
coverY: 0
---

# Partner Fees

## The DeFindex Ecosystem

DeFindex operates with three key participants working together:

* **DeFindex**: The protocol that provides the infrastructure, smart contracts, and yield-generating strategies.
* **Partners**: Wallets, fintechs, or applications that integrate DeFindex to offer yield products to their users. Partners configure fees for their specific integration.
* **End Users**: People who deposit funds through a partner's application and earn yield on their assets.

This model allows partners to monetize their user base while offering competitive yield products, and users benefit from easy access to DeFi opportunities through trusted applications.

---

## Performance-Based Fee Model

Partner fees in DeFindex follow a simple principle: **fees are only charged on the yield generated, never on the deposited capital**.

### How It Works

* If your vault generates yield, a percentage goes to the partner and DeFindex
* If there's no yield, there are no fees
* Your principal investment is never touched by fees

### Fee Limits

* **Maximum fee**: 90% of generated yield
* **Typical range**: 10-50% of generated yield

This performance-based model ensures that partners only earn when users earn. There's no incentive to charge fees on idle capital.

---

## Transparency for Users

One of DeFindex's core principles is transparency. When a user sees an APY displayed in their partner's application:

* The APY shown is **already net of all fees**
* Users see exactly what they will receive
* No hidden deductions or surprise charges

### What Users See vs What Happens

| Displayed | Meaning |
|-----------|---------|
| 8% APY | User will earn approximately 8% annually on their deposit |
| Vault performance | Already accounts for partner fees |
| Balance growth | Reflects actual returns after all fees |

This approach eliminates confusion. The number users see is the number they get.

---

## Aligned Incentives

The performance-based fee model creates natural alignment between all parties:

### For Partners

* Earn revenue only when users profit
* Incentive to promote well-performing vaults
* No temptation to charge fees on underperforming products

### For Users

* Capital is protected from fees
* Only pay when earning
* Confidence that partners want the same outcome: good returns

### For DeFindex

* Protocol grows when users and partners succeed
* Focus on building better yield strategies
* Sustainable ecosystem development

This alignment means everyone benefits from the same goal: generating real yield for depositors.

---

## Fee Distribution

When yield is generated, fees are distributed automatically on-chain:

1. **Yield is generated** by the vault's strategies
2. **Partner fee is calculated** based on their configured percentage
3. **Distribution occurs** when the partner triggers it
4. **Fees are split** between the partner and DeFindex

The split between partner and DeFindex is handled internally by the protocol.

---

## Practical Example

Let's walk through a concrete scenario:

### Setup
* User deposits **$1,000 USDC** through a partner's app
* The vault generates **$100 in yield** over time (10% return)
* Partner has configured a **20% performance fee**

### Calculation
| Item | Amount |
|------|--------|
| Gross yield generated | $100 |
| Partner fee (20% of yield) | $20 |
| Net yield to user | $80 |

### Result
* **User receives**: $80 in yield (8% effective APY)
* **User's capital**: $1,000 remains untouched
* **Partner + DeFindex share**: $20

The user's displayed APY of 8% already reflected this fee structure from the start.

---

## Key Takeaways

| Principle | What It Means |
|-----------|---------------|
| Performance-based | Fees only on yield, never on capital |
| Net APY display | Users see what they actually earn |
| Aligned incentives | Partners profit when users profit |
| On-chain distribution | Transparent, automated fee handling |
| Protected principal | Deposits are never reduced by fees |

---

## Learn More

* [Understanding APY](understanding-apy.md) — How APY is calculated and what it means
* [Vault Roles](vault-roles.md) — Understanding the different roles in vault management
* [Get APY](../api-integration-guide/smart-contracts/get-apy.md) — How to fetch APY programmatically
