# Introduction

With the new Protocol 20 of Stellar, new Smart Contract based Descentralized Protocols have arised in the Stellar Blockchain. Automated Market Makers like **Soroswap.Finance**, or Lending and Borrowing protocols like **Blend Capital** are just the beggining of a new set of DeFi Protocols.

These protocols allow from simple to complex investment stragtegies. Very simple strategies can be just to invest in a Soroswap.Finance AMM Constant Product Liquidity Pool, or just to lend USDC in a Blend Market and harvest the BLND reward to later swap those BLND harvested  to USDC and reinvest them in the lending pool. Other very simple strategy can be to diversify investment in two assets and just hold those assets.

A bit more complex strategy can be to diversify the investment in both protocols, or even, lend USDC, borrow XLM and then participate in a Liquidity Pool USDC/XLM with some reward program that can be harvested.

However, for this to be easyly done by non-experienced users, or even by wallets users that prefer to have a very simple interface, an interface Smart Contract protocol is needed.

DeFindex is a protocol where users can define how their **investments are distributed** among **multiple DeFi protocols and strategies**. The definition of this distribution and its rules involves the creation of an index. The distribution refers to the specification of percentage allocations to protocols and strategies. 

## Core Concepts

- **Index:** 
An index ("DeFindex") or **DeFindex Vault** is a smart contract that **defines a distribution** of an investment into **one or more strategies**. A DeFindex has a fixed list of strategies where investments can be made and the percentage distribution can be fixed?? or variable. Changing the distribution percentage is called rebalancing.

- **Strategy:** A strategy is a set of **steps** to be followed to execute an investment in one or several protocols. This can include farming and auto-compound rewards automatically, leverage lending or leveraged farming strategies for borrowing and lending markets like Blend.Capital.

    Example of Leverage Lending:
    ```
    Investing USDC in Blend, can be as simple as just depositing USDC in Blend harvest the BLND rewards and reinvest them (autocompounding), or it can be more complex with multiple steps: 1) Deposit 100% in Blend, 2) take a 50% loan in XLM, 3) Swap XLM for USDC, 4) Deposit more USDC. Then harvest BLND rewards.
    ```

- **Rebalancing:** Rebalancing involves changing the strategy distributions ratio of a DeFindex. For example, an index with 50% in two protocols could change to 80% and 20%, respectively. This process moves all the investment in the protocols to achieve the desired percentages.

    Rebalancing can be made in order to allways achieve a desired strategy distribution ratio, or in order to change the desired distribution ratio to a new one.

- **dfTokens:** dfTokens are fungible tokens issued to users upon depositing assets into a specific DeFindex Vault. They represent a proportional share of the total assets managed by the DeFindex Valut. Users can burn dfTokens to withdraw their underlying assets, which might be liquidated based on current protocol strategies.

- **Automated Market Makers (AMM):** AMMs are decentralized exchanges that use algorithms to set prices and facilitate trading. In DeFindex, AMM LP tokens represent liquidity provision in various trading pairs. Users can earn yields from trading fees and token incentives by holding or staking these LP tokens.

    Example: [Soroswap.Finance](https://soroswap.finance).

- **Lending Platforms:** Lending platforms allow users to deposit assets in exchange for earning interest. DeFindex incorporates lending strategies to diversify asset allocation and maximize returns. Assets deposited in DeFindex can be lent out to earn additional yield.

    Example: [Blend Capital](https://blend.capital).

- **IDLE Assets:** DeFindex maintains a balance between invested and idle assets. Idle assets are kept liquid to ensure users can easily withdraw funds without disrupting ongoing investments. The Minimum Idle Amount is the threshold of liquid assets required to support smooth operations and withdrawals.

- **Emergengy Withdraw:** Emergency Managers can liquidate risky positions and move all funds into idle assets in order to protect investors from unhealthy or risky strategies.,

- **Price Per Share (PPS):** Price Per Share (PPS) is a key metric that determines the value of one dfTokens relative to the total assets managed by  a DeFindex Vault.
