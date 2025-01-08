import { Address, Asset, nativeToScVal, Networks, xdr } from "@stellar/stellar-sdk";
import { BLEND_POOL_TESTNET, BLEND_TOKEN_TESTNET, SOROSWAP_ROUTER } from "../constants.js";
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
      break;
  }
  const xlmAddress = new Address(xlmContractId);
  const xlmScVal = xlmAddress.toScVal();

  const initArgs = xdr.ScVal.scvVec([
    new Address(BLEND_POOL_TESTNET).toScVal(), //Blend pool on testnet!
    nativeToScVal(0, { type: "u32" }), // ReserveId 0 is XLM
    new Address(BLEND_TOKEN_TESTNET).toScVal(), // BLND Token
    new Address(SOROSWAP_ROUTER).toScVal(), // Soroswap router
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

try {
  await deployBlendStrategy(addressBook);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
