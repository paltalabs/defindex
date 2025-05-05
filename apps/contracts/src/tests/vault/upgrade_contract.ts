import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { admin, manager, upgradeVaultWasm } from "../../utils/vault.js";
import { green, purple, red, yellow } from "../common.js";
import { airdropAccount, installContract } from "../../utils/contract.js";
import { deployDefindexVault } from "./utils.js";
import { CreateVaultParams } from "../types.js";

/* 
// Upgrade tests:
  - [x] try upgrade from unauthorized
  - [x] upgrade
*/
export async function testUpgradeContract(addressBook: AddressBook, params: CreateVaultParams[]) {
  try {
    console.log(yellow, "----------------");
    console.log(yellow, "| updating Wasm |");
    console.log(yellow, "----------------");
    await installContract("defindex_vault", addressBook, admin);
    console.log(green, "----------------");
    console.log(green, "| Wasm updated |");
    console.log(green, "----------------");
  } catch (error: any) {
    throw Error(error);
  }
  
  // Deploy vault
  const { 
    address:vault_address, 
  } = await deployDefindexVault(addressBook, params);

  if (!vault_address) throw new Error("Vault was not deployed");
  //Try upgrade from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try upgrade from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    const wasm_hash = new Uint8Array(Buffer.from(addressBook.getWasmHash("defindex_vault"), "hex"));
    await airdropAccount(random_user);
    const {result} = await upgradeVaultWasm(vault_address, random_user, wasm_hash);
    if( result !== false){
      throw Error("Upgrade from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "------------------------------------------------");
      console.log(green, "| Upgrade from unauthorized failed as expected |");
      console.log(green, "------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  // upgrade success
  const {
    instructions: upgrade_instructions,
    readBytes: upgrade_read_bytes,
    writeBytes: upgrade_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Upgrade");
      console.log(purple, "---------------------------------------");
      const wasm_hash = new Uint8Array(Buffer.from(addressBook.getWasmHash("defindex_vault"), "hex"));
      const {instructions, readBytes, writeBytes} = await upgradeVaultWasm(vault_address, manager, wasm_hash);
      console.log(green, "------------------------");
      console.log(green, "| Upgrade sucessfully  |");
      console.log(green, "------------------------");
      return {instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {instructions: 0, readBytes: 0, writeBytes: 0};
    } 
  } )();
  const budgetData = {
    upgrade: {
      status: !!upgrade_instructions && !!upgrade_read_bytes && !!upgrade_write_bytes ? `success`: `failed`,
      instructions: upgrade_instructions,
      readBytes: upgrade_read_bytes,
      writeBytes: upgrade_write_bytes,
    }
  }
  return { budgetData };

}