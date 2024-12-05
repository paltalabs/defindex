# Design Decisions
We have decided to do:

## Multi Assets Vaults.
We think is important to offer diversified Vaults to our users, not only in the platforms or strategies they will be interacting, but also in the assets they will be exposed to.

## AMM Liquidity Pool Support
When supporting a AMM Liquity Pool, the underlying asset will be considered as the **AMM LP token**, for example, for a Soroswap USDC-XLM liquidity pool, the underlying asset will be the Soroswap-USDC-XLM-LP token and not the USDC or XLM tokens.

## User should provide the exact underlying assets
Even if we would provide the best user experience, every Vault only accepts the corresponding assets it will be using for its strategies. We can help the user to get these assets before investing in the Vault(See Zapper contract). However it is a decision that the Vault will only accept the desired assets in the correct ratio.

To understand better why we decide this please check the [Why we can`t swap on deposit](../10-apendix/01-why-we-cant-swap-on-deposit-or-withdraw.md) section.

## IDLE funds.
- Security: Enables emergency withdrawal
- Performance: Separate high CPU instructions limits tx.
- Transaction Cost: Enable small investment that wont  be affected by costly txs.

## Emergency Withdrawal
- Secutiry. We have roles.

## Roles
- Manager
- Emergency Manager
- Fee Receiver