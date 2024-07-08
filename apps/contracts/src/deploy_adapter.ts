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
  console.log("Deploying Soroswap Adapter");
  console.log("-------------------------------------------------------");
  await installContract("soroswap_adapter", addressBook, loadedConfig.admin);
  await deployContract(
    "soroswap_adapter",
    "soroswap_adapter",
    addressBook,
    loadedConfig.admin
  );

  const routerAddress =
    "CBHNQTKJD76Q55TINIT3PPP3BKLIKIQEXPTQ32GUUU7I3CHBD5JECZLW";
  const soroswapAdapterInitParams: xdr.ScVal[] = [
    new Address(routerAddress).toScVal(),
  ];

  console.log("Initializing Soroswap Adapter");
  await invokeContract(
    "soroswap_adapter",
    addressBook,
    "initialize",
    soroswapAdapterInitParams,
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
