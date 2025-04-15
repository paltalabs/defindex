import { Address, Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import {
  depositToVault,
  Instruction,
  manager,
  rebalanceVault,
  rescueFromStrategy,
  withdrawFromVault
} from "../vault.js";
import { green, purple, red, yellow } from "../common.js";
import { airdropAccount } from "../../utils/contract.js";
import { deployDefindexVault, fetchBalances, fetchStrategiesBalances } from "./utils.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";
import { CreateVaultParams } from "../types.js";
import { USDC_ADDRESS } from "../../constants.js";
/* 
// Two assets one strategy tests:
  - [x] deposit
  - [x] invest
  - [x] deposit and invest
  - [x] try rebalance with unwind and more than invested
  - [x] try rebalance with unwind from unauthorized
  - [x] rebalance with unwind
  - [x] try rebalance with a swap (exact in and out) and wrong input asset
  - [x] rebalance with a unwind, swap exact in, invest
  - [x] try rebalance with a swap (exact in and out) and wrong output asset
  - [x] rebalance with a unwind, swap exact out, invest
  - [x] withdraw more than idle
*/
export async function testVaultTwoAssetsOneStrategy(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair, xlmAddress:Address,) {
  console.log(yellow, "-------------------------------------------");
  console.log(yellow, "Testing two assets one strategy vault");
  console.log(yellow, "-------------------------------------------");

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
  const deposit_amount_0 = 10_0_000_000;
  const deposit_amount_1 = 20_0_000_000;
  const {
    instructions:deposit_instructions, 
    readBytes:deposit_read_bytes, 
    writeBytes:deposit_write_bytes 
  } = await (
    async () => {
      console.log(purple, "------------------------------------------------------------------");
      console.log(purple, `Deposit ${deposit_amount_0}, ${deposit_amount_1} in two strategies`);
      console.log(purple, "------------------------------------------------------------------");
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_amount_0, deposit_amount_1], user);
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
  const invest_amount_0 = 5_0_000_000;
  const invest_amount_1 = 10_0_000_000;
  const { 
    instructions: invest_instructions, 
    readBytes:invest_read_bytes, 
    writeBytes:invest_write_bytes,
    idle_funds_after_invest,
    invested_funds_after_invest,
    fixed_xtar_strategy_balance: fixed_xtar_strategy_balance_after_invest,
    fixed_usdc_strategy_balance: fixed_usdc_strategy_balance_after_invest,
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Try Invest idle_funds*2");
        console.log(purple, "---------------------------------------");
        const invest_amount_0 = Number(idle_funds_after_deposit[0].amount) * 2;
        const invest_amount_1 = Number(idle_funds_after_deposit[1].amount) * 2;
        console.log(yellow, "Invest amount 0:", invest_amount_0);
        console.log(yellow, "Invest amount 1:", invest_amount_1);
        const investArgs: Instruction[] = [
          {
            type: "Invest",
            strategy: addressBook.getContractId("fixed_xtar_strategy"),
            amount: BigInt(invest_amount_0),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("fixed_usdc_strategy"),
            amount: BigInt(invest_amount_1),
          },
        ];
        await rebalanceVault(
            vault_address,
            investArgs,
            manager
          );
      } catch (error:any) {
        if(error.toString().includes("HostError: Error(Contract, #10)") || error.toString().includes("HostError: Error(WasmVm, InvalidAction)")) {
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
          strategy: addressBook.getContractId("fixed_xtar_strategy"),
          amount: BigInt(invest_amount_0),
        },
        {
          type: "Invest",
          strategy: addressBook.getContractId("fixed_usdc_strategy"),
          amount: BigInt(invest_amount_1),
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

        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);

        const expected_idle_funds = [idle_funds_after_deposit[0].amount - BigInt(invest_amount_0), idle_funds_after_deposit[1].amount - BigInt(invest_amount_1)];
        const expected_invested_funds = [invested_funds_after_deposit[0].amount + BigInt(invest_amount_0), invested_funds_after_deposit[1].amount + BigInt(invest_amount_1)];
        const expected_strategy_balances = [BigInt(invest_amount_0), BigInt(invest_amount_1)];

        if(idle_funds_after_invest[0].amount !== expected_idle_funds[0]) {
          console.error(red, `idle funds: ${idle_funds_after_invest[0].amount} !== ${expected_idle_funds[0]}`);
          throw Error("Idle 0 funds after invest failed");
        }

        if(idle_funds_after_invest[1].amount !== expected_idle_funds[1]) {
          console.error(red, `idle funds: ${idle_funds_after_invest[1].amount} !== ${expected_idle_funds[1]}`);
          throw Error("Idle 1 funds after invest failed");
        }

        if(invested_funds_after_invest[0].amount !== expected_invested_funds[0]) {
          console.error(red, `invested funds: ${invested_funds_after_invest[0].amount} !== ${expected_invested_funds[0]}`);
          throw Error("Invested 0 funds after invest failed");
        }

        if(invested_funds_after_invest[1].amount !== expected_invested_funds[1]) {
          console.error(red, `invested funds: ${invested_funds_after_invest[1].amount} !== ${expected_invested_funds[1]}`);
          throw Error("Invested 1 funds after invest failed");
        }

        if(BigInt(current_strategies_balances[0].strategy_balance) !== expected_strategy_balances[0]) {
          console.error(red, `strategy balance: ${current_strategies_balances[0].strategy_balance} !== ${expected_strategy_balances[0]}`);
          throw Error("Strategy 0 balance after invest failed");
        }

        if(BigInt(current_strategies_balances[1].strategy_balance) !== expected_strategy_balances[1]) {
          console.error(red, `strategy balance: ${current_strategies_balances[1].strategy_balance} !== ${expected_strategy_balances[1]}`);
          throw Error("Strategy 1 balance after invest failed");
        }

        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          idle_funds_after_invest, 
          invested_funds_after_invest, 
          fixed_xtar_strategy_balance: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance: current_strategies_balances[1].strategy_balance,
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
          fixed_xtar_strategy_balance: BigInt(0),
          fixed_usdc_strategy_balance: BigInt(0),
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
    fixed_xtar_strategy_balance_after_deposit_and_invest,
    fixed_usdc_strategy_balance_after_deposit_and_invest,
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Deposit and invest in vault");
      console.log(purple, "---------------------------------------");
      const deposit_and_invest_amount_0: number = 10_0_000_000;
      const deposit_and_invest_amount_1: number = 5_0_000_000;
      
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_and_invest_amount_0, deposit_and_invest_amount_1], user, true);
        
        const {
          idle_funds:idle_funds_after_deposit_and_invest, 
          invested_funds:invested_funds_after_deposit_and_invest, 
        } = await fetchBalances(addressBook, vault_address, params, user);

        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);

        const expected_idle_funds = [
          idle_funds_after_invest[0].amount, 
          idle_funds_after_invest[1].amount
        ];
        const expected_invested_funds = [
          invested_funds_after_invest[0].amount + BigInt(deposit_and_invest_amount_0 / 4), 
          invested_funds_after_invest[1].amount + BigInt(deposit_and_invest_amount_1)
        ];
        const expected_strategy_balances = [
          BigInt(fixed_xtar_strategy_balance_after_invest) + BigInt(deposit_and_invest_amount_0 / 4), 
          BigInt(fixed_usdc_strategy_balance_after_invest) + BigInt(deposit_and_invest_amount_1)
        ];

        if(idle_funds_after_deposit_and_invest[0].amount !== expected_idle_funds[0]) {
          console.error(red, `idle funds: ${idle_funds_after_deposit_and_invest[0].amount} !== ${expected_idle_funds[0]}`);
          throw Error("Idle 0 funds after deposit and invest failed");
        }

        if(idle_funds_after_deposit_and_invest[1].amount !== expected_idle_funds[1]) {
          console.error(red, `idle funds: ${idle_funds_after_deposit_and_invest[1].amount} !== ${expected_idle_funds[1]}`);
          throw Error("Idle 1 funds after deposit and invest failed");
        }

        if(invested_funds_after_deposit_and_invest[0].amount !== expected_invested_funds[0]) {
          console.error(red, `invested funds: ${invested_funds_after_deposit_and_invest[0].amount} !== ${expected_invested_funds[0]}`);
          throw Error("Invested 0 funds after deposit and invest failed");
        }

        if(invested_funds_after_deposit_and_invest[1].amount !== expected_invested_funds[1]) {
          console.error(red, `invested funds: ${invested_funds_after_deposit_and_invest[1].amount} !== ${expected_invested_funds[1]}`);
          throw Error("Invested 1 funds after deposit and invest failed");
        }

        if(BigInt(current_strategies_balances[0].strategy_balance) !== expected_strategy_balances[0]) {
          console.error(red, `strategy balance: ${current_strategies_balances[0].strategy_balance} !== ${expected_strategy_balances[0]}`);
          throw Error("Strategy 0 balance after deposit and invest failed");
        }

        if(BigInt(current_strategies_balances[1].strategy_balance) !== expected_strategy_balances[1]) {
          console.error(red, `strategy balance: ${current_strategies_balances[1].strategy_balance} !== ${expected_strategy_balances[1]}`);
          throw Error("Strategy 1 balance after deposit and invest failed");
        }

        return { 
          instructions, 
          readBytes, 
          writeBytes,
          idle_funds_after_deposit_and_invest,
          invested_funds_after_deposit_and_invest,
          fixed_xtar_strategy_balance_after_deposit_and_invest: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance_after_deposit_and_invest: current_strategies_balances[1].strategy_balance
        };
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_deposit_and_invest: [{ amount: BigInt(0) }],
          invested_funds_after_deposit_and_invest: [{ amount: BigInt(0) }],
          fixed_xtar_strategy_balance_after_deposit_and_invest: BigInt(0),
          fixed_usdc_strategy_balance_after_deposit_and_invest: BigInt(0),
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
    fixed_xtar_strategy_balance_after_unwind,
    fixed_usdc_strategy_balance_after_unwind,
  } = await (
    async () =>{
      try {

        // Unwind more than invested
        try { 
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try Unwind more than invested");
          console.log(purple, "---------------------------------------");
          const unwind_amount_0 = 100_0_000_000;
          const unwind_amount_1 = 100_0_000_000;
          const unwind_args: Instruction[] = [
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_xtar_strategy"),
              amount: BigInt(unwind_amount_0),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_usdc_strategy"),
              amount: BigInt(unwind_amount_1),
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
          const unwind_amount_0 = 5_0_000_000;
          const unwind_amount_1 = 5_0_000_000;
          const unwind_args: Instruction[] = [
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_xtar_strategy"),
              amount: BigInt(unwind_amount_0),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_usdc_strategy"),
              amount: BigInt(unwind_amount_1),
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
            //To-do: return status
          }else {
            throw Error(error);
          }
        }

        console.log(purple, "---------------------------------------");
        console.log(purple, "Unwind");
        console.log(purple, "---------------------------------------");
        const unwind_amount_0 = 5_0_000_000;
        const unwind_amount_1 = 1_0_000_000;
        const unwind_args: Instruction[] = [
          {
            type: "Unwind",
            strategy: addressBook.getContractId("fixed_xtar_strategy"),
            amount: BigInt(unwind_amount_0),
          },
          {
            type: "Unwind",
            strategy: addressBook.getContractId("fixed_usdc_strategy"),
            amount: BigInt(unwind_amount_1),
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

        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);

        let expected_idle_funds = [BigInt(idle_funds_after_deposit_and_invest[0].amount) + BigInt(unwind_amount_0), BigInt(idle_funds_after_deposit_and_invest[1].amount) + BigInt(unwind_amount_1)];
        let expected_invested_funds = [BigInt(invested_funds_after_deposit_and_invest[0].amount) - BigInt(unwind_amount_0), BigInt(invested_funds_after_deposit_and_invest[1].amount) - BigInt(unwind_amount_1)];
        let expected_strategy_balances = [BigInt(fixed_xtar_strategy_balance_after_deposit_and_invest) - BigInt(unwind_amount_0), BigInt(fixed_usdc_strategy_balance_after_deposit_and_invest) - BigInt(unwind_amount_1)];
        if(idle_funds_after_unwind[0].amount !== expected_idle_funds[0]) {
          console.error(red, `idle funds 0: ${idle_funds_after_unwind[0].amount} !== ${expected_idle_funds[0]}`);
          throw Error("Idle funds after unwind failed");
        }

        if(idle_funds_after_unwind[1].amount !== expected_idle_funds[1]) {
          console.error(red, `idle funds 1: ${idle_funds_after_unwind[1].amount} !== ${expected_idle_funds[1]}`);
          throw Error("Idle funds after unwind failed");
        }

        if(invested_funds_after_unwind[0].amount !== expected_invested_funds[0]) {
          console.error(red, `invested funds 0: ${invested_funds_after_unwind[0].amount} !== ${expected_invested_funds[0]}`);
          throw Error("Invested funds after unwind failed");
        }

        if(invested_funds_after_unwind[1].amount !== expected_invested_funds[1]) {
          console.error(red, `invested funds 1: ${invested_funds_after_unwind[1].amount} !== ${expected_invested_funds[1]}`);
          throw Error("Invested funds after unwind failed");
        }

        if(BigInt(current_strategies_balances[0].strategy_balance) !== expected_strategy_balances[0]) {
          console.error(red, `strategy balance: ${current_strategies_balances[0].strategy_balance} !== ${expected_strategy_balances[0]}`);
          throw Error("Strategy 0 balance after unwind failed");
        }

        if(BigInt(current_strategies_balances[1].strategy_balance) !== expected_strategy_balances[1]) {
          console.error(red, `strategy balance: ${current_strategies_balances[1].strategy_balance} !== ${expected_strategy_balances[1]}`);
          throw Error("Strategy 1 balance after unwind failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds_after_unwind,
          invested_funds_after_unwind,
          fixed_xtar_strategy_balance_after_unwind: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance_after_unwind: current_strategies_balances[1].strategy_balance,
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_unwind: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          invested_funds_after_unwind: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          fixed_xtar_strategy_balance_after_unwind: BigInt(0),
          fixed_usdc_strategy_balance_after_unwind: BigInt(0),
          error: error
        }
      };
    }
  )();
  
  //rebalance with a unwind, swap exact in, invest 
  const { 
    instructions: rebalance_swap_e_in_instructions, 
    readBytes:rebalance_swap_e_in_read_bytes, 
    writeBytes:rebalance_swap_e_in_write_bytes,
    idle_funds: idle_funds_after_rebalance_swap_e_in,
    invested_funds: invested_funds_after_rebalance_swap_e_in,
    fixed_xtar_strategy_balance: fixed_xtar_strategy_balance_after_rebalance_swap_e_in,
    fixed_usdc_strategy_balance: fixed_usdc_strategy_balance_after_rebalance_swap_e_in,
  } = await (
    async () => {
      try {
        // Rebalance with swap, wrong input asset
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try rebalance swap wrong asset in"); 
          console.log(purple, "---------------------------------------");
          const rebalanceArgs: Instruction[] = [
            {
              type: "Invest",
              strategy: addressBook.getContractId("fixed_xtar_strategy"),
              amount: BigInt(7_0_000),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_xtar_strategy"),
              amount: BigInt(6_0_00),
            },
            {
              type: "SwapExactIn",
              amount_in: BigInt(1_0_000),
              amount_out_min: BigInt(1_0_000),
              token_in: xlmAddress.toString(),
              token_out: USDC_ADDRESS.toString(),
              deadline: BigInt(getCurrentTimePlusOneHour()),
            }
          ];
          await rebalanceVault(
            vault_address,
            rebalanceArgs,
            manager
          );        
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #116)")) {
            console.log(green, "----------------------------------------------------");
            console.log(green, "| Rrebalance swap wrong asset in failed as expected |");
            console.log(green, "----------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rebalance swap exact in"); 
        console.log(purple, "---------------------------------------");

        const rebalanceArgs: Instruction[] = [
          {
            type: "Unwind",
            strategy: addressBook.getContractId("fixed_xtar_strategy"),
            amount: BigInt(5_000_000),
          }, 
          {
            type: "SwapExactIn",
            amount_in: BigInt(5_000_000),
            amount_out_min: BigInt(5_000_000),
            token_in: USDC_ADDRESS.toString(),
            token_out: xlmAddress.toString(),
            deadline: BigInt(getCurrentTimePlusOneHour()),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("fixed_usdc_strategy"),
            amount: BigInt(1_0_000_000),
          },
        ];       
     
        const {instructions, readBytes, writeBytes, result} = await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );

        const {
          idle_funds, 
          invested_funds, 
        } = await fetchBalances(addressBook, vault_address, params, user);

        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);

        const expected_invested_funds = [BigInt(2_0_000_000), BigInt(15_0_000_000)];

        if(!(idle_funds[0].amount > idle_funds_after_unwind[0].amount)) {
          console.error(red, `idle funds 0: ${idle_funds[0].amount} !> ${idle_funds_after_unwind[0].amount}`);
          throw Error("Idle 0 funds after rebalance failed");
        }

        if(!(idle_funds[1].amount < idle_funds_after_unwind[1].amount)) {
          console.error(red, `idle funds 1: ${idle_funds[1].amount} !> ${idle_funds_after_unwind[1].amount}`);
          throw Error("Idle 1 funds after rebalance failed");
        }

        if(invested_funds[0].amount !== expected_invested_funds[0]) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds[0]}`);
          throw Error("Invested 0 funds after rebalance failed");
        }

        if(invested_funds[1].amount !== expected_invested_funds[1]) {
          console.error(red, `invested funds: ${invested_funds[1].amount} !== ${expected_invested_funds[1]}`);
          throw Error("Invested 1 funds after rebalance failed");
        }



        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
          fixed_xtar_strategy_balance: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance: current_strategies_balances[1].strategy_balance,
        }
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          error: e,
          idle_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          fixed_xtar_strategy_balance: BigInt(0),
          fixed_usdc_strategy_balance: BigInt(0),
        };
      }
    }
  )();
  //rebalance with a unwind, swap exact out, invest 
  const { 
    instructions: rebalance_swap_e_out_instructions, 
    readBytes:rebalance_swap_e_out_read_bytes, 
    writeBytes:rebalance_swap_e_out_write_bytes,
    idle_funds: idle_funds_after_rebalance_swap_e_out,
    invested_funds: invested_funds_after_rebalance_swap_e_out,
    fixed_xtar_strategy_balance: fixed_xtar_strategy_balance_after_rebalance_swap_e_out,
    fixed_usdc_strategy_balance: fixed_usdc_strategy_balance_after_rebalance_swap_e_out,
  } = await (
    async () => {
      try {
        // Rebalance with swap, wrong output asset
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try rebalance swap wrong asset out"); 
          console.log(purple, "---------------------------------------");
          const rebalanceArgs: Instruction[] = [
            {
              type: "Invest",
              strategy: addressBook.getContractId("fixed_xtar_strategy"),
              amount: BigInt(7_0_000),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_xtar_strategy"),
              amount: BigInt(6_0_00),
            },
            {
              type: "SwapExactIn",
              amount_in: BigInt(1_0_000),
              amount_out_min: BigInt(0),
              token_in: USDC_ADDRESS.toString(),
              token_out: xlmAddress.toString(),
              deadline: BigInt(getCurrentTimePlusOneHour()),
            }
          ];
          await rebalanceVault(
            vault_address,
            rebalanceArgs,
            manager
          );        
        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #116)")) {
            console.log(green, "----------------------------------------------------");
            console.log(green, "| Rrebalance swap wrong asset out failed as expected |");
            console.log(green, "----------------------------------------------------");
            //To-do: return status
          }else {
            throw Error(error);
          }
        }
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rebalance swap exact out"); 
        console.log(purple, "---------------------------------------");

        const rebalanceArgs: Instruction[] = [
          {
            type: "Unwind",
            strategy: addressBook.getContractId("fixed_xtar_strategy"),
            amount: BigInt(5_000_000),
          }, 
          {
            type: "SwapExactOut",
            amount_out: BigInt(5_000_000),
            amount_in_max: BigInt(10_0_000_000),
            token_in: USDC_ADDRESS.toString(),
            token_out: xlmAddress.toString(),
            deadline: BigInt(getCurrentTimePlusOneHour()),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("fixed_usdc_strategy"),
            amount: BigInt(1_0_000_000),
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
        } = await fetchBalances(addressBook, vault_address, params, user);

        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);

        const expected_invested_funds = [BigInt(1_5_000_000), BigInt(16_0_000_000)];

        if(!(idle_funds[0].amount > idle_funds_after_rebalance_swap_e_in[0].amount)) {
          console.error(red, `idle funds 0: ${idle_funds[0].amount} < ${idle_funds_after_rebalance_swap_e_in[0].amount}`);
          throw Error("Idle 0 funds after rebalance failed");
        }

        if(!(idle_funds[1].amount < idle_funds_after_rebalance_swap_e_in[1].amount)) {
          console.error(red, `idle funds 1: ${idle_funds[1].amount} < ${idle_funds_after_rebalance_swap_e_in[1].amount}`);
          throw Error("Idle 1 funds after rebalance failed");
        }

        if(invested_funds[0].amount !== expected_invested_funds[0]) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds[0]}`);
          throw Error("Invested 0 funds after rebalance failed");
        }

        if(invested_funds[1].amount !== expected_invested_funds[1]) {
          console.error(red, `invested funds: ${invested_funds[1].amount} !== ${expected_invested_funds[1]}`);
          throw Error("Invested 1 funds after rebalance failed");
        }

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
          fixed_xtar_strategy_balance: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance: current_strategies_balances[1].strategy_balance,
        }
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          error: e,
          idle_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          fixed_xtar_strategy_balance: BigInt(0),
          fixed_usdc_strategy_balance: BigInt(0),
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
    fixed_xtar_strategy_balance: fixed_xtar_strategy_balance_after_withdraw,
    fixed_usdc_strategy_balance: fixed_usdc_strategy_balance_after_withdraw,
  } = await (
    async () => {
      let withdraw_amount = 1_0_000_000;
      console.log(purple, "----------------------------------------------");
      console.log(purple,`Withdraw ${withdraw_amount} from two strategies`);
      console.log(purple, "----------------------------------------------");
      try {
        //Try withdraw from unauthorized
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Try withdraw from unauthorized");
          console.log(purple, "---------------------------------------");
          const withdraw_amount = 65_0_000;
          const random_user = Keypair.random();
          await airdropAccount(random_user);

          await withdrawFromVault(vault_address, [0,0], withdraw_amount, random_user);
          
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

          await withdrawFromVault(vault_address, [0,0], 100_0_000_000, user);

        } catch (error:any) {
          if (error.toString().includes("HostError: Error(Contract, #124)") || error.toString().includes("HostError: Error(Contract, #10)")) {
            console.log(green, "-----------------------------------------------------");
            console.log(green, "| Withdraw more than total funds failed as expected |");
            console.log(green, "-----------------------------------------------------");
            //To-do: return status
          }
        }
        //Withdraw
        console.log(yellow, idle_funds_after_rebalance_swap_e_out[0]);
        console.log(yellow, idle_funds_after_rebalance_swap_e_out[1]);

        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, [0,0], withdraw_amount, user);
        
        const { 
          idle_funds, 
          invested_funds, 
        } = await fetchBalances(addressBook, vault_address, params, user);
        
        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);

        if(!(idle_funds[0].amount < idle_funds_after_rebalance_swap_e_out[0].amount)) {
          console.error(red, `idle funds 0: ${idle_funds[0].amount} < ${idle_funds_after_rebalance_swap_e_out[0].amount}`);
          throw Error("Idle 0 funds after withdraw failed");
        }

        if(!(idle_funds[1].amount < idle_funds_after_rebalance_swap_e_out[1].amount)) {
          console.error(red, `idle funds 1: ${idle_funds[1].amount} < ${idle_funds_after_rebalance_swap_e_out[1].amount}`);
          throw Error("Idle 1 funds after withdraw failed");
        }

        if(invested_funds[0].amount !== invested_funds_after_rebalance_swap_e_out[0].amount) {
          console.error(red, `invested funds 0: ${invested_funds[0].amount} !== ${invested_funds_after_rebalance_swap_e_out[0].amount}`);
          throw Error("Invested 0 funds after withdraw failed");
        }

        if(invested_funds[1].amount !== invested_funds_after_rebalance_swap_e_out[1].amount) {
          console.error(red, `invested funds 1: ${invested_funds[1].amount} !== ${invested_funds_after_rebalance_swap_e_out[1].amount}`);
          throw Error("Invested 1 funds after withdraw failed");
        }
        
        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          idle_funds, 
          invested_funds,
          fixed_xtar_strategy_balance: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance: current_strategies_balances[1].strategy_balance,
        };
      } catch (e) {
        console.error(red, e);
        return {
          withdraw_instructions: 0,
          withdraw_read_bytes: 0,
          withdraw_write_bytes: 0,
          idle_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          fixed_xtar_strategy_balance: BigInt(0),
          fixed_usdc_strategy_balance: BigInt(0),
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
    fixed_xtar_strategy_balance: fixed_xtar_strategy_balance_after_rescue,
    fixed_usdc_strategy_balance: fixed_usdc_strategy_balance_after_rescue,
  } = await (
    async () => {
      try {
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
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rescue funds");
        console.log(purple, "---------------------------------------");
        // Rescue
        const { instructions: xtar_rescue_instructions, readBytes: xtar_rescue_readBytes, writeBytes: xtar_rescue_writeBytes} = await rescueFromStrategy(vault_address, addressBook.getContractId("fixed_xtar_strategy"), manager);
        const { instructions: usdc_rescue_instructions, readBytes: usdc_rescue_readBytes, writeBytes: usdc_rescue_writeBytes } = await rescueFromStrategy(vault_address, addressBook.getContractId("fixed_usdc_strategy"), manager);
        const { idle_funds, invested_funds } = await fetchBalances(addressBook, vault_address, params, user);
        const current_strategies_balances = await fetchStrategiesBalances(addressBook, ['fixed_xtar_strategy', 'fixed_usdc_strategy'], vault_address, user);
        const expected_idle_funds = idle_funds_after_withdraw[0].amount + invested_funds_after_withdraw[0].amount;
        const expected_invested_funds = BigInt(0);


        if(idle_funds[0].amount !== expected_idle_funds) {
          console.error(red, `idle funds: ${idle_funds[0].amount} !== ${expected_idle_funds}`);
          throw Error("Idle funds after rescue failed");
        }

        if(invested_funds[0].amount !== expected_invested_funds) {
          console.error(red, `invested funds: ${invested_funds[0].amount} !== ${expected_invested_funds}`);
          throw Error("Invested funds after rescue failed");
        }
        const instructions = [xtar_rescue_instructions, usdc_rescue_instructions];
        const readBytes = [xtar_rescue_readBytes, usdc_rescue_readBytes];
        const writeBytes = [xtar_rescue_writeBytes, usdc_rescue_writeBytes];
        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
          fixed_xtar_strategy_balance: current_strategies_balances[0].strategy_balance,
          fixed_usdc_strategy_balance: current_strategies_balances[1].strategy_balance,
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          fixed_xtar_strategy_balance: BigInt(0),
          fixed_usdc_strategy_balance: BigInt(0),
          error: error,
      }
    }
  }   
  )();

  //Show data
  const tableData = {
    "Initial balance": {
      "Idle funds a_0": idle_funds_before_deposit[0].amount,
      "Idle funds a_1": idle_funds_before_deposit[1].amount,
      "Invested funds a_0": invested_funds_before_deposit[0].amount,
    },
    "After deposit": {
      "Idle funds a_0": idle_funds_after_deposit[0].amount,
      "Idle funds a_1": idle_funds_after_deposit[1].amount,
      "Invested funds a_0": invested_funds_after_deposit[0].amount,
      "Invested funds a_1": invested_funds_after_deposit[1].amount,
    },
    "After invest": {
      "Idle funds a_0": idle_funds_after_invest[0].amount,
      "Idle funds a_1": idle_funds_after_invest[1].amount,
      "Invested funds a_0": invested_funds_after_invest[0].amount,
      "Invested funds a_1": invested_funds_after_invest[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_invest,
      "usdc strategy": fixed_usdc_strategy_balance_after_invest,
    },
    "After deposit and invest": {
      "Idle funds a_0": idle_funds_after_deposit_and_invest[0].amount,
      "Idle funds a_1": idle_funds_after_deposit_and_invest[1].amount,
      "Invested funds a_0": invested_funds_after_deposit_and_invest[0].amount,
      "Invested funds a_1": invested_funds_after_deposit_and_invest[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_deposit_and_invest,
      "usdc strategy": fixed_usdc_strategy_balance_after_deposit_and_invest,
    },
    "After unwind": {
      "Idle funds a_0": idle_funds_after_unwind[0].amount,
      "Idle funds a_1": idle_funds_after_unwind[1].amount,
      "Invested funds a_0": invested_funds_after_unwind[0].amount,
      "Invested funds a_1": invested_funds_after_unwind[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_unwind,
      "usdc strategy": fixed_usdc_strategy_balance_after_unwind,
    },
    "After rebalance swap exact in": {
      "Idle funds a_0": idle_funds_after_rebalance_swap_e_in[0].amount,
      "Idle funds a_1": idle_funds_after_rebalance_swap_e_in[1].amount,
      "Invested funds a_0": invested_funds_after_rebalance_swap_e_in[0].amount,
      "Invested funds a_1": invested_funds_after_rebalance_swap_e_in[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_rebalance_swap_e_in,
      "usdc strategy": fixed_usdc_strategy_balance_after_rebalance_swap_e_in,
    },
    "After rebalance swap exact out": {
      "Idle funds a_0": idle_funds_after_rebalance_swap_e_out[0].amount,
      "Idle funds a_1": idle_funds_after_rebalance_swap_e_out[1].amount,
      "Invested funds a_0": invested_funds_after_rebalance_swap_e_out[0].amount,
      "Invested funds a_1": invested_funds_after_rebalance_swap_e_out[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_rebalance_swap_e_out,
      "usdc strategy": fixed_usdc_strategy_balance_after_rebalance_swap_e_out,
    },
    "After withdraw": {
      "Idle funds a_0": idle_funds_after_withdraw[0].amount,
      "Idle funds a_1": idle_funds_after_withdraw[1].amount,
      "Invested funds a_0": invested_funds_after_withdraw[0].amount,
      "Invested funds a_1": invested_funds_after_withdraw[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_withdraw,
      "usdc strategy": fixed_usdc_strategy_balance_after_withdraw,
    },
    "After rescue": {
      "Idle funds a_0": idle_funds_after_rescue[0].amount,
      "Idle funds a_1": idle_funds_after_rescue[1].amount,
      "Invested funds a_0": invested_funds_after_rescue[0].amount,
      "Invested funds a_1": invested_funds_after_rescue[1].amount,
      "xtar strategy": fixed_xtar_strategy_balance_after_rescue,
      "usdc strategy": fixed_usdc_strategy_balance_after_rescue,
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
    rebalance_swap_e_in: {
      status: !!rebalance_swap_e_in_instructions && !!rebalance_swap_e_in_read_bytes && !!rebalance_swap_e_in_write_bytes ? "success" : "failed",
      instructions: rebalance_swap_e_in_instructions,
      readBytes: rebalance_swap_e_in_read_bytes,
      writeBytes: rebalance_swap_e_in_write_bytes,
    },
    rebalance_swap_e_out: {
      status: !!rebalance_swap_e_out_instructions && !!rebalance_swap_e_out_read_bytes && !!rebalance_swap_e_out_write_bytes ? "success" : "failed",
      instructions: rebalance_swap_e_out_instructions,
      readBytes: rebalance_swap_e_out_read_bytes,
      writeBytes: rebalance_swap_e_out_write_bytes,
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
  }
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}