# SCF Tracker
## Índice

- [SCF #28](#scf-28)
  - [D1] Factory Smart Contract
  - [D2] DeFindex Smart Contract
  - [D3] Adapter Smart Contracts
  - [D4] Index Creator Dashboard (Frontend)
  - [D5] Tutorials for DeFi Protocols and Index Creators
  - [D6] Flutter SDK
  - [D7] TypeScript SDK
- [SCF #32 - DeFindex: Mainnet, Metrics & Keepers](#scf-32---defindex-mainnet-metrics--keepers)
  - [D1] Mainnet Contracts 🚀
  - [D2] Metrics 📊
  - [D3] Rescue Funds Keeper 🛡️
  - [D4] UX Improvement 🎨

## SCF #28

### [D1] Factory Smart Contract

- **Brief Description:**  
  Improve the current smart contract developed at the Consensus EasyA Hackathon by enhancing security and solving duplicated data issues. We plan to apply for audits by banks and incorporate their feedback.

- **How to Measure Completion:**  
  Code is reviewed, tested, and available on GitHub. Successfully passes security audits.
- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/contracts/factory)
    - 🛠️ Security Audits
---

### [D2] DeFindex Smart Contract

- **Brief Description:**  
  This contract allocates different DeFi protocols and collects the fees to be paid. We aim to enhance the Consensus EasyA Hackathon contracts by improving security, adding fees, liquidity pool tokens, rebalancing, and optional admin functions. Optimization for Soroban CPU instruction limits and bank audits for feedback will be implemented.

- **How to Measure Completion:**  
  Contract will be available on [GitHub](https://github.com/paltalabs). Code is reviewed, tested, and successfully passes security audits.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/contracts/vault)
    - 🛠️ Security Audits

---

### [D3] Adapter Smart Contracts

- **Brief Description:**  
  Enable any DeFi protocol to connect to the DeFindex Contract. Engage with the Stellar community to promote Smart Contract standards for interoperability. Improve security, optimize CPU instructions, and review interfaces. Publish the adapter struct at crates.io for community use. Create specific adapters for Blend, Phoenix, Xycloans, and Soroswap.

- **How to Measure Completion:**  
  Contract and Adapter Struct will be available on [GitHub](https://github.com/paltalabs). Adapter Struct will be published at crates.io. Documentation will be available on the website, and a standard proposal will be available as a SEP.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/contracts/strategies)
    - ✅ Adapter Struct published at crates.io
    - 🛠️ SEP proposal

---

### [D4] Index Creator Dashboard (Frontend)

- **Brief Description:**  
  Create a user-friendly dashboard where index creators (wallet providers or others) can create different DeFindexes with specific allocations to protocols and strategies. Adapter creators (DeFi protocols) can upgrade their adapters if they pass audit checks. A simple governance system will allow for adapter approvals (similar to the soroswap token-list).

- **How to Measure Completion:**  
  Dashboard will be available as a Dapp. DeFi protocols can propose their adapters, and index creators can easily allocate to selected DeFi protocols and strategies.

- **Result:**
  - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/apps/dapp)
---

### [D5] Tutorials for DeFi Protocols and Index Creators

- **Brief Description:**  
  Create tutorials on how to create a DeFindex using the dashboard, how to implement it on a mobile or web app, and how to create an adapter.

- **How to Measure Completion:**  
  Tutorials will be available in the docs section of the website, published on Medium and dev.to, and shared on the Stellar Discord Server.

- **Result:**
    - 🛠️ Tutorials available on the website and published on Medium and dev.to
    - 🛠️ Shared on the Stellar Discord Server
---

### [D6] Flutter SDK

- **Brief Description:**  
  Improve the existing Flutter SDK from the Consensus EasyA Hackathon to allow any Flutter-based wallet to integrate DeFindex. Collaborate with Meru and Beans App for feedback, and integrate with Meru.

- **How to Measure Completion:**  
  Code will be available on [GitHub](https://github.com/paltalabs). A Flutter app can call a DeFindex Smart Contract instance with less than 10 lines of code.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/packages/defindex-dart-sdk)
    - ✅ Code published on [pub.dev](https://pub.dev/packages/defindex_sdk)

---

### [D7] TypeScript SDK

- **Brief Description:**  
  Develop a TypeScript SDK for web dapps to integrate DeFindexes.

- **How to Measure Completion:**  
  Code will be available on [GitHub](https://github.com/paltalabs). A React app can call a DeFindex Smart Contract instance with less than 10 lines of code.

- **Result:**
    - ✅ Code available on [GitHub](https://github.com/paltalabs/defindex/tree/main/packages/defindex-sdk)
    - ✅ Published on [npm](https://www.npmjs.com/package/defindex-sdk)



## SCF #32 - DeFindex: Mainnet, Metrics & Keepers

### [D1] Mainnet Contracts 🚀

- **Brief Description:**  
  Bring DeFindex from Testnet beta to Mainnet final version. Run rounds with testers and get their feedback to improve our Vault Contract and Strategy Crate. Move from Total Management Funds fee to Performance Fee (fee is charged on profits made by the Vault). Improve the 2 strategies we have as MVP in our Testnet Deployment to be ready for Mainnet:  
  1) Blend USDC Rewards Autocompound  
  2) Hodl Strategy  
  Work closely with an audit company (funded by the AuditBank) to secure our contracts.

- **How to Measure Completion:**  
  Contracts will be audited and published in our repo.
- **Estimated date of completion:**  
  Mid January 2024
- **Budget:**  
  20k
- **Result:**
    - 🛠️ Contracts audited and published in the repository

---

### [D2] Metrics 📊

- **Brief Description:**  
  Write Zephyr Programs to get metrics to estimate the profitability and health state of Blend Strategies. Show these metrics on our Dashboard. Provide metrics to Keepers and Bots.
- **How to Measure Completion:**  
  Zephyr Contracts will be published on our repo, Dashboard will be available on our frontend.
- **Estimated date of completion:**  
  Mid February 2024
- **Budget:**  
  10k
- **Result:**
    - 🛠️ Zephyr contracts published in the repository
    - 📈 Metrics available on the dashboard

---

### [D3] Rescue Funds Keeper 🛡️

- **Brief Description:**  
  Creation of predetermined Keepers that monitor the health of strategies and can trigger a rescue fund transaction on Soroban to secure funds of end users.
- **How to Measure Completion:**  
  Managers will be able to fork our project, set up the keeper and run a keeper easily.
- **Estimated date of completion:**  
  Early March
- **Budget:**  
  15k
- **Result:**
    - 🛠️ Keeper setup instructions available
    - ✅ Keeper can be run by managers

---

### [D4] UX Improvement 🎨

- **Brief Description:**  
  Hire agency to improve design. Implement design.
- **How to Measure Completion:**  
  Dashboard looks good, with all the needed functionality.
- **Estimated date of completion:**  
  End of March
- **Budget:**  
  5k
- **Result:**
    - 🛠️ Dashboard design improved and implemented
    - ✅ All required functionality present