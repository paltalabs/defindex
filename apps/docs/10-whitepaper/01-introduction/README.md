DeFindex is a protocol where users can define how investment are distributed among multiple DeFi protocols and strategies. The definition of this distribution is the creation of an index. The distribution refers to the specificatoion of percentages allocations to protocols and strategioes. Every index is conected to their protocols through adapters.

## Core Concepts

### Index
An index (”DeFindex”) is a smart contract that defines the percentage distribution of an investment into different smart contracts. An DeFindex has a fixed list of protocols however the percentage distribution can be fixed or variable. The change of the distribution percentage is called rebalancing

### Strategy
An strategy is a set of steps to be followed in order to execute an investment in a certain or several protocols. For example, investing USDC in Blend can be as simple as just depositing in USDC in Blend , or ir can be 1) Deposit 100% in Blend, 2) take 50% in loan in XLM, 3) Swap XLM for USDC, 4) Deposit more USDC… this last strategy is typical in Ethereum and its called leverage lending 

### Rebalancing
Rebalancing would be changing the distributions of a DeFindex, example: an index with 50% and 50% on two protocols → would change to 80% and 20% respectively. This should move all the invetment on the protocols for the desired percetages

### Adapter
An adapter interacts with a DeFi protocol with standarized methods so a DeFindex can use any DeFi protocol
