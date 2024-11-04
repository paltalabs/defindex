import {
  Address,
  Asset,
  nativeToScVal,
  Networks,
  scValToNative,
  xdr
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";
import { depositToVault } from "./tests/vault.js";
import { AddressBook } from "./utils/address_book.js";
import { airdropAccount, invokeContract } from "./utils/contract.js";
import { config } from "./utils/env_config.js";


export async function test_factory(addressBook: AddressBook) {
  if (network !== "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type === "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Testing Create DeFindex on Factory");
  console.log("-------------------------------------------------------");

  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  const assets = [
    {
      address: new Address(xlm.contractId(passphrase)),
      strategies: [
        {
          name: "Strategy 1",
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
    nativeToScVal(100, { type: "u32" }),  // Setting vault_share as 100 bps for demonstration
    nativeToScVal("Test Vault", { type: "string" }),
    nativeToScVal("DFT-Test-Vault", { type: "string" }),
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

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const xlm: Asset = Asset.native()
const passphrase = network === "mainnet" ? Networks.PUBLIC : network === "testnet" ? Networks.TESTNET : Networks.STANDALONE;

const loadedConfig = config(network);

const deployedVault = await test_factory(addressBook);
await depositToVault(deployedVault);
await depositToVault(deployedVault);

// await getDfTokenBalance("CCL54UEU2IGTCMIJOYXELIMVA46CLT3N5OG35XN45APXDZYHYLABF53N", "GDAMXOJUSW6O67UVI6U4LBHI5IIJFUKQVDHPKNFKOIYRLYB2LA6YDAFI", loadedConfig.admin)
// await depositToVault("CCIOE3BLPYOYDFB5KALLDXED2CZT3GJDZSHY453U4TTOIRZLAKMKZPLR");

