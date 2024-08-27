# Design Decisions
We have decided to do:

## Multi Assets Index.
We think is important to offer diversified Indexes to our users, not only in the platforms or strategies they will be interacting, but also in the assets they will be exposed to.

## AMM Liquidity Pool Support
When supporting a AMM Liquity Pool, the underlying asset will be considered as the AMM LP token, for example, for a Soroswap USDC-XLM liquidity pool, the underlying asset will be the Soroswap-USDC-XLM-LP token and not the USDC or XLM tokens.

## User should provide the exact underlying assets
Even if we would provide the best user experience, every Index will only accept the corresponding assets it will be using for its strategies. We can help the user to get these assets before investing in the Index. To uinderstand better why we decide this please check the [Why we can`t swap on deposit](../10-apendix/01-why-we-cant-swap-on-deposit-or-withdraw.md) section.

