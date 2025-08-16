---
cover: ../../../.gitbook/assets/image 31.png
coverY: 0
---

# Why Not Swap in Deposit

When depositing (or investing) in DeFindex, a user receives dfTokens, which represent their share of the DeFindex portfolio. These tokens are minted based on the amount of assets deposited and the current price per share (denoted as p<sub>ps</sub>). Later, the user can burn these tokens to withdraw their assets.

The challenge arises when calculating this price per share.

### Example Scenario

Consider a DeFindex with a single strategy: 100% allocation to XLM on Xycloans, while the DeFindex receives deposits in USDC. The price per share p<sub>ps</sub> could be calculated as the amount of USDC one receives after withdrawing from Xycloans and swapping the XLM to USDC, divided by the total supply of dfTokens:

$$
p_{ps}(m) = \frac{T_{dfTokens}}{p_{XLM}(m) \cdot M_{XLM}}
$$

Where:

* p<sub>XLM</sub>(m) is the price of XLM in terms of USDC after liquidating m XLM.
* M<sub>XLM</sub> is the total amount of XLM held by the DeFindex.
* T<sub>dfTokens</sub> is the total supply of dfTokens.

The problem is that the price of XLM p<sub>XLM</sub>(m) will depend on the amount of XLM we need to withdraw. This price can be manipulated by a large swap. For instance, someone could swap a large amount of USDC for XLM, artificially inflating the price of XLM. As a result, the price per share would increase, allowing the user to receive more USDC when burning their dfTokens.

### Fixed Price Per Share Approach

Given the manipulation risk with a variable price per share, let's consider using a fixed price per share.

Assume p<sub>0</sub> is the nominal (initial or fixed) price of XLM in USDC. The amount of USDC received by a user who swaps m<sub>XLM</sub> XLM for USDC will be:

$$
m_{USDC} = p_{XLM}(m_{XLM}) \cdot m_{XLM}
$$

The price per share would then be:

$$
p_{ps} = \frac{p_0 \cdot M_{XLM}}{T_{dfTokens}}
$$

After burning m<sub>dfTokens</sub> dfTokens, the user should receive:

$$
m_{USDC} = p_{ps} \cdot m_{dfTokens} = \frac{p_0 \cdot M_{XLM} \cdot m_{dfTokens}}{T_{dfTokens}}
$$

Where:

* m<sub>XLM</sub> is the amount of XLM the DeFindex needs to liquidate to pay the user.
* m<sub>dfTokens</sub> is the amount of dfTokens the user is burning to withdraw their share.

The amount of XLM to be liquidated, m<sub>XLM</sub>, is given by:

$$
m_{XLM} = M_{XLM} \cdot \frac{m_{dfTokens}}{T_{dfTokens}}
$$

The USDC received after the swap, denoted as m<sub>USDCout</sub>, would be:

$$
m_{USDCout} = p_{XLM}(m_{XLM}) \cdot m_{XLM} = p_{XLM}(m_{XLM}) \cdot M_{XLM} \cdot \frac{m_{dfTokens}}{T_{dfTokens}}
$$

Since p<sub>0</sub> is the nominal price and p<sub>XLM</sub>(m<sub>XLM</sub>) is the actual price after liquidation, we have:

$$
p_{XLM}(m_{XLM}) < p_0 \quad \forall \, m_{XLM} > 0
$$

Thus, we have:

$$
p_{XLM}(m_{XLM}) \cdot M_{XLM} \cdot \frac{m_{dfTokens}}{T_{dfTokens}} < p_0 \cdot M_{XLM} \cdot \frac{m_{dfTokens}}{T_{dfTokens}}
$$

In summary:

$$
m_{USDCout} < m_{USDC}
$$

This inequality shows that the user would request more USDC than what they can actually receive after the swap. This discrepancy leads to a potential loss of funds for DeFindex, highlighting why we can't rely on swapping assets during the deposit process.

We can argue that when using any fixed price per share, the amount of USDC received after a swap, denoted as m<sub>USDCout</sub>, will differ from the expected amount m<sub>USDC</sub>. This discrepancy introduces a vulnerability, making the protocol susceptible to manipulation.

### **Calculating Price Per Share (PPS)**:

The PPS is a crucial metric for ensuring users receive the correct value for their dfTokens. It is calculated as follows:

$$
PPS = \frac{\text{Total Assets}}{\text{Total Supply of } dfTokens}
$$

Where:

* **Total Assets**: The sum of the value of assets managed by all adapters plus any idle assets held directly by the DeFindex contract.
* **Total Supply of dfTokens**: The total number of dfTokens issued to users.

To illustrate, consider the following scenario:

* DeFindex has three adapters managing different investments:
  * Adapter A manages $50,000 in a liquidity pool.
  * Adapter B manages $30,000 in a lending pool.
  * Adapter C manages $20,000 in a staking protocol.
* The DeFindex contract holds an additional $10,000 in idle assets.

The Total Assets would be:

$$
50,000 + 30,000 + 20,000 + 10,000 = 110,000 \text{ USDC}
$$

If the Total Supply of dfTokens is 100,000, the PPS would be:

$$
PPS = \frac{110,\!000 \ \text{USDC}}{100,\!000 \ \text{dfTokens}} = 1.1 \ \text{USDC/dfToken}
$$

This calculation ensures users can accurately determine the value of their holdings in DeFindex, promoting transparency and trust.
