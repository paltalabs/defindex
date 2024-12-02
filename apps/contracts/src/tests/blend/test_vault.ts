import { Address, Asset, Keypair, nativeToScVal, Networks, scValToNative, xdr } from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";
import { exit } from "process";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount, invokeContract } from "../../utils/contract.js";
import { config } from "../../utils/env_config.js";
import { AssetInvestmentAllocation, depositToVault, investVault } from "../vault.js";

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
  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  const blendStrategyAddress = addressBook.getContractId("blend_strategy");

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

  const createDeFindexParams: xdr.ScVal[] = [
    new Address(emergencyManager.publicKey()).toScVal(),
    new Address(feeReceiver.publicKey()).toScVal(),
    nativeToScVal(100, { type: "u32" }), 
    nativeToScVal("BLND Vault", { type: "string" }),
    nativeToScVal("BLNVLT", { type: "string" }),
    new Address(manager.publicKey()).toScVal(),
    xdr.ScVal.scvVec(assetAllocations),
    nativeToScVal(randomBytes(32)),
  ];

  const initialAmount = 100_0_000_000;
  let blendVaultAddress: string = "";

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

    blendVaultAddress = scValToNative(createResult.returnValue);
    console.log(green, '----------------------- Vault created -------------------------')
    console.log(green, 'createResult', blendVaultAddress)
    console.log(green, '---------------------------------------------------------------')
  } catch(e){
    console.log('❌ Error Creating the vault', e)
    exit("Error Creating");
  }

  try {    
    // Deposit assets to the vault
    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '----------------------- Depositing XLM to the vault -----------------------')
    console.log(purple, '---------------------------------------------------------------------------')
    const { user, balanceBefore: depositBalanceBefore, result: depositResult, balanceAfter: depositBalanceAfter } 
      = await depositToVault(blendVaultAddress, [initialAmount], newUser);
    
    console.log(green, '------------ XLM deposited to the vault ------------')
    console.log(green, 'Deposit balance before: ', depositBalanceBefore)
    console.log(green, 'depositResult', depositResult)
    console.log(green, 'Deposit balance after: ', depositBalanceAfter)
    console.log(green, '----------------------------------------------------')
  } catch (error) {
    console.log('❌ Error depositing into the vault:', error);
    exit("Error Depositing");
  }

  try {
    // Invest in strategy
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
    
    const investResult = await investVault(blendVaultAddress, investParams, manager)
    console.log('🚀 « investResult:', investResult);
    
    console.log(green, '---------------------- Invested in strategy ----------------------')
    console.log(green, 'Invested: ', investResult, ' in the strategy')
    console.log(green, '------------------------------------------------------------------')
  } catch (error) {
    console.log('❌ Error Investing the Vault:', error);
    exit("Error Investing");
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
await testBlendVault();