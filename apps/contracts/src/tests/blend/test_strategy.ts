import { Address, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount, invokeContract } from "../../utils/contract.js";
import { getTransactionBudget } from "../../utils/tx.js";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const purple = '\x1b[35m%s\x1b[0m';
const green = '\x1b[32m%s\x1b[0m';

export async function testBlendStrategy(user?: Keypair) {
  // Create and fund a new user account if not provided
  const newUser = Keypair.random();
  console.log(green, '----------------------- New account created -------------------------')
  console.log(green, 'Public key: ',newUser.publicKey())
  console.log(green, '---------------------------------------------------------------------')
  let depositStatus: boolean;
  let withdrawStatus: boolean;
  let harvestStatus: boolean;

  let depositInstructions: number = 0;
  let depositReadBytes: number = 0;
  let depositWriteBytes: number = 0;

  let withdrawInstructions: number = 0;
  let withdrawReadBytes: number = 0;
  let withdrawWriteBytes: number = 0;

  let harvestInstructions: number = 0;
  let harvestReadBytes: number = 0;
  let harvestWriteBytes: number = 0;

  if (network !== "mainnet") {
    console.log(purple, '-------------------------------------------------------------------')
    console.log(purple, '----------------------- Funding new account -----------------------')
    console.log(purple, '-------------------------------------------------------------------')
    await airdropAccount(newUser);
  }

  // Deposit XLM into Blend Strategy
  try {
      console.log(purple, '---------------------------------------------------------------------------')
      console.log(purple, '----------------------- Depositing XLM to the Strategy -----------------------')
      console.log(purple, '---------------------------------------------------------------------------')
      const depositParams: xdr.ScVal[] = [
        nativeToScVal(1000_0_000_000, { type: "i128" }),
        new Address(newUser.publicKey()).toScVal(),
      ]
      const depositResult = await invokeContract(
        'blend_strategy',
        addressBook,
        'deposit',
        depositParams,
        newUser,
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
  
    // Wait for 1 minute
    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '----------------------- Waiting for 1 minute -----------------------')
    console.log(purple, '---------------------------------------------------------------------------')
    await new Promise(resolve => setTimeout(resolve, 60000));
  
    // Withdrawing XLM from Blend Strategy
    try {
      console.log(purple, '---------------------------------------------------------------------------')
      console.log(purple, '----------------------- Withdrawing XLM from the Strategy -----------------------')
      console.log(purple, '---------------------------------------------------------------------------')
      
      const balanceScVal = await invokeContract(
        'blend_strategy',
        addressBook,
        'balance',
        [new Address(newUser.publicKey()).toScVal()],
        newUser,
        true
      );
      console.log('🚀 « balanceScVal:', balanceScVal);
      
      const balance = scValToNative(balanceScVal.result.retval);
      console.log('🚀 « balance:', balance);

      const withdrawParams: xdr.ScVal[] = [
        nativeToScVal(1000_0_000_000, { type: "i128" }),
        new Address(newUser.publicKey()).toScVal(),
        new Address(newUser.publicKey()).toScVal(),
      ]
      const withdrawResult = await invokeContract(
        'blend_strategy',
        addressBook,
        'withdraw',
        withdrawParams,
        newUser,
        false
      );
      const {
        instructions,
        readBytes,
        writeBytes
      } = getTransactionBudget(withdrawResult);
      const withdrawResultValue = scValToNative(withdrawResult.returnValue);

      withdrawInstructions = instructions;
      withdrawReadBytes = readBytes;
      withdrawWriteBytes = writeBytes;

      console.log(green, '------------ XLM withdrawed from the Strategy ------------')
      console.log(green, 'withdrawResult', withdrawResultValue)
      console.log(green, '----------------------------------------------------')
      withdrawStatus = true
    }catch(e){
      withdrawStatus = false
      console.log('error', e)
    }
  
    // Harvest rewards from Blend Strategy
    try {
      console.log(purple, '---------------------------------------------------------------------------')
      console.log(purple, '----------------------- Harvesting from the Strategy -----------------------')
      console.log(purple, '---------------------------------------------------------------------------')

      const harvestParams: xdr.ScVal[] = [
        new Address(newUser.publicKey()).toScVal(),
      ]
      const harvestResult = await invokeContract(
        'blend_strategy',
        addressBook,
        'harvest',
        harvestParams,
        newUser,
        false
      );
      const {
        instructions,
        readBytes,
        writeBytes
      } = getTransactionBudget(harvestResult);
      const harvestResultValue = scValToNative(harvestResult.returnValue);

      harvestInstructions = instructions;
      harvestReadBytes = readBytes;
      harvestWriteBytes = writeBytes;
      
      console.log(green, '------------ BLND Harvested from the vault ------------')
      console.log(green, 'harvestResult', harvestResultValue)
      console.log(green, '----------------------------------------------------')
      harvestStatus = true
    }catch(e){
      harvestStatus = false
      console.log('error', e)
    }

    return { 
      status:{
        depositStatus: depositStatus ?  '✅ Success' : '❌ Failed', 
        withdrawStatus: withdrawStatus ?  '✅ Success' : '❌ Failed',
        harvestStatus: harvestStatus ?  '✅ Success' : '❌ Failed',
      },
      budget: {
        deposit: {
          instructions: depositInstructions,
          readBytes: depositReadBytes,
          writeBytes: depositWriteBytes
        },
        withdraw: {
          instructions: withdrawInstructions,
          readBytes: withdrawReadBytes,
          writeBytes: withdrawWriteBytes
        },
        harvest: {
          instructions: harvestInstructions,
          readBytes: harvestReadBytes,
          writeBytes: harvestWriteBytes
        }
      }
    }
}

//await testBlendStrategy();