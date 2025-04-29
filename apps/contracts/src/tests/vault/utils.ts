import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { green, purple, red, yellow } from "../common.js";
import { checkUserBalance } from "../strategy.js";
import { deployVault, fetchCurrentIdleFunds, fetchCurrentInvestedFunds } from "../vault.js";
import { CreateVaultParams, StrategyAllocations, TotalManagedFunds } from "../types.js";

export function extractAddresses(params: CreateVaultParams[]): string[] {
  return params.map(param => param.address.toString());
}

export async function fetchBalances(addressBook: AddressBook, vault_address: string, token_address:CreateVaultParams[], user: Keypair) {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idle_funds = await fetchCurrentIdleFunds(
    vault_address, 
    user
  );
  console.log(green, "🟢Idle funds: ", idle_funds);
  const invested_funds = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  console.log(green, "🟢Invested funds: ", invested_funds);
  const hodl_balance = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  return {idle_funds, invested_funds, hodl_balance};
}

export async function fetchStrategiesBalances(addressBook: AddressBook, strategies_keys: string[], vault_address: string, user: Keypair) {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching strategies balances");
  console.log(yellow, "---------------------------------------");

  const strategies_balances = await Promise.all(
    strategies_keys.map(async (strategy_key) => {
      const strategy_balance = await checkUserBalance(
        addressBook.getContractId(strategy_key),
        vault_address,
        user
      );
      return {strategy_key, strategy_balance};
    })
  );

  return strategies_balances;
}

export async function deployDefindexVault(addressBook: AddressBook, params: CreateVaultParams[]) {
  console.log(purple, "-----------------------------------------------");
  console.log(purple, "Deploying defindex vault");
  console.log(purple, "-----------------------------------------------");
  try {
    const { 
      address: vault_address, 
      instructions:deploy_instructions, 
      readBytes:deploy_read_bytes, 
      writeBytes:deploy_write_bytes
    } = await deployVault(
      addressBook,
      params,
      "TestVault",
      "TSTV"
    );
    console.log(vault_address);
    return {address: vault_address, deploy_instructions, deploy_read_bytes, deploy_write_bytes};
  } catch (error:any) {
    console.error(red, error);
    return {
      address: null, 
      deploy_instructions:0, 
      deploy_read_bytes: 0, 
      deploy_write_bytes: 0,
      error
    };
  }
}

export function underlyingToDfTokens(underlying: number | bigint, totalSupply: number | bigint, totalUnderlying: number | bigint) {
  // convert to BigInt
  underlying = BigInt(underlying);
  totalSupply = BigInt(totalSupply);
  totalUnderlying = BigInt(totalUnderlying);
  
  const dfTokensAmount = underlying * totalSupply / totalUnderlying;
  return dfTokensAmount;
}

export function dfTokensToUnderlying(dfTokens: number | bigint, totalSupply: number | bigint, totalUnderlying: number | bigint) {
  // convert to BigInt
  dfTokens = BigInt(dfTokens);
  totalSupply = BigInt(totalSupply);
  totalUnderlying = BigInt(totalUnderlying);
  
  const underlyingAmount = dfTokens * totalUnderlying / totalSupply;
  return underlyingAmount;
}

export function generateExpectedStrategyAllocation(
  strategy: {
    name: string;
    address: string;
    paused: boolean;
  },
  amount: number,
): StrategyAllocations{
  return {
    amount: BigInt(amount),
    paused: strategy.paused,
    strategy_address: strategy.address,
  };
}

export function generateExpectedTotalAmounts(params: CreateVaultParams[], idle_amounts: number[][], invested_amounts: number[][]): TotalManagedFunds[] {
  let i = 0;
  let expected_total_amounts = [];
  for (const param of params) {
    let j = 0;
    const strategy_allocations: StrategyAllocations[] = [];
    if(!idle_amounts[i]) {
      console.error(red, `idle_amounts[${i}] is undefined`);
      throw Error("Amount of strategies in param does not match the amount of expected idle amounts");
    }
    if(param.strategies.length !== invested_amounts[i].length) {
      console.error(red, `Amount of strategies in asset[${i}]: ${param.strategies.length} !== ${invested_amounts[i].length}`);
      throw Error("Amount of strategies in param does not match the amount of expected invested amounts");
    }
    for(const strategy of param.strategies) {
      const expected_strategy_allocation = generateExpectedStrategyAllocation(strategy, invested_amounts[i][j]);
      strategy_allocations.push(expected_strategy_allocation);
      j++;
    }
    const total_idle_amounts = idle_amounts[i].reduce((acc, current_amount) => acc + current_amount, 0);
    const total_invested_amounts = invested_amounts[i].reduce((acc, current_amount) => acc + current_amount, 0);
    const expected_total_amount: TotalManagedFunds = {
      asset: param.address.toString(),
      idle_amount: BigInt(total_idle_amounts),
      invested_amount: BigInt(total_invested_amounts),
      strategy_allocations,
      total_amount: BigInt(total_idle_amounts + total_invested_amounts),
    }
    i++;
    expected_total_amounts.push(expected_total_amount);
  }
  return expected_total_amounts;
}

export function generateTotalAmountsError(params: CreateVaultParams[]): TotalManagedFunds[] {
  let i = 0;
  let expected_total_amounts = [];
  for (const param of params) {
    const strategy_allocations: StrategyAllocations[] = [];
    for(const strategy of param.strategies) {
      const expected_strategy_allocation = generateExpectedStrategyAllocation(strategy, 0);
      strategy_allocations.push(expected_strategy_allocation);
    }
    const expected_total_amount: TotalManagedFunds = {
      asset: param.address.toString(),
      idle_amount: BigInt(0),
      invested_amount: BigInt(0),
      strategy_allocations,
      total_amount: BigInt(0),
    }
    i++;
    expected_total_amounts.push(expected_total_amount);
  }
  return expected_total_amounts;
}

export function compareTotalManagedFunds(expected_total_managed_funds: TotalManagedFunds[], total_managed_funds: TotalManagedFunds[], expected_tolerance: number = 0) {
  const tolerance = BigInt(expected_tolerance);
  for (let i = 0; i < total_managed_funds.length; i++) {
    for (let j = 0; j < total_managed_funds[i].strategy_allocations.length; j++) {
      const expectedAmount = expected_total_managed_funds[i].strategy_allocations[j].amount;
      const actualAmount = total_managed_funds[i].strategy_allocations[j].amount;
      if (actualAmount < expectedAmount - tolerance || actualAmount > expectedAmount + tolerance) {
        console.error(red, `strategy_allocations[${j}].amount: ${actualAmount} !== ${expectedAmount} (with tolerance ${tolerance})`);
        return false;
      }
    }

    const invested_funds_tolerance = BigInt(expected_tolerance * total_managed_funds[i].strategy_allocations.length);

    if (total_managed_funds[i].idle_amount < expected_total_managed_funds[i].idle_amount - tolerance || total_managed_funds[i].idle_amount > expected_total_managed_funds[i].idle_amount + tolerance) {
      console.error(red, `idle_amount[${i}]: ${total_managed_funds[i].idle_amount} !== ${expected_total_managed_funds[i].idle_amount} (with tolerance ${tolerance})`);
      return false;
    }

    if (total_managed_funds[i].invested_amount < expected_total_managed_funds[i].invested_amount - invested_funds_tolerance || total_managed_funds[i].invested_amount > expected_total_managed_funds[i].invested_amount + invested_funds_tolerance) {
      console.error(red, `invested_amount[${i}]: ${total_managed_funds[i].invested_amount} !== ${expected_total_managed_funds[i].invested_amount} (with tolerance ${invested_funds_tolerance})`);
      return false;
    }
    if (total_managed_funds[i].total_amount < expected_total_managed_funds[i].total_amount - invested_funds_tolerance || total_managed_funds[i].total_amount > expected_total_managed_funds[i].total_amount + invested_funds_tolerance) {
      console.error(red, `total_amount[${i}]: ${total_managed_funds[i].total_amount} !== ${expected_total_managed_funds[i].total_amount} (with tolerance ${invested_funds_tolerance})`);
      return false;
    }

 
  }
  return true;
}