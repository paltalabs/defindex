import { DefindexSDK } from '@defindex/sdk';

if (!process.env.DEFINDEX_API_KEY) {
  console.warn('Warning: DEFINDEX_API_KEY is not set');
}

export const defindexClient = new DefindexSDK({
  baseUrl: process.env.DEFINDEX_API_URL || 'http://localhost:4555',
  apiKey: process.env.DEFINDEX_API_KEY || '',
});
