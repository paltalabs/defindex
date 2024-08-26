# Set Protocol

Set Protocol is a decentralized platform that enables the creation, management, and trading of tokenized investment portfolios, known as Sets. By leveraging Ethereum smart contracts, Set Protocol allows users to automate and rebalance their portfolios based on predefined strategies, making complex financial maneuvers accessible to both novice and experienced investors. These Sets can include a diverse range of assets, from cryptocurrencies to tokenized traditional assets, providing broad exposure and diversification. Set Protocol's intuitive interface and advanced features empower users to maximize returns while minimizing risks, making it a powerful tool for modern digital asset management.

## Multi-asset:
It supports multi-asset strategies, allowing users to create Sets composed of various tokens and assets.
However, a user needs to have the underlying assets to mint a Set token, which can be a barrier for some investors.

## How the LPToken are minted when new underlying asset are added? ¿What is the formula?

That’s defined at the beginning as an arbitrary parameter. For example, I can define 1 SetToken to have 1WBTC, 2WETH and 3USDC. Then, if I want to mint 10 SetTokens I need to have 10WBTC, 20WETH, 30USDC.

It can be added a module to mint SetTokens with only one Asset   https://docs.tokensets.com/developers/guides-and-tutorials/protocol/nav-issuance . It uses oracles to identify how much you can mint. “The issuer receives a proportional amount of SetTokens on issuance based on the calculated net asset value of the Set using **oracle prices**.”

# TokenSets

Web: https://www.tokensets.com/#/

Litepaper: https://docs.tokensets.com/protocol/litepaper

Docs: https://docs.tokensets.com/

## Are there some examples of mixing AMM Liquidity Pool tokens with Lending Platform?

Multi-Asset: Set V2 enables the creation and implementation of strategies employing single asset, pairs, and 3+ assets.
Apparently you need to have the assets beforehand
Does it include swaps when investing?

## How the LPToken are minted when new underlying asset are added? ¿What is the formula?

That’s defined at the beginning as an arbitrary parameter. For example, I can define 1 SetToken to have 1WBTC, 2WETH and 3USDC. Then, if I want to mint 10 SetTokens I need to have 10WBTC, 20WETH, 30USDC.

It can be added a module to mint SetTokens with only one Asset   https://docs.tokensets.com/developers/guides-and-tutorials/protocol/nav-issuance . It uses oracles to identify how much you can mint. “The issuer receives a proportional amount of SetTokens on issuance based on the calculated net asset value of the Set using **oracle prices**.”

## What is the concept of rebalancing / reinvesting in this protocol?

Rebalancing can be done using the Trade Module. The Trade Module enables managers of SetTokens to perform atomic trades using aggregators such as 0x and 1inch, and decentralized exchanges such as Sushiswap and Uniswap. This rebalances the Set for all Set holders.

## How do the contracts handle user funds?

Funds are held by the SetToken Contract.

## How does the protocol generate revenue?

This is done through the Streaming Fee Module

The Streaming Fee Module is a module that accrues streaming fees for Set managers. Streaming fees are denominated as percent per year and realized as Set inflation rewarded to the manager.

The formula to solve for fee is:
- (feeQuantity / feeQuantity) + totalSupply = fee / scaleFactor

The simplified formula utilized below is:
- feeQuantity = fee * totalSupply / (scaleFactor - fee)
The streaming fees are fees that are paid out to Set managers over time are based on the entire market cap of the Set (e.g. 2% of market cap over 1 year). This incentivizes managers to increase the value of their Sets over time for their users.

The streaming fee is calculated linearly over the lifespan of the Set. For example, if a Set has a 2% streaming fee and 6 months has passed, 1% of streaming fees can be collected.

Protocol Fees: To allow for protocol sustainability, the Protocol will charge fees for protocol-native transactions such as trading via dutch auctions, borrowing using the protocol’s lending pool, and subscription/profit fee sharing.

Manager Admin: Set V2 gives managers greater control over how and when Sets can be minted and by whom.

Trader Subscription and Performance Fees: Traders can implement time-based (streaming) and performance-based (profit) fees