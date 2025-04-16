import { Address, Asset, nativeToScVal, Networks, xdr } from "@stellar/stellar-sdk";
// import { BLEND_POOL_TESTNET, BLEND_TOKEN_TESTNET, SOROSWAP_ROUTER } from "../constants.js";
import { BLEND_POOL, BLEND_TOKEN, SOROSWAP_ROUTER } from "../constants.js";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";

export async function deployBlendStrategy(addressBook: AddressBook) {
  if (network == "standalone") {
    console.log("Blend Strategy can only be tested in testnet or mainnet");
    console.log("Since it requires Blend protocol to be deployed");
    return;
  };
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Blend Strategy");
  console.log("-------------------------------------------------------");
  await installContract("blend_strategy", addressBook, loadedConfig.admin);

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
  const xlmAddress = new Address(xlmContractId);
  const xlmScVal = xlmAddress.toScVal();

  // CLAIM IDs
  // For XLM we have 
  // * `reserve_token_ids` - The ids of the reserves to claiming emissions for
  const claim_ids = xdr.ScVal.scvVec([
    nativeToScVal(1, { type: "u32" }),
  ]);

  const initArgs = xdr.ScVal.scvVec([
    new Address(BLEND_POOL).toScVal(), // blend_pool_address: The address of the Blend pool where assets are deposited
    new Address(BLEND_TOKEN).toScVal(), // blend_token: The address of the reward token (e.g., BLND) issued by the Blend pool
    new Address(SOROSWAP_ROUTER).toScVal(), // soroswap_router: The address of the Soroswap AMM router for asset swaps
    nativeToScVal(40, { type: "i128" }), // reward_threshold: The minimum reward amount that triggers reinvestment
    new Address(loadedConfig.blendKeeper.publicKey()).toScVal() // keeper: The address of the keeper that can call the harvest function
  ]);

  const args: xdr.ScVal[] = [
    xlmScVal,
    initArgs
  ];

  await deployContract(
    "blend_strategy",
    "blend_strategy",
    addressBook,
    args,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);
console.log("🚀 ~ addressBook:", addressBook)
const othersAddressBook = AddressBook.loadFromFile(network, "../../public");
console.log("🚀 ~ othersAddressBook:", othersAddressBook)

try {
  await deployBlendStrategy(addressBook);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
