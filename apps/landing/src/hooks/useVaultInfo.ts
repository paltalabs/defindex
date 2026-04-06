import useSWR from 'swr';
import { useMemo } from 'react';
import type { VaultInfoResponse } from '@defindex/sdk';
import type { VaultWithAddress, VaultState } from '@/types/vault.types';
import { calculateTotalTVLNumber } from '@/utils/vaultFormatters';

const NETWORK = 'mainnet';

interface UseVaultsProps {
  vaultIds: string[];
}

interface UseVaultsReturn {
  vaultStates: VaultState[];
  sortedVaults: VaultWithAddress[];
  isAllLoaded: boolean;
  isAnyLoading: boolean;
}

async function fetchVaultInfo(vaultId: string): Promise<VaultInfoResponse> {
  const res = await fetch('/api/vaultInfo', {
    headers: {
      vaultId,
      network: NETWORK,
    },
  });

  if (!res.ok) {
    const errorData = await res.json().catch(() => ({ error: 'Unknown error' }));
    throw new Error(errorData.error ?? `Failed to fetch vault ${vaultId}`);
  }

  const { data } = await res.json();
  return data as VaultInfoResponse;
}

export function useVaults({ vaultIds }: UseVaultsProps): UseVaultsReturn {
  // Create individual SWR hooks for each vault for progressive loading
  const vaultResults = vaultIds.map((vaultId) => {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    const { data, error, isLoading } = useSWR(
      ['vault-info', vaultId],
      () => fetchVaultInfo(vaultId),
      {
        revalidateOnFocus: false,
        revalidateOnReconnect: false,
        dedupingInterval: 300000, // 5 min cache
      }
    );

    return {
      vaultId,
      data,
      error,
      isLoading,
    };
  });

  const vaultStates: VaultState[] = useMemo(() => {
    return vaultResults.map(({ vaultId, data, error, isLoading }) => {
      let status: VaultState['status'] = 'pending';
      if (isLoading) status = 'loading';
      else if (error) status = 'error';
      else if (data) status = 'loaded';

      return {
        address: vaultId,
        vault: data ? { ...data, address: vaultId } : null,
        status,
        error: error?.message ?? null,
      };
    });
  }, [vaultResults]);

  const isAllLoaded = useMemo(
    () => vaultStates.every((v) => v.status === 'loaded' || v.status === 'error'),
    [vaultStates]
  );

  const isAnyLoading = useMemo(
    () => vaultStates.some((v) => v.status === 'loading' || v.status === 'pending'),
    [vaultStates]
  );

  const sortedVaults = useMemo(() => {
    const loadedVaults = vaultStates
      .filter((v): v is VaultState & { vault: VaultWithAddress } => v.vault !== null)
      .map((v) => v.vault);

    if (!isAllLoaded) {
      // Return in original order while loading
      return loadedVaults;
    }

    // Sort by TVL (descending) once all loaded
    return [...loadedVaults].sort((a, b) => {
      const tvlA = calculateTotalTVLNumber(a.totalManagedFunds);
      const tvlB = calculateTotalTVLNumber(b.totalManagedFunds);
      return tvlB - tvlA;
    });
  }, [vaultStates, isAllLoaded]);

  return {
    vaultStates,
    sortedVaults,
    isAllLoaded,
    isAnyLoading,
  };
}
