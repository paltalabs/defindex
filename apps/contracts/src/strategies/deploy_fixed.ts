import { Address, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
  invokeCustomContract,
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";

export async function deployFixedAPRStrategy(addressBook: AddressBook) {
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Fixed APR Strategy");
  console.log("-------------------------------------------------------");
  await installContract("fixed_apr_strategy", addressBook, loadedConfig.admin);
  await deployContract(
    "fixed_apr_strategy",
    "fixed_apr_strategy",
    addressBook,
    loadedConfig.admin
  );

  // const xlm = Asset.native();
  // let xlmContractId: string;
  // switch (network) {
  //   case "testnet":
  //     xlmContractId = xlm.contractId(Networks.TESTNET);
  //     break;
  //   case "mainnet":
  //     xlmContractId = xlm.contractId(Networks.PUBLIC);
  //     break;
  //   default:
  //     console.log("Invalid network:", network, "It should be either testnet or mainnet");
  //     return;
  //     break;
  // }
  // const xlmAddress = new Address(xlmContractId);
  // const xlmScVal = xlmAddress.toScVal();

  const soroswapUsdc = "CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI"
  const soroswapScVal = new Address(soroswapUsdc).toScVal();

  const initialAmount = 100_000_000_0_000_000;

  // Mint to the admin the initailAmount
  await invokeCustomContract(
    soroswapUsdc,
    "mint",
    [new Address(loadedConfig.admin.publicKey()).toScVal(), nativeToScVal(initialAmount, { type: "i128" })],
    loadedConfig.getUser("SOROSWAP_MINT_SECRET_KEY")
  )
  
  const initArgs = xdr.ScVal.scvVec([
    nativeToScVal(1000, { type: "u32" }), // 10% APR
    new Address(loadedConfig.admin.publicKey()).toScVal(),
    nativeToScVal(initialAmount, { type: "i128" })
  ]);

  const args: xdr.ScVal[] = [
    soroswapScVal,
    initArgs
  ];

  console.log("Initializing Fixed APR Strategy");
  await invokeContract(
    "fixed_apr_strategy",
    addressBook,
    "initialize",
    args,
    loadedConfig.admin
  );
}

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

try {
  await deployFixedAPRStrategy(addressBook);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
