import { Strategy } from '@/contexts';
import useSWR from 'swr';
import { StrategyMethod, useStrategyCallback } from './useStrategy';
import { WalletNetwork } from 'stellar-react';
import { getNetworkName } from '@/helpers/networkName';
export const usePublicAddresses = (network: string) => {
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

export const soroswapRouterAddress = async (network: WalletNetwork | undefined) => {
  if (!network) {
    throw new Error('Network is undefined');
  }

  const response = await fetch(`https://raw.githubusercontent.com/soroswap/core/refs/heads/main/public/${getNetworkName(network)}.contracts.json`, {
    cache: 'reload',
  });

  if (!response.ok) {
    throw new Error(`Failed to fetch router address for network: ${getNetworkName(network)}`);
  }

  const data = await response.json();
  return data.ids.router;
}

export async function extractStrategies(publicAddresses: Record<string, string>): Promise<Strategy[]> {
  const strategies: Strategy[] = [];

  for (const key in publicAddresses) {
    if (key.endsWith('_strategy')) {
      const address = publicAddresses[key];
      let assetSymbol: 'xlm' | 'usdc';
      let name = key.replace('_strategy', '');

      if (name.startsWith('xlm_')) {
        assetSymbol = 'xlm';
        name = name.replace('xlm_', '');
      } else if (name.startsWith('usdc_')) {
        assetSymbol = 'usdc';
        name = name.replace('usdc_', '');
      } else {
        continue;
      }
      strategies.push({ assetSymbol, name, address, paused: false });
    }
  }
  return strategies;
}
