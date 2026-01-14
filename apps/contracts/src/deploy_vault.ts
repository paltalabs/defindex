import { Address, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { exit } from "process";
import { green, red, yellow } from "./tests/common.js";
import { CreateVaultParams } from "./tests/types.js";
import { AddressBook } from "./utils/address_book.js";
import { airdropAccount, invokeContract } from "./utils/contract.js";
import { AssetFromString, Strategies } from "./utils/deploy_tools.js";
import { config } from "./utils/env_config.js";
import { getTransactionBudget } from "./utils/tx.js";
import {
  getCreateDeFindexParams,
} from "./utils/vault.js";

const network = process.argv[2];
const asset = process.argv[3];

const addressBook = AddressBook.loadFromFile(network);
const externalAddressBook = AddressBook.loadFromFile(network, "../../public");
const publicAddressBook = AddressBook.loadFromFile(network, "../../../../public");

const allowedStrategies = [
  Strategies.BLEND,
  /* Strategies.HODL, */
  /* Strategies.FIXED_APR */
]

const loadedConfig = config(network);

const soroswap_router = externalAddressBook.getContractId("soroswap_router");

if (!soroswap_router) {
  console.error(
    `Please, make sure that ${network}.contracts.json are up to date at the ./soroban and ./public folders.`
  );
  exit(1);
};

const assetAddress = AssetFromString(asset, loadedConfig, externalAddressBook);

const params: CreateVaultParams[] = [
  {
    address: assetAddress,
    strategies: allowedStrategies.map((strategy) => {
      return {
        name: `${asset} ${strategy.charAt(0).toUpperCase() + strategy.slice(1)} Strategy`,
        address: publicAddressBook.getContractId(`${asset}_${strategy}_strategy`),
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
  const assetAllocations = getAssetAllocations(createVaultParams);

  const createDeFindexParams: xdr.ScVal[] = getCreateDeFindexParams(
    Keypair.fromPublicKey(loadedConfig.vaultEmergencyManager), //Emergency manager
    Keypair.fromPublicKey(loadedConfig.vaultRebalanceManager), //Rebalance manager
    Keypair.fromPublicKey(loadedConfig.defindexFeeReceiver), //Fee receiver
    Keypair.fromPublicKey(loadedConfig.vaultManager), //Manager
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

async function deployDefindexVault() {
    if(network != "mainnet") await airdropAccount(loadedConfig.admin);
    console.log(yellow, "--------------------------------------");
    console.log(yellow, `Deploying ${asset} Blend Vault...`);
    console.log(yellow, "--------------------------------------");
    try {
      const { 
        address: vault_address, 
        instructions:deploy_instructions, 
        readBytes:deploy_read_bytes, 
        writeBytes:deploy_write_bytes
      } = await deployVault(
        publicAddressBook,
        params,
        loadedConfig.vaultName,
        loadedConfig.vaultSymbol
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

const deployResult = await deployDefindexVault();
if (deployResult.address) {
  console.log(green, `Deployed vault at address: ${deployResult.address}`);
}
addressBook.setContractId(`${asset.toLowerCase()}_paltalabs_vault`, deployResult.address);
await addressBook.writeToFile();