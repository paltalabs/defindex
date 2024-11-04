import {
    Address,
    scValToNative,
    xdr,
    Keypair,
    nativeToScVal,
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



export async function depositToStrategy(deployedStrategy: string, user: Keypair, amount: number) {

    let balanceBefore: number;
    let balanceAfter: number;
    let result: any;

    // Define deposit parameters
    const depositAmount = BigInt(amount); 
    const amountsDesired = [depositAmount];
    const amountsMin = [BigInt(0)]; // Minimum amount for transaction to succeed

    const depositParams: xdr.ScVal[] = [
        nativeToScVal(depositAmount, { type: "i128" }),
        (new Address(user.publicKey())).toScVal()
    ];

    try {
        // Check the user's strategy
        balanceBefore = await checkUserBalance(deployedStrategy, user.publicKey(), user);
        console.log("🔢 « strategy balance before deposit:", balanceBefore);
    } catch (error) {
        console.error("❌ Balance check before deposit failed:", error);
        throw error;
    }

    try {
        result = await invokeCustomContract(
            deployedStrategy,
            "deposit",
            depositParams,
            user
        );
        console.log("🚀 « Deposit successful:", scValToNative(result.returnValue));
    } catch (error) {
        console.error("❌ Deposit failed:", error);
        throw error;
    }

    try {
        // Check the user's balance after the deposit
        balanceAfter = await checkUserBalance(deployedStrategy, user.publicKey(), user);
        console.log("🔢 « dfToken balance after deposit:", balanceAfter);
    } catch (error) {
        console.error("❌ Balance check after deposit failed:", error);
        throw error;
    }

    return { user, balanceBefore, result, balanceAfter };
}


export async function withdrawFromStrategy(deployedStrategy: string, user: Keypair, amount: number) {
    let balanceBefore: number;
    let balanceAfter: number;
    let result: any;

    // Define withdraw parameters
    const withdrawAmount = BigInt(amount);
    const amountsToWithdraw = [withdrawAmount];

    const withdrawParams: xdr.ScVal[] = [
        nativeToScVal(amount, { type: "i128" }),
        (new Address(user.publicKey())).toScVal()
    ];

    try {
        // Check the user's balance before the withdrawal
        balanceBefore = await checkUserBalance(deployedStrategy, user.publicKey(), user);
        console.log("🔢 « strategy balance before withdraw:", balanceBefore);
    } catch (error) {
        console.error("❌ Balance check before withdraw failed:", error);
        throw error;
    }

    try {
        result = await invokeCustomContract(
            deployedStrategy,
            "withdraw",
            withdrawParams,
            user
        );
        console.log("🚀 « Withdrawal successful:", scValToNative(result.returnValue));
    } catch (error) {
        console.error("❌ Withdrawal failed:", error);
        throw error;
    }

    try {
        // Check the user's balance after the withdrawal
        balanceAfter = await checkUserBalance(deployedStrategy, user.publicKey(), user);
        console.log("🔢 « strategy balance after withdraw:", balanceAfter);
    } catch (error) {
        console.error("❌ Balance check after withdraw failed:", error);
        throw error;
    }

    return { user, balanceBefore, result, balanceAfter };
}