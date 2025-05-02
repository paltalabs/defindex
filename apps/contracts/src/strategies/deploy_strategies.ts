import { Address } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { InitStrategyDeploy, Strategies, StrategyData } from "../utils/deploy_tools.js";

const network = process.argv[2];
const asset = process.argv[3];

const addressBook = AddressBook.loadFromFile(network);
const externalAddressBook = AddressBook.loadFromFile(network, "workspace/apps/contracts/public");
const publicAddressBook = AddressBook.loadFromFile(network, "workspace/public");

const strategiesToDeploy = [
  Strategies.BLEND,
  Strategies.HODL,
  Strategies.FIXED_APR 
]

const loadedConfig = config(network);

const {assetAddress, strategyData} = InitStrategyDeploy(
  asset,
  strategiesToDeploy as Strategies[],
  publicAddressBook,
  externalAddressBook,
  loadedConfig
);


export async function deployContracts(addressBook: AddressBook, assetAddress: Address, strategies: StrategyData[]) {
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  for (const strategy of strategies) {
    console.log("-------------------------------------------------------");
    console.log(`Deploying ${strategy.name}...`);
    console.log("-------------------------------------------------------");
    await installContract(strategy.wasm_key, addressBook, loadedConfig.admin);

    const assetAddressScval = assetAddress.toScVal();
    await deployContract(
      strategy.name,
      strategy.wasm_key,
      addressBook,
      [assetAddressScval, strategy.args],
      loadedConfig.admin
    );
  }
}

try {
  await deployContracts(addressBook, assetAddress, strategyData);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
