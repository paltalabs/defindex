import React from 'react';
export enum AllowedAssets {
  XLM = 'xlm',
  USDC = 'usdc'
}
export const allowedAssets = Object.values(AllowedAssets);
export interface Vault {
  name: string;
  symbol: string;
  address: string;
  assetAllocation: Asset[];
  vaultManager: string;
  emergencyManager: string;
  rebalanceManager: string;
  feeReceiver: string;
  feePercent: number;
  upgradable: boolean;
  totalSupply: number;
}

export interface Asset {
  asset: string;
  total_amount: number;
  idle_amount: number;
  invested_amount: number;
  strategies: Strategy[];
  amount: number;
  assetSymbol: string;
}
export interface Strategy{
  address: string;
  assetSymbol: string;
  assetAddress?: string;
  name: string;
  paused: boolean;
  amount?: number;
}

export type AssetContextType = {
  assets: Asset[];
  setAssets: (assets: Asset[]) => void;
}

export const AssetContext = React.createContext<AssetContextType | null>(null);

export type VaultContextType = {
  newVault: Vault;
  setNewVault: (vault: Vault) => void;
  vaults: Vault[];
  setVaults: (vaults: Vault[]) => void;
  selectedVault: Vault | null;
  setSelectedVault: (vault: Vault | null) => void;
}

export const VaultContext = React.createContext<VaultContextType | null>(null);

export type PublicAddressesContextType = {
  networkName: string;
  setNetworkName: (networkName: string) => void;
  factoryAddress: string;
  setFactoryAddress: (factoryAddress: string) => void;
  soroswapRouterAddress: string;
  setSoroswapRouterAddress: (soroswapRouterAddress: string) => void;
  assets: Asset[];
  setAssets: (assets: Asset[]) => void;
  vaults: any[];
  setVaults: (vaults: any[]) => void;
}

export const PublicAddressesContext = React.createContext<PublicAddressesContextType | null>(null);