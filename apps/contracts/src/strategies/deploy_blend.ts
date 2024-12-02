import { Address, Asset, Networks, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";

export async function deployBlendStrategy(addressBook: AddressBook) {
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
  await deployContract(
    "blend_strategy",
    "blend_strategy",
    addressBook,
    loadedConfig.admin
  );

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
    new Address("CCEVW3EEW4GRUZTZRTAMJAXD6XIF5IG7YQJMEEMKMVVGFPESTRXY2ZAV").toScVal(), //Blend pool on testnet!
  ]);

  const args: xdr.ScVal[] = [
    xlmScVal,
    initArgs
  ];

  console.log("Initializing Blend Strategy");
  await invokeContract(
    "blend_strategy",
    addressBook,
    "initialize",
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
