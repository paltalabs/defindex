/**
 * Description: Deposits a specified amount to the vault for a user and returns the user details along with pre- and post-deposit balances.
 *
 * @param {string} deployedVault - The address of the deployed vault contract.
 * @param {Keypair} [user] - The user Keypair making the deposit. If not provided, a new user will be created.
 * @returns {Promise<{ user: Keypair, balanceBefore: number, result: any, balanceAfter: number }>} Returns an object with the user, balance before, deposit result, and balance after.
 * @throws Will throw an error if the deposit fails or any step encounters an issue.
 * @example
 * const { user, balanceBefore, result, balanceAfter } = await depositToVault("CCE7MLKC7R6TIQA37A7EHWEUC3AIXIH5DSOQUSVAARCWDD7257HS4RUG", user);
 */

import {
    Address,
    nativeToScVal,
    scValToNative,
    xdr,
    Keypair,
    Networks
} from "@stellar/stellar-sdk";
import { airdropAccount, invokeCustomContract } from "../utils/contract.js";
import { randomBytes } from "crypto";
import { config } from "../utils/env_config.js";

const network = process.argv[2];

export async function depositToVault(deployedVault: string, user?: Keypair) {
    // Create and fund a new user account if not provided
    const newUser = user ? user : Keypair.random();
    console.log('🚀 ~ depositToVault ~ newUser.publicKey():', newUser.publicKey());
    console.log('🚀 ~ depositToVault ~ newUser.secret():', newUser.secret());

    if (network !== "mainnet") await airdropAccount(newUser);
    console.log("New user publicKey:", newUser.publicKey());

    let balanceBefore: number;
    let balanceAfter: number;
    let result: any;

    // Define deposit parameters
    const depositAmount = BigInt(10000000); // 1 XLM in stroops (1 XLM = 10^7 stroops)
    const amountsDesired = [depositAmount];
    const amountsMin = [BigInt(0)]; // Minimum amount for transaction to succeed

    const depositParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec(amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))),
        xdr.ScVal.scvVec(amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))),
        (new Address(newUser.publicKey())).toScVal()
    ];

    try {
        // Check the user's balance before the deposit
        balanceBefore = await getDfTokenBalance(deployedVault, newUser.publicKey(), newUser);
        console.log("🔢 « dfToken balance before deposit:", balanceBefore);
    } catch (error) {
        console.error("❌ Balance check before deposit failed:", error);
        throw error;
    }

    try {
        result = await invokeCustomContract(
            deployedVault,
            "deposit",
            depositParams,
            newUser
        );
        console.log("🚀 « Deposit successful:", scValToNative(result.returnValue));
    } catch (error) {
        console.error("❌ Deposit failed:", error);
        throw error;
    }

    try {
        // Check the user's balance after the deposit
        balanceAfter = await getDfTokenBalance(deployedVault, newUser.publicKey(), newUser);
        console.log("🔢 « dfToken balance after deposit:", balanceAfter);
    } catch (error) {
        console.error("❌ Balance check after deposit failed:", error);
        throw error;
    }

    return { user: newUser, balanceBefore, result, balanceAfter };
}


/**
 * Description: Retrieves the dfToken balance for a specified user from the vault contract.
 *
 * @param {string} deployedVault - The address of the deployed vault contract.
 * @param {string} userPublicKey - The public key of the user whose balance to check.
 * @returns {Promise<number>} The balance of dfTokens for the specified user.
 * @throws Will throw an error if the balance retrieval fails.
 * @example
 * const balance = await getDfTokenBalance("CCE7MLKC7R6TIQA37A7EHWEUC3AIXIH5DSOQUSVAARCWDD7257HS4RUG", "GB6JL...");
 */

export async function getDfTokenBalance(deployedVault: string, userPublicKey: string, source?: Keypair): Promise<number> {
    const userAddress = new Address(userPublicKey).toScVal();
    const methodName = "balance"; // Assumes a standard token interface

    try {
        const result = await invokeCustomContract(
            deployedVault,
            methodName,
            [userAddress],
            source ? source : Keypair.random(),  // No specific source is needed as we are just querying the balance
            true   // Set to simulate mode if testing on an uncommitted transaction
        );

        const balance = scValToNative(result.result.retval)
        return balance;
    } catch (error) {
        console.error(`Failed to retrieve dfToken balance for user ${userPublicKey}:`, error);
        throw error;
    }
}

/**
 * Description: Withdraws a specified amount from the vault for the user and returns the pre- and post-withdrawal balances.
 *
 * @param {string} deployedVault - The address of the deployed vault contract.
 * @param {BigInt} withdrawAmount - The amount in stroops to withdraw (1 XLM = 10^7 stroops).
 * @param {Keypair} user - The user Keypair requesting the withdrawal.
 * @returns {Promise<{ balanceBefore: number, result: any, balanceAfter: number }>} Returns an object with balance before, the withdrawal result, and balance after.
 * @throws Will throw an error if the withdrawal fails or any step encounters an issue.
 * @example
 * const { balanceBefore, result, balanceAfter } = await withdrawFromVault("CCE7MLKC7R6TIQA37A7EHWEUC3AIXIH5DSOQUSVAARCWDD7257HS4RUG", 10000000, user);
 */

export async function withdrawFromVault(deployedVault: string, withdrawAmount: BigInt, user: Keypair) {
    console.log('🚀 ~ withdrawFromVault ~ User publicKey:', user.publicKey());

    let balanceBefore: number;
    let balanceAfter: number;
    let result: any;

    try {
        // Check the user's balance before the withdrawal
        balanceBefore = await getDfTokenBalance(deployedVault, user.publicKey(), user);
        console.log("🔢 « dfToken balance before withdraw:", balanceBefore);
    } catch (error) {
        console.error("❌ Balance check before withdraw failed:", error);
        throw error;
    }

    // Define withdraw parameters
    const amountsToWithdraw = [withdrawAmount];
    const withdrawParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec(amountsToWithdraw.map((amount) => nativeToScVal(amount, { type: "i128" }))),
        (new Address(user.publicKey())).toScVal()
    ];

    try {
        result = await invokeCustomContract(
            deployedVault,
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
        balanceAfter = await getDfTokenBalance(deployedVault, user.publicKey(), user);
        console.log("🔢 « dfToken balance after withdraw:", balanceAfter);
    } catch (error) {
        console.error("❌ Balance check after withdraw failed:", error);
        throw error;
    }

    return { balanceBefore, result, balanceAfter };
}
