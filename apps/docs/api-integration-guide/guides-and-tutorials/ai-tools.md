---
description: ⏱️ 3 min read
---

# AI Tools — MCP Server & Claude Code Skill

DeFindex provides two complementary AI integrations that let you query documentation and build integrations using natural language.

---

## 1. Defindex MCP Server

The **Model Context Protocol (MCP)** server exposes the full DeFindex documentation to any compatible AI assistant, including Claude, so it can answer questions about vaults, strategies, API endpoints, and SDK usage without you having to paste docs manually.

### What is it?

The Defindex MCP server (`https://docs.defindex.io/~gitbook/mcp`) gives AI tools direct, searchable access to these docs. Ask your AI assistant about deposit flows, vault roles, APY calculations, or any other Defindex concept and it will answer from the latest documentation.

### Add to Claude Code (CLI)

Add the following to your project's `.claude/settings.json` or to your global `~/.claude/settings.json`:

```json
{
  "mcpServers": {
    "Defindex": {
      "type": "http",
      "url": "https://docs.defindex.io/~gitbook/mcp"
    }
  }
}
```

After saving, restart Claude Code. The server will be available as `mcp__Defindex__*` tools.

### Add to Claude.ai (Web)

1. Open **Claude.ai → Settings → Integrations → Add MCP Server**
2. Enter the server URL: `https://docs.defindex.io/~gitbook/mcp`
3. Give it a name (e.g. `Defindex Docs`) and save

### Add to Claude Desktop

In your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "Defindex": {
      "type": "http",
      "url": "https://docs.defindex.io/~gitbook/mcp"
    }
  }
}
```

### Example Queries

Once configured, you can ask your AI assistant:

- *"How do I deposit into a DeFindex vault?"*
- *"What roles does a DeFindex vault have?"*
- *"Show me the withdraw-shares endpoint"*
- *"How is APY calculated in DeFindex?"*
- *"What is a dfToken?"*

---

## 2. Claude Code Skill — `defindex-api`

The **`defindex-api` skill** is a Claude Code playbook that gives the model a complete, structured reference for the DeFindex REST API. It covers authentication, every endpoint, request/response shapes, error handling, and code examples.

### What it does

When invoked, the skill provides Claude Code with:

- **Auth flow** — register → login → generate API key → use Bearer token
- **User operations** — deposit, withdraw, withdraw-shares, balance, APY, discover vaults
- **Vault administration** — roles (get/set), rebalance, lock/release/distribute fees, rescue, pause/unpause strategies, upgrade WASM
- **Factory** — create-vault, create-vault-deposit, create-vault-auto-invest
- **Submit transactions** — POST `/send` for signed XDRs
- **Rate limits** — tier configs and retry patterns

### Installation

Clone the skill repository into your Claude Code skills directory:

```bash
git clone https://github.com/defindex-io/defindex-skill ~/.claude/skills/defindex-api
```

Or install file-by-file:

```bash
mkdir -p ~/.claude/skills/defindex-api
curl -sL https://raw.githubusercontent.com/defindex-io/defindex-skill/main/SKILL.md -o ~/.claude/skills/defindex-api/SKILL.md
curl -sL https://raw.githubusercontent.com/defindex-io/defindex-skill/main/auth.md -o ~/.claude/skills/defindex-api/auth.md
curl -sL https://raw.githubusercontent.com/defindex-io/defindex-skill/main/endpoints.md -o ~/.claude/skills/defindex-api/endpoints.md
```

### How to use it

In Claude Code, type:

```
/defindex-api
```

Or reference it in your prompt:

```
Use the defindex-api skill to help me deposit 100 USDC into the mainnet vault
```

Argument hints:

| Argument | What you get |
|---|---|
| `auth` | Registration, login, API key generation |
| `vault` | Vault info, balance, APY |
| `deposit` | Deposit flow with code example |
| `withdraw` | Withdraw and withdraw-shares |
| `admin` | Roles, rebalance, fees, rescue, pause, upgrade |
| `factory` | Create vault, create-vault-deposit, auto-invest |
| `send` | Submit signed XDR |
| `rate-limits` | Tier configs, 429 handling |

### Related Skills

- **`stellar-dev`** — general Stellar and Soroban development playbook.

---

## Getting Your API Key

Before you can call protected endpoints, you need an API key:

1. **Register** → [https://api.defindex.io/register](https://api.defindex.io/register)
2. **Login** → [https://api.defindex.io/login](https://api.defindex.io/login)
3. **Generate key** — from the dashboard, create your API key

See the full walkthrough: [Getting Your API Key](./getting-api-key.md)

---

## Additional Resources

- [DeFindex Skill](https://github.com/defindex-io/defindex-skill)
- [Full API Reference](https://api.defindex.io/docs)
- [Postman Collection](https://drive.google.com/drive/folders/1hp02ySFWFeunRCwiZ6oLCjHzcJXpWhX8?usp=drive_link)
- [Discord — developer channel](https://discord.gg/e2qAhJCBmx)
