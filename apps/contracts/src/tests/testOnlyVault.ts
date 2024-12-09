import {
  Address,
  Asset,
  Keypair,
  nativeToScVal,
  scValToNative,
  xdr
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";
import { AddressBook } from "../utils/address_book.js";
import { airdropAccount, invokeContract } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { depositToVault } from "./vault.js";

// const soroswapUSDC = new Address("CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI");

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const hodl_strategy = addressBook.getContractId("hodl_strategy");
const fixed_apr_strategy = addressBook.getContractId("fixed_apr_strategy");
const xlm: Asset = Asset.native()

const loadedConfig = config(network);
// modify the address to the deployed vault
export async function deployVault(addressBook: AddressBook) {
  if (network !== "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type === "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Create Vault on Factory");
  console.log("-------------------------------------------------------");

  console.log("Setting Emergengy Manager, Fee Receiver and Manager accounts");
  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  const assets = [
    {
      address: new Address(xlm.contractId(loadedConfig.passphrase)),
      strategies: [
        {
          name: "Hodl Strategy",
          address: addressBook.getContractId("hodl_strategy"),
          paused: false
        }
      ]
    }
  ];

  const assetAllocations = assets.map((asset) => {
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
                val: nativeToScVal(false, { type: "bool" }),
              }),
            ])
          )
        ),
      }),
    ]);
  });

  const createDeFindexParams: xdr.ScVal[] = [
    new Address(emergencyManager.publicKey()).toScVal(),
    new Address(feeReceiver.publicKey()).toScVal(),
    nativeToScVal(100, { type: "u32" }), 
    nativeToScVal("HODL FIXED Vault", { type: "string" }),
    nativeToScVal("HDFXVLT", { type: "string" }),
    new Address(manager.publicKey()).toScVal(),
    xdr.ScVal.scvVec(assetAllocations),
    nativeToScVal(randomBytes(32)),
  ];

  const result = await invokeContract(
    'defindex_factory',
    addressBook,
    'create_defindex_vault',
    createDeFindexParams,
    loadedConfig.admin
  );

  console.log('🚀 « DeFindex Vault created with address:', scValToNative(result.returnValue));
  return scValToNative(result.returnValue);
}
if (network == "standalone") {
  console.log("This can only be tested in testnet or mainnet");
  console.log("Since it requires a custom token, we are currently using soroswap USDC");
  console.log("TODO: Create our own token for standalone testing");
  process.exit();
};

const testUser = Keypair.random();
if (network !== "mainnet") await airdropAccount(testUser);

const vaultAddress = await deployVault(addressBook);
await depositToVault(vaultAddress, [986754321], testUser);