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

export async function test_vault(deployedVault: string) {
    const network = process.argv[2];
    const loadedConfig = config(network);

    // Create and fund a new user account
    const newUser = Keypair.random();
    if (network !== "mainnet") await airdropAccount(newUser);

    console.log("New user publicKey:", newUser.publicKey());

    // Define deposit parameters
    const depositAmount = BigInt(10000000); // 1 XLM in stroops (1 XLM = 10^7 stroops)
    const amountsDesired = [depositAmount];
    const amountsMin = [BigInt(0)]; // Minimum amount for transaction to succeed

    const depositParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec(amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))),
        xdr.ScVal.scvVec(amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))),
        new Address(newUser.publicKey()).toScVal()
    ];

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
}
