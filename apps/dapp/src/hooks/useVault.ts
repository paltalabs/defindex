import { useSorobanReact } from "@soroban-react/core";
import { useCallback } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import { useAppSelector } from "@/store/lib/storeHooks";

export enum VaultMethod {
    DEPOSIT = "deposit",
    BALANCE = "balance",
    WITHDRAW = "withdraw",
    GETMANAGER = "get_manager",
    GETEMERGENCYMANAGER = "get_emergency_manager",
    GETFEERECEIVER = "get_fee_receiver",
    EMERGENCY_WITHDRAW = "emergency_withdraw",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export function useVaultCallback() {
    const sorobanContext = useSorobanReact();
    const sorobanContextTestnet = sorobanContext
    const activeChain = { id: "testnet", name: "testnet", networkPassphrase: "Test SDF Network ; September 2015", sorobanRpcUrl: "https://soroban-testnet.stellar.org/", network: "testnet", networkUrl: "https://horizon-testnet.stellar.org" } // REMOVE_THIS

    sorobanContextTestnet.activeChain = activeChain

    return useCallback(
        async (method: VaultMethod, address: string, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
            const result = (await contractInvoke({
                contractAddress: address,
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
        , [sorobanContext])
}