import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { green, purple, red, yellow } from "../common.js";
import { checkUserBalance } from "../strategy.js";
import { deployVault, fetchCurrentIdleFunds, fetchCurrentInvestedFunds } from "../vault.js";
import { CreateVaultParams } from "../types.js";

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