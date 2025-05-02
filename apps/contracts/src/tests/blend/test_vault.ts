import { Address, Asset, Keypair, nativeToScVal, Networks, scValToNative, xdr } from "@stellar/stellar-sdk";
import { SOROSWAP_ROUTER } from "../../constants.js";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount, invokeContract } from "../../utils/contract.js";
import { config } from "../../utils/env_config.js";
import { depositToVault, getCreateDeFindexParams, Instruction, rebalanceManager, rebalanceVault } from "../../utils/vault.js";
import { getTransactionBudget } from "../../utils/tx.js";
import { AssetInvestmentAllocation } from "../types.js";

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

const purple = '\x1b[35m%s\x1b[0m';
const green = '\x1b[32m%s\x1b[0m';



export async function testBlendVault(user?: Keypair) {
  const newUser = Keypair.random();
  console.log(green, '----------------------- New account created -------------------------')
  console.log(green, 'Public key: ',newUser.publicKey())
  console.log(green, '---------------------------------------------------------------------')

  if (network !== "mainnet") {
    console.log(purple, '-------------------------------------------------------------------')
    console.log(purple, '----------------------- Funding new account -----------------------')
    console.log(purple, '-------------------------------------------------------------------')
    await airdropAccount(newUser);
  }

  console.log("Setting Emergengy Manager, Fee Receiver and Manager accounts");
  const emergencyManager = loadedConfig.getUser("ADMIN_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("ADMIN_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("ADMIN_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  const blendStrategyAddress = addressBook.getContractId("xlm_blend_strategy");

  const xlm = Asset.native();
  let xlmContractId: string;
  switch (network) {
    case "testnet":
      xlmContractId = xlm.contractId(Networks.TESTNET);
      break;
    case "mainnet":
      xlmContractId = xlm.contractId(Networks.PUBLIC);
      break;
    default:
      console.log("Invalid network:", network, "It should be either testnet or mainnet");
      return;
  }

  const assets = [
    {
      address: new Address(xlmContractId),
      strategies: [
        {
          name: "Blend Strategy",
          address: blendStrategyAddress,
          paused: false
        },
      ]
    }
  ];

  const assetAllocations = assets.map((asset) => {
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
                val: nativeToScVal(false, { type: "bool" }),
              }),
            ])
          )
        ),
      }),
    ]);
  });

  const createDeFindexParams: xdr.ScVal[] = getCreateDeFindexParams(
    emergencyManager,
    rebalanceManager,
    feeReceiver,
    manager,
    "BLND Vault",
    "BLNVLT",
    assetAllocations,
    new Address(SOROSWAP_ROUTER),
    true,
  )

  const initialAmount = 100_0_000_000;
  let blendVaultAddress: string = "";

  let createStatus: boolean;
  let depositStatus: boolean;
  let investStatus: boolean;

  let createInstructions: number = 0;
  let createReadBytes: number = 0;
  let createWriteBytes: number = 0;

  let depositInstructions: number = 0;
  let depositReadBytes: number = 0;
  let depositWriteBytes: number = 0;

  let investInstructions: number = 0;
  let investReadBytes: number = 0;
  let investWriteBytes: number = 0;
  
  // Create vault
  try {
    console.log(purple, '--------------------------------------------------------------')
    console.log(purple, '----------------------- Creating vault -----------------------')
    console.log(purple, '--------------------------------------------------------------')
    const createResult = await invokeContract(
      'defindex_factory',
      addressBook,
      'create_defindex_vault',
      createDeFindexParams,
      manager,
      false
    );
    const {
      instructions,
      readBytes,
      writeBytes
    } = getTransactionBudget(createResult);
    console.log("🟢",instructions, readBytes, writeBytes)
    createInstructions = instructions;
    createReadBytes = readBytes;
    createWriteBytes = writeBytes;

    blendVaultAddress = scValToNative(createResult.returnValue);
    console.log(green, '----------------------- Vault created -------------------------')
    console.log(green, 'createResult', blendVaultAddress)
    console.log(green, '---------------------------------------------------------------')
    createStatus = true;
  } catch(e){
    console.log('❌ Error Creating the vault', e)
    createStatus = false;
  }

  // Deposit assets to the vault
  try {    
    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '----------------------- Depositing XLM to the vault -----------------------')
    console.log(purple, '---------------------------------------------------------------------------')
    const { user, balanceBefore: depositBalanceBefore, result: depositResult, balanceAfter: depositBalanceAfter } 
      = await depositToVault(blendVaultAddress, [initialAmount], newUser, false);
    const {
      instructions,
      readBytes,
      writeBytes
    } = getTransactionBudget(depositResult);

    depositInstructions = instructions;
    depositReadBytes = readBytes;
    depositWriteBytes = writeBytes;

    console.log(green, '------------ XLM deposited to the vault ------------')
    console.log(green, 'Deposit balance before: ', depositBalanceBefore)
    console.log(green, 'depositResult', depositResult)
    console.log(green, 'Deposit balance after: ', depositBalanceAfter)
    console.log(green, '----------------------------------------------------')
    depositStatus = true;
  } catch (error) {
    depositStatus = false;
    console.log('❌ Error depositing into the vault:', error);
  }

  // Invest in strategy
  try {
    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '-------------------------- Investing in strategy --------------------------')
    console.log(purple, '---------------------------------------------------------------------------')

    const investParams: AssetInvestmentAllocation[] = [
      {
        asset: new Address(xlmContractId),
        strategy_investments: [
          {
            amount: BigInt(50_0_000_000),
            strategy: new Address(blendStrategyAddress)
          }
        ]
      }
    ];

    const investArgs: Instruction[] = [
      {
        type: "Invest",
        strategy: blendStrategyAddress,
        amount: BigInt(50_0_000_000),
      },
    ];
    
    const {
      result:investResult,
      instructions,
      readBytes,
      writeBytes
    } = await rebalanceVault(blendVaultAddress, investArgs, manager);
    console.log('🚀 « investResult:', investResult);

    investInstructions = instructions;
    investReadBytes = readBytes;
    investWriteBytes = writeBytes;
    
    console.log(green, '---------------------- Invested in strategy ----------------------')
    console.log(green, 'Invested: ', investResult, ' in the strategy')
    console.log(green, '------------------------------------------------------------------')
    investStatus = true;
  } catch (error) {
    console.log('❌ Error Investing the Vault:', error);
    investStatus = false;
  }
  return { 
    status:{
      createStatus: createStatus ? '✅ Success' : '❌ Failed',
      depositStatus: depositStatus ? '✅ Success' : '❌ Failed', 
      investStatus: investStatus ? '✅ Success' : '❌ Failed' 
    },
    budget: {
      create: {
        instructions: createInstructions,
        readBytes: createReadBytes,
        writeBytes: createWriteBytes
      },
      deposit: {
        instructions: depositInstructions,
        readBytes: depositReadBytes,
        writeBytes: depositWriteBytes
      },
      invest: {
        instructions: investInstructions,
        readBytes: investReadBytes,
        writeBytes: investWriteBytes
      }
    }
  }
  // try { 
  //   // Withdraw assets from the vault
  //   console.log(purple, '------------------------------------------------------------------------------')
  //   console.log(purple, '----------------------- Withdrawing XLM from the vault -----------------------')
  //   console.log(purple, '------------------------------------------------------------------------------')
  //   const withdrawAmount = Math.ceil(100);
  //   const withdrawParams: xdr.ScVal[] = [
  //     nativeToScVal(withdrawAmount, { type: "i128" }),
  //     new Address(newUser.publicKey()).toScVal(),
  //   ]
  //   const withdrawResult = await invokeCustomContract(
  //     blendVaultAddress,
  //     'withdraw',
  //     withdrawParams,
  //     newUser,
  //     false
  //   );
  //   const withdrawResultValue = scValToNative(withdrawResult.returnValue);
  //   console.log(green, '---------------- XLM withdrawn from the vault ----------------')
  //   console.log(green, 'Withdrawed: ', withdrawResultValue, ' from the vault')
  //   console.log(green, '--------------------------------------------------------------')
  // } catch (error) {
  //   console.log('🚀 « error:', error);
    
  // }
}
//await testBlendVault();