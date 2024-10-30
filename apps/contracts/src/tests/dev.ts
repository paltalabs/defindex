import { airdropAccount } from "../utils/contract.js";
import { withdrawFromVault } from "./vault.js";
import { Keypair } from "@stellar/stellar-sdk";

const user = Keypair.fromSecret("SA77N6PLHDFRYDNYE3YJQBPTRNODMVYP5WWF2SG42DXB52GW2FWOG2B3")
const contract = "CCNWF3D7FJCZKYCAD6FAO3JNPRHG6SVXHO5YTFDZRXSPOJXL6TIBWP3Y"
await withdrawFromVault(contract, BigInt(10000), user)

// const badUser = Keypair.random()
// await airdropAccount(badUser);
// await withdrawFromVault(contract, BigInt(1000), badUser)