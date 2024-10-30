import {
    Address,
    scValToNative,
    xdr,
    Keypair
} from "@stellar/stellar-sdk";
import { invokeCustomContract } from "../utils/contract.js";

/**
 * Description: Retrieves the balance for a specified user from the contract.
 *
 * @param {string} contractAddress - The address of the deployed contract.
 * @param {string} userPublicKey - The public key of the user whose balance to check.
 * @param {Keypair} [source] - Optional; the Keypair instance for authorization if required.
 * @returns {Promise<number>} The balance of the specified user in the contract.
 * @throws Will throw an error if the balance retrieval fails.
 * @example
 * const balance = await checkUserBalance("CCE7MLKC7R6TIQA37A7EHWEUC3AIXIH5DSOQUSVAARCWDD7257HS4RUG", "GB6JL...");
 */

export async function checkUserBalance(contractAddress: string, userPublicKey: string, source?: Keypair): Promise<number> {
    const userAddress = new Address(userPublicKey).toScVal();
    const methodName = "balance";

    try {
        // Call the `balance` method from the contract
        const result = await invokeCustomContract(
            contractAddress,
            methodName,
            [userAddress],
            source ? source : Keypair.random()
        );

        // Convert the result to a native JavaScript number
        const balance = scValToNative(result.returnValue) as number;
        console.log(`Balance for user ${userPublicKey}:`, balance);

        return balance;
    } catch (error) {
        console.error(`Failed to retrieve balance for user ${userPublicKey}:`, error);
        throw error;
    }
}
