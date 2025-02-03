import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import {
  depositToVault,
  Instruction,
  manager,
  pauseStrategy,
  rebalanceVault,
  rescueFromStrategy,
  unpauseStrategy,
  withdrawFromVault
} from "../vault.js";
import { green, purple, red, yellow } from "../common.js";
import { airdropAccount } from "../../utils/contract.js";
import { deployDefindexVault, fetchBalances } from "./utils.js";
import { testAccessControl } from "./access_control.js";
import { testUpgradeContract } from "./upgrade_contract.js";
import { CreateVaultParams } from "../types.js";

/* 
// One asset one strategy success flow:
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
export async function oneAssetOneStrategySuccess(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Testing one strategy vault");
  console.log(yellow, "---------------------------------------");

  //Deploy vault
  const { 
    address:vault_address, 
    deploy_instructions, 
    deploy_read_bytes,
    deploy_write_bytes 

  } = await deployDefindexVault(addressBook, params);
  if (!vault_address) throw new Error("Vault was not deployed");

  const { 
    idle_funds:idle_funds_before_deposit, 
    invested_funds:invested_funds_before_deposit, 
    hodl_balance:hodl_balance_before_deposit 
  } = await fetchBalances(addressBook, vault_address, params, user);

  // Deposit to vault
  const deposit_amount = 10_0_000_000;
  const {
    instructions:deposit_instructions, 
    readBytes:deposit_read_bytes, 
    writeBytes:deposit_write_bytes 
  } = await (
    async () => {
      console.log(purple, "-----------------------------------------");
      console.log(purple, `Deposit ${deposit_amount} in one strategy`);
      console.log(purple, "-----------------------------------------");
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
  } = await fetchBalances(addressBook, vault_address, params, user);

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
        } = await fetchBalances(addressBook, vault_address, params, user);
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
        } = await fetchBalances(addressBook, vault_address, params, user);

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
        } = await fetchBalances(addressBook, vault_address, params, user);

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
        } = await fetchBalances(addressBook, vault_address, params, user);

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
        } = await fetchBalances(addressBook, vault_address, params, user);

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
        const { idle_funds, invested_funds, hodl_balance } = await fetchBalances(addressBook, vault_address, params, user);

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
        } = await fetchBalances(addressBook, vault_address, params, user);
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
        } = await fetchBalances(addressBook, vault_address, params, user);
        
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
    "After deposit": {
      "Idle funds": idle_funds_after_deposit[0].amount,
      "Invested funds": invested_funds_after_deposit[0].amount,
      "Hodl strategy": hodl_balance_after_deposit,
    },
    "After invest": {
      "Idle funds": idle_funds_after_invest[0].amount,
      "Invested funds": invested_funds_after_invest[0].amount,
      "Hodl strategy": hodl_balance_after_invest,
    },
    "After deposit and invest": {
      "Idle funds": idle_funds_after_deposit_and_invest[0].amount,
      "Invested funds": invested_funds_after_deposit_and_invest[0].amount,
      "Hodl strategy": hodl_balance_after_deposit_and_invest,
    },
    "After unwind": {
      "Idle funds": idle_funds_after_unwind[0].amount,
      "Invested funds": invested_funds_after_unwind[0].amount,
      "Hodl strategy": hodl_balance_after_unwind,
    },
    "After rebalance": {
      "Idle funds": idle_funds_after_rebalance[0].amount,
      "Invested funds": invested_funds_after_rebalance[0].amount,
      "Hodl strategy": hodl_balance_after_rebalance,
    },
    "After withdraw": {
      "Idle funds": idle_funds_after_withdraw[0].amount,
      "Invested funds": invested_funds_after_withdraw[0].amount,
      "Hodl strategy": hodl_balance_after_withdraw,
    },
    "After rescue": {
      "Idle funds": idle_funds_after_rescue[0].amount,
      "Invested funds": invested_funds_after_rescue[0].amount,
      "Hodl strategy": hodl_balance_after_rescue,
    }
  };
  const budgetData = {
    deploy: {
      status: !!deploy_instructions && !!deploy_read_bytes && !!deploy_write_bytes ? "success" : "failed",
      instructions: deploy_instructions,
      readBytes: deploy_read_bytes,
      writeBytes: deploy_write_bytes,
    },
    deposit: {
      status: !!deposit_instructions && !!deposit_read_bytes && !!deposit_write_bytes ? "success" : "failed",
      instructions: deposit_instructions,
      readBytes: deposit_read_bytes,
      writeBytes: deposit_write_bytes,
    },
    invest: {
      status: !!invest_instructions && !!invest_read_bytes && !!invest_write_bytes ? "success" : "failed",
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    deposit_and_invest: {
      status: !!deposit_and_invest_instructions && !!deposit_and_invest_read_bytes && !!deposit_and_invest_write_bytes ? "success" : "failed",
      instructions: deposit_and_invest_instructions,
      readBytes: deposit_and_invest_read_bytes,
      writeBytes: deposit_and_invest_write_bytes,
    },
    unwind: {
      status: !!unwind_instructions && !!unwind_read_bytes && !!unwind_write_bytes ? "success" : "failed",
      instructions: unwind_instructions,
      readBytes: unwind_read_bytes,
      writeBytes: unwind_write_bytes,
    },
    rebalance: {
      status: !!rebalance_instructions && !!rebalance_read_bytes && !!rebalance_write_bytes ? "success" : "failed",
      instructions: rebalance_instructions,
      readBytes: rebalance_read_bytes,
      writeBytes: rebalance_write_bytes,
    },
    withdraw: {
      status: !!withdraw_instructions && !!withdraw_read_bytes && !!withdraw_write_bytes ? "success" : "failed",
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
    rescue: {
      status: !!rescue_instructions && !!rescue_read_bytes && !!rescue_write_bytes ? "success" : "failed",
      instructions: rescue_instructions,
      readBytes: rescue_read_bytes,
      writeBytes: rescue_write_bytes,
    },
    unpause_strategy: {
      status: !!unpause_strategy_instructions && !!unpause_strategy_read_bytes && !!unpause_strategy_write_bytes ? "success" : "failed",
      instructions: unpause_strategy_instructions,
      readBytes: unpause_strategy_read_bytes,
      writeBytes: unpause_strategy_write_bytes,
    },
    pause_strategy: {
      status: !!pause_strategy_instructions && !!pause_strategy_read_bytes && !!pause_strategy_write_bytes ? "success" : "failed",
      instructions: pause_strategy_instructions,
      readBytes: pause_strategy_read_bytes,
      writeBytes: pause_strategy_write_bytes,
    },
  }
  return {tableData, budgetData};
}



export async function testVaultOneAssetOneStrategy(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  const {tableData: userFlowTable, budgetData: userFlowBudgetData} = await oneAssetOneStrategySuccess(addressBook, params, user);
  const {budgetData: accessControlBudgetData} = await testAccessControl(addressBook, params, user);
  const {budgetData: upgradeBudgetData} = await testUpgradeContract(addressBook, params);

  const tableData:any  = {...userFlowTable,};
  const budgetData:any = { ...userFlowBudgetData, ...accessControlBudgetData,  ...upgradeBudgetData};

  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}