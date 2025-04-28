import { Address, Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { green, purple, red, yellow } from "../common.js";
import {
  depositToVault,
  fetchTotalManagedFunds,
  fetchTotalSupply,
  Instruction,
  manager,
  rebalanceVault,
  withdrawFromVault
} from "../vault.js";
import { compareTotalManagedFunds, deployDefindexVault, generateExpectedTotalAmounts, generateTotalAmountsError, underlyingToDfTokens } from "./utils.js";
import { CreateVaultParams } from "../types.js";
/* 
### One asset one strategy tests:
  - [ ] deposit
  - [ ] invest
  - [ ] deposit and invest
  - [ ] rebalance with unwind
  - [ ] rebalance with `[unwind, invest]`
  - [ ] withdraw more than idle
*/
export async function testVaultOneAssetTwoStrategies(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair, xlmAddress: Address) {
  console.log(yellow, "--------------------------------------");
  console.log(yellow, "Testing one asset two strategies vault");
  console.log(yellow, "--------------------------------------");

  const error_total_managed_funds = generateTotalAmountsError(params);
  //Deploy vault
  const { 
    address:vault_address, 
    deploy_instructions, 
    deploy_read_bytes,
    deploy_write_bytes 

  } = await deployDefindexVault(addressBook, params);
  if (!vault_address) throw new Error("Vault was not deployed");

  const inital_total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

  // Deposit to vault
  const deposit_amount = 10_0_000_000;
  const {
    instructions:deposit_instructions, 
    readBytes:deposit_read_bytes, 
    writeBytes:deposit_write_bytes,
    total_managed_funds: total_managed_funds_after_deposit,
    result: deposit_result,
    error: deposit_error
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

        const expected_idle_funds = [deposit_amount];
        const expected_invested_funds = [0,0];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds);
        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        };
      } catch (e: any) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e.toString()
        };
      }
    }
  )();


  //Invest
  const invest_amount = 5_0_000_000;
  const { 
    instructions: invest_instructions, 
    readBytes:invest_read_bytes, 
    writeBytes:invest_write_bytes,
    total_managed_funds: total_managed_funds_after_invest,
    result: invest_result,
    error: invest_error
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Try Invest idle_funds*2");
        console.log(purple, "---------------------------------------");
        const idleFundsX2 = Number(total_managed_funds_after_deposit![0].idle_amount) * 2;
        const investArgs: Instruction[] = [
          {
            type: "Invest",
            strategy: params[0].strategies[0].address,
            amount: BigInt(invest_amount),
          },
          {
            type: "Invest",
            strategy: params[0].strategies[1].address,
            amount: BigInt(idleFundsX2),
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
          strategy: params[0].strategies[0].address,
          amount: BigInt(invest_amount),
        },
        {
          type: "Invest",
          strategy: params[0].strategies[1].address,
          amount: BigInt(invest_amount),
        },
      ];
      try {
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          investArgs,
          manager
        );

        const expected_idle_funds = [0]
        const expected_invested_funds = [invest_amount, invest_amount];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);

        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);
        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        };
        //To-do: return status
      } catch (e: any) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e.toString()
        };
      }
    }
  )();


  // Deposit and invest
  const {
    instructions: deposit_and_invest_instructions,
    readBytes:deposit_and_invest_read_bytes,
    writeBytes:deposit_and_invest_write_bytes,
    total_managed_funds: total_managed_funds_after_deposit_and_invest,
    result: deposit_and_invest_result,
    error: deposit_and_invest_error
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

        const expected_idle_funds = [0];
        const expected_invested_funds_s0 = invest_amount + (5_0_000_000);
        const expected_invested_funds_s1 = invest_amount + (5_0_000_000);

        const expected_invested_funds = [expected_invested_funds_s0, expected_invested_funds_s1];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);

        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);
        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        };
      } catch (e: any) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e.toString()
        };
      }
    }
  )();
  
  // Unwind
  const {
    instructions: unwind_instructions,
    readBytes:unwind_read_bytes,
    writeBytes:unwind_write_bytes,
    total_managed_funds: total_managed_funds_after_unwind,
    result: unwind_result,
    error: unwind_error
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
              strategy: params[0].strategies[0].address,
              amount: BigInt(unwind_amount),
            },
            {
              type: "Unwind",
              strategy: params[0].strategies[1].address,
              amount: BigInt(unwind_amount),
            },
          ];
          await rebalanceVault(
            vault_address,
            unwind_args,
            manager
          );
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #128)") || error.toString().includes("HostError: Error(Contract, #142)")) {
            console.log(green, "---------------------------------------------------------");
            console.log(green, "| Unwinding more than invested funds failed as expected |");
            console.log(green, "---------------------------------------------------------");
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
            strategy: params[0].strategies[0].address,
            amount: BigInt(unwind_amount),
          },
          {
            type: "Unwind",
            strategy: params[0].strategies[1].address,
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

        const expected_idle_funds = [unwind_amount*2];
        const expected_invested_funds_s0 = Number(total_managed_funds_after_deposit_and_invest![0].strategy_allocations[0].amount) - unwind_amount;
        const expected_invested_funds_s1 = Number(total_managed_funds_after_deposit_and_invest![0].strategy_allocations[1].amount) - unwind_amount;
        const expected_invested_funds = [expected_invested_funds_s0, expected_invested_funds_s1];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);

        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);
        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        };
      } catch (e:any) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e.toString()
        }
      };
    }
  )();

  const total_managed_funds_before_unwind_and_invest = await fetchTotalManagedFunds(vault_address, user);
  // Unwind and invest
  console.log('✅',total_managed_funds_before_unwind_and_invest);
  console.log('✅',total_managed_funds_before_unwind_and_invest[0].strategy_allocations);
  const {
    instructions: unwind_and_invest_instructions,
    readBytes:unwind_and_invest_read_bytes,
    writeBytes:unwind_and_invest_write_bytes,
    total_managed_funds: total_managed_funds_after_unwind_and_invest,
    result: unwind_and_invest_result,
    error: unwind_and_invest_error
  } = await (
    async () =>{
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Unwind and invest");
        console.log(purple, "---------------------------------------");
        const unwind_amount = total_managed_funds_before_unwind_and_invest[0].strategy_allocations[0].amount;
        const unwind_args: Instruction[] = [
          {
            type: "Unwind",
            strategy: params[0].strategies[0].address,
            amount: BigInt(unwind_amount),
          },
          {
            type: "Invest",
            strategy: params[0].strategies[1].address,
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
        
        const expected_idle_funds = [Number(total_managed_funds_before_unwind_and_invest[0].idle_amount)];
        const expected_invested_funds_s0 = Number(0);
        const expected_invested_funds_s1 = Number(total_managed_funds_before_unwind_and_invest[0].strategy_allocations[1].amount + unwind_amount);
        const expected_invested_funds = [expected_invested_funds_s0, expected_invested_funds_s1];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);
        
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
      
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);

        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        };
      } catch (e:any) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e.toString()
        }
      };
    }
  )();

  // withdraw from vault
  const total_managed_funds_before_withdraw = await fetchTotalManagedFunds(vault_address, user);
  const { 
    instructions: withdraw_instructions, 
    readBytes: withdraw_read_bytes, 
    writeBytes: withdraw_write_bytes,
    total_managed_funds: total_managed_funds_after_withdraw,
    result: withdraw_result,
    error: withdraw_error
  } = await (
    async () => {
      let withdraw_amount = total_managed_funds_before_withdraw[0].idle_amount + BigInt(2_0_000_000);
      console.log(purple, "--------------------------------------------------------------");
      console.log(purple,`Withdraw ${withdraw_amount} from one asset two strategies vault`);
      console.log(purple, "--------------------------------------------------------------");
      try {
        
        //Try withdraw more than total funds
        try {
          console.log(purple, "-----------------------------------------------------");
          console.log(purple, "Try withdraw more than total funds");
          console.log(purple, "-----------------------------------------------------");

          await withdrawFromVault(vault_address, [0], 100_0_000_000, user);

        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #124)") || error.toString().includes("HostError: Error(Contract, #10)")) {
            console.log(green, "-----------------------------------------------------");
            console.log(green, "| Withdraw more than total funds failed as expected |");
            console.log(green, "-----------------------------------------------------");
            //To-do: return status
          }
        }
        //Withdraw more than idle funds
        
        const total_supply = await fetchTotalSupply(vault_address, user);
        const prev_total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const withdrawAmountDfTokens = underlyingToDfTokens(withdraw_amount, total_supply, prev_total_managed_funds[0].total_amount);
        
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, [0], Number(withdrawAmountDfTokens), user);

        const expected_idle_funds = [0];
        const expected_invested_funds_s0 = Number(total_managed_funds_before_withdraw[0].strategy_allocations[0].amount);
        const expected_invested_funds_s1 = Number(total_managed_funds_before_withdraw[0].strategy_allocations[1].amount) - 2_0_000_000;
        const expected_invested_funds = [expected_invested_funds_s0, expected_invested_funds_s1];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);

        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);

        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        };
      } catch (e: any) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e.toString()
        };
      }
    }
  )();

  //Show data
  const tableData = {
    "Initial balance": {
      "Idle funds": inital_total_managed_funds[0].idle_amount,
      "Invested funds": inital_total_managed_funds[0].invested_amount,
    },
    "After deposit": {
      "Idle funds": total_managed_funds_after_deposit[0].idle_amount,
      "Invested funds": total_managed_funds_after_deposit[0].invested_amount,
    },
    "After invest": {
      "Idle funds": total_managed_funds_after_invest[0].idle_amount,
      "Invested funds": total_managed_funds_after_invest[0].invested_amount,
    },
    "After deposit and invest": {
      "Idle funds": total_managed_funds_after_deposit_and_invest[0].idle_amount,
      "Invested funds": total_managed_funds_after_deposit_and_invest[0].invested_amount,
    },
    "After unwind": {
      "Idle funds": total_managed_funds_after_unwind[0].idle_amount,
      "Invested funds": total_managed_funds_after_unwind[0].invested_amount,
    },
    "After unwind and invest": {
      "Idle funds": total_managed_funds_after_unwind_and_invest[0].idle_amount,
      "Invested funds": total_managed_funds_after_unwind_and_invest[0].invested_amount,
    },
    "After withdraw": {
      "Idle funds": total_managed_funds_after_withdraw[0].idle_amount,
      "Invested funds": total_managed_funds_after_withdraw[0].invested_amount,
    },
  };
  const budgetData = {
    deploy: {
      status: deploy_instructions && deploy_read_bytes && deploy_write_bytes ? 'success' : 'failed',
      instructions: deploy_instructions,
      readBytes: deploy_read_bytes,
      writeBytes: deploy_write_bytes,
    },
    deposit: {
      status: deposit_instructions && deposit_read_bytes && deposit_write_bytes ? 'success' : 'failed',
      instructions: deposit_instructions,
      readBytes: deposit_read_bytes,
      writeBytes: deposit_write_bytes,
    },
    invest: {
      status: invest_instructions && invest_read_bytes && invest_write_bytes ? 'success' : 'failed',
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    deposit_and_invest: {
      status: deposit_and_invest_instructions && deposit_and_invest_read_bytes && deposit_and_invest_write_bytes ? 'success' : 'failed',
      instructions: deposit_and_invest_instructions,
      readBytes: deposit_and_invest_read_bytes,
      writeBytes: deposit_and_invest_write_bytes,
    },
    unwind: {
      status: unwind_instructions && unwind_read_bytes && unwind_write_bytes ? 'success' : 'failed',
      instructions: unwind_instructions,
      readBytes: unwind_read_bytes,
      writeBytes: unwind_write_bytes,
    },
    unwind_and_invest: {
      status: unwind_and_invest_instructions && unwind_and_invest_read_bytes && unwind_and_invest_write_bytes ? 'success' : 'failed',
      instructions: unwind_and_invest_instructions,
      readBytes: unwind_and_invest_read_bytes,
      writeBytes: unwind_and_invest_write_bytes,
    },
    withdraw: {
      status: withdraw_instructions && withdraw_read_bytes && withdraw_write_bytes ? 'success' : 'failed',
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
  }
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}
