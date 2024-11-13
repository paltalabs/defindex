import { VaultMethod } from "@/hooks/useVault";
import { ChainMetadata } from "@soroban-react/types";

export interface Asset {
  address: string;
  strategies: Strategy[];
  symbol?: string;
}
export interface NewVaultState {
  address: string;
  emergencyManager: string;
  feeReceiver: string;
  manager: string;
  vaultShare: number;
  name: string;
  symbol: string;
  assets: Asset[];
  amounts: number[];
  totalValues?: number;
}

export interface Strategy {
  address: string;
  name: string;
  paused: boolean;
}
export interface VaultData {
  address: string;
  emergencyManager: string;
  feeReceiver: string;
  manager: string;
  name: string;
  assets: Asset[];
  totalValues: number;
}

export interface SelectedVault extends VaultData {
  method: VaultMethod;
}
export interface WalletState {
  address: string;
  selectedChain: ChainMetadata;
  vaults: {
    createdVaults: VaultData[];
    hasError: boolean;
    isLoading: boolean;
    selectedVault: SelectedVault | undefined;
  }
}

