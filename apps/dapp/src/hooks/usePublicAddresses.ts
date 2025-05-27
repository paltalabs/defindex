import { Strategy } from '@/contexts';
import { getNetworkName } from '@/helpers/networkName';
import { WalletNetwork } from 'stellar-react';
import useSWR from 'swr';


export enum AllowedAssets {
  XLM = 'xlm',
  USDC = 'usdc',
  EURC = 'eurc'
}

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

  const response = await fetch(`https://raw.githubusercontent.com/soroswap/core/refs/heads/main/public/${network === WalletNetwork.PUBLIC ? 'mainnet' : 'testnet'}.contracts.json`, {
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
      let assetSymbol: AllowedAssets;
      let name = key.replace('_strategy', '');

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
