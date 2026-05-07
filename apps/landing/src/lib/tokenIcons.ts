// Token data sourced from the Soroswap Protocol curated token list:
// https://raw.githubusercontent.com/soroswap/token-list/refs/heads/main/tokenList.json
// Plus 'native' (XLM) as a special case for the Stellar native asset.

interface TokenInfo {
  symbol: string;
  name: string;
  icon: string;
  decimals: number;
}

export const TOKEN_ICONS: Record<string, TokenInfo> = {
  // XLM — Stellar native asset (special key, not a contract address)
  'native': { symbol: 'XLM', name: 'Stellar Lumens', icon: '/images/xlm.png', decimals: 7 },
  // Etherfuse CETES
  'CAL6ER2TI6CTRAY6BFXWNWA7WTYXUXTQCHUBCIBU5O6KM3HJFG6Z6VXV': { symbol: 'CETES', name: 'Etherfuse CETES', icon: 'https://stablebonds.s3.us-west-2.amazonaws.com/stablebond/spl-cetes.png', decimals: 7 },
  // AQUA Token
  'CAUIKL3IYGMERDRUN6YSCLWVAKIFG5Q4YJHUKM4S4NJZQIA3BAS6OJPK': { symbol: 'AQUA', name: 'AQUA Token', icon: 'https://ipfs.io/ipfs/bafkreigzckkixvbkru2gzve67zu3bfauccugkr6zovkdv6h2yxxrppyyqa', decimals: 7 },
  // Etherfuse USTRY
  'CBLV4ATSIWU67CFSQU2NVRKINQIKUZ2ODSZBUJTJ43VJVRSBTZYOPNUR': { symbol: 'USTRY', name: 'Etherfuse USTRY', icon: 'https://stablebonds.s3.us-west-2.amazonaws.com/stablebond/spl-ustry.png', decimals: 7 },
  // USDC
  'CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75': { symbol: 'USDC', name: 'USD Coin', icon: 'https://ipfs.io/ipfs/bafkreibpzncuhbk5ozhdw7xkcdoyf3xhwhcwcf6sj7axjzimxw6vm6pvyy', decimals: 7 },
  // EURC
  'CDTKPWPLOURQA2SGTKTUQOWRCBZEORB4BWBOMJ3D3ZTQQSGE5F6JBQLV': { symbol: 'EURC', name: 'EUR Coin', icon: 'https://ipfs.io/ipfs/bafkreidizqlammdzrrurfq3o5owta77fiyonn6ri72h6rltqimn2xs2by4', decimals: 7 },
  // Etherfuse TESOURO (Tesouro Nacional Brazil)
  'CD6M4R2322BYCY2LNWM74PEBQAQ63SA3DUJLI3L4225U4ZVCLMSCBCIS': { symbol: 'TESOURO', name: 'Etherfuse Tesouro', icon: '/images/tesouro-icon.webp', decimals: 7 },
};

export function getTokenIcon(contractAddress: string): string | null {
  return TOKEN_ICONS[contractAddress]?.icon ?? null;
}

export function getTokenInfo(contractAddress: string): TokenInfo | null {
  return TOKEN_ICONS[contractAddress] ?? null;
}

export function getTokenSymbol(contractAddress: string): string {
  return TOKEN_ICONS[contractAddress]?.symbol ?? 'TOKEN';
}

export function getTokenDecimals(contractAddress: string): number {
  return TOKEN_ICONS[contractAddress]?.decimals ?? 7;
}
