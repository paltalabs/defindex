import * as StellarSdk from '@stellar/stellar-sdk';
import { useCallback, useEffect, useState } from "react";
import { contractInvoke, SorobanContextType, useSorobanReact } from 'stellar-react';

import { getNetworkName } from "@/helpers/networkName";
import { TxResponse } from 'stellar-react/dist/contracts/types';
import { usePublicAddresses } from './usePublicAddresses';

export enum FactoryMethod {
  CREATE_DEFINDEX_VAULT = "create_defindex_vault",
  CREATE_DEFINDEX_VAULT_DEPOSIT = "create_defindex_vault_deposit",
  TOTAL_VAULTS = "total_vaults",
  GET_VAULT_BY_INDEX = "get_vault_by_index",
  DEFINDEX_FEE = "defindex_fee",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

const findFactoryAddress = (publicAddresses: Record<string, string>): string | undefined => {
  if (!publicAddresses || Object.keys(publicAddresses).length === 0) {
    throw new Error('No public addresses found');
  }
  const factoryAddress = publicAddresses['defindex_factory'];
  if (!factoryAddress) {
    throw new Error('Factory address not found in public addresses');
  }
  return factoryAddress;
}
export const useFactory = () => {
  const sorobanContext: SorobanContextType = useSorobanReact();
  const publicAddresses = usePublicAddresses(getNetworkName(sorobanContext.activeNetwork));
  const { activeNetwork } = sorobanContext;
  if (!activeNetwork) {
    throw new Error('No active network found');
  }
  const [address, setAddress] = useState<string>();
  const networkName = getNetworkName(activeNetwork);
  useEffect(() => {
    if (!sorobanContext || !publicAddresses) return;
    if (networkName !== 'mainnet' && networkName !== 'testnet') {
      throw new Error(`Invalid network when fetching factory address: ${networkName}. It should be mainnet or testnet`);
    }

    if (publicAddresses.isLoading) return;
    if (publicAddresses.error || !publicAddresses.data) {
      throw new Error(`Failed to fetch public addresses: ${publicAddresses.error}`);
    }
    const factoryAddress = findFactoryAddress(publicAddresses.data);
    setAddress(factoryAddress);

  }, [activeNetwork,publicAddresses]);

  return { address };
}

export function useFactoryCallback() {
  const sorobanContext = useSorobanReact();
  const {activeNetwork} = sorobanContext;
  const publicAddresses = usePublicAddresses(
    activeNetwork? getNetworkName(activeNetwork) : 'mainnet'
  ).data;
  const { address: factoryAddress } = useFactory();
  if (!activeNetwork) {
    throw new Error('No active network found');
  }

  return useCallback(
    async (method: FactoryMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
      try {
        let result: TxResponse;
        if(!factoryAddress) {
          const fallbackAddress = findFactoryAddress(publicAddresses);
          if (!fallbackAddress) {
            throw new Error('Failed to fetch fallback address');
          }
          result = (await contractInvoke({
            contractAddress: fallbackAddress,
            method: method,
            args: args,
            sorobanContext,
            signAndSend: signAndSend,
            reconnectAfterTx: false,
          })) as TxResponse;
          return result;
        } else {
          result = (await contractInvoke({
            contractAddress: factoryAddress as string,
            method: method,
            args: args,
            sorobanContext,
            signAndSend: signAndSend,
            reconnectAfterTx: false,
          })) as TxResponse;
        }
        if (!signAndSend) return result;
        if (
          isObject(result) &&
          result?.status !== StellarSdk.rpc.Api.GetTransactionStatus.SUCCESS
        ) throw result;
        return result
      } catch (e: any) {
        const error = e as Error;
        if (error.message.includes('ExistingValue')) throw new Error('Index already exists.')
        if (error.message.includes('The user rejected')) throw new Error('Request denied by user. Please try to sign again.')
        if (error.message.includes('UnexpectedSize')) throw new Error('Invalid arguments length.')
        if (error.message.includes('Error(Contract, #10)')) throw new Error('Insufficient funds.')
        if (error.message.includes('invoke non-existent contract function')) throw new Error('Contract function does not exist.')
        if (error.message.includes('MissingValue')) throw new Error('Contract not found.')
        throw new Error(error.message)
      }
    }, [sorobanContext, factoryAddress, publicAddresses])
}