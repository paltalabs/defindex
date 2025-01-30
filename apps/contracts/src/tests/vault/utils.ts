import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { green, purple, red, yellow } from "../common.js";
import { checkUserBalance } from "../strategy.js";
import { CreateVaultParams, deployVault, fetchCurrentInvestedFunds, fetchParsedCurrentIdleFunds } from "../vault.js";

export async function fetchBalances(addressBook: AddressBook, vault_address: string, user: Keypair) {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idle_funds = await fetchParsedCurrentIdleFunds(
    vault_address,
    [],
    user
  );
  const invested_funds = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  return {idle_funds, invested_funds, hodl_balance};
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