import { NextResponse } from 'next/server';

const baseUrl = process.env.DEFINDEX_API_URL ?? 'https://api.defindex.io';
const MAINNET_CONTRACTS_URL =
  'https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/mainnet.contracts.json';

async function fetchMainnetStrategyAddresses(): Promise<Set<string>> {
  const res = await fetch(MAINNET_CONTRACTS_URL, { next: { revalidate: 86400 } }); //24h
  if (!res.ok) return new Set();
  const json = await res.json() as { ids: Record<string, string> };
  const addresses = Object.entries(json.ids)
    .filter(([key]) => key.endsWith('_strategy'))
    .map(([, address]) => address);
  return new Set(addresses);
}

export async function GET() {
  try {
    const [strategiesRes, mainnetAddresses] = await Promise.all([
      fetch(`${baseUrl}/strategies/apy?network=mainnet`, { next: { revalidate: 60 } }),
      fetchMainnetStrategyAddresses(),
    ]);

    if (!strategiesRes.ok) {
      const text = await strategiesRes.text().catch(() => 'Unknown error');
      return NextResponse.json(
        { error: `Upstream error: ${text}` },
        { status: strategiesRes.status }
      );
    }

    const raw = await strategiesRes.json();
    const all = Array.isArray(raw) ? raw : (raw.data ?? []);
    const data = mainnetAddresses.size > 0
      ? all.filter((s: { address: string }) => mainnetAddresses.has(s.address))
      : all;

    return NextResponse.json({ data });
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : 'Failed to fetch strategies';
    return NextResponse.json({ error: message }, { status: 500 });
  }
}
