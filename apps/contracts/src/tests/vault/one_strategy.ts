import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { checkUserBalance } from "../strategy.js";
import {
  admin,
  CreateVaultParams,
  deployVault,
  depositToVault,
  fetchCurrentInvestedFunds,
  fetchParsedCurrentIdleFunds,
  Instruction,
  manager,
  pauseStrategy,
  queueVaultManager,
  rebalanceVault,
  rescueFromStrategy,
  setEmergencyManager,
  setFeeReceiver,
  setRebalanceManager,
  setVaultManager,
  unpauseStrategy,
  upgradeVaultWasm,
  withdrawFromVault
} from "../vault.js";
import { green, purple, red, yellow } from "../common.js";
import { airdropAccount, installContract } from "../../utils/contract.js";

async function fetchBalances(addressBook: AddressBook, vault_address: string, user: Keypair) {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idle_funds = await fetchParsedCurrentIdleFunds(
    vault_address,
    user
  );
  const invested_funds = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  return {idle_funds, invested_funds, hodl_balance};
}
async function deployOneStrategyVault(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  console.log(purple, "---------------------------------------");
  console.log(purple, "Deploying vault with one strategy");
  console.log(purple, "---------------------------------------");
  try {
    const { 
      address: vault_address, 
      instructions:deploy_instructions, 
      readBytes:deploy_read_bytes, 
      writeBytes:deploy_write_bytes
    } = await deployVault(
      addressBook,
      params,
      "TestVault",
      "TSTV"
    );
    console.log(vault_address);
    return {address: vault_address, deploy_instructions, deploy_read_bytes, deploy_write_bytes};
  } catch (error:any) {
    console.error(red, error);
    return {
      address: null, 
      deploy_instructions:0, 
      deploy_read_bytes: 0, 
      deploy_write_bytes: 0,
      error
    };
  }
}

/* 
// Success flow:
  - [x] deposit
  - [x] check balance

  - [x] try invest more than idle
  - [x] invest
  - [x] check balance

  - [x] deposit and invest
  - [x] check balance

  - [x] try unwind more than invested
  - [x] try unwind from unauthorized
  - [x] unwind
  - [x] check balance

  - [x] try rebalance from unauthorized
  - [x] rebalance
  - [x] check balance

  - [x] try withdraw from unauthorized
  - [x] try withdraw more than total funds
  - [x] withdraw
  - [x] check balance

  - [x] unauthorized rescue
  - [x] rescue
  
  - [x] try unpause from unauthorized
  - [x] try unpause non existent strategy
  - [x] unpause strategy
  - [x] pause strategy
*/
export async function successFlow(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Testing one strategy vault tests");
  console.log(yellow, "---------------------------------------");

  //Deploy vault
  const { 
    address:vault_address, 
    deploy_instructions, 
    deploy_read_bytes,
    deploy_write_bytes 

  } = await deployOneStrategyVault(addressBook, params, user);
  if (!vault_address) throw new Error("Vault was not deployed");

  const { 
    idle_funds:idle_funds_before_deposit, 
    invested_funds:invested_funds_before_deposit, 
    hodl_balance:hodl_balance_before_deposit 
  } = await fetchBalances(addressBook, vault_address, user);

  // Deposit to vault
  const deposit_amount = 10_0_000_000;
  const {
    instructions:deposit_instructions, 
    readBytes:deposit_read_bytes, 
    writeBytes:deposit_write_bytes 
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple, `Deposit ${deposit_amount} in one strategy`);
      console.log(purple, "---------------------------------------");
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_amount], user);
        return { instructions, readBytes, writeBytes };
      } catch (e) {
        console.error(red, e);
        return {
          deposit_instructions: 0,
          deposit_read_bytes: 0,
          deposit_write_bytes: 0,
          error: e,
        };
      }
    }
  )();
  const {
    idle_funds:idle_funds_after_deposit,
    invested_funds:invested_funds_after_deposit,
    hodl_balance:hodl_balance_after_deposit
  } = await fetchBalances(addressBook, vault_address, user);

  //Invest
  const invest_amount = 5_0_000_000;
  const { 
    instructions: invest_instructions, 
    readBytes:invest_read_bytes, 
    writeBytes:invest_write_bytes,
    idle_funds_after_invest,
    invested_funds_after_invest,
    hodl_balance_after_invest
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Try Invest idle_funds*2");
        console.log(purple, "---------------------------------------");
        const investAmount = parseInt(idle_funds_after_deposit[0].amount.toString()) * 2;
        const investArgs: Instruction[] = [
          {
            type: "Invest",
            strategy: addressBook.getContractId("hodl_strategy"),
            amount: BigInt(investAmount),
          },
        ];
        await rebalanceVault(
            vault_address,
            investArgs,
            manager
          );
      } catch (error:any) {
        if(error.toString().includes("HostError: Error(Contract, #10)")) {
          console.log(green, "-----------------------------------------------------");
          console.log(green, "| Investing more than idle funds failed as expected |");
          console.log(green, "-----------------------------------------------------");
          //To-do: return status
        } else {
          throw Error(error);
        }
      };
      
      console.log(purple, "---------------------------------------");
      console.log(purple, "Investing in vault");
      console.log(purple, "---------------------------------------");
      const investArgs: Instruction[] = [
        {
          type: "Invest",
          strategy: addressBook.getContractId("hodl_strategy"),
          amount: BigInt(invest_amount),
        },
      ];
      try {
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          investArgs,
          manager
        );
        const { 
          idle_funds:idle_funds_after_invest, 
          invested_funds:invested_funds_after_invest, 
          hodl_balance:hodl_balance_after_invest 
        } = await fetchBalances(addressBook, vault_address, user);
        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          idle_funds_after_invest, 
          invested_funds_after_invest, 
          hodl_balance_after_invest 
        };
        //To-do: return status
      } catch (e) {
        console.error(red, e);
        return {
          result: null,
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_invest:[{ amount: BigInt(0) }], 
          invested_funds_after_invest:[{ amount: BigInt(0) }], 
          hodl_balance_after_invest:0,
          error: e,
        };
      }
    }
  )();

  // Deposit and invest
  const {
    instructions: deposit_and_invest_instructions,
    readBytes:deposit_and_invest_read_bytes,
    writeBytes:deposit_and_invest_write_bytes,
    idle_funds_after_deposit_and_invest,
    invested_funds_after_deposit_and_invest,
    hodl_balance_after_deposit_and_invest
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Deposit and invest in vault");
      console.log(purple, "---------------------------------------");
      const deposit_and_invest_amount: number = 10_0_000_000;
      
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_and_invest_amount], user, true);
        const {
          idle_funds:idle_funds_after_deposit_and_invest, 
          invested_funds:invested_funds_after_deposit_and_invest, 
          hodl_balance:hodl_balance_after_deposit_and_invest
        } = await fetchBalances(addressBook, vault_address, user);

        const expected_idle_funds = (BigInt(deposit_and_invest_amount) - invested_funds_after_invest[0].amount);
        const expected_invested_funds = BigInt(deposit_and_invest_amount) + invested_funds_after_invest[0].amount;
        const expected_hodl_balance: number = deposit_and_invest_amount + parseInt(hodl_balance_after_invest.toString());

        if(idle_funds_after_deposit_and_invest[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds_after_deposit_and_invest[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after deposit and invest  failed");
        }

        if (invested_funds_after_deposit_and_invest[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds_after_deposit_and_invest[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after deposit and invest  failed");
        }

        if (parseInt(hodl_balance_after_deposit_and_invest.toString()) !== expected_hodl_balance) {
          console.error(red, `hodl balance: ${hodl_balance_after_deposit_and_invest} !== ${expected_hodl_balance}`);
          throw Error("Hodl balance after deposit and invest failed");
        }
        return { 
          instructions, 
          readBytes, 
          writeBytes,
          idle_funds_after_deposit_and_invest,
          invested_funds_after_deposit_and_invest,
          hodl_balance_after_deposit_and_invest
        };
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_deposit_and_invest: [{ amount: BigInt(0) }],
          invested_funds_after_deposit_and_invest: [{ amount: BigInt(0) }],
          hodl_balance_after_deposit_and_invest: 0,
          error: e,
        };
      }
    }
  )();

  // Unwind
  const {
    instructions: unwind_instructions,
    readBytes:unwind_read_bytes,
    writeBytes:unwind_write_bytes,
    idle_funds_after_unwind,
    invested_funds_after_unwind,
    hodl_balance_after_unwind
  } = await (
    async () =>{
      try {

        // Unwind more than invested
        try { 
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try Unwind more than invested");
          console.log(purple, "---------------------------------------");
          const unwind_amount = 100_0_000_000;
          const unwind_args: Instruction[] = [
            {
              type: "Unwind",
              strategy: addressBook.getContractId("hodl_strategy"),
              amount: BigInt(unwind_amount),
            },
          ];
          await rebalanceVault(
            vault_address,
            unwind_args,
            manager
          );
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #10)") || error.toString().includes("HostError: Error(Contract, #142)")) {
            console.log(green, "---------------------------------------------------------");
            console.log(green, "| Unwinding more than invested funds failed as expected |");
            console.log(green, "---------------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }

        // Unwind from unauthorized
        try { 
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try Unwind from unauthorized");
          console.log(purple, "---------------------------------------");
          const unwind_amount = 5_0_000_000;
          const unwind_args: Instruction[] = [
            {
              type: "Unwind",
              strategy: addressBook.getContractId("hodl_strategy"),
              amount: BigInt(unwind_amount),
            },
          ];
          await rebalanceVault(
            vault_address,
            unwind_args,
            user
          );
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "---------------------------------------------------------");
            console.log(green, "| Unwinding more than invested funds failed as expected |");
            console.log(green, "---------------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }

        console.log(purple, "---------------------------------------");
        console.log(purple, "Unwind");
        console.log(purple, "---------------------------------------");
        const unwind_amount = 5_0_000_000;
        const unwind_args: Instruction[] = [
          {
            type: "Unwind",
            strategy: addressBook.getContractId("hodl_strategy"),
            amount: BigInt(unwind_amount),
          },
        ];
        const { 
          instructions, 
          readBytes, 
          writeBytes
        } = await rebalanceVault(
          vault_address,
          unwind_args,
          manager
        );
        const { 
          idle_funds:idle_funds_after_unwind, 
          invested_funds:invested_funds_after_unwind, 
          hodl_balance:hodl_balance_after_unwind
        } = await fetchBalances(addressBook, vault_address, user);

        let expected_idle_funds = BigInt(idle_funds_after_deposit_and_invest[0].amount) + BigInt(unwind_amount);
        let expected_invested_funds = BigInt(invested_funds_after_deposit_and_invest[0].amount) - BigInt(unwind_amount);
        let expected_hodl_balance = parseInt(hodl_balance_after_deposit_and_invest.toString()) - unwind_amount;

        if(idle_funds_after_unwind[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds_after_unwind[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after unwind failed");
        }

        if(invested_funds_after_unwind[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds_after_unwind[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after unwind failed");
        }

        if(parseInt(hodl_balance_after_unwind.toString()) !== expected_hodl_balance) {
          console.error(red, `hodl balance: ${hodl_balance_after_unwind} !== ${expected_hodl_balance}`);
          throw Error("Hodl balance after unwind failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds_after_unwind,
          invested_funds_after_unwind,
          hodl_balance_after_unwind
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_unwind: [{ amount: BigInt(0) }],
          invested_funds_after_unwind: [{ amount: BigInt(0) }],
          hodl_balance_after_unwind: 0,
          error: error
        }
      };
    }
  )();

  // Rebalance vault
  const { 
    instructions: rebalance_instructions, 
    readBytes:rebalance_read_bytes, 
    writeBytes:rebalance_write_bytes,
    idle_funds: idle_funds_after_rebalance,
    invested_funds: invested_funds_after_rebalance,
    hodl_balance: hodl_balance_after_rebalance
  } = await (
    async () => {
      try {
        // Rebalance from unauthorized
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try rebalance from unauthorized"); 
          console.log(purple, "---------------------------------------");
          const rebalanceArgs: Instruction[] = [
            {
              type: "Invest",
              strategy: addressBook.getContractId("hodl_strategy"),
              amount: BigInt(7_0_000),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("hodl_strategy"),
              amount: BigInt(6_0_00),
            },
          ];
          await rebalanceVault(
            vault_address,
            rebalanceArgs,
            user
          );        
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "----------------------------------------------------");
            console.log(green, "| Rebalancing from unauthorized failed as expected |");
            console.log(green, "----------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rebalancing vault"); 
        console.log(purple, "---------------------------------------");
        console.log(yellow, "idle funds:", idle_funds_after_unwind[0].amount);
        const invest_amount = 7_0_000_000;
        const unwind_amount = 3_0_000_000;
        const rebalanceArgs: Instruction[] = [
          {
            type: "Invest",
            strategy: addressBook.getContractId("hodl_strategy"),
            amount: BigInt(invest_amount),
          },
          {
            type: "Unwind",
            strategy: addressBook.getContractId("hodl_strategy"),
            amount: BigInt(unwind_amount),
          },
        ];
     
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );

        const {
          idle_funds, 
          invested_funds, 
          hodl_balance
        } = await fetchBalances(addressBook, vault_address, user);

        const expected_idle_funds = idle_funds_after_unwind[0].amount - BigInt(invest_amount) + BigInt(unwind_amount);
        const expected_invested_funds = invested_funds_after_unwind[0].amount + BigInt(invest_amount) - BigInt(unwind_amount);
        const expected_hodl_balance = parseInt(hodl_balance_after_unwind.toString()) + invest_amount - unwind_amount;

        if(idle_funds[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after rebalance failed");
        }

        if (invested_funds[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after rebalance failed");
        }

        if (parseInt(hodl_balance.toString()) !== expected_hodl_balance) {
          console.error(red, `hodl balance: ${hodl_balance} !== ${expected_hodl_balance}`);
          throw Error("Hodl balance after rebalance failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
          hodl_balance
        }
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          error: e,
          idle_funds: [{ amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }],
          hodl_balance: 0
        };
      }
    }
  )();

  // withdraw from vault
  const { 
    instructions: withdraw_instructions, 
    readBytes: withdraw_read_bytes, 
    writeBytes: withdraw_write_bytes,
    idle_funds: idle_funds_after_withdraw,
    invested_funds: invested_funds_after_withdraw,
    hodl_balance: hodl_balance_after_withdraw
  } = await (
    async () => {
      let withdraw_amount = 2_0_000_000;
      console.log(purple, "---------------------------------------");
      console.log(purple,`Withdraw ${withdraw_amount} from one strategy`);
      console.log(purple, "---------------------------------------");
      try {
        //Try withdraw from unauthorized
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try withdraw from unauthorized");
          console.log(purple, "---------------------------------------");
          const withdraw_amount = 65_0_000;
          const random_user = Keypair.random();
          await airdropAccount(random_user);

          await withdrawFromVault(vault_address, withdraw_amount, random_user);
          
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #111)") || error.toString().includes("HostError: Error(Contract, #10)")) {
            console.log(green, "-------------------------------------------------");
            console.log(green, "| Withdraw from unauthorized failed as expected |");
            console.log(green, "-------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }
        //Try withdraw more than total funds
        try {
          console.log(purple, "-----------------------------------------------------");
          console.log(purple, "Try withdraw more than total funds");
          console.log(purple, "-----------------------------------------------------");

          await withdrawFromVault(vault_address, 100_0_000_000, user);

        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #124)") || error.toString().includes("HostError: Error(Contract, #10)")) {
            console.log(green, "-----------------------------------------------------");
            console.log(green, "| Withdraw more than total funds failed as expected |");
            console.log(green, "-----------------------------------------------------");
            //To-do: return status
          }
        }
        //Withdraw
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, withdraw_amount, user);

        const { 
          idle_funds, 
          invested_funds, 
          hodl_balance 
        } = await fetchBalances(addressBook, vault_address, user);

        const expected_idle_funds = idle_funds_after_rebalance[0].amount - BigInt(withdraw_amount);
        const expected_invested_funds = invested_funds_after_rebalance[0].amount;
        const expected_hodl_balance = parseInt(hodl_balance_after_rebalance.toString());

        if(idle_funds[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after withdraw failed");
        }

        if(invested_funds[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after withdraw failed");
        }

        if(parseInt(hodl_balance.toString()) !== expected_hodl_balance) {
          console.error(red, `hodl balance: ${hodl_balance} !== ${expected_hodl_balance}`);
          throw Error("Hodl balance after withdraw failed");
        }

        return { instructions, readBytes, writeBytes, idle_funds, invested_funds, hodl_balance };
      } catch (e) {
        console.error(red, e);
        return {
          withdraw_instructions: 0,
          withdraw_read_bytes: 0,
          withdraw_write_bytes: 0,
          idle_funds: [{ amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }],
          hodl_balance: 0,
          error: e,
        };
      }
    }
  )();

  //rescue funds
  const {
    instructions: rescue_instructions,
    readBytes: rescue_read_bytes,
    writeBytes: rescue_write_bytes,
    idle_funds: idle_funds_after_rescue,
    invested_funds: invested_funds_after_rescue,
    hodl_balance: hodl_balance_after_rescue
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rescue funds");
        console.log(purple, "---------------------------------------");
        // Unauthorized rescue
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try rescue from unauthorized");
          console.log(purple, "---------------------------------------");
          await rescueFromStrategy(vault_address, addressBook.getContractId("hodl_strategy"), user);
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "-----------------------------------------------");
            console.log(green, "| Rescue from unauthorized failed as expected |");
            console.log(green, "-----------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }
        // Non existing strategy rescue
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try rescue from random address");
          console.log(purple, "---------------------------------------");
          const random_address = Keypair.random();
          await airdropAccount(random_address);
          await rescueFromStrategy(vault_address, random_address.publicKey(), manager);
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #140)")) {
            console.log(green, "-------------------------------------------------");
            console.log(green, "| Rescue from random address failed as expected |");
            console.log(green, "-------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }
        // Rescue
        const { instructions, readBytes, writeBytes } = await rescueFromStrategy(vault_address, addressBook.getContractId("hodl_strategy"), manager);
        const { idle_funds, invested_funds, hodl_balance } = await fetchBalances(addressBook, vault_address, user);

        const expected_idle_funds = idle_funds_after_withdraw[0].amount + invested_funds_after_withdraw[0].amount;
        const expected_invested_funds = BigInt(0);
        const expected_hodl_balance = 0;

        if(idle_funds[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after rescue failed");
        }

        if(invested_funds[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after rescue failed");
        }

        if(parseInt(hodl_balance.toString()) !== expected_hodl_balance) {
          console.error(red, `hodl balance: ${hodl_balance} !== ${expected_hodl_balance}`);
          throw Error("Hodl balance after rescue failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
          hodl_balance,
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds: [{ amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }],
          hodl_balance: 0,
          error: error,
      }
    }
  }   
  )();

  // Unpause strategy
  const {
    instructions: unpause_strategy_instructions,
    readBytes: unpause_strategy_read_bytes,
    writeBytes: unpause_strategy_write_bytes,
    idle_funds: idle_funds_after_unpause_strategy,
    invested_funds: invested_funds_after_unpause_strategy,
    hodl_balance: hodl_balance_after_unpause_strategy
  } = await (
    async () => {
      try {
        //try unpause from unauthorized
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try unpause from unauthorized");
          console.log(purple, "---------------------------------------");
          await unpauseStrategy(vault_address, addressBook.getContractId("hodl_strategy"), user);
        } catch (error:any) {
          if(error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "--------------------------------------------------");
            console.log(green, "| Unpausing from unauthorized failed as expected |");
            console.log(green, "--------------------------------------------------");
            //To-do: return status
          } else {
            throw Error(error);
          }
        }
        //unpause non existent strategy
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Unpause non existent strategy");
          console.log(purple, "---------------------------------------");
          const random_address = Keypair.random();
          await airdropAccount(random_address);
          await unpauseStrategy(vault_address, random_address.publicKey(), manager);
        } catch (error:any) {
          if(error.toString().includes("HostError: Error(Contract, #140)")) {
            console.log(green, "------------------------------------------------------");
            console.log(green, "| Unpausing non existent strategy failed as expected |");
            console.log(green, "------------------------------------------------------");
            //To-do: return status
          } else {
            throw Error(error);
          }
        }
        //unpause strategy
        const { instructions, readBytes, writeBytes } = await unpauseStrategy(vault_address, addressBook.getContractId("hodl_strategy"), manager);

        await depositToVault(vault_address, [10_0_000_000], user);
        const {
          idle_funds,
          invested_funds,
          hodl_balance
        } = await fetchBalances(addressBook, vault_address, user);
        const expected_idle_funds = idle_funds_after_rescue[0].amount + BigInt(10_0_000_000);
        const expected_invested_funds = invested_funds_after_rescue[0].amount;
        const expected_hodl_balance = parseInt(hodl_balance_after_rescue.toString());

        if (idle_funds[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after unpause failed");
        }

        if (invested_funds[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after unpause failed");
        }

        if (parseInt(hodl_balance.toString()) !== expected_hodl_balance) {
          console.error(red, `hodl balance: ${hodl_balance} !== ${expected_hodl_balance}`);
          throw Error("Hodl balance after unpause failed");
        }
        
        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds: idle_funds_after_rescue[0].amount,
          invested_funds: invested_funds_after_rescue[0].amount,
          hodl_balance: hodl_balance_after_rescue,
        }
      } catch (error) {
        console.error(red, error);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds: [{ amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }],
          hodl_balance: 0,
          error: error,
        }
      }
    }
  )();

  //Pause strategy
  const {
    instructions: pause_strategy_instructions,
    readBytes: pause_strategy_read_bytes,
    writeBytes: pause_strategy_write_bytes,
    idle_funds: idle_funds_after_pause_strategy,
    invested_funds: invested_funds_after_pause_strategy,
    hodl_balance: hodl_balance_after_pause_strategy
  } = await (
    async () => {
      try {
        // try pause from unauthorized
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try pause from unauthorized");
          console.log(purple, "---------------------------------------");
          await pauseStrategy(vault_address, addressBook.getContractId("hodl_strategy"), user);
        } catch (error:any) {
          if(error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "--------------------------------------------------");
            console.log(green, "| Pausing from unauthorized failed as expected |");
            console.log(green, "--------------------------------------------------");
            //To-do: return status
          } else {
            throw Error(error);
          }
        }
        // try pause non existent strategy
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Pause non existent strategy");
          console.log(purple, "---------------------------------------");
          const random_address = Keypair.random();
          await airdropAccount(random_address);
          await pauseStrategy(vault_address, random_address.publicKey(), manager);
        } catch (error:any) {
          if(error.toString().includes("HostError: Error(Contract, #140)")) {
            console.log(green, "------------------------------------------------------");
            console.log(green, "| Pausing non existent strategy failed as expected |");
            console.log(green, "------------------------------------------------------");
            //To-do: return status
          } else {
            throw Error(error);
          }
        }
        // pause strategy
        const { instructions, readBytes, writeBytes } = await pauseStrategy(vault_address, addressBook.getContractId("hodl_strategy"), manager);

        try {
          const invest_instructions: Instruction[] = [
            {
              type: "Invest",
              strategy: addressBook.getContractId("hodl_strategy"),
              amount: BigInt(1_000_000),
            },
          ];
          await rebalanceVault(vault_address, invest_instructions, manager);
        } catch (error:any) {
          if(error.toString().includes("HostError: Error(Contract, #144)")) {
            console.log(green, "----------------------------------------------");
            console.log(green, "| Investing in paused strategy failed as expected |");
            console.log(green, "----------------------------------------------");
            //To-do: return status
          } else {
            throw Error(error);
          }
        }

        const {
          idle_funds, 
          invested_funds, 
          hodl_balance
        } = await fetchBalances(addressBook, vault_address, user);
        
        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
          hodl_balance,
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds: [{ amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }],
          hodl_balance: 0,
          error: error,
        }
      }
    }
  )();

  //Show data
  const tableData = {
    "Initial balance": {
      "Idle funds": idle_funds_before_deposit[0].amount,
      "Invested funds": invested_funds_before_deposit[0].amount,
      hodlStrategy: hodl_balance_before_deposit,
    },
    "Balance after deposit": {
      "Idle funds": idle_funds_after_deposit[0].amount,
      "Invested funds": invested_funds_after_deposit[0].amount,
      hodlStrategy: hodl_balance_after_deposit,
    },
    "Balance after invest": {
      "Idle funds": idle_funds_after_invest[0].amount,
      "Invested funds": invested_funds_after_invest[0].amount,
      hodlStrategy: hodl_balance_after_invest,
    },
    "Balance after deposit and invest": {
      "Idle funds": idle_funds_after_deposit_and_invest[0].amount,
      "Invested funds": invested_funds_after_deposit_and_invest[0].amount,
      hodlStrategy: hodl_balance_after_deposit_and_invest,
    },
    "Balance after unwind": {
      "Idle funds": idle_funds_after_unwind[0].amount,
      "Invested funds": invested_funds_after_unwind[0].amount,
      hodlStrategy: hodl_balance_after_unwind,
    },
    "Balance after rebalance": {
      "Idle funds": idle_funds_after_rebalance[0].amount,
      "Invested funds": invested_funds_after_rebalance[0].amount,
      hodlStrategy: hodl_balance_after_rebalance,
    },
    "Balance after withdraw": {
      "Idle funds": idle_funds_after_withdraw[0].amount,
      "Invested funds": invested_funds_after_withdraw[0].amount,
      hodlStrategy: hodl_balance_after_withdraw,
    },
    "Balance after rescue": {
      "Idle funds": idle_funds_after_rescue[0].amount,
      "Invested funds": invested_funds_after_rescue[0].amount,
      hodlStrategy: hodl_balance_after_rescue,
    }
  };
  const budgetData = {
    deploy: {
      instructions: deploy_instructions,
      readBytes: deploy_read_bytes,
      writeBytes: deploy_write_bytes,
    },
    deposit: {
      instructions: deposit_instructions,
      readBytes: deposit_read_bytes,
      writeBytes: deposit_write_bytes,
    },
    invest: {
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    deposit_and_invest: {
      instructions: deposit_and_invest_instructions,
      readBytes: deposit_and_invest_read_bytes,
      writeBytes: deposit_and_invest_write_bytes,
    },
    unwind: {
      instructions: unwind_instructions,
      readBytes: unwind_read_bytes,
      writeBytes: unwind_write_bytes,
    },
    rebalance: {
      instructions: rebalance_instructions,
      readBytes: rebalance_read_bytes,
      writeBytes: rebalance_write_bytes,
    },
    withdraw: {
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
    rescue: {
      instructions: rescue_instructions,
      readBytes: rescue_read_bytes,
      writeBytes: rescue_write_bytes,
    },
    unpause_strategy: {
      instructions: unpause_strategy_instructions,
      readBytes: unpause_strategy_read_bytes,
      writeBytes: unpause_strategy_write_bytes,
    },
    pause_strategy: {
      instructions: pause_strategy_instructions,
      readBytes: pause_strategy_read_bytes,
      writeBytes: pause_strategy_write_bytes,
    },
  }
  return {tableData, budgetData};
}

/* 
// Access control tests:
  - [x] try setManager without previous queued manager
  - [x] try queueManager from unauthorized
  - [x] queueManager
  - [x] try setManager before time is up
  - [ ] try setManager from unauthorized
  - [ ] setManager

  - [x] try setRebalanceManager from unauthorized
  - [x] setRebalanceManager

  - [x] try setFeeReceiver from unauthorized
  - [x] setFeeReceiver

  - [x] try setEmergencyManager from unauthorized
  - [x] setEmergencyManager
*/
async function testAccessControl(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  //Deploy vault
  const { 
    address:vault_address, 
  } = await deployOneStrategyVault(addressBook, params, user);
  if (!vault_address) throw new Error("Vault was not deployed");

  //Try set manager without previous queued manager
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Set manager");
    console.log(purple, "---------------------------------------");
    const { instructions, readBytes, writeBytes } = await setVaultManager(vault_address, user);
    return { instructions, readBytes, writeBytes };
  } catch (error: any ) {
    if (error.toString().includes("HostError: Error(Contract, #134)")) {
      console.log(green, "------------------------------------------------------------------");
      console.log(green, "| Set manager without previous queued manager failed as expected |");
      console.log(green, "------------------------------------------------------------------");
    } else {
      throw Error(error);
    }
  }

  //Try queue manager from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try queue manager from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    const  {result} = await queueVaultManager(vault_address, random_user, random_user);
    if( result !== false){
      throw Error("Queue manager from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "------------------------------------------------------");
      console.log(green, "| Queue manager from unauthorized failed as expected |");
      console.log(green, "------------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  const new_manager = Keypair.random();
  await airdropAccount(new_manager);
  // Queue manager
  const {
    result,
    instructions: queue_instructions,
    readBytes: queue_read_bytes,
    writeBytes: queue_write_bytes
  } = await (async ()=>{
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Queue manager");
        console.log(purple, "---------------------------------------");
        const  {result, instructions, readBytes, writeBytes } = await queueVaultManager(vault_address, manager, new_manager);
        if( result !== true){
          console.log(green, "-----------------------------------");
          console.log(green, "| Queue manager set successufully |");
          console.log(green, "-----------------------------------");
        } else if (result === false) {
          console.error(red, "Queue manager from unauthorized validation failed");
        }
        return {result, instructions, readBytes, writeBytes};
      } catch (error: any) {
        console.error(red, error);
        return {result: false, instructions: 0, readBytes: 0, writeBytes: 0};
      }
    }
  )();

  //Try setManager before time is up
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Set manager before time");
    console.log(purple, "---------------------------------------");
    const { instructions, readBytes, writeBytes } = await setVaultManager(vault_address, user);
    return { instructions, readBytes, writeBytes };
  } catch (error: any ) {
    if (error.toString().includes("HostError: Error(Contract, #133)")) {
      console.log(green, "----------------------------------------------");
      console.log(green, "| Set manager before time failed as expected |");
      console.log(green, "----------------------------------------------");
    } else {
      throw Error(error);
    }
  } 

  // Try setRebalanceManager from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try setRebalanceManager from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    const {result}  = await setRebalanceManager(vault_address, random_user, user.publicKey());
    if( result !== false){
      throw Error("Set rebalance manager from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "--------------------------------------------------------------");
      console.log(green, "| Set rebalance manager from unauthorized failed as expected |");
      console.log(green, "--------------------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  // setRebalanceManager success
  const {
    instructions: set_rebalance_manager_instructions, 
    readBytes: set_rebalance_manager_read_bytes, 
    writeBytes: set_rebalance_manager_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "setRebalanceManager");
      console.log(purple, "---------------------------------------");
      const random_user = Keypair.random();
      await airdropAccount(random_user);
      const {result, instructions, readBytes, writeBytes}  = await setRebalanceManager(vault_address, manager, random_user.publicKey());
      if( result === null){
        console.log(green, "--------------------------------------");
        console.log(green, "| Rebalance manager set sucessfully  |");
        console.log(green, "--------------------------------------");
      } else if (result === false) {
        throw Error("Set rebalance manager failed");
      }
      return {result, instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {result: false, instructions: 0, readBytes: 0, writeBytes: 0};
    } 
  } )();

  // Try set fee reciever from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try set fee receiver from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    await setFeeReceiver(vault_address, random_user, user.publicKey());

  } catch (error: any) {
    if( error.toString().includes("HostError: Error(Contract, #130)")) {
      console.log(green, "----------------------------------------------------------");
      console.log(green, "| Set fee receiver from unauthorized failed as expected |");
      console.log(green, "----------------------------------------------------------");
    } else {     
      throw Error(error);
    }
  } 

  // setFeeReceiver success
  const {
    instructions: set_fee_receiver_instructions,
    readBytes: set_fee_receiver_read_bytes,
    writeBytes: set_fee_receiver_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "setFeeReceiver");
      console.log(purple, "---------------------------------------");
      const {instructions, readBytes, writeBytes} = await setFeeReceiver(vault_address, manager, user.publicKey());
      console.log(green, "---------------------------------");
      console.log(green, "| Fee receiver set sucessfully  |");
      console.log(green, "---------------------------------");
      return {instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {instructions: 0, readBytes: 0, writeBytes: 0};
    } 
  } )();
 

  // Try set emergency manager from unauthorized
  try {
    console.log(purple, "-------------------------------------------");
    console.log(purple, "Try set emergency manager from unauthorized");
    console.log(purple, "-------------------------------------------");
    const random_user = Keypair.random();
    await airdropAccount(random_user);
    const {result} = await setEmergencyManager(vault_address, random_user, user.publicKey());
    if( result !== false){
      throw Error("Set emergency manager from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "--------------------------------------------------------------");
      console.log(green, "| Set emergency manager from unauthorized failed as expected |");
      console.log(green, "--------------------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  // setEmergencyManager success
  const {
    instructions: set_emergency_manager_instructions,
    readBytes: set_emergency_manager_read_bytes,
    writeBytes: set_emergency_manager_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "setEmergencyManager");
      console.log(purple, "---------------------------------------");
      const random_user = Keypair.random();
      await airdropAccount(random_user);
      const {result, instructions, readBytes, writeBytes} = await setEmergencyManager(vault_address, manager, random_user.publicKey());
      if( result === null){
        console.log(green, "--------------------------------------");
        console.log(green, "| Emergency manager set sucessfully  |");
        console.log(green, "--------------------------------------");
      } else if (result === false) {
        throw Error("Set emergency manager failed");
      }
      return {result, instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {result: false, instructions: 0, readBytes: 0, writeBytes: 0};
    }
  } )();
  const budgetData = {
    queue: {
      instructions: queue_instructions,
      readBytes: queue_read_bytes,
      writeBytes: queue_write_bytes,
    },
    set_rebalance_manager: {
      instructions: set_rebalance_manager_instructions,
      readBytes: set_rebalance_manager_read_bytes,
      writeBytes: set_rebalance_manager_write_bytes,
    },
    set_fee_receiver: {
      instructions: set_fee_receiver_instructions,
      readBytes: set_fee_receiver_read_bytes,
      writeBytes: set_fee_receiver_write_bytes,
    },
    set_emergency_manager: {
      instructions: set_emergency_manager_instructions,
      readBytes: set_emergency_manager_read_bytes,
      writeBytes: set_emergency_manager_write_bytes,
    }
  }
  return {
    budgetData
  };
};


/* 
// Upgrade tests:
  - [x] try upgrade from unauthorized
  - [x] upgrade
*/
export async function testUpgradeContract(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  try {
    console.log(yellow, "----------------");
    console.log(yellow, "| updating Wasm |");
    console.log(yellow, "----------------");
    await installContract("defindex_vault", addressBook, admin);
    console.log(green, "----------------");
    console.log(green, "| Wasm updated |");
    console.log(green, "----------------");
  } catch (error: any) {
    throw Error(error);
  }
  
  // Deploy vault
  const { 
    address:vault_address, 
  } = await deployOneStrategyVault(addressBook, params, user);

  if (!vault_address) throw new Error("Vault was not deployed");
  //Try upgrade from unauthorized
  try {
    console.log(purple, "---------------------------------------");
    console.log(purple, "Try upgrade from unauthorized");
    console.log(purple, "---------------------------------------");
    const random_user = Keypair.random();
    const wasm_hash = Buffer.from(addressBook.getWasmHash("defindex_vault"), "hex");
    await airdropAccount(random_user);
    const {result} = await upgradeVaultWasm(vault_address, random_user, wasm_hash);
    if( result !== false){
      throw Error("Upgrade from unauthorized validation failed");
    } else if (result === false) {
      console.log(green, "------------------------------------------------");
      console.log(green, "| Upgrade from unauthorized failed as expected |");
      console.log(green, "------------------------------------------------");
    }
  } catch (error: any) {
    throw Error(error);
  }

  // upgrade success
  const {
    instructions: upgrade_instructions,
    readBytes: upgrade_read_bytes,
    writeBytes: upgrade_write_bytes
  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Upgrade");
      console.log(purple, "---------------------------------------");
      const wasm_hash = Buffer.from(addressBook.getWasmHash("defindex_vault"), "hex");
      const {instructions, readBytes, writeBytes} = await upgradeVaultWasm(vault_address, manager, wasm_hash);
      console.log(green, "------------------------");
      console.log(green, "| Upgrade sucessfully  |");
      console.log(green, "------------------------");
      return {instructions, readBytes, writeBytes};
    } catch (error: any) {
      console.error(red, error);
      return {instructions: 0, readBytes: 0, writeBytes: 0};
    } 
  } )();
  const budgetData = {
    upgrade: {
      instructions: upgrade_instructions,
      readBytes: upgrade_read_bytes,
      writeBytes: upgrade_write_bytes,
    }
  }
  return { budgetData };

}

export async function testVaultOneStrategy(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  const {tableData: userFlowTable, budgetData: userFlowBudgetData} = await successFlow(addressBook, params, user);
  const {budgetData: accessControlBudgetData} = await testAccessControl(addressBook, params, user);
  const {budgetData: upgradeBudgetData} = await testUpgradeContract(addressBook, params, user);

  const tableData:any  = {...userFlowTable,};
  const budgetData:any = { ...userFlowBudgetData, ...accessControlBudgetData,  ...upgradeBudgetData};
  
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}