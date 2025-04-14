import { contractInvoke, SorobanContextType, useSorobanReact } from 'stellar-react';
import * as StellarSdk from '@stellar/stellar-sdk';
import { useCallback, useEffect, useState } from "react";

import { getNetworkName } from "@/helpers/networkName";
import { fetchFactoryAddress } from "@/utils/factory";
import { TxResponse } from 'stellar-react/dist/contracts/types';

export enum FactoryMethod {
  CREATE_DEFINDEX_VAULT = "create_defindex_vault",
  CREATE_DEFINDEX_VAULT_DEPOSIT = "create_defindex_vault_deposit",
  TOTAL_VAULTS = "total_vaults",
  GET_VAULT_BY_INDEX = "get_vault_by_index",
  DEFINDEX_FEE = "defindex_fee",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);
export const useFactory = () => {
  const sorobanContext: SorobanContextType = useSorobanReact();
  const { activeNetwork } = sorobanContext;
  if (!activeNetwork) {
    throw new Error('No active network found');
  }
  const [address, setAddress] = useState<string>();
  const networkName = getNetworkName(activeNetwork);
  useEffect(() => {
    if (!sorobanContext) return;
    if (networkName !== 'mainnet' && networkName !== 'testnet') {
      throw new Error(`Invalid network when fetching factory address: ${networkName}. It should be mainnet or testnet`);
    }

    fetchFactoryAddress(networkName).then(
      (factoryAddress) => {
        setAddress(factoryAddress);
      }
    ).catch((error) => {
      throw new Error(`Failed to fetch factory address: ${error}`);
    });

  }, [activeNetwork]);

  return { address };
}

export function useFactoryCallback() {
  const sorobanContext = useSorobanReact();
  const {activeNetwork} = sorobanContext;
  const { address: factoryAddress } = useFactory();
  if (!activeNetwork) {
    throw new Error('No active network found');
  }
  const networkName = getNetworkName(activeNetwork);

  return useCallback(
    async (method: FactoryMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
      try {
        let result: TxResponse;
        if(!factoryAddress) {
          const fallbackAddress = await fetchFactoryAddress(networkName)
          .catch((error) => {
            console.warn(`Failed to fetch fallback address: ${error}`);
            return undefined;
          });
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
        console.log("Factory Callback result", result)
        if (!signAndSend) return result;
        if (
          isObject(result) &&
          result?.status !== StellarSdk.rpc.Api.GetTransactionStatus.SUCCESS
        ) throw result;
        return result
      } catch (e: any) {
        console.log(e)
        const error = e.toString()
        if (error.includes('ExistingValue')) throw new Error('Index already exists.')
        if (error.includes('The user rejected')) throw new Error('Request denied by user. Please try to sign again.')
        if (error.includes('UnexpectedSize')) throw new Error('Invalid arguments length.')
        if (error.includes('Error(Contract, #10)')) throw new Error('Insufficient funds.')
        if (error.includes('invoke non-existent contract function')) throw new Error('Contract function does not exist.')
        if (error.includes('MissingValue')) throw new Error('Contract not found.')
        throw new Error('Failed to fetch factory data.', e)
      }
    }, [sorobanContext, factoryAddress])
}