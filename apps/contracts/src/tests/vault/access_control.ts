import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import { green, purple, red } from "../common.js";
import { CreateVaultParams, manager, setEmergencyManager, setFeeReceiver, setRebalanceManager, setVaultManager } from "../vault.js";
import { deployDefindexVault } from "./utils.js";

/* 
// Access control tests:
  - [x] try setManager from unauthorized
  - [x] setManager

  - [x] try setRebalanceManager from unauthorized
  - [x] setRebalanceManager

  - [x] try setFeeReceiver from unauthorized
  - [x] setFeeReceiver

  - [x] try setEmergencyManager from unauthorized
  - [x] setEmergencyManager
*/
export async function testAccessControl(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  //Deploy vault
  const { 
    address:vault_address, 
  } = await deployDefindexVault(addressBook, params);
  if (!vault_address) throw new Error("Vault was not deployed");

  const new_manager = Keypair.random();
  await airdropAccount(new_manager);

  // Try setRebalanceManager from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try setRebalanceManager from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    const {result}  = await setRebalanceManager(vault_address, random_user, user.publicKey());
    if( result !== false){
      throw Error("Set rebalance manager from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "--------------------------------------------------------------");
      console.log(green, "| Set rebalance manager from unauthorized failed as expected |");
      console.log(green, "--------------------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  // setRebalanceManager success
  const {
    instructions: set_rebalance_manager_instructions, 
    readBytes: set_rebalance_manager_read_bytes, 
    writeBytes: set_rebalance_manager_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "setRebalanceManager");
      console.log(purple, "---------------------------------------");
      const random_user = Keypair.random();
      await airdropAccount(random_user);
      const {result, instructions, readBytes, writeBytes}  = await setRebalanceManager(vault_address, manager, random_user.publicKey());
      if( result === null){
        console.log(green, "--------------------------------------");
        console.log(green, "| Rebalance manager set sucessfully  |");
        console.log(green, "--------------------------------------");
      } else if (result === false) {
        throw Error("Set rebalance manager failed");
      }
      return {result, instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {result: false, instructions: 0, readBytes: 0, writeBytes: 0};
    } 
  } )();

  // Try set fee reciever from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try set fee receiver from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    await setFeeReceiver(vault_address, random_user, user.publicKey());

  } catch (error: any) {
    if( error.toString().includes("HostError: Error(Contract, #130)")) {
      console.log(green, "----------------------------------------------------------");
      console.log(green, "| Set fee receiver from unauthorized failed as expected |");
      console.log(green, "----------------------------------------------------------");
    } else {     
      throw Error(error);
    }
  } 

  // setFeeReceiver success
  const {
    instructions: set_fee_receiver_instructions,
    readBytes: set_fee_receiver_read_bytes,
    writeBytes: set_fee_receiver_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "setFeeReceiver");
      console.log(purple, "---------------------------------------");
      const {instructions, readBytes, writeBytes} = await setFeeReceiver(vault_address, manager, user.publicKey());
      console.log(green, "---------------------------------");
      console.log(green, "| Fee receiver set sucessfully  |");
      console.log(green, "---------------------------------");
      return {instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {instructions: 0, readBytes: 0, writeBytes: 0};
    } 
  } )();
 

  // Try set emergency manager from unauthorized
  try {
    console.log(purple, "-------------------------------------------");
    console.log(purple, "Try set emergency manager from unauthorized");
    console.log(purple, "-------------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    const {result} = await setEmergencyManager(vault_address, random_user, user.publicKey());
    if( result !== false){
      throw Error("Set emergency manager from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "--------------------------------------------------------------");
      console.log(green, "| Set emergency manager from unauthorized failed as expected |");
      console.log(green, "--------------------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  // setEmergencyManager success
  const {
    instructions: set_emergency_manager_instructions,
    readBytes: set_emergency_manager_read_bytes,
    writeBytes: set_emergency_manager_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "setEmergencyManager");
      console.log(purple, "---------------------------------------");
      const random_user = Keypair.random();
      await airdropAccount(random_user);
      const {result, instructions, readBytes, writeBytes} = await setEmergencyManager(vault_address, manager, random_user.publicKey());
      if( result === null){
        console.log(green, "--------------------------------------");
        console.log(green, "| Emergency manager set sucessfully  |");
        console.log(green, "--------------------------------------");
      } else if (result === false) {
        throw Error("Set emergency manager failed");
      }
      return {result, instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {result: false, instructions: 0, readBytes: 0, writeBytes: 0};
    }
    })();
  
    //Try set manager from unauthorized
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Try set manager from unauthorized");
      console.log(purple, "---------------------------------------");
      const random_user = Keypair.random();
      await airdropAccount(random_user);
      const  {result} = await setVaultManager(vault_address, random_user, random_user);
      if( result !== false){
        throw Error("Set manager from unauthorized validation failed");
      } else if (result === false) {
        console.log(green, "------------------------------------------------------");
        console.log(green, "| Set manager from unauthorized failed as expected |");
        console.log(green, "------------------------------------------------------");
      }
    } catch (error: any) {
      throw Error(error);
    }
  
    //setManager succesfully
    const {
      instructions: set_manager_instructions, 
      readBytes: set_manager_read_bytes, 
      writeBytes: set_manager_write_bytes
    } = await (async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "setRebalanceManager");
        console.log(purple, "---------------------------------------");
        const {result, instructions, readBytes, writeBytes}  = await setVaultManager(vault_address, new_manager, manager);
        if( result === null){
          console.log(green, "--------------------------------------");
          console.log(green, "| Manager set sucessfully  |");
          console.log(green, "--------------------------------------");
        } else if (result === false) {
          throw Error("Set manager failed");
        }
        return {result, instructions, readBytes, writeBytes};
      } catch (error: any) {
        console.error(red, error);
        return {result: false, instructions: 0, readBytes: 0, writeBytes: 0};
      } 
    } )();

  const tests_status = {
    set_manager: {
      status: !!set_manager_instructions && !!set_manager_read_bytes && !!set_manager_write_bytes,
    },
    set_rebalance_manager: {
      status: !!set_rebalance_manager_instructions && !!set_rebalance_manager_read_bytes && !!set_rebalance_manager_write_bytes,
    },
    set_fee_receiver: {
      status: !!set_fee_receiver_instructions && !!set_fee_receiver_read_bytes && !!set_fee_receiver_write_bytes,
    },
    set_emergency_manager: {
      status: !!set_emergency_manager_instructions && !!set_emergency_manager_read_bytes && !!set_emergency_manager_write_bytes,
    }
  }

  const budgetData = {
    set_manager: {
      status: tests_status.set_manager.status ? `success`: `failed`,
      instructions: set_manager_instructions,
      readBytes: set_manager_read_bytes,
      writeBytes: set_manager_write_bytes,
    },
    set_rebalance_manager: {
      status: tests_status.set_rebalance_manager.status ? `success`: `failed`,
      instructions: set_rebalance_manager_instructions,
      readBytes: set_rebalance_manager_read_bytes,
      writeBytes: set_rebalance_manager_write_bytes,
    },
    set_fee_receiver: {
      status: tests_status.set_fee_receiver.status ? `success`: `failed`,
      instructions: set_fee_receiver_instructions,
      readBytes: set_fee_receiver_read_bytes,
      writeBytes: set_fee_receiver_write_bytes,
    },
    set_emergency_manager: {
      status: tests_status.set_emergency_manager.status ? `success`: `failed`,
      instructions: set_emergency_manager_instructions,
      readBytes: set_emergency_manager_read_bytes,
      writeBytes: set_emergency_manager_write_bytes,
    }
  }

  return {
    budgetData
  };
};
