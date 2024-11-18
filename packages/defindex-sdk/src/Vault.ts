import { xdr, Address, nativeToScVal, scValToNative } from "@stellar/stellar-sdk";
import { TxResponse, contractInvoke } from '@soroban-react/contracts';
import { SorobanContextType } from "@soroban-react/core";

export enum SorobanNetwork {
    TESTNET = "TESTNET",
    PUBLIC = "PUBLIC"
}

export enum VaultMethod {
    DEPOSIT = "deposit",
    BALANCE = "balance",
    WITHDRAW = "withdraw",
    // Other methods as needed
}

interface VaultOptions {
    network: SorobanNetwork;
    contractId: string;
}

export class Vault {
    private contractId: string;

    constructor(options: VaultOptions) {
        this.contractId = options.contractId;
    }

    private async invokeContract(
        method: VaultMethod,
        args: xdr.ScVal[],
        sorobanContext: SorobanContextType,
        signAndSend: boolean = false,
        secretKey?: string
    ): Promise<TxResponse> {
        const result = await contractInvoke({
            contractAddress: this.contractId,
            method: method,
            args: args,
            sorobanContext,
            signAndSend,
            secretKey,
            reconnectAfterTx: false,
        }) as TxResponse;

        if (!signAndSend) return result;

        if (signAndSend && result.status !== "SUCCESS") {
            throw new Error(`Transaction failed with status: ${result.status}`);
        }
        return result;
    }

    async deposit(
        account: string,
        amount: number,
        signAndSend: boolean,
        sorobanContext: SorobanContextType,
        secretKey?: string
    ): Promise<string> {
        const args = [
            xdr.ScVal.scvVec([nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" })]),
            xdr.ScVal.scvVec([nativeToScVal(((amount * 0.9) * Math.pow(10, 7)), { type: "i128" })]),
            new Address(account).toScVal()
        ];
        const response = await this.invokeContract(
            VaultMethod.DEPOSIT,
            args,
            sorobanContext,
            signAndSend,
            secretKey
        );
        return response.txHash;
    }

    async balance(account: string, sorobanContext: SorobanContextType): Promise<number> {
        const args = [new Address(account).toScVal()];
        const response = await this.invokeContract(VaultMethod.BALANCE, args, sorobanContext);
        return scValToNative(response as any);
    }

    async withdraw(
        account: string,
        amount: number,
        signAndSend: boolean,
        sorobanContext: SorobanContextType,
        secretKey?: string
    ): Promise<string> {
        const args = [
            nativeToScVal((amount * Math.pow(10, 7)), { type: "i128" }),
            new Address(account).toScVal()
        ];
        const response = await this.invokeContract(VaultMethod.WITHDRAW, args, sorobanContext, signAndSend, secretKey);
        return response.txHash;
    }
}
