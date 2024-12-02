import {
  Address,
  Asset,
  Keypair,
  nativeToScVal,
  scValToNative,
  xdr
} from "@stellar/stellar-sdk";
import { randomBytes } from "crypto";
import { AddressBook } from "../utils/address_book.js";
import { airdropAccount, invokeContract, invokeCustomContract } from "../utils/contract.js";
import { config } from "../utils/env_config.js";
import { ActionType, AssetInvestmentAllocation, depositToVault, getVaultBalanceInStrategy, Instruction, investVault, rebalanceVault, fetchParsedCurrentIdleFunds, fetchCurrentInvestedFunds } from "./vault.js";
import { checkUserBalance } from "./strategy.js";

const soroswapUSDC = new Address("CAAFIHB4I7WQMJMKC22CZVQNNX7EONWSOMT6SUXK6I3G3F6J4XFRWNDI");
  
export async function deployVaultTwoStrategies(addressBook: AddressBook) {
  if (network !== "mainnet") await airdropAccount(loadedConfig.admin);
  let account = await loadedConfig.horizonRpc.loadAccount(
    loadedConfig.admin.publicKey()
  );
  console.log("publicKey", loadedConfig.admin.publicKey());
  let balance = account.balances.filter((item) => item.asset_type === "native");
  console.log("Current Admin account balance:", balance[0].balance);

  console.log("-------------------------------------------------------");
  console.log("Create Two Strategies Vault on Factory");
  console.log("-------------------------------------------------------");

  console.log("Setting Emergengy Manager, Fee Receiver and Manager accounts");
  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  const assets = [
    {
      address: soroswapUSDC,
      strategies: [
        {
          name: "Hodl Strategy",
          address: addressBook.getContractId("hodl_strategy"),
          paused: false
        },
        {
          name: "Fixed Strategy",
          address: addressBook.getContractId("fixed_apr_strategy"),
          paused: false
        }
      ]
    }
  ];

  const assetAllocations = assets.map((asset) => {
    return xdr.ScVal.scvMap([
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("address"),
        val: asset.address.toScVal(),
      }),
      new xdr.ScMapEntry({
        key: xdr.ScVal.scvSymbol("strategies"),
        val: xdr.ScVal.scvVec(
          asset.strategies.map((strategy) =>
            xdr.ScVal.scvMap([
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("address"),
                val: new Address(strategy.address).toScVal(),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("name"),
                val: nativeToScVal(strategy.name, { type: "string" }),
              }),
              new xdr.ScMapEntry({
                key: xdr.ScVal.scvSymbol("paused"),
                val: nativeToScVal(false, { type: "bool" }),
              }),
            ])
          )
        ),
      }),
    ]);
  });

  const createDeFindexParams: xdr.ScVal[] = [
    new Address(emergencyManager.publicKey()).toScVal(),
    new Address(feeReceiver.publicKey()).toScVal(),
    nativeToScVal(100, { type: "u32" }), 
    nativeToScVal("HODL FIXED Vault", { type: "string" }),
    nativeToScVal("HDFXVLT", { type: "string" }),
    new Address(manager.publicKey()).toScVal(),
    xdr.ScVal.scvVec(assetAllocations),
    nativeToScVal(randomBytes(32)),
  ];

  const result = await invokeContract(
    'defindex_factory',
    addressBook,
    'create_defindex_vault',
    createDeFindexParams,
    loadedConfig.admin
  );

  console.log('🚀 « DeFindex Vault created with address:', scValToNative(result.returnValue));
  return scValToNative(result.returnValue);
}

const network = process.argv[2];
const addressBook = AddressBook.loadFromFile(network);
const hodl_strategy = addressBook.getContractId("hodl_strategy");
const fixed_apr_strategy = addressBook.getContractId("fixed_apr_strategy");
const xlm: Asset = Asset.native()

const loadedConfig = config(network);

// Step 0: Deploy the vault
const deployedVault = await deployVaultTwoStrategies(addressBook);
console.log(" -- ")
console.log(" -- ")
console.log("Step 0: Deployed Vault:", deployedVault);
console.log(" -- ")
console.log(" -- ")

const testUser = Keypair.random();
if (network !== "mainnet") await airdropAccount(testUser);
const initialAmount = 10000_0_000_000;

const mintToken = async () => {
  await invokeCustomContract(
    soroswapUSDC.toString(),
    "mint",
    [new Address(testUser.publicKey()).toScVal(), nativeToScVal(initialAmount, { type: "i128" })],
    loadedConfig.getUser("SOROSWAP_MINT_SECRET_KEY")
  )
}

await mintToken();
// Step 1: Deposit to vault and capture initial balances
const { user, balanceBefore: depositBalanceBefore, result: depositResult, balanceAfter: depositBalanceAfter, status: depositStatus } 
  = await depositToVault(deployedVault, [initialAmount], testUser);
console.log(" -- ")
console.log(" -- ")
console.log("Step 1: Deposited to Vault using user:", user.publicKey(), "with balance before:", depositBalanceBefore, "and balance after:", depositBalanceAfter);
console.log(" -- ")
console.log(" -- ")


const idleFundsAfterDeposit = await fetchParsedCurrentIdleFunds(deployedVault, user);
const investedFundsAfterDeposit = await fetchCurrentInvestedFunds(deployedVault, user);

const hodlBalanceBeforeInvest = await checkUserBalance(hodl_strategy, deployedVault, user);
const fixedBalanceBeforeInvest = await checkUserBalance(fixed_apr_strategy, deployedVault, user);

// Step 2: Invest in vault idle funds
const investParams: AssetInvestmentAllocation[] = [
  {
    asset: soroswapUSDC,
    strategy_investments: [
      {
        amount: BigInt(1500_0_000_000),
        strategy: new Address(addressBook.getContractId("hodl_strategy"))
      },
      {
        amount: BigInt(2000_0_000_000),
        strategy: new Address(addressBook.getContractId("fixed_apr_strategy"))
      }
    ]
  }
];

const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
const investResult = await investVault(deployedVault, investParams, manager)
console.log('🚀 « investResult:', investResult);

const idleFundsAfterInvest = await fetchParsedCurrentIdleFunds(deployedVault, user);
const investedFundsAfterInvest = await fetchCurrentInvestedFunds(deployedVault, user);

const afterInvestHodlBalance = await checkUserBalance(hodl_strategy, deployedVault, user);
const afterInvestFixedBalance = await checkUserBalance(fixed_apr_strategy, deployedVault, user);

// 10000 USDC -> Total Balance
// 1500 USDC -> Hodl Strategy
// 2000 USDC -> Fixed Strategy
// 6500 USDC -> Idle

// Step 3: Rebalance Vault

const rebalanceParams: Instruction[] = [
  {
    action: ActionType.Withdraw,
    strategy: addressBook.getContractId("hodl_strategy"),
    amount: BigInt(500_0_000_000),
    swap_details_exact_in: undefined,
    swap_details_exact_out: undefined,
  },
  {
    action: ActionType.Invest,
    strategy: addressBook.getContractId("fixed_apr_strategy"),
    amount: BigInt(3500_0_000_000),
    swap_details_exact_in: undefined,
    swap_details_exact_out: undefined,
  }
];

console.log('🚀 « rebalanceParams:', rebalanceParams);

// this should leave us with:

// this should leave us with:
// 10000 USDC -> Total Balance
// 1000 USDC -> Hodl Strategy
// 5500 USDC -> Fixed Strategy
// 3500 USDC -> Idle

const rebalanceResult = await rebalanceVault(deployedVault, rebalanceParams, manager);

const idleFundsAfterRebalance = await fetchParsedCurrentIdleFunds(deployedVault, user);
const investedFundsAfterRebalance = await fetchCurrentInvestedFunds(deployedVault, user);

const afterRebalanceHodlBalance = await checkUserBalance(hodl_strategy, deployedVault, user);
const afterRebalanceFixedBalance = await checkUserBalance(fixed_apr_strategy, deployedVault, user);

console.table({
  hodlStrategy: {
    'Balance before invest': hodlBalanceBeforeInvest,
    'Balance after invest': afterInvestHodlBalance,
    'Balance after rebalance': afterRebalanceHodlBalance
  },
  fixedStrategy: {
    'Balance before invest': fixedBalanceBeforeInvest,
    'Balance after invest': afterInvestFixedBalance,
    'Balance after rebalance': afterRebalanceFixedBalance
  },
  'Invested funds': {
    'Balance before invest': investedFundsAfterDeposit[0].amount,
    'Balance after invest': investedFundsAfterInvest[0].amount,
    'Balance after rebalance': investedFundsAfterRebalance[0].amount
  },
  'Idle funds': {
    'Balance before invest': idleFundsAfterDeposit[0].amount,
    'Balance after invest': idleFundsAfterInvest[0].amount,
    'Balance after rebalance': idleFundsAfterRebalance[0].amount
  }
})

console.table({
  deposit: {
    'Status': depositStatus ? '🟢 Success' : '🔴 Failed',
  },
  invest: {
    'Status': investResult.status ? '🟢 Success' : '🔴 Failed',
  },
  rebalance: {
    'Status': rebalanceResult.status ? '🟢 Success' : '🔴 Failed',
  }
})