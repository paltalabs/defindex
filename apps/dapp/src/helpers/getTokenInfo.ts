import { contractInvoke, SorobanContextType } from "stellar-react";
import { scValToNative, xdr } from "@stellar/stellar-sdk";


export const getTokenSymbol = async (
  tokenId: string,
  sorobanContext: SorobanContextType,
): Promise<string | null> => {
  try {
    let result = await contractInvoke({
      contractAddress: tokenId as string,
      method: 'symbol',
      args: [],
      sorobanContext,
    });

    return scValToNative(result as xdr.ScVal);
  } catch (error) {
    console.log(`Error fetching token symbol for ${tokenId}:`, error);
    return null;
  }
};
