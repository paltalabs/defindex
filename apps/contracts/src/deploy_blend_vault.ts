import { Address, Asset, Keypair, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import { exit } from "process";
import { AddressBook } from "./utils/address_book.js";
import { config } from "./utils/env_config.js";
import { green, purple, red, usdcAddress, xtarAddress, yellow } from "./tests/common.js";
import {
  CreateVaultParams,
  emergencyManager,
  feeReceiver,
  getCreateDeFindexParams,
  manager,
  mintToken
} from "./tests/vault.js";
import { deployDefindexVault } from "./tests/vault/utils.js";
import { depositToStrategy } from "./tests/strategy.js";
import { airdropAccount, invokeContract } from "./utils/contract.js";
import { getTransactionBudget } from "./utils/tx.js";

const args = process.argv.slice(2);
const network = args[0];

const addressBook = AddressBook.loadFromFile(network);
const othersAddressBook = AddressBook.loadFromFile(network, "../../public");


const loadedConfig = config(network);
const xlmAddress = new Address(
  Asset.native().contractId(loadedConfig.passphrase)
);
const blend_strategy = addressBook.getContractId("blend_strategy");
const soroswap_router = othersAddressBook.getContractId("soroswap_router");

if (!blend_strategy || !soroswap_router) {
  console.error(
    `Please, make sure that ${network}.contracts.json are up to date at the ./soroban and ./public folders.`
  );
  exit(1);
};
const params: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "Blend Strategy",
        address: blend_strategy,
        paused: false,
      },
    ],
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
    loadedConfig.admin,
    loadedConfig.admin,
    loadedConfig.admin,
    manager,
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
    addressBook.setContractId('blend_strategy_vault', address);
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