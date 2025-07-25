import { Address, Keypair, scValToNative, xdr } from "@stellar/stellar-base";
import { invokeCustomContract, createToolkit } from "soroban-toolkit";

export const toolkitLoader = createToolkit({
  adminSecret: process.env.ADMIN_SECRET!,
  customNetworks: [
    {
      network: 'mainnet',
      sorobanRpcUrl: process.env.SOROBAN_RPC_URL!,
      networkPassphrase: 'Public Global Stellar Network ; September 2015',
      horizonRpcUrl: 'https://horizon.stellar.org',
    },
  ],
  addressBookPath: './public',
  verbose: 'none'
});

export const toolkit = toolkitLoader.getNetworkToolkit('testnet');
export const addressFor = (contractId:string) => toolkit.addressBook.getContractId(contractId);

export const simulateInvocation = (contractId: string, methodName: string, args: xdr.ScVal[]) => invokeCustomContract(
  toolkit,
  contractId,
  methodName,
  args,
  true,
  Keypair.fromSecret(process.env.ADMIN_SECRET!)
);

export const getBalance = async (contractKey: string,  addressKey: string) => {
  try {
    const rawBalance = await simulateInvocation(
      addressFor(contractKey),
      'balance',
      [new Address(addressFor(addressKey)).toScVal()],
    );
    const parsedBalance = scValToNative(rawBalance.result.retval);
    return parsedBalance;
  } catch (error: any) {
    throw new Error(error);
  }
};