import { Strategy } from '@/contexts';
import useSWR from 'swr';
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
    data: data as string[],
    isLoading,
    error,
  };
};

export async function extractStrategies(publicAddresses: string[]): Promise<Strategy[]> {
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
      strategies.push({ assetSymbol, name, address });
    }
  }
  return strategies;
}
