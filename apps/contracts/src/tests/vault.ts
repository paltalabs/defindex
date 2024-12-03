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
    Keypair,
    nativeToScVal,
    scValToNative,
    xdr
} from "@stellar/stellar-sdk";
import { i128, u32, u64 } from "@stellar/stellar-sdk/contract";
import { airdropAccount, invokeCustomContract } from "../utils/contract.js";

const network = process.argv[2];

export async function depositToVault(deployedVault: string, amount: number[], user?: Keypair, invest?: boolean) {
    // Create and fund a new user account if not provided
    const newUser = user ? user : Keypair.random();
    const investBool = invest ? invest : false;
    console.log('🚀 ~ depositToVault ~ newUser.publicKey():', newUser.publicKey());
    console.log('🚀 ~ depositToVault ~ newUser.secret():', newUser.secret());

    if (network !== "mainnet") await airdropAccount(newUser);
    console.log("New user publicKey:", newUser.publicKey());

    let balanceBefore: number;
    let balanceAfter: number;
    let result: any;

    // Define deposit parameters
    const amountsDesired = amount.map((am) => BigInt(am)); // 1 XLM in stroops (1 XLM = 10^7 stroops)
    const amountsMin = amount.map((_) => BigInt(0));; // Minimum amount for transaction to succeed

    const depositParams: xdr.ScVal[] = [
        xdr.ScVal.scvVec(amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))),
        xdr.ScVal.scvVec(amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))),
        new Address(newUser.publicKey()).toScVal(),
        xdr.ScVal.scvBool(investBool)
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

    return { user: newUser, balanceBefore, result, balanceAfter, status:true };
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
 * @param {BigInt | number} withdrawAmount - The amount in stroops to withdraw (1 XLM = 10^7 stroops).
 * @param {Keypair} user - The user Keypair requesting the withdrawal.
 * @returns {Promise<{ balanceBefore: number, result: any, balanceAfter: number }>} Returns an object with balance before, the withdrawal result, and balance after.
 * @throws Will throw an error if the withdrawal fails or any step encounters an issue.
 * @example
 * const { balanceBefore, result, balanceAfter } = await withdrawFromVault("CCE7MLKC7R6TIQA37A7EHWEUC3AIXIH5DSOQUSVAARCWDD7257HS4RUG", 10000000, user);
 */

export async function withdrawFromVault(deployedVault: string, withdrawAmount: number, user: Keypair) {
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
    // const amountsToWithdraw = [BigInt(withdrawAmount)];
    // const withdrawParams: xdr.ScVal[] = [
    //     xdr.ScVal.scvVec(amountsToWithdraw.map((amount) => nativeToScVal(amount, { type: "i128" }))),
    //     (new Address(user.publicKey())).toScVal()
    // ];

    const withdrawParams: xdr.ScVal[] = [
        nativeToScVal(BigInt(withdrawAmount), { type: "i128" }),
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

/**
 * Retrieves the current idle funds of the vault.
 * 
 * @param {string} deployedVault - The address of the deployed vault contract.
 * @returns {Promise<Map<Address, bigint>>} A promise that resolves with a map of asset addresses to idle amounts.
 */
export async function fetchCurrentIdleFunds(deployedVault: string, user: Keypair): Promise<Map<Address, bigint>> {
    try {
        const result = await invokeCustomContract(deployedVault, "fetch_current_idle_funds", [], user, false);
        const parsedResult = scValToNative(result.returnValue);
        return parsedResult; // Convert result to native format if needed
    } catch (error) {
        console.error("❌ Failed to fetch current idle funds:", error);
        throw error;
    }
}

export async function fetchParsedCurrentIdleFunds(deployedVault: string, user: Keypair): Promise<{ address: string, amount: bigint }[]> {
    try {
        const res = await fetchCurrentIdleFunds(deployedVault, user);
        const mappedFunds = Object.entries(res).map(([key, value]) => ({
            address: key,
            amount: value,
        }));
        return mappedFunds;
    } catch (error) {
        console.error("Error:", error);
        throw error;
    }
}
export interface AssetInvestmentAllocation {
    asset: Address;
    strategy_investments: { amount: bigint, strategy: Address }[];
}

export async function investVault(
    deployedVault: string,
    investParams: AssetInvestmentAllocation[],
    manager: Keypair
) {

    const mappedParam = xdr.ScVal.scvVec(
        investParams.map((entry) =>
            xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("asset"),
                    val: entry.asset.toScVal(), // Convert asset address to ScVal
                }),
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("strategy_investments"),
                    val: xdr.ScVal.scvVec(
                        entry.strategy_investments.map((investment) =>
                            xdr.ScVal.scvMap([
                                new xdr.ScMapEntry({
                                    key: xdr.ScVal.scvSymbol("amount"),
                                    val: nativeToScVal(BigInt(investment.amount), { type: "i128" }), // Ensure i128 conversion
                                }),
                                new xdr.ScMapEntry({
                                    key: xdr.ScVal.scvSymbol("strategy"),
                                    val: investment.strategy.toScVal(), // Convert strategy address
                                }),
                            ])
                        )
                    ),
                }),
            ])
        )
    );

    try {
        // Invoke contract with the mapped parameters
        const investResult = await invokeCustomContract(
            deployedVault,
            "invest",
            [mappedParam],
            manager
        );
        console.log("Investment successful:", scValToNative(investResult.returnValue));
        return {result: investResult, status: true};
    } catch (error) {
        console.error("Investment failed:", error);
        throw error;
    }
}
  
export enum ActionType {
    Withdraw = 0,
    Invest = 1,
    SwapExactIn = 2,
    SwapExactOut = 3,
    Zapper = 4,
  }

export interface DexDistribution {
    parts: u32;
    path: Array<string>;
    protocol_id: string;
  }

export interface SwapDetailsExactIn {
    amount_in: i128;
    amount_out_min: i128;
    deadline: u64;
    distribution: Array<DexDistribution>;
    token_in: string;
    token_out: string;
}


export interface SwapDetailsExactOut {
    amount_in_max: i128;
    amount_out: i128;
    deadline: u64;
    distribution: Array<DexDistribution>;
    token_in: string;
    token_out: string;
}

export type Option<T> = T | undefined;

export interface Instruction {
    action: ActionType;
    amount: Option<i128>;
    strategy: Option<string>;
    swap_details_exact_in: Option<SwapDetailsExactIn>;
    swap_details_exact_out: Option<SwapDetailsExactOut>;
}

export async function rebalanceVault(deployedVault: string, instructions: Instruction[], manager: Keypair) {
    const mappedInstructions = xdr.ScVal.scvVec(
        instructions.map((instruction) =>
            xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("action"),
                    val: nativeToScVal(instruction.action, { type: "u32" }),
                }),
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("amount"),
                    val: instruction.amount !== undefined
                        ? nativeToScVal(instruction.amount, { type: "i128" })
                        : xdr.ScVal.scvVoid(),
                }),
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("strategy"),
                    val: instruction.strategy
                        ? new Address(instruction.strategy).toScVal()
                        : xdr.ScVal.scvVoid(),
                }),
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("swap_details_exact_in"),
                    val: instruction.swap_details_exact_in
                        ? xdr.ScVal.scvMap(
                              mapSwapDetailsExactIn(instruction.swap_details_exact_in)
                          )
                        : xdr.ScVal.scvVec([xdr.ScVal.scvSymbol("None")]),
                }),
                new xdr.ScMapEntry({
                    key: xdr.ScVal.scvSymbol("swap_details_exact_out"),
                    val: instruction.swap_details_exact_out
                        ? xdr.ScVal.scvMap(
                              mapSwapDetailsExactOut(instruction.swap_details_exact_out)
                          )
                        : xdr.ScVal.scvVec([xdr.ScVal.scvSymbol("None")]),
                }),
            ])
        )
    );
    
    try {
        const investResult = await invokeCustomContract(
            deployedVault,
            "rebalance",
            [mappedInstructions],
            manager
        );
        console.log("Rebalance successful:", scValToNative(investResult.returnValue));
        return {result: investResult, status: true};
    } catch (error) {
        console.error("Rebalance failed:", error);
        throw error;
    }
}

// Helper function to map SwapDetailsExactIn
function mapSwapDetailsExactIn(details: SwapDetailsExactIn) {
    return [
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("token_in"),
            val: new Address(details.token_in).toScVal(),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("token_out"),
            val: new Address(details.token_out).toScVal(),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("amount_in"),
            val: nativeToScVal(details.amount_in, { type: "i128" }),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("amount_out_min"),
            val: nativeToScVal(details.amount_out_min, { type: "i128" }),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("deadline"),
            val: nativeToScVal(details.deadline, { type: "u64" }),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("distribution"),
            val: xdr.ScVal.scvVec(
                details.distribution.map((d) =>
                    xdr.ScVal.scvMap([
                        new xdr.ScMapEntry({
                            key: xdr.ScVal.scvSymbol("protocol_id"),
                            val: xdr.ScVal.scvString(d.protocol_id),
                        }),
                        new xdr.ScMapEntry({
                            key: xdr.ScVal.scvSymbol("path"),
                            val: xdr.ScVal.scvVec(d.path.map((address) => new Address(address).toScVal())),
                        }),
                        new xdr.ScMapEntry({
                            key: xdr.ScVal.scvSymbol("parts"),
                            val: nativeToScVal(d.parts, { type: "u32" }),
                        }),
                    ])
                )
            ),
        }),
    ];
}

// Helper function to map SwapDetailsExactOut
function mapSwapDetailsExactOut(details: SwapDetailsExactOut) {
    return [
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("token_in"),
            val: new Address(details.token_in).toScVal(),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("token_out"),
            val: new Address(details.token_out).toScVal(),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("amount_out"),
            val: nativeToScVal(details.amount_out, { type: "i128" }),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("amount_in_max"),
            val: nativeToScVal(details.amount_in_max, { type: "i128" }),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("deadline"),
            val: nativeToScVal(details.deadline, { type: "u64" }),
        }),
        new xdr.ScMapEntry({
            key: xdr.ScVal.scvSymbol("distribution"),
            val: xdr.ScVal.scvVec(
                details.distribution.map((d) =>
                    xdr.ScVal.scvMap([
                        new xdr.ScMapEntry({
                            key: xdr.ScVal.scvSymbol("protocol_id"),
                            val: xdr.ScVal.scvString(d.protocol_id),
                        }),
                        new xdr.ScMapEntry({
                            key: xdr.ScVal.scvSymbol("path"),
                            val: xdr.ScVal.scvVec(d.path.map((address) => new Address(address).toScVal())),
                        }),
                        new xdr.ScMapEntry({
                            key: xdr.ScVal.scvSymbol("parts"),
                            val: nativeToScVal(d.parts, { type: "u32" }),
                        }),
                    ])
                )
            ),
        }),
    ];
}

export async function fetchCurrentInvestedFunds(deployedVault:string, user:Keypair) {
    try {
        const res = await invokeCustomContract(deployedVault, "fetch_current_invested_funds", [], user);
        const funds = scValToNative(res.returnValue);
        const mappedFunds = Object.entries(funds).map(([key, value]) => ({
            address: key,
            amount: value,
        }));
        return mappedFunds;
    } catch (error) {
        console.error("Error:", error);
        throw error;
    }
}