import { SorobanContextType, useSorobanReact } from "@soroban-react/core";
import { useCallback, useEffect, useState } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import { getRemoteConfig } from "@/helpers/getRemoteConfig";
import { fetchFactoryAddress } from "@/utils/factory";

export enum FactoryMethod {
  CREATE_DEFINDEX_VAULT = "create_defindex_vault",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export const useFactory = () => {
  const sorobanContext: SorobanContextType = useSorobanReact();
  const { activeChain } = sorobanContext;
  const [address, setAddress] = useState<string>();

  useEffect(() => {
    if (!sorobanContext) return;

    if (activeChain?.name?.toLowerCase() !== 'public' && activeChain?.name?.toLowerCase() !== 'testnet') {
      throw new Error(`Invalid network when fetching factory address: ${activeChain?.id}. It should be mainnet or testnet`);
    }

    fetchFactoryAddress(activeChain?.id as string).then(
      (factoryAddress) => {
        setAddress(factoryAddress);
      }
    ).catch((error) => {
      throw new Error(`Failed to fetch factory address: ${error}`);
    });

  }, [activeChain?.id, sorobanContext]);

  return { address };
}

export function useFactoryCallback() {
  const sorobanContext = useSorobanReact();
  const { address: factoryAddress } = useFactory();

  return useCallback(
    async (method: FactoryMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
      console.log("Factory Callback called")
      try {
        const result = (await contractInvoke({
          contractAddress: factoryAddress as string,
          method: method,
          args: args,
          sorobanContext,
          signAndSend: signAndSend,
          reconnectAfterTx: false,
        })) as TxResponse;
        console.log("Factory Callback result", result)
        if (!signAndSend) return result;

        if (
          isObject(result) &&
          result?.status !== StellarSdk.SorobanRpc.Api.GetTransactionStatus.SUCCESS
        ) throw result;
        return result
      } catch (e: any) {
        console.log(e)
        const error = e.toString()
        if (error.includes('ExistingValue')) throw new Error('Index already exists.')
        if (error.includes('Sign')) throw new Error('Request denied by user. Please try to sign again.')
        throw new Error('Failed to create index. If the problem persists, please contact support.')
      }
    }, [sorobanContext, factoryAddress])
}