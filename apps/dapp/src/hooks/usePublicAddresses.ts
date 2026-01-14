import { Strategy } from '@/contexts';
import { NetworkType } from '@/helpers/networkName';
import useSWR from 'swr';


export enum AllowedAssets {
  XLM = 'xlm',
  USDC = 'usdc',
  EURC = 'eurc',
  CETES = 'cetes',
  USTRY = 'ustry',
  AQUA = 'aqua',
  USDGLO = 'usdglo'
}

export const usePublicAddresses = (network: NetworkType) => {
  const fetcher = async (url: string) => {
    const response = await fetch(url, {cache: 'reload'});
    const result = await response.json();
    return result.ids;
  };
  const { data, error, isLoading } = useSWR(
    network ? `https://raw.githubusercontent.com/paltalabs/defindex/refs/heads/main/public/${network}.contracts.json` : null,
    fetcher
  );

  return {
    data: data as Record<string, string>,
    isLoading,
    error,
  };
};

export const soroswapRouterAddress = async (network: NetworkType | undefined) => {
  if (!network) {
    throw new Error('Network is undefined');
  }

  const response = await fetch(`https://raw.githubusercontent.com/soroswap/core/refs/heads/main/public/${network}.contracts.json`, {
    cache: 'reload',
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch router address for network: ${network}`);
  }

  const data = await response.json();
  return data.ids.router;
}

export async function extractStrategies(publicAddresses: Record<string, string>): Promise<Strategy[]> {
  const strategies: Strategy[] = [];

  for (const key in publicAddresses) {
    if (key.endsWith('_strategy')) {
      const address = publicAddresses[key];
      let assetSymbol: AllowedAssets;
      const name = key.replace('_strategy', '').toLowerCase();

      Object.values(AllowedAssets).forEach((asset) => {
        if (name.startsWith(asset)) {
          assetSymbol = asset as AllowedAssets;
          strategies.push({ assetSymbol, name, address, paused: false });
        }
      });
    }
  }
  return strategies;
}
