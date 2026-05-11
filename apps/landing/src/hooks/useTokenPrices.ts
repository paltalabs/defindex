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
      dedupingInterval: 7_200_000, // 2 h — matches server cache
    }
  );

  return {
    prices: data ?? {},
    isLoading,
    error,
  };
}
