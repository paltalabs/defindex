import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";
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
  console.log("Deploying DeFindex Factory");
  console.log("-------------------------------------------------------");
  await installContract("defindex", addressBook, loadedConfig.admin);
  await installContract("defindex_factory", addressBook, loadedConfig.admin);
  await deployContract(
    "defindex_factory",
    "defindex_factory",
    addressBook,
    loadedConfig.admin
  );

  const factoryInitParams: xdr.ScVal[] = [
    nativeToScVal(Buffer.from(addressBook.getWasmHash("defindex"), "hex")),
  ];

  console.log("Initializing DeFindex Factory");
  await invokeContract(
    "defindex_factory",
    addressBook,
    "initialize",
    factoryInitParams,
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