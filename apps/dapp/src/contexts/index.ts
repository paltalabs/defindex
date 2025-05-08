import React from 'react';
export enum AllowedAssets {
  XLM = 'xlm',
  USDC = 'usdc'
}
interface Vault {
  name: string;
  symbol: string;
  assets: {
    assetSymbol: AllowedAssets;
    amount?: number;
  }[];
  strategies: Strategy[];
  vaultManager: string;
  emergencyManager: string;
  rebalanceManager: string;
  feeReceiver: string;
  feePercent: number;
}
export interface Strategy{
  address: string;
  assetSymbol: string;
  name: string;
}

export type StrategiesContextType = {
  strategies: Strategy[];
  setStrategies: (strategies: Strategy[]) => void;
}

export const StrategiesContext = React.createContext<StrategiesContextType | null>(null);