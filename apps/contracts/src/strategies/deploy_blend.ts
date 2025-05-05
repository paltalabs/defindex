import { Address, Asset, nativeToScVal, Networks, xdr } from "@stellar/stellar-sdk";
import { BLEND_POOL, BLEND_TOKEN, SOROSWAP_ROUTER } from "../constants.js";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
const network = process.argv[2];
const asset = process.argv[3];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);
const othersAddressBook = AddressBook.loadFromFile(network, "../../public");


const allowedAssets = ["XLM", "USDC"];

export async function deployBlendStrategy(addressBook: AddressBook, asset_symbol?: string) {
  if (!asset_symbol || !allowedAssets.includes(asset_symbol.toUpperCase())) {
    console.log("Please provide a valid asset symbol");
    console.log("Allowed assets are: \n - XLM \n - USDC");
    return;
  }
  if (network == "standalone") {
    console.log("Blend Strategy can only be tested in testnet or mainnet");
    console.log("Since it requires Blend protocol to be deployed");
    return;
  };
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);

  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  ).catch((e: any) => {
    if (e.response && e.response.status === 404) {
      console.error("Account not found. Please check that the public key has enough funds.");
      throw new Error("Account not found");
    } else {
      console.error("An unexpected error occurred:", e);
      throw e;
    }
  }).then((account) => {
    return account;
  });

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
  const usdcAddress = new Address(othersAddressBook.getContractId("blend_pool_usdc"));
  const xlmScVal = xlmAddress.toScVal();
  const usdcScVal = usdcAddress.toScVal();

  const initArgs = xdr.ScVal.scvVec([
    new Address(BLEND_POOL).toScVal(), // blend_pool_address: The address of the Blend pool where assets are deposited
    new Address(BLEND_TOKEN).toScVal(), // blend_token: The address of the reward token (e.g., BLND) issued by the Blend pool
    new Address(SOROSWAP_ROUTER).toScVal(), // soroswap_router: The address of the Soroswap AMM router for asset swaps
    nativeToScVal(40, { type: "i128" }), // reward_threshold: The minimum reward amount that triggers reinvestment
    new Address(loadedConfig.blendKeeper).toScVal() // keeper: The address of the keeper that can call the harvest function
  ]);

  const args: xdr.ScVal[] = [
    asset_symbol.toUpperCase() == "USDC" ? usdcScVal : xlmScVal, // asset: The asset to be managed by the strategy (XLM or USDC)
    initArgs
  ];

  await deployContract(
    `${asset_symbol.toLowerCase()}_blend_strategy`,
    "blend_strategy",
    addressBook,
    args,
    loadedConfig.admin
  );
}

try {
  await deployBlendStrategy(addressBook, asset);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
