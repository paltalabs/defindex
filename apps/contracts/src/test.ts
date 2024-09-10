import {
  Address,
  nativeToScVal,
  scValToNative,
  xdr
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";
import { AddressBook } from "./utils/address_book.js";
import { airdropAccount, invokeContract } from "./utils/contract.js";
import { config } from "./utils/env_config.js";

export async function test_factory(addressBook: AddressBook) {
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Testing Create DeFindex on Factory");
  console.log("-------------------------------------------------------");

  const usdc_address = "CCGCRYUTDRP52NOPS35FL7XIOZKKGQWSP3IYFE6B66KD4YOGJMWVC5PR"
  const xtar_address = "CDPU5TPNUMZ5JY3AUSENSINOEB324WI65AHI7PJBUKR3DJP2ULCBWQCS"

  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network != "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network != "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network != "mainnet") await airdropAccount(manager);

  const tokens = [usdc_address, xtar_address];
  const ratios = [1, 1];

  const strategyParamsRaw = [
    {
      name: "Strategy 1",
      address: usdc_address, //TODO: Use a deployed strategy address here
    },
  ];

  const strategyParamsScVal = strategyParamsRaw.map((param) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('address'),
        val: new Address(param.address).toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol('name'),
        val: nativeToScVal(param.name, {type: "string"}),
      }),
    ]);
  });

  const strategyParamsScValVec = xdr.ScVal.scvVec(strategyParamsScVal);

  const createDeFindexParams: xdr.ScVal[] = [
    new Address(emergencyManager.publicKey()).toScVal(),
    new Address(feeReceiver.publicKey()).toScVal(),
    new Address(manager.publicKey()).toScVal(),
    xdr.ScVal.scvVec(tokens.map((token) => new Address(token).toScVal())),
    xdr.ScVal.scvVec(ratios.map((ratio) => nativeToScVal(ratio, {type: "u32"}))),
    strategyParamsScValVec,
    nativeToScVal(randomBytes(32)),
  ];

  const result = await invokeContract(
    'defindex_factory',
    addressBook,
    'create_defindex_vault',
    createDeFindexParams,
    loadedConfig.admin
  );

  console.log('🚀 « result:', scValToNative(result.returnValue));

  // console.log(nativeToScVal(result.result.retval));
  // const dexDistributionRaw = [
  //   {
  //     protocol_id: "soroswap",
  //     path: [xtar_address, usdc_address],
  //     parts: 1,
  //     is_exact_in: true,
  //   },
  // ];

  // const dexDistributionScVal = dexDistributionRaw.map((distribution) => {
  //   return xdr.ScVal.scvMap([
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('is_exact_in'),
  //       val: xdr.ScVal.scvBool(distribution.is_exact_in),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('parts'),
  //       val: nativeToScVal(distribution.parts, {type: "i128"}),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('path'),
  //       val: nativeToScVal(distribution.path.map((pathAddress) => new Address(pathAddress))),
  //     }),
  //     new xdr.ScMapEntry({
  //       key: xdr.ScVal.scvSymbol('protocol_id'),
  //       val: xdr.ScVal.scvString(distribution.protocol_id),
  //     }),
  //   ]);
  // });

  // const dexDistributionScValVec = xdr.ScVal.scvVec(dexDistributionScVal);

  // const aggregatorSwapParams: xdr.ScVal[] = [
  //   new Address(xtar_address).toScVal(), //_from_token: Address,
  //   new Address(usdc_address).toScVal(), //_dest_token: Address,
  //   nativeToScVal(1000000000, {type: "i128"}),
  //   nativeToScVal(0, {type: "i128"}),
  //   dexDistributionScValVec, // proxy_addresses: Vec<ProxyAddressPair>,
  //   new Address(loadedConfig.admin.publicKey()).toScVal(), //admin: Address,
  //   nativeToScVal(getCurrentTimePlusOneHour()), //deadline
  // ];

  // console.log("Initializing Aggregator")
  // await invokeContract(
  //   'aggregator',
  //   addressBook,
  //   'swap',
  //   aggregatorSwapParams,
  //   loadedConfig.admin
  // );

  // console.log("-------------------------------------------------------");
  // console.log("Ending Balances");
  // console.log("-------------------------------------------------------");
  // usdcUserBalance = await invokeCustomContract(
  //   usdc_address,
  //   "balance",
  //   [new Address(loadedConfig.admin.publicKey()).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log(
  //   "USDC USER BALANCE:",
  //   scValToNative(usdcUserBalance.result.retval)
  // );
  // xtarUserBalance = await invokeCustomContract(
  //   xtar_address,
  //   "balance",
  //   [new Address(loadedConfig.admin.publicKey()).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log("XTAR USER BALANCE:", scValToNative(xtarUserBalance.result.retval));
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);

await test_factory(addressBook);
