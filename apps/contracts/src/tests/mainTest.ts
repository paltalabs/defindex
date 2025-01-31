import { Address, Asset, Keypair } from "@stellar/stellar-sdk";
import { exit } from "process";
import { AddressBook } from "../utils/address_book.js";
import { airdropAccount } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { testBlendStrategy } from "./blend/test_strategy.js";
import { testBlendVault } from "./blend/test_vault.js";
import { green, red, yellow } from "./common.js";
import {
  admin,
  emergencyManager,
  feeReceiver,
  manager,
  mintToken
} from "./vault.js";
import { testVaultOneAssetTwoStrategies } from "./vault/one_aset_two_strategies.js";
import { testVaultOneAssetOneStrategy } from "./vault/one_asset_one_strategy.js";
import { testVaultTwoAssetsOneStrategy } from "./vault/two_assets_one_strategy.js";
import { testVaultTwoAssetsTwoStrategies } from "./vault/two_assets_two_strategies.js";
import { CreateVaultParams } from "./types.js";
import { USDC_ADDRESS } from "../constants.js";

const args = process.argv.slice(2);
const network = args[0];
const tests = args[1];

const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);
const xlmAddress = new Address(
  Asset.native().contractId(loadedConfig.passphrase)
);

const testUser = Keypair.random();


const oneAssetOneStrategyParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "Blend Strategy",
        address: addressBook.getContractId("blend_strategy"),
        paused: false,
      },
    ],
  },
];

const oneAssetTwoStrategyParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "Blend Strategy 0",
        address: addressBook.getContractId("xlm_blend_strategy_0"),
        paused: false,
      },
      {
        name: "Blend Strategy 0",
        address: addressBook.getContractId("xlm_blend_strategy_1"),
        paused: false,
      },
    ],
  },
];

const twoAssetOneStrategyParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "Blend xlm Strategy",
        address: addressBook.getContractId("xlm_blend_strategy_0"),
        paused: false,
      },
    ],
  },
  {
    address: USDC_ADDRESS,
    strategies: [
      {
        name: "Blend usdc Strategy",
        address: addressBook.getContractId("usdc_blend_strategy_0"),
        paused: false,
      },
    ],
  },
];

const twoAssetTwoStrategiesParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "blend xlm Strategy 0",
        address: addressBook.getContractId("xlm_blend_strategy_0"),
        paused: false,
      },
      {
        name: "blend xlm Strategy 1",
        address: addressBook.getContractId("xlm_blend_strategy_1"),
        paused: false,
      },
    ],
  },
  {
    address: USDC_ADDRESS,
    strategies: [
      {
        name: "blend usdc Strategy 0",
        address: addressBook.getContractId("usdc_blend_strategy_0"),
        paused: false,
      },
      {
        name: "blend usdc Strategy 1",
        address: addressBook.getContractId("usdc_blend_strategy_1"),
        paused: false,
      },
    ],
  },
];



async function prepareEnvironment() {
  if (network !== "mainnet") {
    await airdropAccount(testUser);
    await airdropAccount(admin);
    await airdropAccount(emergencyManager);
    await airdropAccount(feeReceiver);
    await airdropAccount(manager);
    await airdropAccount(testUser);
    await mintToken(testUser, 987654321)
  }
}


switch (tests) {
  case "-h":
    console.log("");
    console.log(green, "Usage: yarn test <network> <test>");
    console.log("");
    console.log(yellow, "   Options:");
    console.log(
      yellow,
      "       Network: mainnet (not avaliable yet) | testnet"
    );
    console.log("");
    console.log(
      yellow,
      `   Tests:       
       -a  all tests,
       -oaos one strategy tests,
       -oats two strategy tests, 
       -taos two assets one strategy test,
       -tats two assets two strategies test,
       -bs blend strategy tests,
       -bv blend vault tests`       
    );
    exit(0);
  case "-a":
    console.log(yellow, "Running all tests");
    try {
      await prepareEnvironment();
      const oneAssetOneStrategy = await testVaultOneAssetOneStrategy(addressBook, oneAssetOneStrategyParams, testUser);
      const oneAssetTwoStrategies = await testVaultOneAssetTwoStrategies(addressBook, oneAssetTwoStrategyParams, testUser, xlmAddress);
      const twoAssetsOneStrategy = await testVaultTwoAssetsOneStrategy(addressBook, twoAssetOneStrategyParams, testUser, xlmAddress);
      const twoAssetsTwoStrategies = await testVaultTwoAssetsTwoStrategies(addressBook, twoAssetTwoStrategiesParams, testUser, xlmAddress);
      const blendStrategy = await testBlendStrategy();
      const blendVault = await testBlendVault();
      console.log(yellow, "----------------------------------------------------------------------------------------------------------------------------------------------")
      console.log(yellow, "All tests finished, results:");
      console.log(yellow, "----------------------------------------------------------------------------------------------------------------------------------------------")
      console.log("")
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------")
      console.log(green, "One strategy results")
      console.table(oneAssetOneStrategy.tableData);
      console.table(oneAssetOneStrategy.budgetData);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log("");
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log(green, "Two strategies results");
      console.table(oneAssetTwoStrategies.tableData);
      console.table(oneAssetTwoStrategies.budgetData);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log("");
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log(green, "Two asssets one strategy results");
      console.table(twoAssetsOneStrategy.tableData);
      console.table(twoAssetsOneStrategy.budgetData);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log("");
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log(green, "Two asssets two strategies results");
      console.table(twoAssetsTwoStrategies.tableData);
      console.table(twoAssetsTwoStrategies.budgetData);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log("");
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log(green, "Blend strategy test status");
      console.table(blendStrategy.status);
      console.table(blendStrategy.budget);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log("");
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log(green, "Blend vault test status");
      console.table(blendVault!.status);
      console.table(blendVault!.budget);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-oaos":
    console.log(yellow, "Testing one strategy vault");
    try {
      await prepareEnvironment();
      await testVaultOneAssetOneStrategy(addressBook, oneAssetOneStrategyParams, testUser);
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-oats":
    console.log(yellow, "Testing two strategies vault");
    try {
      await prepareEnvironment();
      await testVaultOneAssetTwoStrategies(addressBook, oneAssetTwoStrategyParams, testUser, xlmAddress);
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-taos":
    console.log(yellow, "Testing two assets one strategy vault");
    try {
      await prepareEnvironment();
      await testVaultTwoAssetsOneStrategy(addressBook, twoAssetOneStrategyParams, testUser, xlmAddress);
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-tats":
    console.log(yellow, "Testing two assets one strategy vault");
    try {
      await prepareEnvironment();
      await testVaultTwoAssetsTwoStrategies(addressBook, twoAssetTwoStrategiesParams, testUser, xlmAddress);
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-bs":
    console.log(yellow, "Testing blend strategy");
    try {
      const blendStrategy = await testBlendStrategy();
      console.log(green, "Blend strategy test status");
      console.table(blendStrategy.status);
      console.table(blendStrategy.budget);
      console.log(green, "---------------------------------------");
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-bv":
    console.log(yellow, "Testing blend vault");
    try {
      const blendVault = await testBlendVault();
      console.log(green, "Blend vault test status");
      console.table(blendVault!.status);
      console.table(blendVault!.budget);
      console.log(green, "---------------------------------------");
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  default:
    console.log(yellow, "For help run: yarn test <network> -h");
    exit(0);
}
