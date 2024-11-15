import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import { useSorobanReact } from "@soroban-react/core";
import * as StellarSdk from '@stellar/stellar-sdk';
import { scValToNative } from "@stellar/stellar-sdk";
import { useCallback } from "react";

import { getTokenSymbol } from "@/helpers/getTokenInfo";
import { AssetAmmount, VaultData } from "@/store/lib/types";


export enum VaultMethod {
    DEPOSIT = "deposit",
    BALANCE = "balance",
    WITHDRAW = "withdraw",
    GETMANAGER = "get_manager",
    GETEMERGENCYMANAGER = "get_emergency_manager",
    GETFEERECEIVER = "get_fee_receiver",
    EMERGENCY_WITHDRAW = "emergency_withdraw",
    GETNAME= "name",
    TOTALMANAGEDFUNDS = "fetch_total_managed_funds",
    TOTALSUPPLY = "total_supply",
    GETASSETS = "get_assets",
    GETASSETAMMOUNT = "get_asset_amounts_for_dftokens",
    GETIDLEFUNDS = "fetch_current_idle_funds",
    GETINVESTEDFUNDS = "fetch_current_invested_funds",
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
    const sorobanContext = useSorobanReact();
    const {address} = sorobanContext;
    const getVaultInfo = async (vaultAddress: string) => {
    if (!vaultAddress) return;
    try {
        const [
            manager, 
            emergencyManager, 
            feeReceiver, 
            name, 
            assets,
            TVL,
            totalSupply,
            idleFunds,
            investedFunds
        ] = await Promise.all([
            getVaultManager(vaultAddress),
            getVaultEmergencyManager(vaultAddress),
            getVaultFeeReceiver(vaultAddress),
            getVaultName(vaultAddress),
            getVaultAssets(vaultAddress),
            getTVL(vaultAddress),
            getVaultTotalSupply(vaultAddress),
            getIdleFunds(vaultAddress),
            getInvestedFunds(vaultAddress)
        ]);
        for (let asset of assets){
            const symbol = await getTokenSymbol(asset.address, sorobanContext);
            if(symbol === 'native') asset.symbol = 'XLM';
        }
        getInvestedFunds(vaultAddress);
        const newData: VaultData = {
            name: name || '',
            address: vaultAddress,
            manager: manager,
            emergencyManager: emergencyManager,
            feeReceiver: feeReceiver,
            assets: assets || [],
            TVL: TVL || 0,
            totalSupply: totalSupply || 0,
            idleFunds: idleFunds || [],
            investedFunds: investedFunds || [],
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
    const getVaultTotalSupply = async (selectedVault: string) => {
        try {
        const totalSupply = await vault(VaultMethod.TOTALSUPPLY, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        const parsedTotalSupply = Number(totalSupply) / 10 ** 7;
        return parsedTotalSupply;
        } catch (error) {
        console.error(error);
        }
    }
    const getTVL = async (selectedVault: string) => {
        try {
        const totalValues = await vault(VaultMethod.TOTALMANAGEDFUNDS, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        const value = Object.values(totalValues)[0];
        const parsedValue = Number(value) / 10 ** 7;
        return parsedValue;
        } catch (error) {
        console.error(error);
        }
    }
    const getUserBalance = async (vaultAddress: string, address: string) => {
        try {
            const formattedAddress = new StellarSdk.Address(address).toScVal();
            const dfTokens = await vault(VaultMethod.BALANCE, vaultAddress, [formattedAddress], false).then((res: any) => scValToNative(res));
            if(Number(dfTokens) === 0) return 0;
            const amount = await vault(VaultMethod.GETASSETAMMOUNT, vaultAddress, [StellarSdk.nativeToScVal(dfTokens, {type: 'i128'})], false).then((res: any) => scValToNative(res));
            const amountValue = isObject(amount) ? Object.values(amount)[0] : 0;
            const parsedAmount = Number(amountValue) / 10 ** 7;
        return parsedAmount;
        } catch (error) {
        console.error(error);
        }
    }
    const getIdleFunds = async (vaultAddress: string) => {
        try {
        const rawIdleFunds = await vault(VaultMethod.GETIDLEFUNDS, vaultAddress, undefined, false).then((res: any) => scValToNative(res));
        const assets = Object.keys(rawIdleFunds);
        const idleFunds: AssetAmmount[] = [];
        assets.forEach((asset)=>{
            idleFunds.push({address: asset, amount:  Number(rawIdleFunds[asset]) / 10 ** 7})
        })
        console.log(idleFunds);
        return idleFunds;
        } catch (error) {
        console.error(error);
        }
    }
    const getInvestedFunds = async (vaultAddress: string) => {
        try {
        const rawInvestedFunds = await vault(VaultMethod.GETINVESTEDFUNDS, vaultAddress, undefined, false).then((res: any) => scValToNative(res));
        const assets = Object.keys(rawInvestedFunds);
        const investedFunds: AssetAmmount[] = [];
        assets.forEach((asset)=>{
            investedFunds.push({address: asset, amount:  Number(rawInvestedFunds[asset]) / 10 ** 7})
        })
        console.log(investedFunds);
        return investedFunds;
        } catch (error) {
        console.error(error);
        }
    }

    const vaultInfo = getVaultInfo(vaultAddress!);
    return { 
        vaultInfo, 
        getVaultInfo, 
        getVaultManager, 
        getVaultEmergencyManager, 
        getVaultFeeReceiver, 
        getVaultName, 
        getVaultAssets, 
        getVaultTotalSupply, 
        getUserBalance, 
        getTVL,
        getIdleFunds,
        getInvestedFunds, 
    };
}