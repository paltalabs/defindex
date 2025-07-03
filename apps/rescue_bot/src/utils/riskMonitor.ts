import { Address, Keypair, scValToNative } from "@stellar/stellar-base";
import { exit } from "process";
import { invokeContract } from "soroban-toolkit";
import { RISK_CHECK_INTERVAL_MS, RISK_TOLERANCE_FACTOR } from "./constants";
import { addressFor, getBalance, simulateInvocation, toolkit } from "./soroban-toolkit";

export async function totalUnderlyingInCometPool(): Promise<bigint> {
  const assetBalance = await getBalance("assetAddress", "cometPool");
  return assetBalance * BigInt(5);
}

export async function LPBackstopBalance(): Promise<bigint> {
  const lpBalance = await getBalance("cometPool", "backstop");
  return lpBalance;
}

export async function cometPoolTotalSupply(): Promise<bigint> {
  const rawTotalSupply = await simulateInvocation(
    addressFor("cometPool"),
    "get_total_supply",
    [],
  );
  const parsedTotalSupply = scValToNative(rawTotalSupply.result.retval);
  return parsedTotalSupply;
}

export async function getBackstopBalance(): Promise<bigint> {
  const totalUnderlying = await totalUnderlyingInCometPool();
  const lpBackstopBalance = await LPBackstopBalance();
  const totalSupply = await cometPoolTotalSupply();

  if (totalSupply === BigInt(0)) {
    console.error("Total supply is zero, cannot calculate backstop balance.");
    return BigInt(0);
  }

  return (totalUnderlying * lpBackstopBalance) / totalSupply;
}

export async function vaultBalanceInStrategy(): Promise<bigint> {
  return getBalance("defindexVault", "defindexStrategy");
}

export async function getVaultTotalManagedFunds(): Promise<bigint> {
  try {
    const rawTotalManagedFunds = await simulateInvocation(
      addressFor("defindexVault"),
      "fetch_total_managed_funds",
      [],
    );
    const parsedTotalManagedFunds = scValToNative(rawTotalManagedFunds.result.retval);
    const total_amount = parsedTotalManagedFunds[0].total_amount && parsedTotalManagedFunds[0].total_amount;
    return BigInt(total_amount);
  } catch (error) {
    console.error("Error fetching vault total managed funds:", error);
    return BigInt(0);
  }
}

export function calculateRiskTolerance(vaultBalance: bigint): number {
  return Number(vaultBalance) * RISK_TOLERANCE_FACTOR;
}

export function isBackstopRisky(backstopBalance: bigint, riskTolerance: number): boolean {
  return Number(backstopBalance) > riskTolerance;
}

export async function evaluateVaultRisk(): Promise<boolean | undefined> {
  try {
    const backstopBalance = await getBackstopBalance();
    const vaultBalance = await vaultBalanceInStrategy();
    const riskTolerance = calculateRiskTolerance(vaultBalance);
    const risky = isBackstopRisky(backstopBalance, riskTolerance);

    if (risky) {
      console.log("Backstop balance exceeds risk tolerance.");
    } else {
      console.log("Backstop balance is within risk tolerance.");
    }
    return risky;
  } catch (error) {
    console.error("Error evaluating vault risk:", error);
  }
}

async function getAdminKeypairAndAddresses() {
  if (!process.env.ADMIN_SECRET) {
    throw new Error("ADMIN_SECRET environment variable is not set.");
  }
  const adminKeypair = Keypair.fromSecret(process.env.ADMIN_SECRET);
  const adminAddress = new Address(adminKeypair.publicKey()).toScVal();
  const strategyAddress = new Address(addressFor("defindexStrategy")).toScVal();
  return { adminKeypair, adminAddress, strategyAddress };
}

export async function executeRescue(): Promise<void> {
  try {
    const { adminKeypair, adminAddress, strategyAddress } = await getAdminKeypairAndAddresses();
    const rescueResult = await invokeContract(
      toolkit,
      "defindexVault",
      "rescue",
      [strategyAddress, adminAddress],
      false,
      adminKeypair
    );
    return rescueResult;
  } catch (error) {
    console.error("Error executing rescue:", error);
  }
}

export async function unpauseStrategy(): Promise<void> {
  try {
    const { adminKeypair, adminAddress, strategyAddress } = await getAdminKeypairAndAddresses();
    const unpauseResult = await invokeContract(
      toolkit,
      "defindexVault",
      "unpause_strategy",
      [strategyAddress, adminAddress],
      false,
      adminKeypair
    );
    return unpauseResult;
  } catch (error) {
    console.error("Error unpausing strategy:", error);
  }
}

export async function startRiskMonitoring(): Promise<void> {
  while (true) {
    console.log("Checking strategy risk...");
    const risk = await evaluateVaultRisk();
    if (risk && risk) {
      console.error("Risky strategy detected. do something!");
      //await executeRescue();
      exit(0);
    }
    await new Promise((resolve) => setTimeout(resolve, RISK_CHECK_INTERVAL_MS));
  }
}
