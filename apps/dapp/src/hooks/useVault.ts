import { useSorobanReact } from "@soroban-react/core";
import { useCallback } from "react";
import * as StellarSdk from '@stellar/stellar-sdk';
import { scValToNative } from "@stellar/stellar-sdk";
import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import { VaultData } from "@/store/lib/features/walletStore";


export enum VaultMethod {
    DEPOSIT = "deposit",
    BALANCE = "balance",
    WITHDRAW = "withdraw",
    GETMANAGER = "get_manager",
    GETEMERGENCYMANAGER = "get_emergency_manager",
    GETFEERECEIVER = "get_fee_receiver",
    EMERGENCY_WITHDRAW = "emergency_withdraw",
    GETNAME= "name",
    GETTOTALVALUES = "total_supply",
    GETASSETS = "get_assets",
}   

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export function useVaultCallback() {
    const sorobanContext = useSorobanReact();


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
export const useVault = (vaultAddress?: string | undefined) => {
    const vault = useVaultCallback();
    const getVaultInfo = async (vaultAddress: string) => {
    if (!vaultAddress) return;
    try {
        const [manager, emergencyManager, feeReceiver, name, strategies, totalValues] = await Promise.all([
            getVaultManager(vaultAddress),
            getVaultEmergencyManager(vaultAddress),
            getVaultFeeReceiver(vaultAddress),
            getVaultName(vaultAddress),
            getVaultAssets(vaultAddress),
            getVaultTotalValues(vaultAddress),
        ]);
        const parsedTotalValues = Number(totalValues) / 10 ** 7;
        const newData: VaultData = {
            name: name || '',
            address: vaultAddress,
            manager: manager,
            emergencyManager: emergencyManager,
            feeReceiver: feeReceiver,
            strategies: strategies[0].strategies || [],
            totalValues: parsedTotalValues || 0,
        }
    return newData
    } catch (error) {
        console.error(error);
    }
}

    const getVaultManager = async (selectedVault: string) => {
        try {
        const manager = await vault(VaultMethod.GETMANAGER, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return manager;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultEmergencyManager = async (selectedVault: string) => {
        try {
        const emergencyManager = await vault(VaultMethod.GETEMERGENCYMANAGER, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return emergencyManager;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultFeeReceiver = async (selectedVault: string) => {
        try {
        const feeReceiver = await vault(VaultMethod.GETFEERECEIVER, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return feeReceiver;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultName = async (selectedVault: string) => {
        try {
        const name = await vault(VaultMethod.GETNAME, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return name;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultAssets = async (selectedVault: string) => {
        try {
        const assets = await vault(VaultMethod.GETASSETS, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return assets;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultTotalValues = async (selectedVault: string) => {
        try {
        const totalValues = await vault(VaultMethod.GETTOTALVALUES, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return totalValues;
        } catch (error) {
        console.error(error);
        }
    }

    const vaultInfo = getVaultInfo(vaultAddress!);
    return { vaultInfo, getVaultInfo, getVaultManager, getVaultEmergencyManager, getVaultFeeReceiver, getVaultName, getVaultAssets, getVaultTotalValues };
}