import { Address, nativeToScVal, xdr, Asset, Networks } from "@stellar/stellar-sdk";
import { AddressBook } from "./utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
} from "./utils/contract.js";
import { config } from "./utils/env_config.js";

export async function deployContracts(addressBook: AddressBook) {
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Hodl Strategy");
  console.log("-------------------------------------------------------");
  await installContract("hodl_strategy", addressBook, loadedConfig.admin);
  await deployContract(
    "hodl_strategy",
    "hodl_strategy",
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

  const emptyVecScVal = xdr.ScVal.scvVec([]);

  console.log("Initializing DeFindex HODL Strategy");
  await invokeContract(
    "hodl_strategy",
    addressBook,
    "initialize",
    [xlmScVal, emptyVecScVal],
    loadedConfig.admin
  );
}

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

try {
  await deployContracts(addressBook);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
