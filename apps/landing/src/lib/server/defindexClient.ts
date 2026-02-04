import { DefindexSDK, SupportedNetworks } from '@defindex/sdk';

const apiKey = process.env.DEFINDEX_API_KEY;
const baseUrl = process.env.DEFINDEX_API_URL ?? 'https://api.defindex.io';

if (!apiKey) {
  console.warn('DEFINDEX_API_KEY is not set. Vault API calls will fail.');
}

export const defindexClient = new DefindexSDK({
  apiKey,
  baseUrl,
  defaultNetwork: SupportedNetworks.MAINNET,
});
