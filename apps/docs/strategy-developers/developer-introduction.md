# Smart Contracts Development

Welcome to the DeFindex Smart Contract Development Guide! This guide will help you understand how to create a strategy that uses your protocol.

## What is a DeFindex Vault?

DeFindex vaults let users deposit tokens into a pooled account that automatically executes diversified yield strategies across DeFi protocols. The vault issues shares representing each depositor’s stake, rebalances holdings to capture returns and reduce risk, and provides simple actions like deposit, withdraw, and view balance — all without users having to manage multiple protocols themselves.

## What is a DeFindex Strategy?

A DeFindex Strategy is a smart contract that implements a specific investment logic for a DeFi protocol. In other words, it is the connection to a DeFi protocol. Also, It allows users to automate complex DeFi operations and optimize their yield through a single interface.

It only needs to comply with the strategy interface, which can be found in [github](../../contracts/strategies/core/src/lib.rs). Once it complies with this interface, it can be used by vaults.

## Resources

1. Review the [Strategy Contract](../10-whitepaper/02-contracts/02-strategy-contract.md) on whitepaper docs
2. Check out our [Strategies](../../../public/mainnet.contracts.json)
3. Join our [developer community](https://discord.gg/ftPKMPm38f) for support
