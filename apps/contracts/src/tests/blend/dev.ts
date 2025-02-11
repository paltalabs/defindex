import { Address, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount, invokeContract } from "../../utils/contract.js";
import { getTransactionBudget } from "../../utils/tx.js";
import { config } from "../../utils/env_config.js";
import { green, purple } from "../common.js";

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);
async function dev(){
  const newUser = Keypair.random();
  if (network !== "mainnet") {
    console.log(green, '----------------------- New account created -------------------------')
    console.log(green, 'Public key: ',newUser.publicKey())
    console.log(green, '---------------------------------------------------------------------')  
    console.log(purple, '-------------------------------------------------------------------')
    console.log(purple, '----------------------- Funding new account -----------------------')
    console.log(purple, '-------------------------------------------------------------------')
    await airdropAccount(newUser);
  }
  const userAccount = network === "mainnet" ? loadedConfig.admin : newUser;
  console.log("Setting Emergengy Manager, Fee Receiver and Manager accounts");
  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  let depositStatus: boolean;
  let depositInstructions: number = 0;
  let depositReadBytes: number = 0;
  let depositWriteBytes: number = 0;
  
  try {
      console.log(purple, '---------------------------------------------------------------------------')
      console.log(purple, '----------------------- Depositing XLM to the Strategy -----------------------')
      console.log(purple, '---------------------------------------------------------------------------')
      const depositParams: xdr.ScVal[] = [
        nativeToScVal(0, { type: "i128" }),
        new Address(userAccount.publicKey()).toScVal(),
      ]
      const depositResult = await invokeContract(
        'blend_strategy',
        addressBook,
        'deposit',
        depositParams,
        userAccount,
        false
      );
      console.log('🚀 « depositResult:', depositResult);
      const {
        instructions,
        readBytes,
        writeBytes
      } = getTransactionBudget(depositResult);
      depositInstructions = instructions;
      depositReadBytes = readBytes;
      depositWriteBytes = writeBytes;
      const depositResultValue = scValToNative(depositResult.returnValue);
      
      console.log(green, '------------ XLM deposited to the Strategy ------------')
      console.log(green, 'depositResult', depositResultValue)
      console.log(green, '----------------------------------------------------')
      depositStatus = true
  }catch(e){
    depositStatus = false
    console.log('error', e)
  }
}

await dev();