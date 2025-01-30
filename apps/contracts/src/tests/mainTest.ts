import { Address, Asset, Keypair } from "@stellar/stellar-sdk";
import { exit } from "process";
import { AddressBook } from "../utils/address_book.js";
import { airdropAccount } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { testBlendStrategy } from "./blend/test_strategy.js";
import { testBlendVault } from "./blend/test_vault.js";
import { green, red, usdcAddress, xtarAddress, yellow } from "./common.js";
import {
  admin,
  CreateVaultParams,
  emergencyManager,
  feeReceiver,
  manager,
  mintToken
} from "./vault.js";
import { testVaultOneAssetTwoStrategies } from "./vault/one_aset_two_strategies.js";
import { testVaultOneAssetOneStrategy } from "./vault/one_asset_one_strategy.js";
import { testVaultTwoAssetsOneStrategy } from "./vault/two_assets_one_strategy.js";
import { testVaultTwoAssetsTwoStrategies } from "./vault/two_assets_two_strategies.js";

const args = process.argv.slice(2);
const network = args[0];
const tests = args[1];

const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);
const xlmAddress = new Address(
  Asset.native().contractId(loadedConfig.passphrase)
);

const testUser = Keypair.random();


const oneStrategyParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "Hodl Strategy",
        address: addressBook.getContractId("hodl_strategy"),
        paused: false,
      },
    ],
  },
];
const twoStrategyParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "Blend Strategy",
        address: addressBook.getContractId("blend_strategy"),
        paused: false,
      },
      {
        name: "Fixed Strategy",
        address: addressBook.getContractId("fixed_apr_strategy"),
        paused: false,
      },
    ],
  },
];
const twoAssetOneStrategyParams: CreateVaultParams[] = [
  {
    address: xtarAddress,
    strategies: [
      {
        name: "Strategy 1",
        address: addressBook.getContractId("fixed_xtar_strategy"),
        paused: false,
      },
    ],
  },
  {
    address: usdcAddress,
    strategies: [
      {
        name: "Stretegy 2",
        address: addressBook.getContractId("fixed_usdc_strategy"),
        paused: false,
      },
    ],
  },
];
const twoAssetTwoStrategyParams: CreateVaultParams[] = [
  {
    address: xlmAddress,
    strategies: [
      {
        name: "A0 S1",
        address: addressBook.getContractId("blend_strategy"),
        paused: false,
      },
      {
        name: "A0 S2",
        address: addressBook.getContractId("fixed_apr_strategy"),
        paused: false,
      },
    ],
  },
  {
    address: usdcAddress,
    strategies: [
      {
        name: "A1 S1",
        address: addressBook.getContractId("hodl_usdc_strategy"),
        paused: false,
      },
      {
        name: "A1 S2",
        address: addressBook.getContractId("fixed_usdc_strategy"),
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
    await mintToken(testUser, 987654321, xtarAddress)
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
      const oneAssetOneStrategy = await testVaultOneAssetOneStrategy(addressBook, oneStrategyParams, testUser);
      const oneAssetTwoStrategies = await testVaultOneAssetTwoStrategies(addressBook, twoStrategyParams, testUser, xlmAddress);
      const twoAssetsOneStrategy = await testVaultTwoAssetsOneStrategy(addressBook, twoAssetOneStrategyParams, testUser, xlmAddress);
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
      await testVaultOneAssetOneStrategy(addressBook, oneStrategyParams, testUser);
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-oats":
    console.log(yellow, "Testing two strategies vault");
    try {
      await prepareEnvironment();
      await testVaultOneAssetTwoStrategies(addressBook, twoStrategyParams, testUser, xlmAddress);
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
      await testVaultTwoAssetsTwoStrategies(addressBook, twoAssetTwoStrategyParams, testUser, xlmAddress);
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
