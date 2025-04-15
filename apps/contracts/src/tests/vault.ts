import {
  Address,
  Keypair,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { i128, u64 } from "@stellar/stellar-sdk/contract";
import { SOROSWAP_ROUTER, USDC_ADDRESS } from "../constants.js";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  getTokenBalance,
  invokeContract,
  invokeCustomContract,
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { getTransactionBudget } from "../utils/tx.js";
import { green } from "./common.js";
import { AssetInvestmentAllocation, CreateVaultParams, TotalManagedFunds } from "./types.js";

const network = process.argv[2];
const loadedConfig = config(network);
export const admin = loadedConfig.admin;
export const emergencyManager = loadedConfig.getUser(
  "DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY"
);
export const rebalanceManager = loadedConfig.getUser(
  "DEFINDEX_REBALANCE_MANAGER_SECRET_KEY"
);
export const feeReceiver = loadedConfig.getUser(
  "DEFINDEX_FEE_RECEIVER_SECRET_KEY"
);

export const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");


export type Option<T> = T | undefined;

// export type Address = string; // Simplified representation of Address as a string
// export type i128 = bigint; // TypeScript equivalent for large integers
// export type u64 = number; // Simplified as a number for UNIX timestamps

export type Instruction =
  | { type: "Unwind"; strategy: string; amount: i128 }
  | { type: "Invest"; strategy: string; amount: i128 }
  | {
      type: "SwapExactIn";
      token_in: string;
      token_out: string;
      amount_in: i128;
      amount_out_min: i128;
      deadline: u64;
    }
  | {
      type: "SwapExactOut";
      token_in: string;
      token_out: string;
      amount_out: i128;
      amount_in_max: i128;
      deadline: u64;
    };





export function mapInstructionsToParams(
  instructions: Instruction[]
): xdr.ScVal {
  return xdr.ScVal.scvVec(
    instructions.map((instruction) => {
      switch (instruction.type) {
        case "Invest":
        case "Unwind":
          // Handle Invest and Withdraw actions
          return xdr.ScVal.scvVec([
            xdr.ScVal.scvSymbol(instruction.type), // "Invest" or "Withdraw"
            new Address(instruction.strategy).toScVal(),
            nativeToScVal(instruction.amount, { type: "i128" }), // amount
          ]);

        case "SwapExactIn":
          // Handle SwapExactIn action
          return xdr.ScVal.scvVec([
            xdr.ScVal.scvSymbol("SwapExactIn"),
            new Address(instruction.token_in).toScVal(),
            new Address(instruction.token_out).toScVal(),
            nativeToScVal(instruction.amount_in, { type: "i128" }),
            nativeToScVal(instruction.amount_out_min, { type: "i128" }),
            nativeToScVal(instruction.deadline, { type: "u64" }),
          ]);

        case "SwapExactOut":
          // Handle SwapExactOut action
          return xdr.ScVal.scvVec([
            xdr.ScVal.scvSymbol("SwapExactOut"),
            new Address(instruction.token_in).toScVal(),
            new Address(instruction.token_out).toScVal(),
            nativeToScVal(instruction.amount_out, { type: "i128" }),
            nativeToScVal(instruction.amount_in_max, { type: "i128" }),
            nativeToScVal(instruction.deadline, { type: "u64" }),
          ]);

        default:
          throw new Error(`Unsupported action type: ${instruction}`);
      }
    })
  );
}

/**
 * Mints a specified amount of tokens for a given user.
 *
 * @param user - The Keypair of the user for whom the tokens will be minted.
 * @param amount - The amount of tokens to mint.
 * @returns A promise that resolves when the minting operation is complete.
 */
export async function mintToken(user: Keypair, amount: number, tokenAddress?: Address) {
  await invokeCustomContract(

    tokenAddress ? tokenAddress.toString() : USDC_ADDRESS.toString(),
    "mint",
    [
      new Address(user.publicKey()).toScVal(),
      nativeToScVal(amount, { type: "i128" }),
    ],
    loadedConfig.getUser("SOROSWAP_MINT_SECRET_KEY")
  );
}

/**
 * Generates the parameters required to create a DeFindex vault.
 *
 * @param {Keypair} emergencyManager - The keypair of the emergency manager.
 * @param {Keypair} rebalanceManager - The keypair of the rebalance manager.
 * @param {Keypair} feeReceiver - The keypair of the fee receiver.
 * @param {Keypair} manager - The keypair of the manager.
 * @param {string} vaultName - The name of the vault.
 * @param {string} vaultSymbol - The symbol of the vault.
 * @param {xdr.ScVal[]} assetAllocations - The asset allocations for the vault.
 * @param {Address} router_address - The address of the Soroswap router.
 * @returns {xdr.ScVal[]} An array of ScVal objects representing the parameters.
 */
export function getCreateDeFindexParams(
  emergencyManager: Keypair,
  rebalanceManager: Keypair,
  feeReceiver: Keypair,
  manager: Keypair,
  vaultName: string,
  vaultSymbol: string,
  assetAllocations: xdr.ScVal[],
  router_address: Address,
  upgradable: boolean,
): xdr.ScVal[] {
  const roles = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(0),
      val: new Address(emergencyManager.publicKey()).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(1),
      val: new Address(feeReceiver.publicKey()).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(2),
      val: new Address(manager.publicKey()).toScVal(),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvU32(3),
      val: new Address(rebalanceManager.publicKey()).toScVal(),
    }),
  ]);

  const nameSymbol = xdr.ScVal.scvMap([
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvString("name"),
      val: nativeToScVal(vaultName ?? "TestVault", { type: "string" }),
    }),
    new xdr.ScMapEntry({
      key: xdr.ScVal.scvString("symbol"),
      val: nativeToScVal(vaultSymbol ?? "TSTV", { type: "string" }),
    }),
  ])


    /* 
     fn create_defindex_vault(
        e: Env,
        roles: Map<u32, Address>,
        vault_fee: u32,
        assets: Vec<AssetStrategySet>,
        soroswap_router: Address,
        name_symbol: Map<String, String>,
    ) -> Result<Address, FactoryError>;
  */
  return [
    roles,
    nativeToScVal(100, { type: "u32" }), // Setting vault_fee as 100 bps for demonstration
    xdr.ScVal.scvVec(assetAllocations),
    router_address.toScVal(),
    nameSymbol,
    nativeToScVal(upgradable, { type: "bool" })
  ];
}

/**
 * Converts an array of asset allocation parameters into an array of xdr.ScVal objects.
 *
 * @param {CreateVaultParams[]} assets - An array of asset allocation parameters.
 * Each asset contains an address and an array of strategies.
 * @returns {xdr.ScVal[]} An array of xdr.ScVal objects representing the asset allocations.
 *
 * Each asset is converted into an xdr.ScVal map with the following structure:
 * - `address`: The address of the asset, converted to an xdr.ScVal.
 * - `strategies`: An array of strategies, each converted to an xdr.ScVal map with the following structure:
 *   - `address`: The address of the strategy, converted to an xdr.ScVal.
 *   - `name`: The name of the strategy, converted to an xdr.ScVal.
 *   - `paused`: A boolean indicating if the strategy is paused, converted to an xdr.ScVal.
 */
function getAssetAllocations(assets: CreateVaultParams[]): xdr.ScVal[] {
  return assets.map((asset) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("address"),
        val: asset.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("strategies"),
        val: xdr.ScVal.scvVec(
          asset.strategies.map((strategy) =>
            xdr.ScVal.scvMap([
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("address"),
                val: new Address(strategy.address).toScVal(),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("name"),
                val: nativeToScVal(strategy.name, { type: "string" }),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("paused"),
                val: nativeToScVal(strategy.paused, { type: "bool" }),
              }),
            ])
          )
        ),
      }),
    ]);
  });
}

/**
 * Deploys a new DeFindex Vault.
 *
 * @param addressBook - The address book containing necessary addresses.
 * @param createVaultParams - An array of parameters required to create the vault.
 * @param vaultName - The name of the vault to be created.
 * @param vaultSymbol - The symbol of the vault to be created.
 * @returns A promise that resolves to the address of the newly created vault.
 *
 * @throws Will throw an error if the contract invocation fails.
 */
export async function deployVault(
  addressBook: AddressBook,
  createVaultParams: CreateVaultParams[],
  vaultName: string,
  vaultSymbol: string
): Promise<any> {
  const assets: CreateVaultParams[] = createVaultParams;
  const assetAllocations = getAssetAllocations(assets);

  const createDeFindexParams: xdr.ScVal[] = getCreateDeFindexParams(
    emergencyManager,
    rebalanceManager,
    feeReceiver,
    manager,
    vaultName,
    vaultSymbol,
    assetAllocations,
    new Address(SOROSWAP_ROUTER),
    true,
  );
  try {
    const result = await invokeContract(
      "defindex_factory",
      addressBook,
      "create_defindex_vault",
      createDeFindexParams,
      loadedConfig.admin
    );
    console.log(
      "🚀 « DeFindex Vault created with address:",
      scValToNative(result.returnValue)
    );
    const address = scValToNative(result.returnValue);
    const budget = getTransactionBudget(result);
    return { address: address, ...budget };
  } catch (error) {
    console.error("Error deploying vault:", error);
    throw error;
  }
}

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
export async function depositToVault(
  deployedVault: string,
  amount: number[],
  user?: Keypair,
  invest?: boolean
) {
  // Create and fund a new user account if not provided
  const newUser = user ? user : Keypair.random();
  const investBool = invest ? invest : false;
  console.log(
    "🚀 ~ depositToVault ~ newUser.publicKey():",
    newUser.publicKey()
  );
  console.log("🚀 ~ depositToVault ~ newUser.secret():", newUser.secret());

  if (network !== "mainnet") await airdropAccount(newUser);
  console.log("New user publicKey:", newUser.publicKey());

  let balanceBefore: number;
  let balanceAfter: number;
  let result: any;

  // Define deposit parameters
  const amountsDesired = amount.map((am) => BigInt(am)); // 1 XLM in stroops (1 XLM = 10^7 stroops)
  const amountsMin = amount.map((_) => BigInt(0)); // Minimum amount for transaction to succeed

  const depositParams: xdr.ScVal[] = [
    xdr.ScVal.scvVec(
      amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))
    ),
    xdr.ScVal.scvVec(
      amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))
    ),
    new Address(newUser.publicKey()).toScVal(),
    xdr.ScVal.scvBool(investBool),
  ];

  try {
    // Check the user's balance before the deposit
    balanceBefore = await getDfTokenBalance(
      deployedVault,
      newUser.publicKey(),
      newUser
    );
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
    balanceAfter = await getDfTokenBalance(
      deployedVault,
      newUser.publicKey(),
      newUser
    );
    console.log("🔢 « dfToken balance after deposit:", balanceAfter);
  } catch (error) {
    console.error("❌ Balance check after deposit failed:", error);
    throw error;
  }
  const budget = getTransactionBudget(result);
  return { user: newUser, balanceBefore, result, balanceAfter, status: true, ...budget };
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
export async function getDfTokenBalance(
  deployedVault: string,
  userPublicKey: string,
  source?: Keypair
): Promise<number> {
  const userAddress = new Address(userPublicKey).toScVal();
  const methodName = "balance"; // Assumes a standard token interface

  try {
    const result = await invokeCustomContract(
      deployedVault,
      methodName,
      [userAddress],
      source ? source : Keypair.random(), // No specific source is needed as we are just querying the balance
      true // Set to simulate mode if testing on an uncommitted transaction
    );
    const balance = scValToNative(result.result.retval);
    return balance;
  } catch (error) {
    console.error(
      `Failed to retrieve dfToken balance for user ${userPublicKey}:`,
      error
    );
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
export async function withdrawFromVault(
  deployedVault: string,
  min_amounts_out: number[],
  withdrawAmount: number,
  user: Keypair
) {
  console.log("🚀 ~ withdrawFromVault ~ User publicKey:", user.publicKey());

  let balanceBefore: number;
  let balanceAfter: number;
  let result: any;

  try {
    // Check the user's balance before the withdrawal
    balanceBefore = await getDfTokenBalance(
      deployedVault,
      user.publicKey(),
      user
    );
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
  const minAmountsOut: xdr.ScVal[] = min_amounts_out.map((amount) =>
    nativeToScVal(BigInt(amount), { type: "i128" })
  );
  const withdrawParams: xdr.ScVal[] = [
    nativeToScVal(BigInt(withdrawAmount), { type: "i128" }),
    xdr.ScVal.scvVec(minAmountsOut),
    new Address(user.publicKey()).toScVal()
  ];

  try {
    result = await invokeCustomContract(
      deployedVault,
      "withdraw",
      withdrawParams,
      user
    );
    console.log(
      "🚀 « Withdrawal successful:",
      scValToNative(result.returnValue)
    );
  } catch (error) {
    console.error("❌ Withdrawal failed:", error);
    throw error;
  }

  try {
    // Check the user's balance after the withdrawal
    balanceAfter = await getDfTokenBalance(
      deployedVault,
      user.publicKey(),
      user
    );
    console.log("🔢 « dfToken balance after withdraw:", balanceAfter);
  } catch (error) {
    console.error("❌ Balance check after withdraw failed:", error);
    throw error;
  }
  const budget = getTransactionBudget(result);
  return { balanceBefore, result, balanceAfter, ...budget };
}

export async function fetchParsedCurrentIdleFunds(
  deployedVault: string,
  tokens: string[],
  user: Keypair
): Promise<{ address: string; amount: bigint }[]> {
  try {
    const idle_funds = await Promise.all(
      tokens.map(async (token_id) => {
        const balance = await getTokenBalance(token_id, deployedVault, user);
        return {
          address: token_id,
          amount: balance
        };
      })
    );

    return idle_funds;
  } catch (error) {
    console.error("Error:", error);
    throw error;
  }
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
          key: xdr.ScVal.scvSymbol("strategy_allocations"),
          val: xdr.ScVal.scvVec(
            entry.strategy_investments.map((investment) =>
              xdr.ScVal.scvMap([
                new xdr.ScMapEntry({
                  key: xdr.ScVal.scvSymbol("amount"),
                  val: nativeToScVal(BigInt(investment.amount), {
                    type: "i128",
                  }), // Ensure i128 conversion
                }),
                new xdr.ScMapEntry({
                  key: xdr.ScVal.scvSymbol("strategy_address"),
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
    console.log(
      "Investment successful:",
      scValToNative(investResult.returnValue)
    );
    return { result: investResult, status: true };
  } catch (error) {
    console.error("Investment failed:", error);
    throw error;
  }
}

export async function rebalanceVault(deployedVault: string, instructions: Instruction[], manager: Keypair) {
  const params = mapInstructionsToParams(instructions);

  try {
    const rebalanceResult = await invokeCustomContract(
      deployedVault,
      "rebalance",
      [new Address(manager.publicKey()).toScVal(), params],
      manager
    );
    console.log(green, 'Rebalance result:', scValToNative(rebalanceResult.returnValue));
    const budget = getTransactionBudget(rebalanceResult);
    return { result: rebalanceResult, status: true, ...budget };
  } catch (error) {
    console.error("Rebalance failed:", error);
    throw error;
  }
}

export async function getVaultBalance(deployedVault: string, user: Keypair) {
  try {
    const result = await invokeCustomContract(
      deployedVault,
      "balance",
      [new Address(user.publicKey()).toScVal()],
      user
    );
    return scValToNative(result.returnValue);
  } catch (error) {
    console.error("Failed to get vault balance:", error);
    throw error;
  }
}

export async function rescueFromStrategy(deployedVault:string ,strategyAddress: string, manager: Keypair) {
  try {
    const args: xdr.ScVal[] = [
      new Address(strategyAddress).toScVal(),
      new Address(manager.publicKey()).toScVal(),
    ];
    const result = await invokeCustomContract(
      deployedVault,
      "rescue",
      args,
      manager
    );
    const { instructions, readBytes, writeBytes } = getTransactionBudget(result);
    const parsed_result = scValToNative(result.returnValue);
    return {
      instructions,
      readBytes,
      writeBytes,
      result: parsed_result,
    }
  } catch (error) {
    console.error("Failed to rescue from strategy:", error);
    throw error;
  }
}

export async function pauseStrategy(deployedVault:string ,strategyAddress: string, manager: Keypair) {
  try {
    const args: xdr.ScVal[] = [
      new Address(strategyAddress).toScVal(),
      new Address(manager.publicKey()).toScVal(),
    ];
    const result = await invokeCustomContract(
      deployedVault,
      "pause_strategy",
      args,
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const {instructions, readBytes, writeBytes} = getTransactionBudget(result);
    return {
      instructions,
      readBytes,
      writeBytes,
      result: parsed_result,
    };
  } catch (error) {
    console.error("Failed to pause strategy:", error);
    throw error;
  }
}
export async function unpauseStrategy(deployedVault:string ,strategyAddress: string, manager: Keypair) {
  try {
    const args: xdr.ScVal[] = [
      new Address(strategyAddress).toScVal(),
      new Address(manager.publicKey()).toScVal(),
    ];
    const result = await invokeCustomContract(
      deployedVault,
      "unpause_strategy",
      args,
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const {instructions, readBytes, writeBytes} = getTransactionBudget(result);
    return {
      instructions,
      readBytes,
      writeBytes,
      result: parsed_result,
    };
  } catch (error) {
    console.error("Failed to pause strategy:", error);
    throw error;
  }
}

// export async function rebalanceVault(deployedVault: string, instructions: Instruction[], manager: Keypair) {
//     const mappedInstructions = xdr.ScVal.scvVec(
//         instructions.map((instruction) =>
//             xdr.ScVal.scvMap([
//                 new xdr.ScMapEntry({
//                     key: xdr.ScVal.scvSymbol("action"),
//                     val: nativeToScVal(instruction.action, { type: "u32" }),
//                 }),
//                 new xdr.ScMapEntry({
//                     key: xdr.ScVal.scvSymbol("amount"),
//                     val: instruction.amount !== undefined
//                         ? nativeToScVal(instruction.amount, { type: "i128" })
//                         : xdr.ScVal.scvVoid(),
//                 }),
//                 new xdr.ScMapEntry({
//                     key: xdr.ScVal.scvSymbol("strategy"),
//                     val: instruction.strategy
//                         ? new Address(instruction.strategy).toScVal()
//                         : xdr.ScVal.scvVoid(),
//                 }),
//                 new xdr.ScMapEntry({
//                     key: xdr.ScVal.scvSymbol("swap_details_exact_in"),
//                     val: instruction.swap_details_exact_in
//                         ? xdr.ScVal.scvMap(
//                               mapSwapDetailsExactIn(instruction.swap_details_exact_in)
//                           )
//                         : xdr.ScVal.scvVec([xdr.ScVal.scvSymbol("None")]),
//                 }),
//                 new xdr.ScMapEntry({
//                     key: xdr.ScVal.scvSymbol("swap_details_exact_out"),
//                     val: instruction.swap_details_exact_out
//                         ? xdr.ScVal.scvMap(
//                               mapSwapDetailsExactOut(instruction.swap_details_exact_out)
//                           )
//                         : xdr.ScVal.scvVec([xdr.ScVal.scvSymbol("None")]),
//                 }),
//             ])
//         )
//     );

//     try {
//         const investResult = await invokeCustomContract(
//             deployedVault,
//             "rebalance",
//             [new Address(manager.publicKey()).toScVal(), mappedInstructions],
//             manager
//         );
//         console.log("Rebalance successful:", scValToNative(investResult.returnValue));
//         return {result: investResult, status: true};
//     } catch (error) {
//         console.error("Rebalance failed:", error);
//         throw error;
//     }
// }

// // Helper function to map SwapDetailsExactIn
// function mapSwapDetailsExactIn(details: SwapDetailsExactIn) {
//     return [
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("token_in"),
//             val: new Address(details.token_in).toScVal(),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("token_out"),
//             val: new Address(details.token_out).toScVal(),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("amount_in"),
//             val: nativeToScVal(details.amount_in, { type: "i128" }),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("amount_out_min"),
//             val: nativeToScVal(details.amount_out_min, { type: "i128" }),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("deadline"),
//             val: nativeToScVal(details.deadline, { type: "u64" }),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("distribution"),
//             val: xdr.ScVal.scvVec(
//                 details.distribution.map((d) =>
//                     xdr.ScVal.scvMap([
//                         new xdr.ScMapEntry({
//                             key: xdr.ScVal.scvSymbol("protocol_id"),
//                             val: xdr.ScVal.scvString(d.protocol_id),
//                         }),
//                         new xdr.ScMapEntry({
//                             key: xdr.ScVal.scvSymbol("path"),
//                             val: xdr.ScVal.scvVec(d.path.map((address) => new Address(address).toScVal())),
//                         }),
//                         new xdr.ScMapEntry({
//                             key: xdr.ScVal.scvSymbol("parts"),
//                             val: nativeToScVal(d.parts, { type: "u32" }),
//                         }),
//                     ])
//                 )
//             ),
//         }),
//     ];
// }

// // Helper function to map SwapDetailsExactOut
// function mapSwapDetailsExactOut(details: SwapDetailsExactOut) {
//     return [
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("token_in"),
//             val: new Address(details.token_in).toScVal(),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("token_out"),
//             val: new Address(details.token_out).toScVal(),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("amount_out"),
//             val: nativeToScVal(details.amount_out, { type: "i128" }),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("amount_in_max"),
//             val: nativeToScVal(details.amount_in_max, { type: "i128" }),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("deadline"),
//             val: nativeToScVal(details.deadline, { type: "u64" }),
//         }),
//         new xdr.ScMapEntry({
//             key: xdr.ScVal.scvSymbol("distribution"),
//             val: xdr.ScVal.scvVec(
//                 details.distribution.map((d) =>
//                     xdr.ScVal.scvMap([
//                         new xdr.ScMapEntry({
//                             key: xdr.ScVal.scvSymbol("protocol_id"),
//                             val: xdr.ScVal.scvString(d.protocol_id),
//                         }),
//                         new xdr.ScMapEntry({
//                             key: xdr.ScVal.scvSymbol("path"),
//                             val: xdr.ScVal.scvVec(d.path.map((address) => new Address(address).toScVal())),
//                         }),
//                         new xdr.ScMapEntry({
//                             key: xdr.ScVal.scvSymbol("parts"),
//                             val: nativeToScVal(d.parts, { type: "u32" }),
//                         }),
//                     ])
//                 )
//             ),
//         }),
//     ];
// }

export async function fetchCurrentInvestedFunds(
  deployedVault: string,
  user: Keypair
) {
  try {
    const res = await invokeCustomContract(
      deployedVault,
      "fetch_total_managed_funds",
      [],
      user,
      true
    );
    const funds = scValToNative(res.result.retval) as TotalManagedFunds[];
    const mappedFunds = funds.map((fund) => {
      return {
        asset: fund.asset,
        amount: fund.invested_amount,
      };
    });
    return mappedFunds;
  } catch (error) {
    console.error("Error:", error);
    throw error;
  }
}
export async function fetchCurrentIdleFunds(
  deployedVault: string,
  user: Keypair
) {
  try {
    const res = await invokeCustomContract(
      deployedVault,
      "fetch_total_managed_funds",
      [],
      user,
      true,
    );
    const funds = scValToNative(res.result.retval) as TotalManagedFunds[];
    const mappedFunds = funds.map((fund) => {
      return {
        asset: fund.asset,
        amount: fund.idle_amount,
      };
    });
    return mappedFunds;
  } catch (error) {
    console.error("Error:", error);
    throw error;
  }
}

export async function setVaultManager(
  deployedVault: string,
  newManager: Keypair,
  manager: Keypair
) {
  try {
    const result = await invokeCustomContract(
      deployedVault,
      "set_manager",
      [new Address(newManager.publicKey()).toScVal()],
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const { instructions, readBytes, writeBytes } = getTransactionBudget(result);
    console.log("Set manager successful:", scValToNative(result.returnValue));
    return { result: parsed_result, instructions, readBytes, writeBytes };
  } catch (error) {
    console.error("Set manager failed:", error);
    throw error;
  }
}

export async function setRebalanceManager(deployedVault:Address, manager:Keypair, new_rebalance_manager:string){
  try {
    const result = await invokeCustomContract(
      deployedVault.toString(),
      "set_rebalance_manager",
      [new Address(new_rebalance_manager).toScVal()],
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const { instructions, readBytes, writeBytes } = getTransactionBudget(result);
    console.log("Set rebalance manager successful:", scValToNative(result.returnValue));
    return { result: parsed_result, instructions, readBytes, writeBytes };
  } catch (error) {
    console.error("Set rebalance manager failed:", error);
    throw error;
  }
}

export async function setFeeReceiver(deployedVault:Address, manager:Keypair, new_fee_receiver:string){
  try {
    const result = await invokeCustomContract(
      deployedVault.toString(),
      "set_fee_receiver",
      [
        new Address(manager.publicKey()).toScVal(),
        new Address(new_fee_receiver).toScVal()
      ],
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const { instructions, readBytes, writeBytes } = getTransactionBudget(result);
    console.log("Set fee receiver successful:", scValToNative(result.returnValue));
    return { result: parsed_result, instructions, readBytes, writeBytes };
  } catch (error) {
    console.error("Set fee receiver failed:", error);
    throw error;
  }
}

//fn set_emergency_manager(emergency_manager: address)
export async function setEmergencyManager(deployedVault:Address, manager:Keypair, new_emergency_manager:string){
  try {
    const result = await invokeCustomContract(
      deployedVault.toString(),
      "set_emergency_manager",
      [new Address(new_emergency_manager).toScVal()],
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const { instructions, readBytes, writeBytes } = getTransactionBudget(result);
    console.log("Set emergency manager successful:", scValToNative(result.returnValue));
    return { result: parsed_result, instructions, readBytes, writeBytes };
  } catch (error) {
    console.error("Set emergency manager failed:", error);
    throw error;
  }
}

export async function upgradeVaultWasm(deployedVault:Address, manager:Keypair, new_wasm_hash:Uint8Array){
  try {
    const result = await invokeCustomContract(
      deployedVault.toString(),
      "upgrade",
      [nativeToScVal(new_wasm_hash)],
      manager
    );
    const parsed_result = scValToNative(result.returnValue);
    const { instructions, readBytes, writeBytes } = getTransactionBudget(result);
    console.log("Upgrade successful:", scValToNative(result.returnValue));
    return { result: parsed_result, instructions, readBytes, writeBytes };
  } catch (error) {
    console.error("Upgrade failed:", error);
    throw error;
  }
}

export async function fetchTotalManagedFunds(deployedVault:Address, user:Keypair): Promise<TotalManagedFunds[]>{
  const res = await invokeCustomContract(
    deployedVault.toString(),
    "fetch_total_managed_funds",
    [],
    user,
    true
  );
  const funds = scValToNative(res.result.retval);
  return funds;
}

export async function fetchTotalSupply(deployedVault:Address, user:Keypair){
  const res = await invokeCustomContract(
    deployedVault.toString(),
    "total_supply",
    [],
    user,
    true
  );
  const supply = scValToNative(res.result.retval);
  return supply;
}