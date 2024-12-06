import { fileURLToPath } from "url";

import dotenv from "dotenv";
import path from "path";
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
dotenv.config({ path: path.join(__dirname, "../.env") });

import { airdropAccount, createToolkit, deployContract, deploySorobanToken, installContract, invokeCustomContract, SorobanToolkit } from "soroban-toolkit";
import { Address, Asset, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";

export async function deployContracts() {
  const toolkitLoader = createToolkit({
    adminSecret: process.env.ADMIN_SECRET_KEY!,
    contractPaths: {
      defindex_vault:
        "./target/wasm32-unknown-unknown/release/defindex_vault.optimized.wasm",
      defindex_factory:
        "./target/wasm32-unknown-unknown/release/defindex_factory.optimized.wasm",
      soroswap_adapter:
        "./target/wasm32-unknown-unknown/release/soroswap_adapter.optimized.wasm",
      hodl_strategy:
        "./target/wasm32-unknown-unknown/release/hodl_strategy.optimized.wasm",
      blend_strategy:
        "./target/wasm32-unknown-unknown/release/blend_strategy.optimized.wasm",
      fixed_apr_strategy:
        "./target/wasm32-unknown-unknown/release/fixed_apr_strategy.optimized.wasm",
    },
    customNetworks: [{
      network: "standalone",
      friendbotUrl: "http://localhost:8000/friendbot",
      horizonRpcUrl: "http://localhost:8000",
      sorobanRpcUrl: "http://localhost:8000/soroban/rpc",
      networkPassphrase: "Standalone Network ; February 2017"
    }],
    verbose: "full"
  });

  const toolkit: SorobanToolkit = toolkitLoader.getNetworkToolkit(network)
  
  await airdropAccount(toolkit, toolkit.admin);
  
  let account = await toolkit.horizonRpc.loadAccount(
    toolkit.admin.publicKey()
  );

  console.log("publicKey", toolkit.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type == "native");
  console.log("Current Admin account balance:", balance[0].balance, "XLM");

  console.log("-------------------------------------------------------");
  console.log("Deploying Hodl Strategy");
  console.log("-------------------------------------------------------");
  const xlmAddress = Asset.native().contractId(toolkit.passphrase);
  const xlmScVal = new Address(xlmAddress).toScVal();
  const emptyVecScVal = xdr.ScVal.scvVec([]);

  await installContract(toolkit, "hodl_strategy");
  await deployContract(
    toolkit,
    "hodl_strategy",
    [xlmScVal, emptyVecScVal],
  );

//   const routerAddress =
//     "CBHNQTKJD76Q55TINIT3PPP3BKLIKIQEXPTQ32GUUU7I3CHBD5JECZLW";
//   const soroswapAdapterInitParams: xdr.ScVal[] = [
//     new Address(routerAddress).toScVal(),
//   ];

//   console.log("Initializing Soroswap Adapter");
//   await invokeContract(
//     "soroswap_adapter",
//     addressBook,
//     "initialize",
//     soroswapAdapterInitParams,
//     loadedConfig.admin
//   );
}

const network = process.argv[2];

try {
  await deployContracts();
} catch (e) {
  console.error(e);
}
