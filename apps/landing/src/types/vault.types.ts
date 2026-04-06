import type { VaultInfoResponse, VaultAsset, VaultRole, VaultFees } from '@defindex/sdk';

// Re-export SDK types for convenience
export type { VaultInfoResponse, VaultAsset, VaultRole, VaultFees };

// Extended type for totalManagedFunds (SDK types it as any[])
export interface ManagedFunds {
  asset: string;
  idle_amount: string;
  invested_amount: string;
  strategy_allocations: StrategyAllocation[];
  total_amount: string;
}

export interface StrategyAllocation {
  amount: string;
  paused: boolean;
  strategy_address: string;
}

// Extended vault type with address for client-side usage
export interface VaultWithAddress extends VaultInfoResponse {
  address: string;
}

// Loading states for progressive loading
export type VaultLoadingState = 'pending' | 'loading' | 'loaded' | 'error';

export interface VaultState {
  address: string;
  vault: VaultWithAddress | null;
  status: VaultLoadingState;
  error: string | null;
}

// API response types
export interface SingleVaultAPIResponse {
  address: string;
  data: VaultInfoResponse | null;
  error: string | null;
}
