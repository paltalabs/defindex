import { Address, Asset, Keypair } from "@stellar/stellar-sdk";
import { exit } from "process";
import { SOROSWAP_USDC, SOROSWAP_XTAR } from "../constants.js";
import { AddressBook } from "../utils/address_book.js";
import { airdropAccount } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { testBlendStrategy } from "./blend/test_strategy.js";
import { testBlendVault } from "./blend/test_vault.js";
import { checkUserBalance } from "./strategy.js";
import {
  admin,
  CreateVaultParams,
  deployVault,
  depositToVault,
  emergencyManager,
  feeReceiver,
  fetchCurrentInvestedFunds,
  fetchParsedCurrentIdleFunds,
  Instruction,
  manager,
  mintToken,
  rebalanceVault,
  withdrawFromVault
} from "./vault.js";
import { testVaultOneAssetOneStrategy } from "./vault/one_strategy.js";

const args = process.argv.slice(2);
const network = args[0];
const tests = args[1];

const addressBook = AddressBook.loadFromFile(network);

const loadedConfig = config(network);
const xlmAddress = new Address(
  Asset.native().contractId(loadedConfig.passphrase)
);

const usdcAddress = new Address(SOROSWAP_USDC);
const xtarAddress = new Address(SOROSWAP_XTAR);

const testUser = Keypair.random();

const initialAmount = 10000_0_000_000;

const yellow = "\x1b[33m%s\x1b[0m";
const green = "\x1b[32m%s\x1b[0m";
const purple = "\x1b[35m%s\x1b[0m";
const red = "\x1b[31m%s\x1b[0m";

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
        name: "Hodl Strategy",
        address: addressBook.getContractId("hodl_strategy"),
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

function extractAddresses(params: CreateVaultParams[]): string[] {
  return params.map(param => param.address.toString());
}

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



async function testVaultTwoStrategies() {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Running two strategies vault tests");
  console.log(yellow, "---------------------------------------");
  // deploy vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Deploying vault with two strategies");
  console.log(purple, "---------------------------------------");
  const {
    address:vaultAddress,
    instructions:deployInstructions,
    readBytes:deployReadBytes,
    writeBytes:deployWriteBytes
  } = await deployVault(
    addressBook,
    twoStrategyParams,
    "TestVault",
    "TSTV"
  );

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idleFundsBeforeDeposit = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoStrategyParams),
    testUser
  );
  const investedFundsBeforeDeposit = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const hodlBalanceBeforeDeposit = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vaultAddress,
    testUser
  );
  const fixedBalanceBeforeDeposit = await checkUserBalance(
    addressBook.getContractId("fixed_apr_strategy"),
    vaultAddress,
    testUser
  );

  // deposit to vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Depositing to vault");
  console.log(purple, "---------------------------------------");

  const {
    instructions:depositInstructions,
    readBytes:depositReadBytes,
    writeBytes:depositWriteBytes
  } = await depositToVault(vaultAddress, [987654321], testUser);

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idleFundsAfterDeposit = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoStrategyParams),
    testUser
  );
  const investedFundsAfterDeposit = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const hodlBalanceAfterDeposit = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vaultAddress,
    testUser
  );
  const fixedBalanceAfterDeposit = await checkUserBalance(
    addressBook.getContractId("fixed_apr_strategy"),
    vaultAddress,
    testUser
  );

  // withdraw from vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Withdrawing from vault");
  console.log(purple, "---------------------------------------");

  const {
    instructions:withdrawInstructions,
    readBytes:withdrawReadBytes,
    writeBytes:withdrawWriteBytes
  } = await withdrawFromVault(vaultAddress, 65_0_000, testUser);

  const idleFundsAfterWithdraw = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoStrategyParams),
    testUser
  );
  const investedFundsAfterWithdraw = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const hodlBalanceAfterWithdraw = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vaultAddress,
    testUser
  );
  const fixedBalanceAfterWithdraw = await checkUserBalance(
    addressBook.getContractId("fixed_apr_strategy"),
    vaultAddress,
    testUser
  );

  // invest in vault
  console.log(purple, "---------------------------------------");
  console.log(purple, "Investing in vault");
  console.log(purple, "---------------------------------------");

  const investArgs: Instruction[] = [
    {
      type: "Invest",
      strategy: addressBook.getContractId("hodl_strategy"),
      amount: BigInt(1500),
    },
    {
      type: "Invest",
      strategy: addressBook.getContractId("fixed_apr_strategy"),
      amount: BigInt(2000),
    },
  ];
  
  const {
    instructions:investInstructions,
    readBytes:investReadBytes,
    writeBytes:investWriteBytes
  } = await rebalanceVault(vaultAddress, investArgs, manager);
  
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idleFundsAfterInvest = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoStrategyParams),
    testUser
  );
  const investedFundsAfterInvest = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const hodlBalanceAfterInvest = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vaultAddress,
    testUser
  );
  const fixedBalanceAfterInvest = await checkUserBalance(
    addressBook.getContractId("fixed_apr_strategy"),
    vaultAddress,
    testUser
  );

  // rebalance vault
  console.log(purple, "---------------------------------------");
  console.log(purple, "Rebalancing vault");
  console.log(purple, "---------------------------------------");

  const rebalanceArgs: Instruction[] = [
      {
          type: "Invest",
          strategy: addressBook.getContractId("hodl_strategy"),
          amount: BigInt(7_000_000),
      },
      {
          type: "Unwind",
          strategy: addressBook.getContractId("hodl_strategy"),
          amount: BigInt(6_000_00),
      },
      {
          type: "Invest",
          strategy: addressBook.getContractId("fixed_apr_strategy"),
          amount: BigInt(8_000_000),
      },
      {
          type: "Unwind",
          strategy: addressBook.getContractId("fixed_apr_strategy"),
          amount: BigInt(3_000_00),
      },
  ];


  const {
    instructions:rebalanceInstructions,
    readBytes:rebalanceReadBytes,
    writeBytes:rebalanceWriteBytes
  } = await rebalanceVault(vaultAddress, rebalanceArgs, manager);


  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");
  const idleFundsAfterRebalance = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoStrategyParams),
    testUser
  );
  const investedFundsAfterRebalance = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const hodlBalanceAfterRebalance = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vaultAddress,
    testUser
  );
  const fixedBalanceAfterRebalance = await checkUserBalance(
    addressBook.getContractId("fixed_apr_strategy"),
    vaultAddress,
    testUser
  );

  console.log(green, "---------------------------------------");
  console.log(green, "Tests completed successfully");
  console.log(green, "---------------------------------------");
  const tableData = {
    hodlStrategy: {
      "Balance before deposit": hodlBalanceBeforeDeposit,
      "Balance after deposit": hodlBalanceAfterDeposit,
      "Balance after withdraw": hodlBalanceAfterWithdraw,
      "Balance after invest": hodlBalanceAfterInvest,
      "Balance after rebalance": hodlBalanceAfterRebalance,
    },
    fixedStrategy: {
      "Balance before deposit": fixedBalanceBeforeDeposit,
      "Balance after deposit": fixedBalanceAfterDeposit,
      "Balance after withdraw": fixedBalanceAfterWithdraw,
      "Balance after invest": fixedBalanceAfterInvest,
      "Balance after rebalance": fixedBalanceAfterRebalance,
    },
    "Invested funds": {
      "Balance before deposit": investedFundsBeforeDeposit[0].amount,
      "Balance after deposit": investedFundsAfterDeposit[0].amount,
      "Balance after withdraw": investedFundsAfterWithdraw[0].amount,
      "Balance after invest": investedFundsAfterInvest[0].amount,
      "Balance after rebalance": investedFundsAfterRebalance[0].amount,
    },
    "Idle funds": {
      "Balance before deposit": idleFundsBeforeDeposit[0].amount,
      "Balance after deposit": idleFundsAfterDeposit[0].amount,
      "Balance after withdraw": idleFundsAfterWithdraw[0].amount,
      "Balance after invest": idleFundsAfterInvest[0].amount,
      "Balance after rebalance": idleFundsAfterRebalance[0].amount,
    },
  };

  const budgetData = {  
    deploy: {
      instructions: deployInstructions,
      readBytes: deployReadBytes,
      writeBytes: deployWriteBytes,
    },
    deposit: {
      instructions: depositInstructions,
      readBytes: depositReadBytes,
      writeBytes: depositWriteBytes,
    },
    withdraw: {
      instructions: withdrawInstructions,
      readBytes: withdrawReadBytes,
      writeBytes: withdrawWriteBytes,
    },
    invest: {
      instructions: investInstructions,
      readBytes: investReadBytes,
      writeBytes: investWriteBytes,
    },
    rebalance: {
      instructions: rebalanceInstructions,
      readBytes: rebalanceReadBytes,
      writeBytes: rebalanceWriteBytes,
    },
  }

  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}

async function testVaultTwoAssetsOneStrategy() {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Running two strategies vault tests");
  console.log(yellow, "---------------------------------------");
  // deploy vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Deploying vault with two strategies");
  console.log(purple, "---------------------------------------");
  const {
    address:vaultAddress,
    instructions:deployInstructions,
    readBytes:deployReadBytes,
    writeBytes:deployWriteBytes
  } = await deployVault(
    addressBook,
    twoAssetOneStrategyParams,
    "TestVault",
    "TSTV"
  );

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idleFundsBeforeDeposit = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoAssetOneStrategyParams),
    testUser
  );
  const investedFundsBeforeDeposit = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const fixedUsdcBalanceBeforeDeposit = await checkUserBalance(
    addressBook.getContractId("fixed_usdc_strategy"),
    vaultAddress,
    testUser
  );
  const fixedXtarBalanceBeforeDeposit = await checkUserBalance(
    addressBook.getContractId("fixed_xtar_strategy"),
    vaultAddress,
    testUser
  );

  // deposit to vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Depositing to vault");
  console.log(purple, "---------------------------------------");

  const {
    instructions:depositInstructions,
    readBytes:depositReadBytes,
    writeBytes:depositWriteBytes
  } = await depositToVault(vaultAddress, [98_7_654_321, 98_7_654_321], testUser);

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idleFundsAfterDeposit = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoAssetOneStrategyParams),
    testUser
  );
  const investedFundsAfterDeposit = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const fixedUsdcBalanceAfterDeposit = await checkUserBalance(
    addressBook.getContractId("fixed_usdc_strategy"),
    vaultAddress,
    testUser
  );
  const fixedXtarBalanceAfterDeposit = await checkUserBalance(
    addressBook.getContractId("fixed_xtar_strategy"),
    vaultAddress,
    testUser
  );

  // withdraw from vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Withdrawing from vault");
  console.log(purple, "---------------------------------------");

  const {
    instructions:withdrawInstructions,
    readBytes:withdrawReadBytes,
    writeBytes:withdrawWriteBytes
  } = await withdrawFromVault(vaultAddress, 7_0_000_000, testUser);

  const idleFundsAfterWithdraw = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoAssetOneStrategyParams),
    testUser
  );
  const investedFundsAfterWithdraw = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const fixedUsdcBalanceAfterWithdraw = await checkUserBalance(
    addressBook.getContractId("fixed_usdc_strategy"),
    vaultAddress,
    testUser
  );
  const fixedXtarBalanceAfterWithdraw = await checkUserBalance(
    addressBook.getContractId("fixed_xtar_strategy"),
    vaultAddress,
    testUser
  );

  // invest in vault
  console.log(purple, "---------------------------------------");
  console.log(purple, "Investing in vault");
  console.log(purple, "---------------------------------------");

  const investArgs: Instruction[] = [
    {
      type: "Invest",
      strategy: addressBook.getContractId("fixed_xtar_strategy"),
      amount: BigInt(10_000_000),
    },
    {
      type: "Invest",
      strategy: addressBook.getContractId("fixed_usdc_strategy"),
      amount: BigInt(10_000_000),
    },
  ];

  const {
    instructions:investInstructions,
    readBytes:investReadBytes,
    writeBytes:investWriteBytes
  } = await rebalanceVault(vaultAddress, investArgs, manager);

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idleFundsAfterInvest = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoAssetOneStrategyParams),
    testUser
  );
  const investedFundsAfterInvest = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const fixedUsdcBalanceAfterInvest = await checkUserBalance(
    addressBook.getContractId("fixed_usdc_strategy"),
    vaultAddress,
    testUser
  );
  const fixedXtarBalanceAfterInvest = await checkUserBalance(
    addressBook.getContractId("fixed_xtar_strategy"),
    vaultAddress,
    testUser
  );

  // rebalance vault
  console.log(purple, "---------------------------------------");
  console.log(purple, "Rebalancing vault");
  console.log(purple, "---------------------------------------");

  const rebalanceArgs: Instruction[] = [
    {
        type: "SwapExactIn",
        token_in: xtarAddress.toString(),
        token_out: usdcAddress.toString(),
        amount_in: BigInt(2_0_000_000),
        amount_out_min: BigInt(1_0_000_000),
        deadline: BigInt(Math.floor(Date.now() / 1000) + 3600),
    },
    {
      type: "SwapExactOut",
      token_in: usdcAddress.toString(),
      token_out: xtarAddress.toString(),
      amount_out: BigInt(1_000_000),
      amount_in_max: BigInt(1_000_000),
      deadline: BigInt(Math.floor(Date.now() / 1000) + 3600),
    }
  ];

  const {
      instructions:rebalanceInstructions,
      readBytes:rebalanceReadBytes,
      writeBytes:rebalanceWriteBytes
  } = await rebalanceVault(vaultAddress, rebalanceArgs, manager);
    

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");
  const idleFundsAfterRebalance = await fetchParsedCurrentIdleFunds(
    vaultAddress,
    extractAddresses(twoAssetOneStrategyParams),
    testUser
  );
  const investedFundsAfterRebalance = await fetchCurrentInvestedFunds(
    vaultAddress,
    testUser
  );
  const fixedUsdcBalanceAfterRebalance = await checkUserBalance(
    addressBook.getContractId("fixed_usdc_strategy"),
    vaultAddress,
    testUser
  );
  const fixedXtarBalanceAfterRebalance = await checkUserBalance(
    addressBook.getContractId("fixed_xtar_strategy"),
    vaultAddress,
    testUser
  );

  console.log(green, "---------------------------------------");
  console.log(green, "Tests completed successfully");
  console.log(green, "---------------------------------------");
  const tableData = {
    fixedUsdcStrategy: {
      "Balance before deposit": fixedUsdcBalanceBeforeDeposit,
      "Balance after deposit": fixedUsdcBalanceAfterDeposit,
      "Balance after withdraw": fixedUsdcBalanceAfterWithdraw,
      "Balance after invest": fixedUsdcBalanceAfterInvest,
      "Balance after rebalance": fixedUsdcBalanceAfterRebalance,
    },
    fixedXtarStrategy: {
      "Balance before deposit": fixedXtarBalanceBeforeDeposit,
      "Balance after deposit": fixedXtarBalanceAfterDeposit,
      "Balance after withdraw": fixedXtarBalanceAfterWithdraw,
      "Balance after invest": fixedXtarBalanceAfterInvest,
      "Balance after rebalance": fixedXtarBalanceAfterRebalance,
    },
    "Invested funds": {
      "Balance before deposit": investedFundsBeforeDeposit[0].amount,
      "Balance after deposit": investedFundsAfterDeposit[0].amount,
      "Balance after withdraw": investedFundsAfterWithdraw[0].amount,
      "Balance after invest": investedFundsAfterInvest[0].amount,
      "Balance after rebalance": investedFundsAfterRebalance[0].amount,
    },
    "Idle funds": {
      "Balance before deposit": idleFundsBeforeDeposit[0].amount,
      "Balance after deposit": idleFundsAfterDeposit[0].amount,
      "Balance after withdraw": idleFundsAfterWithdraw[0].amount,
      "Balance after invest": idleFundsAfterInvest[0].amount,
      "Balance after rebalance": idleFundsAfterRebalance[0].amount,
    },
  };
  const budgetData = {
    deploy: {
      instructions: deployInstructions,
      readBytes: deployReadBytes,
      writeBytes: deployWriteBytes,
    },
    deposit: {
      instructions: depositInstructions,
      readBytes: depositReadBytes,
      writeBytes: depositWriteBytes,
    },
    withdraw: {
      instructions: withdrawInstructions,
      readBytes: withdrawReadBytes,
      writeBytes: withdrawWriteBytes,
    },
    invest: {
      instructions: investInstructions,
      readBytes: investReadBytes,
      writeBytes: investWriteBytes,
    },
    rebalance: {
      instructions: rebalanceInstructions,
      readBytes: rebalanceReadBytes,
      writeBytes: rebalanceWriteBytes,
    },
  }
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
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
       -os one strategy tests,
       -ts two strategy tests, 
       -taos two assets one strategy test,
       -bs blend strategy tests,
       -bv blend vault tests`       
    );
    exit(0);
  case "-a":
    console.log(yellow, "Running all tests");
    try {
      await prepareEnvironment();
      const oneStrategy = await testVaultOneAssetOneStrategy(addressBook, oneStrategyParams, testUser);
      const twoStrategies = await testVaultTwoStrategies();
      const twoAssetsOneStrategy = await testVaultTwoAssetsOneStrategy();
      const blendStrategy = await testBlendStrategy();
      const blendVault = await testBlendVault();
      console.log(yellow, "----------------------------------------------------------------------------------------------------------------------------------------------")
      console.log(yellow, "All tests finished, results:");
      console.log(yellow, "----------------------------------------------------------------------------------------------------------------------------------------------")
      console.log("")
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------")
      console.log(green, "One strategy results")
      console.table(oneStrategy.tableData);
      console.table(oneStrategy.budgetData);
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log("");
      console.log(green, "----------------------------------------------------------------------------------------------------------------------------------------------");
      console.log(green, "Two strategies results");
      console.table(twoStrategies.tableData);
      console.table(twoStrategies.budgetData);
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
  case "-os":
    console.log(yellow, "Testing one strategy vault");
    try {
      await prepareEnvironment();
      await testVaultOneAssetOneStrategy(addressBook, oneStrategyParams, testUser);
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-ts":
    console.log(yellow, "Testing two strategies vault");
    try {
      await prepareEnvironment();
      await testVaultTwoStrategies();
      exit(0);
    } catch (error) {
      console.log(red, "Tests failed:", error);
      exit(1);
    }
  case "-taos":
    console.log(yellow, "Testing two assets one strategy vault");
    try {
      await prepareEnvironment();
      await testVaultTwoAssetsOneStrategy();
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
