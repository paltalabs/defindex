import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "./utils/address_book.js";
import {
  airdropAccount,
  airdropAddress,
  deployContract,
  installContract
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
  await installContract("defindex_vault", addressBook, loadedConfig.admin);
  await installContract("defindex_factory", addressBook, loadedConfig.admin);

  const defindexReceiver = loadedConfig.defindexFeeReceiver;
  if (network != "mainnet") await airdropAddress(defindexReceiver);

  const factoryInitParams: xdr.ScVal[] = [
    new Address(loadedConfig.defindexFactoryAdmin).toScVal(),
    new Address(defindexReceiver).toScVal(),
    nativeToScVal(50, {type: "u32"}),
    nativeToScVal(Buffer.from(addressBook.getWasmHash("defindex_vault"), "hex")),
  ];

  await deployContract(
    "defindex_factory",
    "defindex_factory",
    addressBook,
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
