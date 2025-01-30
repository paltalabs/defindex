import { Address, Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import { green, purple, red, yellow } from "../common.js";
import {
  CreateVaultParams,
  depositToVault,
  fetchTotalManagedFunds,
  fetchTotalSupply,
  Instruction,
  manager,
  rebalanceVault,
  withdrawFromVault
} from "../vault.js";
import { deployDefindexVault, fetchBalances, underlyingToDfTokens } from "./utils.js";
/* 
### One asset one strategy tests:
- [x] deposit
- [x] try rebalance with invest and more than idle
- [x] invest
- [x] deposit and invest
- [x] try rebalance with unwind and more than invested
- [x] rebalance with unwind
- [x] rebalance with `[unwind, invest]`
- [x] withdraw more than idle
*/
export async function testVaultOneAssetTwoStrategies(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair, xlmAddress: Address) {
  console.log(yellow, "--------------------------------------");
  console.log(yellow, "Testing one asset two strategies vault");
  console.log(yellow, "--------------------------------------");

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
      console.log(purple, `Deposit ${deposit_amount} in one asset`);
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
  } = await fetchBalances(addressBook, vault_address, params, user);

  //Invest
  const invest_amount = 5_0_000_000;
  const { 
    instructions: invest_instructions, 
    readBytes:invest_read_bytes, 
    writeBytes:invest_write_bytes,
    idle_funds_after_invest,
    invested_funds_after_invest,
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
            strategy: addressBook.getContractId("fixed_apr_strategy"),
            amount: BigInt(invest_amount),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("blend_strategy"),
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
          strategy: addressBook.getContractId("fixed_apr_strategy"),
          amount: BigInt(invest_amount),
        },
        {
          type: "Invest",
          strategy: addressBook.getContractId("blend_strategy"),
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
        } = await fetchBalances(addressBook, vault_address, params, user);

        const tolerance = BigInt(1_000); // Define a tolerance for approximation
        const expected_invested_funds = BigInt(invest_amount * 2);
        if (
          invested_funds_after_invest[0].amount < expected_invested_funds - tolerance || 
          invested_funds_after_invest[0].amount > expected_invested_funds + tolerance) {
          console.error(red, `invested funds: ${invested_funds_after_invest[0].amount} !== approximately ${expected_invested_funds}`);
          throw Error("Invested funds after invest failed");
        }

        if(idle_funds_after_invest[0].amount !== BigInt(0)) {
          console.error(red, `idle funds: ${idle_funds_after_invest[0].amount} !== 0}`);
          throw Error("Idle funds after invest failed");
        }

        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          idle_funds_after_invest, 
          invested_funds_after_invest, 
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
        console.log(green, "Instructions", instructions);
        console.log(green, "Read Bytes", readBytes);
        console.log(green, "Write Bytes", writeBytes);
        const {
          idle_funds:idle_funds_after_deposit_and_invest, 
          invested_funds:invested_funds_after_deposit_and_invest, 
          hodl_balance:hodl_balance_after_deposit_and_invest
        } = await fetchBalances(addressBook, vault_address, params, user);

        const expected_idle_funds = BigInt(0);
        const expected_invested_funds = BigInt(deposit_and_invest_amount) + invested_funds_after_invest[0].amount;

        if(idle_funds_after_deposit_and_invest[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds_after_deposit_and_invest[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after deposit and invest  failed");
        }

        if (invested_funds_after_deposit_and_invest[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds_after_deposit_and_invest[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after deposit and invest  failed");
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
              strategy: addressBook.getContractId("blend_strategy"),
              amount: BigInt(unwind_amount),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_apr_strategy"),
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
              strategy: addressBook.getContractId("blend_strategy"),
              amount: BigInt(unwind_amount),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_apr_strategy"),
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
            strategy: addressBook.getContractId("blend_strategy"),
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
        } = await fetchBalances(addressBook, vault_address, params, user);

        const tolerance = BigInt(1_000);
        const expected_idle_funds = idle_funds_after_deposit_and_invest[0].amount + BigInt(unwind_amount);
        const expected_invested_funds = invested_funds_after_deposit_and_invest[0].amount - BigInt(unwind_amount);
        
        if (
          idle_funds_after_unwind[0].amount < expected_idle_funds - tolerance || 
          idle_funds_after_unwind[0].amount > expected_idle_funds + tolerance
        ) {
          console.error(red, `idle funds: ${idle_funds_after_unwind[0].amount} !== approximately ${expected_idle_funds}`);
          throw Error("Idle funds after unwind failed");
        }
        if(
          invested_funds_after_unwind[0].amount < expected_invested_funds - tolerance || 
          invested_funds_after_unwind[0].amount > expected_invested_funds + tolerance
        ) {
          console.error(red, `invested funds: ${invested_funds_after_unwind[0].amount} !== approximately ${expected_invested_funds}`);
          throw Error("Invested funds after unwind failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds_after_unwind,
          invested_funds_after_unwind,
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
  
  // Unwind and invest
  const {
    instructions: unwind_and_invest_instructions,
    readBytes:unwind_and_invest_read_bytes,
    writeBytes:unwind_and_invest_write_bytes,
    idle_funds_after_unwind_and_invest,
    invested_funds_after_unwind_and_invest,
  } = await (
    async () =>{
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Unwind and invest");
        console.log(purple, "---------------------------------------");
        const unwind_amount = 5_0_000_000;
        const unwind_args: Instruction[] = [
          {
            type: "Unwind",
            strategy: addressBook.getContractId("fixed_apr_strategy"),
            amount: BigInt(unwind_amount),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("blend_strategy"),
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
          idle_funds:idle_funds_after_unwind_and_invest, 
          invested_funds:invested_funds_after_unwind_and_invest, 
        } = await fetchBalances(addressBook, vault_address, params, user);
        
        const tolerance = BigInt(1_000);
        const expected_idle_funds = idle_funds_after_unwind[0].amount;
        const expected_invested_funds = invested_funds_after_unwind[0].amount;

        if (
          idle_funds_after_unwind_and_invest[0].amount < expected_idle_funds - tolerance || 
          idle_funds_after_unwind_and_invest[0].amount > expected_idle_funds + tolerance
        ) {
          console.error(red, `idle funds: ${idle_funds_after_unwind_and_invest[0].amount} !== approximately ${expected_idle_funds}`);
          throw Error("Idle funds after unwind and invest failed");
        }

        if(
          invested_funds_after_unwind_and_invest[0].amount < expected_invested_funds - tolerance || 
          invested_funds_after_unwind_and_invest[0].amount > expected_invested_funds + tolerance
        ) {
          console.error(red, `invested funds: ${invested_funds_after_unwind_and_invest[0].amount} !== approximately ${expected_invested_funds}`);
          throw Error("Invested funds after unwind and invest failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds_after_unwind_and_invest,
          invested_funds_after_unwind_and_invest,
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_unwind_and_invest: [{ amount: BigInt(0) }],
          invested_funds_after_unwind_and_invest: [{ amount: BigInt(0) }],
          error: error
        }
      };
    }
  )();

  

  // withdraw from vault
  const { 
    instructions: withdraw_instructions, 
    readBytes: withdraw_read_bytes, 
    writeBytes: withdraw_write_bytes,
    idle_funds: idle_funds_after_withdraw,
    invested_funds: invested_funds_after_withdraw,
  } = await (
    async () => {
      let withdraw_amount = idle_funds_after_unwind_and_invest[0].amount + BigInt(2_0_000_000);
      console.log(purple, "--------------------------------------------------------------");
      console.log(purple,`Withdraw ${withdraw_amount} from one asset two strategies vault`);
      console.log(purple, "--------------------------------------------------------------");
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
        //Withdraw more than idle funds
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, Number(withdraw_amount), user);

        const { 
          idle_funds, 
          invested_funds, 
        } = await fetchBalances(addressBook, vault_address, params, user);

        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const total_supply = await fetchTotalSupply(vault_address, user);

        const expected_idle_funds = underlyingToDfTokens(idle_funds[0].amount, total_supply, total_managed_funds);
        const expected_invested_funds = underlyingToDfTokens(invested_funds[0].amount, total_supply, total_managed_funds);

        if (
          idle_funds[0].amount !== expected_idle_funds 
        ) {
          console.error(red, `idle funds: ${idle_funds[0].amount} !== approximately ${expected_idle_funds}`);
          throw Error("Idle funds after withdraw failed");
        }

        if(
          invested_funds[0].amount !== expected_invested_funds
        ) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after withdraw failed");
        }

        return { instructions, readBytes, writeBytes, idle_funds, invested_funds };
      } catch (e) {
        console.error(red, e);
        return {
          withdraw_instructions: 0,
          withdraw_read_bytes: 0,
          withdraw_write_bytes: 0,
          idle_funds: [{ amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }],
          error: e,
        };
      }
    }
  )();


  //Show data
  const tableData = {
    "Initial balance": {
      "Idle funds": idle_funds_before_deposit[0].amount,
      "Invested funds": invested_funds_before_deposit[0].amount,
    },
    "After deposit": {
      "Idle funds": idle_funds_after_deposit[0].amount,
      "Invested funds": invested_funds_after_deposit[0].amount,
    },
    "After invest": {
      "Idle funds": idle_funds_after_invest[0].amount,
      "Invested funds": invested_funds_after_invest[0].amount,
    },
    "After deposit and invest": {
      "Idle funds": idle_funds_after_deposit_and_invest[0].amount,
      "Invested funds": invested_funds_after_deposit_and_invest[0].amount,
    },
    "After unwind": {
      "Idle funds": idle_funds_after_unwind[0].amount,
      "Invested funds": invested_funds_after_unwind[0].amount,
    },
    "After unwind and invest": {
      "Idle funds": idle_funds_after_unwind_and_invest[0].amount,
      "Invested funds": invested_funds_after_unwind_and_invest[0].amount,
    },
    "After withdraw": {
      "Idle funds": idle_funds_after_withdraw[0].amount,
      "Invested funds": invested_funds_after_withdraw[0].amount,
    },
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
    unwind_and_invest: {
      status: !!unwind_and_invest_instructions && !!unwind_and_invest_read_bytes && !!unwind_and_invest_write_bytes ? "success" : "failed",
      instructions: unwind_and_invest_instructions,
      readBytes: unwind_and_invest_read_bytes,
      writeBytes: unwind_and_invest_write_bytes,
    },
    withdraw: {
      status: !!withdraw_instructions && !!withdraw_read_bytes && !!withdraw_write_bytes ? "success" : "failed",
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
    
  }
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}
/* 
Passing:
  Instructions 30807765
  Read Bytes 157072
  Write Bytes 6640

Error:
  Instructions 30679749
  Read Bytes 157160
  Write Bytes 6580
*/
