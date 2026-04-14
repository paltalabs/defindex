---
description: ⏱️ 2 min read
---
# Getting Your API Key

Before integrating with the DeFindex API, you need an API key. This guide walks you through the self-service registration and key creation process.

---

## Step 1: Register an Account

Go to the registration page and create your account:

👉 [https://api.defindex.io/register](https://api.defindex.io/register)

Fill in your email and password, then submit the form. Then refresh your browser tab.

---

## Step 2: Log In

Once registered, log in to your account:

👉 [https://api.defindex.io/login](https://api.defindex.io/login)

Enter your credentials and submit. You will be redirected to your dashboard.

---

## Step 3: Create an API Key

From your dashboard, navigate to the **API Keys** section and click **Create API Key**.

You will receive:

- **`api_key`** — the key you include in the `Authorization` header of every request.
- **`refresh_token`** — used to obtain a new `api_key` when the current one expires.

> **Keep these values secure.** Do not commit them to public repositories or expose them in client-side code.

---

## Step 4: Use the API Key in Requests

Include the `api_key` as a Bearer token in the `Authorization` header:

```http
Authorization: Bearer <your_api_key>
```

TypeScript example:

```typescript
const response = await fetch(`https://api.defindex.io/vault/${vaultAddress}/deposit`, {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
        'Authorization': `Bearer ${process.env.DEFINDEX_API_KEY}`
    },
    body: JSON.stringify(params)
});
```

---

## Refreshing an Expired Key

When your `api_key` expires, use the `refresh_token` to get a new one by calling the `/refresh` endpoint with your `refresh_token`. Check the [API Reference](https://api.defindex.io/docs) for the exact request format.

---

## Next Steps

- [Getting Started with the API](../api.md)
- [Beginner Guide — Vault Deposit Example](./beginner-guide.md)
- [Full API Reference](https://api.defindex.io/docs)

---

## Need Help?

If you run into issues, join our [Discord](https://discord.gg/ftPKMPm38f) and ask in the developer channel.
