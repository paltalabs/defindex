import { Address, Asset, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { exit } from "process";
import { AddressBook } from "./utils/address_book.js";
import { config } from "./utils/env_config.js";
import { green, red, yellow } from "./tests/common.js";
import {
  getCreateDeFindexParams,
  manager,
} from "./tests/vault.js";
import { CreateVaultParams } from "./tests/types.js";
import { airdropAccount, invokeContract } from "./utils/contract.js";
import { getTransactionBudget } from "./utils/tx.js";

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const othersAddressBook = AddressBook.loadFromFile(network, "../../public");
const publicAddressBook = AddressBook.loadFromFile(network, "../../../../public");

const asset = process.argv[3];
const allowedAssets = ["XLM", "USDC"];
const allowedStrategies = [
  "blend",
 /*  "hodl",
  "fixed_apr" */
]

if (!asset || !allowedAssets.includes(asset.toUpperCase())) {
  console.log("Please provide a valid asset symbol");
  console.log("Allowed assets are: \n - XLM \n - USDC");
  exit(1);
}

const loadedConfig = config(network);
const xlmAddress = new Address(
  Asset.native().contractId(loadedConfig.passphrase)
);
let assetAddress: Address;
switch (asset.toUpperCase()) {
  case "XLM":
    assetAddress = xlmAddress;
    break;
  case "USDC":
    assetAddress = new Address(othersAddressBook.getContractId("blend_pool_usdc"));
    break;
  default:
    console.log("Invalid asset symbol");
    exit(1);
}
const blend_strategy = addressBook.getContractId(`${asset.toLowerCase()}_blend_strategy`);
const soroswap_router = othersAddressBook.getContractId("soroswap_router");

if (!blend_strategy || !soroswap_router) {
  console.error(
    `Please, make sure that ${network}.contracts.json are up to date at the ./soroban and ./public folders.`
  );
  exit(1);
};

const params: CreateVaultParams[] = [
  {
    address: assetAddress,
    strategies: allowedStrategies.map((strategy) => {
      return {
        name: `${asset} ${strategy.charAt(0).toUpperCase() + strategy.slice(1)} Strategy`,
        address: publicAddressBook.getContractId(`${asset.toLowerCase()}_${strategy}_strategy`),
        paused: false,
      };
    })
  },
];

function getAssetAllocations(assets: CreateVaultParams[]): xdr.ScVal[] {
  return assets.map((asset) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("address"),
        val: asset.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("strategies"),
        val: xdr.ScVal.scvVec(
          asset.strategies.map((strategy) =>
            xdr.ScVal.scvMap([
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("address"),
                val: new Address(strategy.address).toScVal(),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("name"),
                val: nativeToScVal(strategy.name, { type: "string" }),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("paused"),
                val: nativeToScVal(strategy.paused, { type: "bool" }),
              }),
            ])
          )
        ),
      }),
    ]);
  });
}

export async function deployVault(
  addressBook: AddressBook,
  createVaultParams: CreateVaultParams[],
  vaultName: string,
  vaultSymbol: string
): Promise<any> {
  const assets: CreateVaultParams[] = createVaultParams;
  const assetAllocations = getAssetAllocations(assets);

  const createDeFindexParams: xdr.ScVal[] = getCreateDeFindexParams(
    Keypair.fromPublicKey(loadedConfig.vaultEmergencyManager), //Emergency manager
    Keypair.fromPublicKey(loadedConfig.vaultRebalanceManager), //Rebalance manager
    Keypair.fromPublicKey(loadedConfig.defindexFeeReceiver), //Fee receiver
    manager, //Manager
    vaultName, 
    vaultSymbol,
    assetAllocations,
    new Address(soroswap_router),
    true,
  );
  try {
    const result = await invokeContract(
      "defindex_factory",
      addressBook,
      "create_defindex_vault",
      createDeFindexParams,
      loadedConfig.admin,
      false
    );
    console.log(
      "🚀 « DeFindex Vault created with address:",
      scValToNative(result.returnValue)
    );
    const address = scValToNative(result.returnValue);
    const budget = getTransactionBudget(result);
    addressBook.setContractId(`${asset.toLowerCase()}_blend_vault`, address);
    return { address: address, ...budget };
  } catch (error) {
    console.error("Error deploying vault:", error);
    throw error;
  }
}

async function deployBlendVault() {
    if(network != "mainnet") await airdropAccount(loadedConfig.admin);
    console.log(yellow, "--------------------------------------");
    console.log(yellow, "Deploying XLM Blend strategy vault");
    console.log(yellow, "--------------------------------------");
    try {
      const { 
        address: vault_address, 
        instructions:deploy_instructions, 
        readBytes:deploy_read_bytes, 
        writeBytes:deploy_write_bytes
      } = await deployVault(
        addressBook,
        params,
        "Blend Strategy Vault",
        "BSVLT"
      );
      console.log(green, vault_address);
      return {address: vault_address, deploy_instructions, deploy_read_bytes, deploy_write_bytes};
    } catch (error:any) {
      console.error(red, error);
      return {
        address: null, 
        deploy_instructions:0, 
        deploy_read_bytes: 0, 
        deploy_write_bytes: 0,
        error
      };
    }
};

await deployBlendVault();
addressBook.writeToFile();