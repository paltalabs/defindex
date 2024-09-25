import { useSorobanReact } from "@soroban-react/core";
import { useCallback } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import configs from '@/constants/constants.json'

export enum FactoryMethod {
    CREATE_DEFINDEX = "create_defindex",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export function useFactoryCallback() {
    const sorobanContext = useSorobanReact();
    const factoryAddress = ()=>{
      const address = configs.filter((item:any)=>item.network === sorobanContext.activeChain?.network)[0]?.factory
      return address;
    }

    return useCallback(
        async (method: FactoryMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
            console.log("Factory Callback called")
            try {
              const result = (await contractInvoke({
                contractAddress: factoryAddress() as string,
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
              if(error.includes('ExistingValue')) throw new Error('Index already exists.')
              if(error.includes('Sign')) throw new Error('Request denied by user. Please try to sign again.')
              throw new Error('Failed to create index. If the problem persists, please contact support.')
            }
        }, [sorobanContext, factoryAddress])
}