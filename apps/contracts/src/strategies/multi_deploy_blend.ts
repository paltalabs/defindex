import { Address, Asset, nativeToScVal, Networks, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { exit } from "process";
import { red, yellow } from "../tests/common.js";

export async function multiDeployBlendStrategies(quantity: number, asset_key: string) {
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
  console.log("Installing Blend Strategy");
  console.log("-------------------------------------------------------");
  await installContract("blend_strategy", addressBook, loadedConfig.admin);

  const blendFixedXlmUsdcPool: string = othersAddressBook.getContractId("blend_fixed_xlm_usdc_pool");
  const blndToken: string = othersAddressBook.getContractId("blnd_token");
  const soroswapRouter: string  = othersAddressBook.getContractId("soroswap_router");
  
  const network_passphrase = (
    ()=> {
      switch (network) {
        case "testnet":
          return Networks.TESTNET;
          case "mainnet":
            return Networks.PUBLIC;
            default:
              console.error("Invalid network:", network, "It should be either testnet or mainnet");
              exit(1);
            }
          }
        )();
        
  const init_args = (
    ()=> {
      switch (asset_key) {
        case "soroswap_usdc":
          return {
            address: othersAddressBook.getContractId("soroswap_usdc"),
            claim_id: xdr.ScVal.scvVec([
              nativeToScVal(3, { type: "u32" }),
            ])
          };
        case "xlm":
          return {
            address: Asset.native().contractId(network_passphrase),
            claim_id: xdr.ScVal.scvVec([
              nativeToScVal(1, { type: "u32" }),
            ])
          };
        default:
          console.error("Invalid asset key:", asset_key, "It should be either soroswap_usdc or xlm");
          exit(1);
      }
    }
  )();

  const asset_symbol = asset_key === "soroswap_usdc" ? "USDC" : "XLM";

  console.log(yellow, "----------------------------------------------------------------------");
  console.log(yellow, `Deploying ${quantity}, Blend Strategies with ${asset_key} in ${network}`);
  console.log(yellow, "----------------------------------------------------------------------");
  for (let i = 0; i < quantity; i++) {

    const initArgs = xdr.ScVal.scvVec([
      new Address(blendFixedXlmUsdcPool).toScVal(), //Blend pool on testnet!
      nativeToScVal(0, { type: "u32" }), // ReserveId 0 is XLM
      new Address(blndToken).toScVal(), // BLND Token
      new Address(soroswapRouter).toScVal(), // Soroswap router
      init_args.claim_id,
    ]);
  
    const args: xdr.ScVal[] = [
      new Address(init_args.address).toScVal(),
      initArgs
    ];
  
    await deployContract(
      `${asset_symbol}_blend_strategy_${i}`,
      "blend_strategy",
      addressBook,
      args,
      loadedConfig.admin
    );
  }
  return;
}

const network = process.argv[2];
const quantity = process.argv[3];
const asset_key = process.argv[4].toLowerCase();
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);
const othersAddressBook = AddressBook.loadFromFile(network, "../../public");

if(!Number(quantity) || Number(quantity) <= 0 || typeof Number(quantity) !== "number") {
  console.log(red, "Please provide a valid number to deploy multiple strategies");
  exit(1);
};

if(!asset_key || typeof asset_key !== "string") {
  console.log(red, "Please provide a valid asset key to deploy the strategies");
  exit(1);
}

const allowed_assets = ['soroswap_usdc', 'xlm']

if(!allowed_assets.includes(asset_key)) {
  console.log(red, "Please provide a valid asset key to deploy the strategies");
  console.log(yellow, "Allowed assets are: soroswap_usdc, xlm");
  exit(1);
}

try {
  await multiDeployBlendStrategies(Number(quantity), asset_key);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
