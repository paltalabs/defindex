import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import {
  depositToVault,
  fetchTotalManagedFunds,
  fetchTotalSupply,
  Instruction,
  manager,
  pauseStrategy,
  rebalanceVault,
  rescueFromStrategy,
  unpauseStrategy,
  withdrawFromVault
} from "../../utils/vault.js";
import { green, purple, red, yellow } from "../common.js";
import { CreateVaultParams } from "../types.js";
import { testAccessControl } from "./access_control.js";
import { testUpgradeContract } from "./upgrade_contract.js";
import {
  compareTotalManagedFunds,
  deployDefindexVault,
  generateExpectedTotalAmounts,
  generateTotalAmountsError,
  underlyingToDfTokens
} from "./utils.js";


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
      console.log(purple, `Deposit ${deposit_amount} in one strategy`);
      console.log(purple, "-----------------------------------------");
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_amount], user);

        const expected_idle_funds = [deposit_amount];
        const expected_invested_funds = [0];
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
      ];
      try {
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          investArgs,
          manager
        );

        const expected_idle_funds = [Number(total_managed_funds_after_deposit![0].idle_amount) - invest_amount];
        const expected_invested_funds = [invest_amount];
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

  // Deposit and invest
  const deposit_and_invest_amount: number = 10_0_000_000;
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
      
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_and_invest_amount], user, true);

        const expected_idle_funds = [0];
        const expected_invested_funds = [invest_amount + deposit_and_invest_amount];
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
  
  // Unwind
  const unwind_amount = 5_0_000_000;
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
          const large_unwind_amount = 100_0_000_000;
          const unwind_args: Instruction[] = [
            {
              type: "Unwind",
              strategy: params[0].strategies[0].address,
              amount: BigInt(large_unwind_amount),
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

        // Unwind from unauthorized
        try { 
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try Unwind from unauthorized");
          console.log(purple, "---------------------------------------");
          const unwind_args: Instruction[] = [
            {
              type: "Unwind",
              strategy: params[0].strategies[0].address,
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
            console.log(green, "| Unwinding from unauthorized failed as expected |");
            console.log(green, "---------------------------------------------------------");
          }else {
            throw Error(error);
          }
        }

        console.log(purple, "---------------------------------------");
        console.log(purple, "Unwind");
        console.log(purple, "---------------------------------------");
        const unwind_args: Instruction[] = [
          {
            type: "Unwind",
            strategy: params[0].strategies[0].address,
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

        const expected_idle_funds = [unwind_amount];
        const expected_invested_funds = [Number(total_managed_funds_after_deposit_and_invest![0].strategy_allocations[0].amount) - unwind_amount];
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

  // Rebalance vault
  const invest_rebalance_amount = 7_0_000_000;
  const unwind_rebalance_amount = 3_0_000_000;
  const { 
    instructions: rebalance_instructions, 
    readBytes:rebalance_read_bytes, 
    writeBytes:rebalance_write_bytes,
    total_managed_funds: total_managed_funds_after_rebalance,
    result: rebalance_result,
    error: rebalance_error
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
              strategy: params[0].strategies[0].address,
              amount: BigInt(7_0_000),
            },
            {
              type: "Unwind",
              strategy: params[0].strategies[0].address,
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
          }else {
            throw Error(error);
          }
        }
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rebalancing vault"); 
        console.log(purple, "---------------------------------------");
        
        const rebalanceArgs: Instruction[] = [
          {
            type: "Invest",
            strategy: params[0].strategies[0].address,
            amount: BigInt(invest_rebalance_amount),
          },
          {
            type: "Unwind",
            strategy: params[0].strategies[0].address,
            amount: BigInt(unwind_rebalance_amount),
          },
        ];
     
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );

        const expected_idle_funds = [Number(total_managed_funds_after_unwind![0].idle_amount) - invest_rebalance_amount + unwind_rebalance_amount];
        const expected_invested_funds = [Number(total_managed_funds_after_unwind![0].strategy_allocations[0].amount) + invest_rebalance_amount - unwind_rebalance_amount];
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

  // withdraw from vault
  const withdraw_amount = 2_0_000_000;
  const { 
    instructions: withdraw_instructions, 
    readBytes: withdraw_read_bytes, 
    writeBytes: withdraw_write_bytes,
    total_managed_funds: total_managed_funds_after_withdraw,
    result: withdraw_result,
    error: withdraw_error
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple,`Withdraw ${withdraw_amount} from one strategy`);
      console.log(purple, "---------------------------------------");
      try {
        //Try withdraw from unauthorized
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try withdraw from unauthorized");
          console.log(purple, "---------------------------------------");
          const unauthorized_withdraw_amount = 65_0_000;
          const random_user = Keypair.random();
          await airdropAccount(random_user);

          await withdrawFromVault(vault_address, [0], unauthorized_withdraw_amount, random_user);
          
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #111)") || error.toString().includes("HostError: Error(Contract, #10)")) {
            console.log(green, "-------------------------------------------------");
            console.log(green, "| Withdraw from unauthorized failed as expected |");
            console.log(green, "-------------------------------------------------");
          }else {
            throw Error(error);
          }
        }
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
          } else {
            throw Error(error);
          }
        }
        
        const total_supply = await fetchTotalSupply(vault_address, user);
        const total_managed_funds_before_withdraw = await fetchTotalManagedFunds(vault_address, user);
        const withdrawAmountDfTokens = underlyingToDfTokens(withdraw_amount, total_supply, total_managed_funds_before_withdraw[0].total_amount);
        
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, [0], Number(withdrawAmountDfTokens), user);

        const expected_idle_funds = [Number(total_managed_funds_before_withdraw[0].idle_amount) - withdraw_amount];
        const expected_invested_funds = [Number(total_managed_funds_before_withdraw[0].strategy_allocations[0].amount)];
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

  //rescue funds
  const {
    instructions: rescue_instructions,
    readBytes: rescue_read_bytes,
    writeBytes: rescue_write_bytes,
    total_managed_funds: total_managed_funds_after_rescue,
    result: rescue_result,
    error: rescue_error
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
          await rescueFromStrategy(vault_address, params[0].strategies[0].address, user);
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "-----------------------------------------------");
            console.log(green, "| Rescue from unauthorized failed as expected |");
            console.log(green, "-----------------------------------------------");
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
          }else {
            throw Error(error);
          }
        }
        // Rescue
        const { instructions, readBytes, writeBytes } = await rescueFromStrategy(vault_address, params[0].strategies[0].address, manager);
        
        const expected_idle_funds = [Number(total_managed_funds_after_withdraw![0].idle_amount) + Number(total_managed_funds_after_withdraw![0].strategy_allocations[0].amount)];
        const expected_invested_funds = [0];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, [expected_idle_funds], [expected_invested_funds]);

        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds);
        
        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: error.toString()
        }
      }
    }
  )();

  // unpause strategy
  const {
    instructions: unpause_instructions,
    readBytes: unpause_read_bytes,
    writeBytes: unpause_write_bytes,
    total_managed_funds: total_managed_funds_after_unpause,
    result: unpause_result,
    error: unpause_error
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Unpause strategy");
        console.log(purple, "---------------------------------------");
        // Try unpause from unauthorized
        try { 
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try unpause from unauthorized");
          console.log(purple, "---------------------------------------");
          await unpauseStrategy(vault_address, params[0].strategies[0].address, user);
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #130)")) {
            console.log(green, "-----------------------------------------------");
            console.log(green, "| Unpause from unauthorized failed as expected |");
            console.log(green, "-----------------------------------------------");
          }else {
            throw Error(error);
          }
        }
        
        // Try unpause non existent strategy
        try { 
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try unpause non existent strategy");
          console.log(purple, "---------------------------------------");
          const random_address = Keypair.random();
          await airdropAccount(random_address);
          await unpauseStrategy(vault_address, random_address.publicKey(), manager);
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #140)")) {
            console.log(green, "---------------------------------------------------------");
            console.log(green, "| Unpause non existent strategy failed as expected |");
            console.log(green, "---------------------------------------------------------");
          }else {
            throw Error(error);
          }
        }
        
        // Actual unpause
        const {instructions, readBytes, writeBytes} = await unpauseStrategy(vault_address, params[0].strategies[0].address, manager);

        const expected_idle_funds = [Number(total_managed_funds_after_rescue![0].idle_amount)];
        const expected_invested_funds = [0];
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
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: error.toString()
        }
      }
    }
  )();

  // pause strategy
  const {
    instructions: pause_instructions,
    readBytes: pause_read_bytes,
    writeBytes: pause_write_bytes,
    total_managed_funds: total_managed_funds_after_pause,
    result: pause_result,
    error: pause_error
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Pause strategy");
        console.log(purple, "---------------------------------------");
        const { instructions, readBytes, writeBytes } = await pauseStrategy(vault_address, params[0].strategies[0].address, manager);

        const expected_idle_funds = [Number(total_managed_funds_after_unpause![0].idle_amount)];
        const expected_invested_funds = [0];
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
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: error.toString()
        }
      }
    }
  )();

  // Mostrar resultados en formato de tabla
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
    "After rebalance": {
      "Idle funds": total_managed_funds_after_rebalance[0].idle_amount,
      "Invested funds": total_managed_funds_after_rebalance[0].invested_amount,
    },
    "After withdraw": {
      "Idle funds": total_managed_funds_after_withdraw[0].idle_amount,
      "Invested funds": total_managed_funds_after_withdraw[0].invested_amount,
    },
    "After rescue": {
      "Idle funds": total_managed_funds_after_rescue[0].idle_amount,
      "Invested funds": total_managed_funds_after_rescue[0].invested_amount,
    },
    "After unpause": {
      "Idle funds": total_managed_funds_after_unpause[0].idle_amount,
      "Invested funds": total_managed_funds_after_unpause[0].invested_amount,
    },
    "After pause": {
      "Idle funds": total_managed_funds_after_pause[0].idle_amount,
      "Invested funds": total_managed_funds_after_pause[0].invested_amount,
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
    unpause: {
      instructions: unpause_instructions,
      readBytes: unpause_read_bytes,
      writeBytes: unpause_write_bytes,
    },
    pause: {
      instructions: pause_instructions,
      readBytes: pause_read_bytes,
      writeBytes: pause_write_bytes,
    }
  };

  console.table(tableData);
  console.table(budgetData);

  return { tableData, budgetData };
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