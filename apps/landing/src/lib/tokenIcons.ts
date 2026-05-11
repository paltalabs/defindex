// Token data sourced from the Soroswap Protocol curated token list:
// https://raw.githubusercontent.com/soroswap/token-list/refs/heads/main/tokenList.json
// Plus 'native' (XLM) as a special case for the Stellar native asset.
// To add or update tokens, edit src/lib/tokens.json directly.

import TOKEN_ICONS from './tokens.json';

export type TokenInfo = {
  symbol: string;
  name: string;
  icon: string;
  decimals: number;
};

export { TOKEN_ICONS };

export function getTokenIcon(contractAddress: string): string | null {
  return (TOKEN_ICONS as Record<string, TokenInfo>)[contractAddress]?.icon ?? null;
}

export function getTokenInfo(contractAddress: string): TokenInfo | null {
  return (TOKEN_ICONS as Record<string, TokenInfo>)[contractAddress] ?? null;
}

export function getTokenSymbol(contractAddress: string): string {
  return (TOKEN_ICONS as Record<string, TokenInfo>)[contractAddress]?.symbol ?? 'TOKEN';
}

export function getTokenDecimals(contractAddress: string): number {
  return (TOKEN_ICONS as Record<string, TokenInfo>)[contractAddress]?.decimals ?? 7;
}
