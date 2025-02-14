import { Address, Asset, Keypair, nativeToScVal, xdr } from "@stellar/stellar-sdk";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeCustomContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { mintToken } from "../tests/vault.js";
import { USDC_ADDRESS } from "../constants.js";


export async function deployContracts(addressBook: AddressBook) {
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Hodl Strategy");
  console.log("-------------------------------------------------------");
  await installContract("hodl_strategy", addressBook, loadedConfig.admin);


  const usdcScVal = USDC_ADDRESS.toScVal();

  const emptyVecScVal = xdr.ScVal.scvVec([]);

  await deployContract(
    "hodl_usdc_strategy",
    "hodl_strategy",
    addressBook,
    [usdcScVal, emptyVecScVal],
    loadedConfig.admin
  );

   const deployedAddress = addressBook.getContractId("fixed_xtar_strategy")
  
    const temp_user = Keypair.random();
    if (network != "mainnet"){ 
      await airdropAccount(temp_user);
      await mintToken(temp_user, 9000_0000000, USDC_ADDRESS);
    }
  
    // Mint to the admin the initailAmount
    await invokeCustomContract(
      USDC_ADDRESS.toString(),
      "transfer",
      [new Address(temp_user.publicKey()).toScVal(), new Address(deployedAddress).toScVal(), nativeToScVal(9000_0000000, { type: "i128" })],
      temp_user
    )
}

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

try {
  await deployContracts(addressBook);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
