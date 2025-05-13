import { Address, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { green } from "./tests/common.js";
import { AddressBook } from "./utils/address_book.js";
import { airdropAccount, invokeContract, invokeCustomContract } from "./utils/contract.js";
import { config } from "./utils/env_config.js";
import { Instruction, mapInstructionsToParams } from "./utils/vault.js";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network, "../../../../public");

const deposit = async () => {
  const amount = [1_0_000_000];
  const invest = false;
  try {
    const vault_address = addressBook.getContractId("usdc_palta_vault");
    
    const user = config(network).getUser('TEST_USER');
    if (network !== "mainnet") await airdropAccount(user);

    const amountsDesired = amount.map((am) => BigInt(am)); // Amounts to deposit
    const amountsMin = amount.map((_) => BigInt(0)); // Minimum amount for transaction to succeed

    const depositParams: xdr.ScVal[] = [
      xdr.ScVal.scvVec(
        amountsDesired.map((amount) => nativeToScVal(amount, { type: "i128" }))
      ),
      xdr.ScVal.scvVec(
        amountsMin.map((min) => nativeToScVal(min, { type: "i128" }))
      ),
      new Address(user.publicKey()).toScVal(),
      xdr.ScVal.scvBool(!!invest),
    ];
    const result = await invokeCustomContract(
      vault_address,
      "deposit",
      depositParams,
      user
    );
    console.log(green, "Deposit successful:", scValToNative(result.returnValue));
    
  } catch (error: any) {
    console.error("Error in depositToVault:", error);
  }
};

const withdraw = async (
) => {
  const withdrawAmount = 2_0_000_000;
  const min_amounts_out = [0];
  try {
    const vault_address = addressBook.getContractId("usdc_palta_vault");
    
    const user = config(network).getUser('TEST_USER');
    if (network !== "mainnet") await airdropAccount(user);

    const minAmountsOut: xdr.ScVal[] = min_amounts_out.map((amount) =>
      nativeToScVal(BigInt(amount), { type: "i128" })
    );
    const withdrawParams: xdr.ScVal[] = [
      nativeToScVal(BigInt(withdrawAmount), { type: "i128" }),
      xdr.ScVal.scvVec(minAmountsOut),
      new Address(user.publicKey()).toScVal()
    ];
    const result = await invokeCustomContract(
      vault_address,
      "withdraw",
      withdrawParams,
      user
    );
    console.log(
      green, "Withdrawal successful:",
      scValToNative(result.returnValue)
    );
  } catch (error: any) {
    console.error("Error in withdraw:", error);
  }
};

const invest = async () => {
  const amount = 1_0_000_000;
  try {
    const vault_address = addressBook.getContractId("usdc_palta_vault");
    const admin = config(network).getUser('DEPLOYER_SECRET_KEY');
    if (network !== "mainnet") await airdropAccount(admin);

    const invest_amount = BigInt(Math.ceil(amount));
    const instructions: Instruction[] = [
      {
        type: "Invest",
        strategy: addressBook.getContractId("usdc_blend_autocompound_fixed_strategy"),
        amount: invest_amount,
      }
    ];
    const params = mapInstructionsToParams(instructions);
    const rebalanceResult = await invokeCustomContract(
      vault_address,
      "rebalance",
      [new Address(admin.publicKey()).toScVal(), params],
      admin
    );
    if(rebalanceResult.status != 'ERROR') {
      console.log(green, "Invest result:", rebalanceResult.status);
      console.log("Tx Hash:", rebalanceResult.txHash);
      return { result: rebalanceResult, status: 'ok' };
    }
    else {
      console.error('Invest failed:', rebalanceResult.errorResult.result());
      throw rebalanceResult.errorResult;
    }
  } catch (error: any) { 
    console.error("Error in invest:", error);
  }
};

const unwind = async () => {
  const amount = 4_0_000_000;
  try {
    const vault_address = addressBook.getContractId("usdc_palta_vault");
    const admin = config(network).getUser('DEPLOYER_SECRET_KEY');
    if (network !== "mainnet") await airdropAccount(admin);

    const invest_amount = BigInt(Math.ceil(amount));
    const instructions: Instruction[] = [
      {
        type: "Unwind",
        strategy: addressBook.getContractId("usdc_blend_autocompound_fixed_strategy"),
        amount: invest_amount,
      }
    ];
    const params = mapInstructionsToParams(instructions);
    const rebalanceResult = await invokeCustomContract(
      vault_address,
      "rebalance",
      [new Address(admin.publicKey()).toScVal(), params],
      admin
    );
    if(rebalanceResult.status != 'ERROR') {
      console.log(green, "Unwind result:", rebalanceResult.status);
      console.log("Tx Hash:", rebalanceResult.txHash);
      return { result: rebalanceResult, status: 'ok' };
    }
    else {
      console.error('Unwind failed:', rebalanceResult.errorResult.result());
      throw rebalanceResult.errorResult;
    }

  } catch (error: any) {
    console.error("Error in unwind:", error);
  }
};

const harvest = async () => {
  try {
    const blend_keeper = config(network).getUser('BLEND_KEEPER_SECRET_KEY');
    const data = nativeToScVal(null)
    const harvestParams: xdr.ScVal[] = [
      new Address(blend_keeper.publicKey()).toScVal(),
      data
    ]
    const harvestResult = await invokeContract(
      'xlm_blend_strategy',
      addressBook,
      'harvest',
      harvestParams,
      blend_keeper,
      false
    );
    console.log(green, "Harvest result:", harvestResult.status);
    const harvestResultValue = scValToNative(harvestResult.returnValue);
    console.log("Harvest result value:", harvestResultValue);
  } catch (error: any) {
    console.error("Error in harvest:", error);
  }
}
// await deposit();
// await invest();
//await withdraw();
//await unwind();