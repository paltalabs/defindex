import * as StellarSdk from '@stellar/stellar-sdk';
import { useCallback } from "react";
import { useUser } from "@/contexts/UserContext";

export enum StrategyMethod {
    ASSET = "asset",
}

/**
 * Simulates a contract call for read-only operations
 * Uses Soroban RPC to invoke contract methods without signing
 */
async function simulateContractCall(
    rpcUrl: string,
    networkPassphrase: string,
    contractAddress: string,
    method: string,
    args: StellarSdk.xdr.ScVal[] = []
): Promise<StellarSdk.xdr.ScVal> {
    const server = new StellarSdk.rpc.Server(rpcUrl);
    const contract = new StellarSdk.Contract(contractAddress);

    // Create a temporary source account for simulation
    const sourceKeypair = StellarSdk.Keypair.random();
    const sourceAccount = new StellarSdk.Account(sourceKeypair.publicKey(), "0");

    // Build the transaction
    const transaction = new StellarSdk.TransactionBuilder(sourceAccount, {
        fee: "100",
        networkPassphrase,
    })
        .addOperation(contract.call(method, ...args))
        .setTimeout(30)
        .build();

    // Simulate the transaction
    const simulation = await server.simulateTransaction(transaction);

    if (StellarSdk.rpc.Api.isSimulationError(simulation)) {
        throw new Error(`Simulation failed: ${simulation.error}`);
    }

    if (!StellarSdk.rpc.Api.isSimulationSuccess(simulation)) {
        throw new Error('Simulation did not return a successful result');
    }

    // Extract the result
    const result = simulation.result?.retval;
    if (!result) {
        throw new Error('No return value from simulation');
    }

    return result;
}

export function useStrategyCallback() {
    const { networkConfig } = useUser();

    return useCallback(
        async (
            address: string,
            method: StrategyMethod,
            args?: StellarSdk.xdr.ScVal[],
            // eslint-disable-next-line @typescript-eslint/no-unused-vars
            _signAndSend?: boolean
        ): Promise<StellarSdk.xdr.ScVal> => {
            try {
                const result = await simulateContractCall(
                    networkConfig.sorobanRpcUrl,
                    networkConfig.networkPassphrase,
                    address,
                    method,
                    args
                );
                return result;
            } catch (e: unknown) {
                const error = String(e);
                if (error.includes('non-existing value for contract instance')) {
                    throw new Error(`Strategy: ${address} not found.`);
                }
                throw new Error('Failed to interact with strategy. If the problem persists, please contact support.');
            }
        },
        [networkConfig]
    );
}
