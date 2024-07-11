import { useSorobanReact } from "@soroban-react/core";
import { useCallback } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { TxResponse, contractInvoke } from '@soroban-react/contracts';

export enum FactoryMethod {
    CREATE_DEFINDEX = "create_defindex",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export function useFactoryCallback() {
    const sorobanContext = useSorobanReact();
    const factoryAddress = "CD2YCYJZOZPN22PESYMUYDKDSW63OQ2EJOW27BOVFXQ4ZTVX6ELMIJ6P"

    return useCallback(
        async (method: FactoryMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
            console.log("Factory Callback called")
            const result = (await contractInvoke({
                contractAddress: factoryAddress as string,
                method: method,
                args: args,
                sorobanContext,
                signAndSend: signAndSend,
                reconnectAfterTx: false,
            })) as TxResponse;

            if (!signAndSend) return result;

            if (
                isObject(result) &&
                result?.status !== StellarSdk.SorobanRpc.Api.GetTransactionStatus.SUCCESS
            ) throw result;
            return result
        }
        , [sorobanContext, factoryAddress])
}