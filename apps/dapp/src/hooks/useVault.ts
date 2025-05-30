import { contractInvoke, useSorobanReact } from 'stellar-react';
import * as StellarSdk from '@stellar/stellar-sdk';
import { xdr } from '@stellar/stellar-sdk';
import { scValToNative } from "@stellar/stellar-sdk";
import { useCallback } from "react";

import { AssetAmmount, Vault } from "@/contexts";
import { TxResponse } from 'stellar-react/dist/contracts/types';
import { getTokenSymbol } from '@/helpers/getTokenInfo';

export enum VaultMethod {
    // VaultTrait methods
    DEPOSIT = "deposit",
    WITHDRAW = "withdraw",
    RESCUE = "rescue",
    PAUSE = "pause_strategy",
    UNPAUSE = "unpause_strategy",
    GET_ASSETS = "get_assets",
    TOTAL_MANAGED_FUNDS = "fetch_total_managed_funds",
    GET_ASSET_AMOUNT = "get_asset_amounts_per_shares",
    GET_FEES = "get_fees",
    REPORT = "report",

    // AdminInterfaceTrait methods
    SET_FEE_RECEIVER = "set_fee_receiver",
    GET_FEE_RECEIVER = "get_fee_receiver",
    SET_MANAGER = "set_manager",
    GET_MANAGER = "get_manager",
    SET_EMERGENCY_MANAGER = "set_emergency_manager",
    GET_EMERGENCY_MANAGER = "get_emergency_manager",
    SET_REBALANCE_MANAGER = "set_rebalance_manager",
    GET_REBALANCE_MANAGER = "get_rebalance_manager",
    UPGRADE = "upgrade",

    // VaultManagementTrait methods
    REBALANCE = "rebalance",
    LOCK_FEES = "lock_fees",
    RELEASE_FEES = "release_fees",
    DISTRIBUTE_FEES = "distribute_fees",

    // Additional methods
    BALANCE = "balance",
    GET_NAME = "name",
    GET_SYMBOL = "symbol",
    TOTAL_SUPPLY = "total_supply",
}

const isObject = (val: unknown) => typeof val === 'object' && val !== null && !Array.isArray(val);

export function useVaultCallback() {
    const sorobanContext = useSorobanReact();
    return useCallback(
        async (method: VaultMethod, address: string, args?: StellarSdk.xdr.ScVal[], signAndSend?: boolean) => {
            try {
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
                    result?.status !== StellarSdk.rpc.Api.GetTransactionStatus.SUCCESS
                ) throw result;
                return result
            } catch (e: any) {
                console.log(e)
                const error = e.toString()
                if (error.includes('The user rejected')) throw new Error('Request denied by user. Please try to sign again.')
                if (error.includes('UnexpectedSize')) throw new Error('Invalid arguments length.')
                if (error.includes('Error(Contract, #10)')) throw new Error('Insufficient funds.')
                if (error.includes('Error(Contract, #117)')) throw new Error('Insufficient amount.')
                if (error.includes('Error(Contract, #128)')) throw new Error('Unwind more than available.')
                if (error.includes('Error(Contract, #130)')) throw new Error('Action requires authorization.')
                if (error.includes('Error(Contract, #144)')) throw new Error('Strategy paused.')
                throw new Error('Failed to process the request.', error)
            }
        }
        , [sorobanContext])
}
export const useVault = (vaultAddress?: string | undefined) => {
    const vault = useVaultCallback();
    const sorobanContext = useSorobanReact();
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
            investedFunds,
            fees
        ] = await Promise.all([
            getVaultManager(vaultAddress),
            getVaultEmergencyManager(vaultAddress),
            getVaultFeeReceiver(vaultAddress),
            getVaultName(vaultAddress),
            getVaultAssets(vaultAddress),
            getTVL(vaultAddress),
            getVaultTotalSupply(vaultAddress),
            getIdleFunds(vaultAddress),
            getInvestedFunds(vaultAddress),
            getFees(vaultAddress)
        ]);
        for (let asset of assets){
            const symbol = await getTokenSymbol(asset.address, sorobanContext);
            if(symbol === 'native') asset.symbol = 'XLM'
            else asset.symbol = symbol
        }
        getInvestedFunds(vaultAddress);
        const newData: Vault = {
            name: name || '',
            symbol: name || '',
            address: vaultAddress,
            vaultManager: manager,
            rebalanceManager: manager,
            emergencyManager: emergencyManager,
            feeReceiver: feeReceiver,
            assetAllocation: assets || [],
            feePercent: fees || [50,0],
            upgradable: true,

        }
    return newData
    } catch (error) {
        console.error(error);
    }
}

    const getVaultManager = async (selectedVault: string) => {
        try {
        const manager = await vault(VaultMethod.GET_MANAGER, selectedVault, undefined, false).then((res: any) => scValToNative(res as xdr.ScVal));
        return manager;
        } catch (error) {
        console.error(error);
        throw new Error('Failed to fetch vault manager');
        }
    }
    const getVaultEmergencyManager = async (selectedVault: string) => {
        try {
        const emergencyManager = await vault(VaultMethod.GET_EMERGENCY_MANAGER, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return emergencyManager;
        } catch (error) {
        console.error(error);
        throw new Error('Failed to fetch vault emergency manager');
        }
    }
    const getVaultFeeReceiver = async (selectedVault: string) => {
        try {
        const feeReceiver = await vault(VaultMethod.GET_FEE_RECEIVER, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return feeReceiver;
        } catch (error) {
        console.error(error);
        throw new Error('Failed to fetch vault fee receiver');
        }
    }
    const getVaultName = async (selectedVault: string) => {
        try {
        const name = await vault(VaultMethod.GET_NAME, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return name;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultAssets = async (selectedVault: string) => {
        try {
        const assets = await vault(VaultMethod.GET_ASSETS, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        return assets;
        } catch (error) {
        console.error(error);
        }
    }
    const getVaultTotalSupply = async (selectedVault: string) => {
        try {
        const totalSupply = await vault(VaultMethod.TOTAL_SUPPLY, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        const parsedTotalSupply = Number(totalSupply) / 10 ** 7;
        return parsedTotalSupply;
        } catch (error) {
        console.error(error);
        }
    }
    interface TotalManagedFunds {
        asset: string;
        idle_amounts: number;
        invested_amounts: number;
        strategy_allocation: any[];
        total_amount: number;
    }

    const getTVL = async (selectedVault: string) => {
        try {
        const totalValues = await vault(VaultMethod.TOTAL_MANAGED_FUNDS, selectedVault, undefined, false).then((res: any) => scValToNative(res));
        const {total_amount:value} = Object.values(totalValues)[0] as TotalManagedFunds;
        const parsedValue = Number(value) / 10 ** 7;
        return parsedValue;
        } catch (error) {
        console.error(error);
        }
    }
    const getUserBalance = async (vaultAddress: string, address: string) => {
        try {
            const formattedAddress = new StellarSdk.Address(address).toScVal();
            const dfTokens = await vault(VaultMethod.BALANCE, vaultAddress, [formattedAddress], false).then((res: any) => res);
            const parsedDfTokens = scValToNative(dfTokens);
            if(parsedDfTokens == '0') return 0;
            const amount = await vault(VaultMethod.GET_ASSET_AMOUNT, vaultAddress, [dfTokens], false).then((res: any) => scValToNative(res));
            const amountValue = amount[0];
            const parsedAmount = Number(amountValue) / 10 ** 7;
        return parsedAmount;
        } catch (error) {
        console.error(error);
        }
    }
    
    const getIdleFunds = async (vaultAddress: string) => {
        try {
            const assets = await getVaultAssets(vaultAddress);
            console.log('🚀 « assets:', assets);
            const idleFunds: AssetAmmount[] = [];
            for (const asset of assets) {
                const rawBalance: any = await contractInvoke({
                    contractAddress: asset.address,
                    method: "balance",
                    args: [new StellarSdk.Address(vaultAddress).toScVal()],
                    sorobanContext,
                    signAndSend: false,
                });
                const balance = scValToNative(rawBalance);
                console.log('🚀 « balance:', balance);
                idleFunds.push({ address: asset.address, amount: Number(balance) / 10 ** 7 });
            }
            return idleFunds;
        } catch (error) {
            console.error(error);
        }
    }

    const getInvestedFunds = async (vaultAddress: string) => {
        try {
        const rawInvestedFunds = await vault(VaultMethod.TOTAL_MANAGED_FUNDS, vaultAddress, undefined, false).then((res: any) => scValToNative(res));
        const assets = Object.keys(rawInvestedFunds);
        const investedFunds: AssetAmmount[] = [];
        assets.forEach((asset)=>{
            const address = rawInvestedFunds[asset].asset;
            const amount =  Number(rawInvestedFunds[asset].invested_amount) / 10 ** 7;
            investedFunds.push({address: address, amount: amount});
        })
        return investedFunds;
        } catch (error) {
            console.error(error);
        }
    }

    const getFees = async (vaultAddress: string) => {
        try {
        const fees = await vault(VaultMethod.GET_FEES, vaultAddress, undefined, false).then((res: any) => scValToNative(res));
        return fees || [50,0];
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
        getFees
    };
}