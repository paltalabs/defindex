# Rescue Bot

This project is a rescue bot for DeFindex. Below are the steps to initialize and configure the project correctly.

## Prerequisites
- Node.js (recommended v18+)
- Yarn (v1.x)

## Installation

1. Clone the repository and navigate to the bot folder:
   ```bash
   cd apps/rescue_bot
   ```
2. Install dependencies:
   ```bash
   yarn install
   ```

## Environment configuration (`.env`)

You must create a `.env` file in the root of `apps/rescue_bot` based on the `.env.example` file:

1. Copy the example file:
   ```bash
   cp .env.example .env
   ```
2. Fill in the values:
   - `ADMIN_SECRET`: Admin private key (required).
   - `SOROBAN_RPC_URL`: Soroban RPC node URL (required).

Example:
```env
ADMIN_SECRET=SCX...AFS #Soroban private key
SOROBAN_RPC_URL=https://soroban.mainnet.com/
```

## Addressbook configuration

The file `public/{network}.contracts.json` contains the contract addresses used by the bot. If you need to add or update addresses:

1. Edit the file `public/{network}.contracts.json`.
2. Make sure the keys and values are correct and in valid JSON format.

Example structure:
```json
{
  "ids": {
    "blendPool": "...",
    "backstop": "...",
    "cometPool": "...",
    "assetAddress": "...",
    "blndAddress": "...",
    "defindexVault": "...",
    "defindexStrategy": "..."
  },
  "hashes": {}
}
```

## Running

- For development (with HMR):
  ```bash
  yarn dev
  ```
- To start the bot manually:
  ```bash
  yarn start
  ```

## Notes
- If you change the contract addresses, make sure to restart the bot.
- Keep your `ADMIN_SECRET` safe and never share it publicly.

---

For questions or issues, contact the PaltaLabs team.
