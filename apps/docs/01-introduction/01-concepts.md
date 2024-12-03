## Concepts

These are some concepts to understand the DeFindex protocol:
- **Vault:** 
A **DeFindex Vault** is a smart contract that **defines a distribution** of an investment into **one or more strategies**. It works like an index fund or an ETF, where the underlying assets are invested in DeFi protocols. In order to be exposed to DeFi strategies, a user just needs to deposit assets into the Vault. Then, the Vault will take care of automatically investing those assets in the defined strategies.

- **Strategy:** A strategy is a set of **steps** to be followed to execute an investment in one or several protocols. This could be as simple as just holding assets, or as complex as farming and auto-compound rewards automatically, leverage lending or leveraged farming strategies for borrowing and lending markets like Blend.Capital.

    Example of Leverage Lending:
    ```
    Investing USDC in Blend, can be as simple as just depositing USDC in Blend harvest the BLND rewards and reinvest them (autocompounding), or it can be more complex with multiple steps: 1) Deposit 100% in Blend, 2) take a 50% loan in XLM, 3) Swap XLM for USDC, 4) Deposit more USDC. Then harvest BLND rewards.
    ```

- **Rebalancing:** Rebalancing involves changing the allocation of funds between strategies of a DeFindex Vault. For example, a vault with 50% in two strategies could change to 80% and 20%, respectively. 

- **Shares** or **dfTokens:** Shares are fungible tokens issued to users upon depositing assets into a specific DeFindex Vault. They represent a proportional share of the total assets managed by the DeFindex Vault. Users can burn shares to withdraw their underlying assets, which might be liquidated based on current protocol strategies.

- **Automated Market Makers (AMM):** AMMs are decentralized exchanges that use algorithms to set prices and facilitate trading. In DeFindex, AMM LP tokens represent liquidity provision in various trading pairs. Users can earn yields from trading fees and token incentives by holding or staking these LP tokens.

    Example: [Soroswap.Finance](https://soroswap.finance).

- **Lending Platforms:** Lending platforms allow users to deposit assets in exchange for earning interest. DeFindex incorporates lending strategies to diversify asset allocation and maximize returns. Assets deposited in DeFindex can be lent out to earn additional yield.

    Example: [Blend Capital](https://blend.capital).

- **Autocompounding:** Autocompounding is the process of reinvesting rewards automatically into the same strategy. This allows for changing from APR to APY! This allows for continuous growth of the investment without the need for manual intervention. Let's see an example:

    If a user deposits 100 USDC in a strategy with 30% APR, after one year the user will have 130 USDC. However, if the user reinvests the rewards every day, she will get more! Let's see how this works:
    1. A 30% APR is 0.082191781% per day. Because daily return is APR/365 = 0.082191781%    
    2. If the user reinvests the rewards every day, after one year the user will have ~135 USDC. Because $$(1 + 0.00082191781)^{365} = 1,349692488$$ Meaning that instead of 30% APR, the user will have 34.96% APY.

    This shows how powerful the autocompounding is!

- **Farming:** Farming is the process of earning rewards by staking assets in DeFi protocols.
- **Harvesting:** Harvesting is the process of collecting the rewards earned by the strategy.