import { useCallback } from 'react';
import { useUser } from '@/contexts/UserContext';

// Types based on SDK
interface VaultAsset {
  address: string;
  strategies: Array<{
    address: string;
    name: string;
    paused: boolean;
  }>;
}

interface VaultInfoResponse {
  name: string;
  symbol: string;
  roles: {
    manager: string;
    emergencyManager: string;
    rebalanceManager: string;
    feeReceiver: string;
  };
  assets: VaultAsset[];
  totalManagedFunds: string[];
  feesBps: {
    vaultFee: number;
    defindexFee: number;
  };
  apy?: number;
}

interface VaultBalanceResponse {
  dfTokens: string;
  underlyingBalance: string[];
}

// Config for createVault (no deposit)
interface CreateVaultConfig {
  roles: {
    0: string; // Emergency Manager
    1: string; // Fee Receiver
    2: string; // Manager
    3: string; // Rebalance Manager
  };
  vault_fee_bps: number;
  assets: Array<{
    address: string;
    strategies: Array<{
      address: string;
      name: string;
      paused: boolean;
    }>;
  }>;
  name_symbol: {
    name: string;
    symbol: string;
  };
  upgradable: boolean;
  caller: string;
}

// Config for createVaultAutoInvest (with deposit + auto-invest)
interface CreateVaultAutoInvestConfig {
  caller: string;
  roles: {
    emergencyManager: string;
    rebalanceManager: string;
    feeReceiver: string;
    manager: string;
  };
  name: string;
  symbol: string;
  vaultFee: number; // basis points
  upgradable: boolean;
  assets: Array<{
    address: string;
    symbol: string;
    amount: number; // Total amount in stroops
    strategies: Array<{
      address: string;
      name: string;
      amount: number; // Amount to invest in this strategy
    }>;
  }>;
}

interface CreateVaultResponse {
  xdr: string;
  predictedVaultAddress?: string;
  warning?: string;
}

interface SendTransactionResponse {
  txHash?: string;
  hash?: string;
  status: string;
  predictedVaultAddress?: string;
}

export function useDefindexSDK() {
  const { address, signTransaction, activeNetwork } = useUser();

  // Get vault information
  const getVaultInfo = useCallback(
    async (vaultAddress: string): Promise<VaultInfoResponse> => {
      const response = await fetch(
        `/api/defindex/vault-info?vaultAddress=${vaultAddress}&network=${activeNetwork}`
      );

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to fetch vault info');
      }

      const { data } = await response.json();
      return data;
    },
    [activeNetwork]
  );

  // Get user balance in vault
  const getVaultBalance = useCallback(
    async (vaultAddress: string, userAddress?: string): Promise<VaultBalanceResponse> => {
      const user = userAddress || address;
      if (!user) {
        throw new Error('User address is required');
      }

      const response = await fetch(
        `/api/defindex/vault-balance?vaultAddress=${vaultAddress}&userAddress=${user}&network=${activeNetwork}`
      );

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || 'Failed to fetch vault balance');
      }

      const { data } = await response.json();
      return data;
    },
    [activeNetwork, address]
  );

  // Create vault without deposit
  const createVault = useCallback(
    async (vaultConfig: CreateVaultConfig): Promise<SendTransactionResponse> => {
      if (!address) {
        throw new Error('Wallet not connected');
      }

      // Step 1: Build XDR
      const buildResponse = await fetch('/api/defindex/create-vault', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          vaultConfig,
          network: activeNetwork,
          withDeposit: false,
        }),
      });

      if (!buildResponse.ok) {
        const error = await buildResponse.json();
        throw new Error(error.error || 'Failed to build vault creation transaction');
      }

      const { data: buildResult } = await buildResponse.json() as { data: CreateVaultResponse };
      const { xdr } = buildResult;

      // Step 2: Sign XDR
      const signedXdr = await signTransaction(xdr, address);

      // Step 3: Send signed transaction
      const sendResponse = await fetch('/api/defindex/send', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          signedXdr,
          network: activeNetwork,
        }),
      });

      if (!sendResponse.ok) {
        const error = await sendResponse.json();
        throw new Error(error.error || 'Failed to send transaction');
      }

      const { data: sendResult } = await sendResponse.json();
      return sendResult;
    },
    [activeNetwork, address, signTransaction]
  );

  // Create vault with deposit and auto-invest into strategies
  const createVaultAutoInvest = useCallback(
    async (vaultConfig: CreateVaultAutoInvestConfig): Promise<SendTransactionResponse> => {
      if (!address) {
        throw new Error('Wallet not connected');
      }

      // Step 1: Build XDR
      const buildResponse = await fetch('/api/defindex/create-vault', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          vaultConfig,
          network: activeNetwork,
          withDeposit: true,
        }),
      });

      if (!buildResponse.ok) {
        const error = await buildResponse.json();
        throw new Error(error.error || 'Failed to build vault creation transaction');
      }

      const { data: buildResult } = await buildResponse.json() as { data: CreateVaultResponse };
      const { xdr, predictedVaultAddress } = buildResult;

      // Step 2: Sign XDR
      const signedXdr = await signTransaction(xdr, address);

      // Step 3: Send signed transaction
      const sendResponse = await fetch('/api/defindex/send', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          signedXdr,
          network: activeNetwork,
        }),
      });

      if (!sendResponse.ok) {
        const error = await sendResponse.json();
        throw new Error(error.error || 'Failed to send transaction');
      }

      const { data: sendResult } = await sendResponse.json();
      return {
        ...sendResult,
        predictedVaultAddress,
      };
    },
    [activeNetwork, address, signTransaction]
  );

  return {
    getVaultInfo,
    getVaultBalance,
    createVault,
    createVaultAutoInvest,
    address,
    activeNetwork,
  };
}

export type { CreateVaultConfig, CreateVaultAutoInvestConfig };
