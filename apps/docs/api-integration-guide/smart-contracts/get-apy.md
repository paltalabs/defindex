# Get APY

Fetches the current APY for the vault. It considers the fee charged by the vault.

```typescript
const vault = 'CAQ6PAG4X6L7LJVGOKSQ6RU2LADWK4EQXRJGMUWL7SECS7LXUEQLM5U7';

async function apy(apiClient: ApiClient): number {
    const {apy} = await apiClient.getData("apy", vault);
    return apy;
}
```

