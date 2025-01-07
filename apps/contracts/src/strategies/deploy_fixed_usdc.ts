import { Address, Keypair, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import { SOROSWAP_USDC } from "../constants.js";
import { mintToken } from "../tests/vault.js";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeCustomContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";

export async function deployFixedAPRStrategy(addressBook: AddressBook) {
  if (network == "standalone") {
    console.log("Fixed Strategy can only be tested in testnet or mainnet");
    console.log("Since it requires a custom token, we are currently using soroswap USDC");
    console.log("TODO: Create our own token for standalone testing");
    return;
  };
  if(network == 'mainnet'){
    return;
  }
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Fixed APR with USDC Strategy");
  console.log("-------------------------------------------------------");
  await installContract("fixed_apr_strategy", addressBook, loadedConfig.admin);

  const usdcAddress = new Address(SOROSWAP_USDC);
  
  const initArgs = xdr.ScVal.scvVec([
    nativeToScVal(1000, { type: "u32" }), // 10% APR
  ]);

  const args: xdr.ScVal[] = [
    usdcAddress.toScVal(),
    initArgs
  ];

  await deployContract(
    "fixed_usdc_strategy",
    "fixed_apr_strategy",
    addressBook,
    args,
    loadedConfig.admin
  );

  const deployedAddress = addressBook.getContractId("fixed_usdc_strategy")

  const temp_user = Keypair.random();
  if (network != "mainnet"){ 
    await airdropAccount(temp_user);
    await mintToken(temp_user, 9000_0000000);
  }

  // Mint to the admin the initailAmount
  await invokeCustomContract(
    usdcAddress.toString(),
    "transfer",
    [new Address(temp_user.publicKey()).toScVal(), new Address(deployedAddress).toScVal(), nativeToScVal(9000_0000000, { type: "i128" })],
    temp_user
  )
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
