## [D1] Factory Smart Contract

- **Brief Description:**  
  Improve the current smart contract developed at the Consensus EasyA Hackathon by enhancing security and solving duplicated data issues. We plan to apply for audits by banks and incorporate their feedback.

- **How to Measure Completion:**  
  Code is reviewed, tested, and available on GitHub. Successfully passes security audits.
- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/contracts/factory)
    - 🛠️ Security Audits
---

## [D2] DeFindex Smart Contract

- **Brief Description:**  
  This contract allocates different DeFi protocols and collects the fees to be paid. We aim to enhance the Consensus EasyA Hackathon contracts by improving security, adding fees, liquidity pool tokens, rebalancing, and optional admin functions. Optimization for Soroban CPU instruction limits and bank audits for feedback will be implemented.

- **How to Measure Completion:**  
  Contract will be available on [GitHub](https://github.com/paltalabs). Code is reviewed, tested, and successfully passes security audits.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/contracts/vault)
    - 🛠️ Security Audits

---

## [D3] Adapter Smart Contracts

- **Brief Description:**  
  Enable any DeFi protocol to connect to the DeFindex Contract. Engage with the Stellar community to promote Smart Contract standards for interoperability. Improve security, optimize CPU instructions, and review interfaces. Publish the adapter struct at crates.io for community use. Create specific adapters for Blend, Phoenix, Xycloans, and Soroswap.

- **How to Measure Completion:**  
  Contract and Adapter Struct will be available on [GitHub](https://github.com/paltalabs). Adapter Struct will be published at crates.io. Documentation will be available on the website, and a standard proposal will be available as a SEP.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/contracts/strategies)
    - 🛠️ Adapter Struct published at crates.io
    - 🛠️ SEP proposal

---

## [D4] Index Creator Dashboard (Frontend)

- **Brief Description:**  
  Create a user-friendly dashboard where index creators (wallet providers or others) can create different DeFindexes with specific allocations to protocols and strategies. Adapter creators (DeFi protocols) can upgrade their adapters if they pass audit checks. A simple governance system will allow for adapter approvals (similar to the soroswap token-list).

- **How to Measure Completion:**  
  Dashboard will be available as a Dapp. DeFi protocols can propose their adapters, and index creators can easily allocate to selected DeFi protocols and strategies.

- **Result:**
  - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/dapp)
---

## [D5] Tutorials for DeFi Protocols and Index Creators

- **Brief Description:**  
  Create tutorials on how to create a DeFindex using the dashboard, how to implement it on a mobile or web app, and how to create an adapter.

- **How to Measure Completion:**  
  Tutorials will be available in the docs section of the website, published on Medium and dev.to, and shared on the Stellar Discord Server.

- **Result:**
    - 🛠️ Tutorials available on the website and published on Medium and dev.to
    - 🛠️ Shared on the Stellar Discord Server
---

## [D6] Flutter SDK

- **Brief Description:**  
  Improve the existing Flutter SDK from the Consensus EasyA Hackathon to allow any Flutter-based wallet to integrate DeFindex. Collaborate with Meru and Beans App for feedback, and integrate with Meru.

- **How to Measure Completion:**  
  Code will be available on [GitHub](https://github.com/paltalabs). A Flutter app can call a DeFindex Smart Contract instance with less than 10 lines of code.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/packages/defindex-dart-sdk)
    - ✅ Code published on [pub.dev](https://pub.dev/packages/defindex_sdk)

---

## [D7] TypeScript SDK

- **Brief Description:**  
  Develop a TypeScript SDK for web dapps to integrate DeFindexes.

- **How to Measure Completion:**  
  Code will be available on [GitHub](https://github.com/paltalabs). A React app can call a DeFindex Smart Contract instance with less than 10 lines of code.

- **Result:**
    - 🛠️ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/packages/defindex-sdk)
