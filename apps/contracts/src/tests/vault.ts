/**
 * Description: Tests the vault by creating a new user, airdropping funds, and making a deposit.
 *
 * @param {string} deployedVault - The address of the deployed vault contract.
 * @returns {Promise<void>} Logs the result of the deposit action.
 * @throws Will throw an error if the deposit fails or any step encounters an issue.
 * @example
 * await test_vault("CCE7MLKC7R6TIQA37A7EHWEUC3AIXIH5DSOQUSVAARCWDD7257HS4RUG");
 */

// ./tests/vault.ts

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

export async function test_vault(deployedVault: string, user?: Keypair) {
    const network = process.argv[2];
    const loadedConfig = config(network);

    // Create and fund a new user account
    const newUser = user ? user : Keypair.random();
    console.log('🚀 ~ test_vault ~ newUser.publicKey():', newUser.publicKey());
    console.log('🚀 ~ test_vault ~ newUser.secret():', newUser.secret());

    if (network !== "mainnet") await airdropAccount(newUser);

    console.log("New user publicKey:", newUser.publicKey());

    // Define deposit parameters
    const depositAmount = BigInt(10000000); // 1 XLM in stroops (1 XLM = 10^7 stroops)
    const amountsDesired = [depositAmount];
    const amountsMin = [BigInt(0)]; // Minimum amount for transaction to succeed

    const depositParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec(amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))),
        xdr.ScVal.scvVec(amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))),
        (new Address(newUser.publicKey())).toScVal()
    ];
    // console.log('🚀 ~ test_vault ~ depositParams:', depositParams);

    try {

        // Check the user's balance after the deposit
        const balanceBefore = await getDfTokenBalance(deployedVault, newUser.publicKey(), newUser);
        console.log("🔢 « dfToken balance before deposit:", balanceBefore)
    } catch (error) {
        console.error("❌ Balance failed:", error);
    }
    try {
        // TODO: Would this work on Mainnet or Standalone? How does it know which network to use?
        const result = await invokeCustomContract(
            deployedVault,
            "deposit",
            depositParams,
            newUser
        );

        console.log("🚀 « Deposit successful:", scValToNative(result.returnValue));

    } catch (error) {
        console.error("❌ Deposit failed:", error);
    }
    try {

        // Check the user's balance after the deposit
        const balanceAfter = await getDfTokenBalance(deployedVault, newUser.publicKey(), newUser);
        console.log("🔢 « dfToken balance after deposit:", balanceAfter)
    } catch (error) {
        console.error("❌ Balance failed:", error);
    }
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
