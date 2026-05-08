import useSWR from 'swr';

const fetcher = (url: string): Promise<Record<string, number>> =>
  fetch(url).then(r => r.json());

export function useTokenPrices() {
  const { data, error, isLoading } = useSWR<Record<string, number>>(
    '/api/prices',
    fetcher,
    {
      revalidateOnFocus: false,
      revalidateOnReconnect: false,
      dedupingInterval: 300_000, // 5 min — matches server cache
    }
  );

  return {
    prices: data ?? {},
    isLoading,
    error,
  };
}
