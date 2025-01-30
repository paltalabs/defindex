import { Address, Asset, Keypair, nativeToScVal, Networks, scValToNative, xdr } from "@stellar/stellar-sdk";
import { SOROSWAP_ROUTER } from "../../constants.js";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount, invokeContract, invokeCustomContract } from "../../utils/contract.js";
import { config } from "../../utils/env_config.js";
import { AssetInvestmentAllocation, depositToVault, getCreateDeFindexParams, Instruction, rebalanceManager, rebalanceVault } from "../vault.js";
import { getTransactionBudget } from "../../utils/tx.js";
import { yellow } from "../common.js";

const network = process.argv[2];
const loadedConfig = config(network);
const addressBook = AddressBook.loadFromFile(network);

const purple = '\x1b[35m%s\x1b[0m';
const green = '\x1b[32m%s\x1b[0m';

interface TotalManagedFunds {
  asset: string,
  idle_amount: bigint,
  invested_amount: bigint,
  strategy_allocations: any[],
  total_amount: bigint
}

interface mappedFunds {
  asset: string,
  total_amount: bigint,
  idle_amount: bigint,
  invested_amount: bigint
}

export async function fetchCurrentFunds(
  deployedVault: string,
  user: Keypair
): Promise<mappedFunds[]> {
  try {
    const res = await invokeCustomContract(
      deployedVault,
      "fetch_total_managed_funds",
      [],
      user,
      true,
    );
    const funds = scValToNative(res.result.retval);
    const mappedFunds = Object.entries(funds).map(([key, value]) => {
      const fund = value as TotalManagedFunds;
      return {
        asset: key,
        total_amount: fund.total_amount,
        idle_amount: fund.idle_amount,
        invested_amount: fund.invested_amount,
      };
    });
    return mappedFunds;
  } catch (error) {
    console.error("Error:", error);
    throw error;
  }
}


export async function testBlendVault() {
  const newUser = Keypair.random();
  if (network !== "mainnet") {
    console.log(green, '----------------------- New account created -------------------------')
    console.log(green, 'Public key: ',newUser.publicKey())
    console.log(green, '---------------------------------------------------------------------')  
    console.log(purple, '-------------------------------------------------------------------')
    console.log(purple, '----------------------- Funding new account -----------------------')
    console.log(purple, '-------------------------------------------------------------------')
    await airdropAccount(newUser);
  }
  const userAccount = network === "mainnet" ? loadedConfig.admin : newUser;
  console.log("Setting Emergengy Manager, Fee Receiver and Manager accounts");
  const emergencyManager = loadedConfig.getUser("DEFINDEX_EMERGENCY_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(emergencyManager);

  const feeReceiver = loadedConfig.getUser("DEFINDEX_FEE_RECEIVER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(feeReceiver);

  const manager = loadedConfig.getUser("DEFINDEX_MANAGER_SECRET_KEY");
  if (network !== "mainnet") await airdropAccount(manager);

  const blendStrategyAddress = addressBook.getContractId("blend_strategy");

  const xlm = Asset.native();
  let xlmContractId: string;
  switch (network) {
    case "testnet":
      xlmContractId = xlm.contractId(Networks.TESTNET);
      break;
    case "mainnet":
      xlmContractId = xlm.contractId(Networks.PUBLIC);
      break;
    default:
      console.log("Invalid network:", network, "It should be either testnet or mainnet");
      return;
  }


  const initialAmount = 1_0_000_000;
  let blendVaultAddress: string = addressBook.getContractId("blend_vault");
  
  let deposit_status: boolean;
  let deposit_balances: any;
  let deposit_instructions: number = 0;
  let deposit_read_bytes: number = 0;
  let deposit_write_bytes: number = 0;
  
  let invest_status: boolean;
  let invest_balances: any;
  let invest_instructions: number = 0;
  let invest_read_bytes: number = 0;
  let invest_write_bytes: number = 0;
  


  // Deposit assets to the vault
  try {    
    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '----------------------- Depositing XLM to the vault -----------------------')
    console.log(purple, '---------------------------------------------------------------------------')
    const { balanceBefore: depositBalanceBefore, result: depositResult, balanceAfter: depositBalanceAfter } 
      = await depositToVault(blendVaultAddress, [initialAmount], userAccount, false);
    const {
      instructions,
      readBytes,
      writeBytes
    } = getTransactionBudget(depositResult);

    deposit_instructions = instructions;
    deposit_read_bytes = readBytes;
    deposit_write_bytes = writeBytes;

    console.log(green, '------------ XLM deposited to the vault ------------')
    console.log(green, 'Deposit balance before: ', depositBalanceBefore)
    console.log(green, 'depositResult', depositResult)
    console.log(green, 'Deposit balance after: ', depositBalanceAfter)
    console.log(green, '----------------------------------------------------')

    deposit_status = true;
  } catch (error) {
    deposit_status = false;
    console.log('❌ Error depositing into the vault:', error);
  }
  const deposit_total_managed_funds: mappedFunds[] = await fetchCurrentFunds(blendVaultAddress, userAccount);

  // Invest in strategy
  try {
    console.log(purple, '---------------------------------------------------------------------------')
    console.log(purple, '-------------------------- Investing in strategy --------------------------')
    console.log(purple, '---------------------------------------------------------------------------')

    const investArgs: Instruction[] = [
      {
        type: "Invest",
        strategy: blendStrategyAddress,
        amount: BigInt(5_000_000),
      },
    ];
    
    const {
      result:investResult,
      instructions,
      readBytes,
      writeBytes
    } = await rebalanceVault(blendVaultAddress, investArgs, manager);
    console.log('🚀 « investResult:', investResult);

    invest_instructions = instructions;
    invest_read_bytes = readBytes;
    invest_write_bytes = writeBytes;
    console.log(green, '---------------------- Invested in strategy ----------------------')
    console.log(green, 'Invested: ', investResult, ' in the strategy')
    console.log(green, '------------------------------------------------------------------')
    invest_status = true;
  } catch (error) {
    console.log('❌ Error Investing the Vault:', error);
    invest_status = false;
  }
  const invest_total_managed_funds: mappedFunds[] = await fetchCurrentFunds(blendVaultAddress, userAccount);

  const status_result={
    status:{
      "deposit status": deposit_status ? '✅ Success' : '❌ Failed', 
      "invest status": invest_status ? '✅ Success' : '❌ Failed' 
    },
  }
  const balances_result ={
    deposit: {
      idle_amount: deposit_total_managed_funds[0].idle_amount,
      invested_amount: deposit_total_managed_funds[0].invested_amount,
      total_amount: deposit_total_managed_funds[0].total_amount,
    },
    invest: {
      idle_amount: invest_total_managed_funds[0].idle_amount,
      invested_amount: invest_total_managed_funds[0].invested_amount,
      total_amount: invest_total_managed_funds[0].total_amount
    }
  }

  const budget_result={
    deposit: {
      instructions: deposit_instructions,
      readBytes: deposit_read_bytes,
      writeBytes: deposit_write_bytes
    },
    invest: {
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes
    }
  }
  console.table(status_result);
  console.table(balances_result);
  console.table(budget_result);
  return {status_result, budget_result};
}

await testBlendVault();