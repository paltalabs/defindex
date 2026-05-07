import useSWR from 'swr';
import type { StrategyApySnapshot } from '@/types/vault.types';

async function fetchStrategies(): Promise<StrategyApySnapshot[]> {
  const res = await fetch('/api/strategies');
  if (!res.ok) {
    const errorData = await res.json().catch(() => ({ error: 'Unknown error' }));
    throw new Error(errorData.error ?? 'Failed to fetch strategies');
  }
  const { data } = await res.json();
  return data as StrategyApySnapshot[];
}

interface UseStrategiesReturn {
  strategies: StrategyApySnapshot[];
  isLoading: boolean;
  error: string | null;
}

export function useStrategies(): UseStrategiesReturn {
  const { data, error, isLoading } = useSWR<StrategyApySnapshot[]>(
    'strategies-apy',
    fetchStrategies,
    {
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
      dedupingInterval: 60000,
    }
  );

  return {
    strategies: data ?? [],
    isLoading: isLoading || (!data && !error),
    error: error?.message ?? null,
  };
}
