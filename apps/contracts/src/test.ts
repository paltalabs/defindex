import {
  Address,
  nativeToScVal,
  scValToNative,
  xdr,
} from "@stellar/stellar-sdk";
import { AddressBook } from "./utils/address_book.js";
import { invokeContract, invokeCustomContract } from "./utils/contract.js";
import { config } from "./utils/env_config.js";
import { getCurrentTimePlusOneHour } from "./utils/tx.js";

export async function testAggregator(addressBook: AddressBook) {
  console.log("-------------------------------------------------------");
  console.log("Testing Soroswap Aggregator");
  console.log("-------------------------------------------------------");

  // const usdc_address = "CCGCRYUTDRP52NOPS35FL7XIOZKKGQWSP3IYFE6B66KD4YOGJMWVC5PR"
  // const xtar_address = "CDPU5TPNUMZ5JY3AUSENSINOEB324WI65AHI7PJBUKR3DJP2ULCBWQCS"

  // console.log("Getting protocols")
  // const result = await invokeContract(
  //   'aggregator',
  //   addressBook,
  //   'get_protocols',
  //   [],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log('🚀 « result:', scValToNative(result.result.retval));

  // console.log("-------------------------------------------------------");
  // console.log("Starting Balances");
  // console.log("-------------------------------------------------------");
  // let usdcUserBalance = await invokeCustomContract(
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
  // let xtarUserBalance = await invokeCustomContract(
  //   xtar_address,
  //   "balance",
  //   [new Address(loadedConfig.admin.publicKey()).toScVal()],
  //   loadedConfig.admin,
  //   true
  // );
  // console.log("XTAR USER BALANCE:", scValToNative(xtarUserBalance.result.retval));

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

await testAggregator(addressBook);
