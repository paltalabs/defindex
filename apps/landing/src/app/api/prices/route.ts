import { NextResponse } from 'next/server';

// Mapping of Stellar contract addresses to CoinGecko IDs
const COINGECKO_IDS: Record<string, string> = {
  'CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75': 'usd-coin',   // USDC
  'CDTKPWPLOURQA2SGTKTUQOWRCBZEORB4BWBOMJ3D3ZTQQSGE5F6JBQLV': 'euro-coin',  // EURC
  'CAL6ER2TI6CTRAY6BFXWNWA7WTYXUXTQCHUBCIBU5O6KM3HJFG6Z6VXV': 'cetes',      // CETES
  'CBLV4ATSIWU67CFSQU2NVRKINQIKUZ2ODSZBUJTJ43VJVRSBTZYOPNUR': 'cetes',       // USTRY (same as CETES basket)
  'CD6M4R2322BYCY2LNWM74PEBQAQ63SA3DUJLI3L4225U4ZVCLMSCBCIS': 'cetes',       // TESOURO (same basket category)
  'native': 'stellar',                                                           // XLM
};

const FALLBACK_PRICES: Record<string, number> = {
  'CCW67TSZV3SSS2HXMBQ5JFGCKJNXKZM7UQUWUZPUTHXSTZLEO7SJMI75': 1,   // USDC
  'CDTKPWPLOURQA2SGTKTUQOWRCBZEORB4BWBOMJ3D3ZTQQSGE5F6JBQLV': 1.1, // EURC
  'CAL6ER2TI6CTRAY6BFXWNWA7WTYXUXTQCHUBCIBU5O6KM3HJFG6Z6VXV': 0.067, // CETES
  'CBLV4ATSIWU67CFSQU2NVRKINQIKUZ2ODSZBUJTJ43VJVRSBTZYOPNUR': 1,   // USTRY
  'CD6M4R2322BYCY2LNWM74PEBQAQ63SA3DUJLI3L4225U4ZVCLMSCBCIS': 1,   // TESOURO
  'native': 0.1,                                                       // XLM
};

// In-memory cache shared across requests in the same server instance
let cache: { prices: Record<string, number>; expiresAt: number } | null = null;

export async function GET() {
  if (cache && Date.now() < cache.expiresAt) {
    return NextResponse.json(cache.prices);
  }

  const geckoIds = [...new Set(Object.values(COINGECKO_IDS))].join(',');

  try {
    const res = await fetch(
      `https://api.coingecko.com/api/v3/simple/price?ids=${geckoIds}&vs_currencies=usd`,
      { next: { revalidate: 7200 } }
    );

    if (!res.ok) throw new Error(`CoinGecko responded ${res.status}`);

    const data = (await res.json()) as Record<string, { usd: number }>;

    const prices: Record<string, number> = { ...FALLBACK_PRICES };
    for (const [address, geckoId] of Object.entries(COINGECKO_IDS)) {
      const usd = data[geckoId]?.usd;
      if (usd != null) prices[address] = usd;
    }

    cache = { prices, expiresAt: Date.now() + 2 * 60 * 60 * 1000 };
    return NextResponse.json(prices);
  } catch {
    return NextResponse.json(FALLBACK_PRICES);
  }
}
