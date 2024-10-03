import { SorobanContextType, useSorobanReact } from "@soroban-react/core";
import { useCallback, useEffect, useState } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { TxResponse, contractInvoke } from '@soroban-react/contracts';

export enum StrategyMethod {
    INITIALIZE = "initialize",
    ASSET = "asset",
    DEPOSIT = "deposit",
    HARVEST = "harvest",
    BALANCE = "balance",
    WITHDRAW = "withdraw",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

// Type guard to check if result is TxResponse
function isTxResponse(result: any): result is TxResponse {
    return result && typeof result === 'object' && 'status' in result;
}

// Type guard to check if result is StellarSdk.xdr.ScVal
function isScVal(result: any): result is StellarSdk.xdr.ScVal {
    return result instanceof StellarSdk.xdr.ScVal;
}


export function useStrategyCallback() {
    const sorobanContext = useSorobanReact();

    return useCallback(
        async (address: string, method: StrategyMethod, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
            console.log("Strategy Callback called");
            try {
                const result = (await contractInvoke({
                    contractAddress: address,
                    method: method,
                    args: args,
                    sorobanContext,
                    signAndSend: signAndSend,
                    reconnectAfterTx: false,
                }));
                console.log("Strategy Callback result", result);
                if (!signAndSend) return result;
                if (isTxResponse(result)) {
                    if (
                        isObject(result) &&
                        result?.status !== StellarSdk.SorobanRpc.Api.GetTransactionStatus.SUCCESS
                    ) throw result;
                    return result;
                }
            } catch (e: any) {
                console.log(e);
                const error = e.toString();
                if (error.includes('ExistingValue')) throw new Error('Strategy already initialized.');
                if (error.includes('Sign')) throw new Error('Request denied by user. Please try to sign again.');
                throw new Error('Failed to interact with strategy. If the problem persists, please contact support.');
            }
        }, [sorobanContext]
    );
}
