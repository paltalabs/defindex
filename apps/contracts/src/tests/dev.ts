import { Address, Keypair } from "@stellar/stellar-sdk";
import { usdcAddress } from "./common.js";
import { invokeCustomContract } from "../utils/contract.js";
async function main(){
  const kp = Keypair.fromSecret("SC4NWRMSMK6CY4ZUYOLVWSL76GZQVK5FLKRT6JUZQKV224BK3SCFHBC4");
  console.log("Public Key:", kp.publicKey());
  const balance = await invokeCustomContract(usdcAddress.toString(), "balance", [new Address(kp.publicKey()).toScVal()], kp, false);
  console.log("Account Balance:", balance);
}

main().catch(err => {
  console.error("Error runnning the script:", err);
});