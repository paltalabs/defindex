import { Address } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract
} from "../utils/contract.js";
import { InitStrategyDeploy, Strategies, StrategyData } from "../utils/deploy_tools.js";
import { config } from "../utils/env_config.js";

const network = process.argv[2];
const asset = process.argv[3];
const number_of_strategies = process.argv[4] || "1";
const forceInstall = process.argv[5] || false;

const addressBook = AddressBook.loadFromFile(network);
const externalAddressBook = AddressBook.loadFromFile(network, "../../public");

const strategiesToDeploy = [
  Strategies.BLEND,
  // Strategies.HODL,
  // Strategies.FIXED_APR 
]

const loadedConfig = config(network);

const {assetAddress, strategyData} = InitStrategyDeploy(
  asset,
  strategiesToDeploy as Strategies[],
  externalAddressBook,
  loadedConfig
);


export async function deployContracts(addressBook: AddressBook, assetAddress: Address, strategies: StrategyData[], quantity = 1) {

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
    if (forceInstall) {
      console.log("Installing contract")
      await installContract(strategy.wasm_key, addressBook, loadedConfig.admin);
    }

    const assetAddressScval = assetAddress.toScVal();
    for (let i = 0; i < quantity; i++) {
      const strategyName = quantity > 1 ? `${strategy.name}_${i}` : strategy.name;
      await deployContract(
        strategyName,
        strategy.wasm_key,
        addressBook,
        [assetAddressScval, strategy.args],
        loadedConfig.admin
      );
    }
  }
}

try {
  await deployContracts(addressBook, assetAddress, strategyData, parseInt(number_of_strategies));
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
