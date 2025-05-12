import { Address, nativeToScVal, scValToNative, xdr } from "@stellar/stellar-sdk";
import * as fs from 'fs';
import * as path from 'path';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
import { BLEND_TOKEN, SOROSWAP_ROUTER } from "../constants.js";
import { AddressBook } from "../utils/address_book.js";
import {
  airdropAccount,
  deployContract,
  installContract,
  invokeContract,
  invokeCustomContract
} from "../utils/contract.js";
import { config } from "../utils/env_config.js";

const network = process.argv[2];
const asset = process.argv[3];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);
const othersAddressBook = AddressBook.loadFromFile(network, "../../public");

function loadBlendDeployConfig() {
  const __filename = fileURLToPath(import.meta.url);
  const __dirname = dirname(__filename);
  const configPath = path.join(__dirname, "../../src/strategies/blend_deploy_config.json");
  const configData = fs.readFileSync(configPath, 'utf8');
  return JSON.parse(configData);
}

function checkBlendDeployConfigFormat(config: any) {
  const requiredEnvs = ["mainnet", "testnet"];
  const requiredStrategyFields = [
    "name",
    "keeper",
    "asset",
    "asset_symbol",
    "reward_threshold",
    "blend_pool_address",
    "blend_pool_name"
  ];
  for (const env of requiredEnvs) {
    if (!(env in config)) {
      throw new Error(`Missing environment '${env}' in blend_deploy_config.json`);
    }
    if (!Array.isArray(config[env].strategies)) {
      throw new Error(`'strategies' for '${env}' must be an array in blend_deploy_config.json`);
    }
    for (const [i, strategy] of config[env].strategies.entries()) {
      for (const field of requiredStrategyFields) {
        if (!(field in strategy)) {
          throw new Error(`Missing field '${field}' in strategy[${i}] for '${env}' in blend_deploy_config.json`);
        }
      }
    }
  }
}

async function loadAdminAccount(horizonRpc: any, admin: any) {
  return await horizonRpc.loadAccount(admin.publicKey())
  .catch((e: any) => {
    if (e.response && e.response.status === 404) {
        console.error("Account not found. Please check that the public key has enough funds.");
        throw new Error("Account not found");
      } else {
        console.error("An unexpected error occurred:", e);
        throw e;
      }
    })
    .then((account: any) => {
      return account;
    });
  }
  
function getNativeBalance(account: any): Array<{ asset_type: string; balance: string; [key: string]: any }> {
  return account.balances.filter((item: { asset_type: string; balance: string; [key: string]: any }) => item.asset_type == "native");
}
  
function checkAllowedAsset(asset_symbol: string | undefined, allowedAssets: string[]): boolean {
  return asset_symbol !== undefined && allowedAssets.includes(asset_symbol.toUpperCase());
}

function constructBlendStrategyArgs(strategy: any): xdr.ScVal[] {
  const initArgs = xdr.ScVal.scvVec([
    new Address(strategy.blend_pool_address).toScVal(), // blend_pool_address: The address of the Blend pool where assets are deposited
    new Address(BLEND_TOKEN).toScVal(), // blend_token: The address of the reward token (e.g., BLND) issued by the Blend pool
    new Address(SOROSWAP_ROUTER).toScVal(), // soroswap_router: The address of the Soroswap AMM router for asset swaps
    nativeToScVal(40, { type: "i128" }), // reward_threshold: The minimum reward amount that triggers reinvestment
    new Address(strategy.keeper).toScVal() // keeper: The address of the keeper that can call the harvest function
  ]);

  return [
    new Address(strategy.asset).toScVal(), // asset: The asset to be managed by the strategy (XLM or USDC)
    initArgs
  ];
}

async function calculateDepositAmount(
  blendPoolAddress: string,
  asset: string,
  admin: any
): Promise<number> {
  const reserves = await invokeCustomContract(
    blendPoolAddress,
    "get_reserve",
    [
      new Address(asset).toScVal(),
    ],
    admin,
    true
  );
  const reservesData = scValToNative(reserves.result.retval);
  const b_rate = reservesData.data.b_rate;
  
  return b_rate * 1001 / 1000000000000 + 1;
}

export async function deployBlendStrategy(addressBook: AddressBook, asset_symbol?: string) {
  if (network == "standalone") {
    console.log("Blend Strategy can only be tested in testnet or mainnet");
    console.log("Since it requires Blend protocol to be deployed");
    return;
  };
  if (network != "mainnet") await airdropAccount(loadedConfig.admin);
  
  const blendDeployConfig = loadBlendDeployConfig();
  checkBlendDeployConfigFormat(blendDeployConfig);

  let account = await loadAdminAccount(loadedConfig.horizonRpc, loadedConfig.admin);

  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = getNativeBalance(account);
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Deploying Blend Strategy");
  console.log("-------------------------------------------------------");
  if (blendDeployConfig[network].install_contract == "true") {
    await installContract("blend_strategy", addressBook, loadedConfig.admin);
  }


  for (const strategy of blendDeployConfig[network].strategies) {
    const args = constructBlendStrategyArgs(strategy);
    
    let contractKey = `${strategy.asset_symbol.toLowerCase()}_blend_${strategy.name}_${strategy.blend_pool_name}_strategy`
    await deployContract(
      contractKey,
      "blend_strategy",
      addressBook,
      args,
      loadedConfig.admin
    );

    // We need to  get how much is the deposit amount
    // To do that we need to get the b_rate from the reserves of the blend pool

    const depositAmount = await calculateDepositAmount(
      strategy.blend_pool_address,
      strategy.asset,
      loadedConfig.admin
    );

    // Deposit 2000 stroops to the strategy
    // We need the first depositor to deposit at least 1001 stroops
    // But since optimal deposit amount may reduce the amount of stroops,
    // we deposit 2000 to ensure we're above the minimum threshold

    await invokeContract(
      contractKey,
      addressBook,
      "deposit",
      [
        nativeToScVal(depositAmount, { type: "i128" }),
        new Address(loadedConfig.admin.publicKey()).toScVal(),
      ],
      loadedConfig.admin
    );
    
  }
  
}

try {
  await deployBlendStrategy(addressBook, asset);
} catch (e) {
  console.error(e);
}
addressBook.writeToFile();
