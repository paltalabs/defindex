import { Address, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { config } from "../utils/env_config.js";
import { invokeContract } from "../utils/contract.js";
import { green } from "../tests/common.js";
import { AddressBook } from "../utils/address_book.js";
const network = process.argv[2];
const asset = process.argv[3];
const allowedAssets = ["XLM", "USDC"];
if (!asset || !allowedAssets.includes(asset.toUpperCase())) {
  console.log("Please provide a valid asset symbol");
  console.log("Allowed assets are: \n - XLM \n - USDC");
  process.exit(1);
}
const addressBook = AddressBook.loadFromFile(network);

const harvest = async () => {
  try {
    const blend_keeper = config(network).getUser('BLEND_KEEPER_SECRET_KEY');
    const data = nativeToScVal(null)
    const harvestParams: xdr.ScVal[] = [
      new Address(blend_keeper.publicKey()).toScVal(),
      data
    ]
    const harvestResult = await invokeContract(
      `${asset.toLowerCase()}_blend_strategy`,
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

await harvest();