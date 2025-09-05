import { Address, Keypair } from "@stellar/stellar-sdk";
import { USDC_ADDRESS } from "../../constants.js";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";
import {
  depositToVault,
  fetchTotalManagedFunds,
  Instruction,
  manager,
  rebalanceVault,
  rescueFromStrategy,
  withdrawFromVault
} from "../../utils/vault.js";
import { green, purple, red, yellow } from "../common.js";
import { CreateVaultParams } from "../types.js";
import { compareTotalManagedFunds, deployDefindexVault, generateExpectedTotalAmounts, generateTotalAmountsError } from "./utils.js";
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
  const error_total_managed_funds = generateTotalAmountsError(params);
  
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

  const initial_total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

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
  const total_managed_funds_after_deposit = await fetchTotalManagedFunds(vault_address, user);

  //Invest
  const invest_amount_0 = 5_0_000_000;
  const invest_amount_1 = 10_0_000_000;
  const { 
    instructions: invest_instructions, 
    readBytes:invest_read_bytes, 
    writeBytes:invest_write_bytes,
    total_managed_funds: total_managed_funds_after_invest,
    result: invest_result,
    error: invest_error,
  } = await (
    async () => {
      try {
        console.log(purple, "---------------------------------------");
        console.log(purple, "Try Invest idle_funds*2");
        console.log(purple, "---------------------------------------");
        const invest_amount_0 = Number(total_managed_funds_after_deposit[0].idle_amount) * 2;
        const invest_amount_1 = Number(total_managed_funds_after_deposit[1].idle_amount) * 2;
        console.log(yellow, "Invest amount 0:", invest_amount_0);
        console.log(yellow, "Invest amount 1:", invest_amount_1);
        const investArgs: Instruction[] = [
          {
            type: "Invest",
            strategy: params[0].strategies[0].address,
            amount: BigInt(invest_amount_0),
          },
          {
            type: "Invest",
            strategy: params[1].strategies[0].address,
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
          strategy: params[0].strategies[0].address,
          amount: BigInt(invest_amount_0),
        },
        {
          type: "Invest",
          strategy: params[1].strategies[0].address,
          amount: BigInt(invest_amount_1),
        },
      ];

      try {
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          investArgs,
          manager
        );
        const expected_idle_funds = [
          [Number(total_managed_funds_after_deposit[0].idle_amount) - invest_amount_0], 
          [Number(total_managed_funds_after_deposit[1].idle_amount) - invest_amount_1]
        ]
        const expected_invested_funds = [
          [Number(total_managed_funds_after_deposit[0].invested_amount) + invest_amount_0], 
          [Number(total_managed_funds_after_deposit[1].invested_amount) + invest_amount_1]
        ];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds)
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
    error: deposit_and_invest_error,
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Deposit and invest in vault");
      console.log(purple, "---------------------------------------");
      const deposit_and_invest_amount_0: number = 2_5_000_000;
      const deposit_and_invest_amount_1: number = 5_0_000_000;
      
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_and_invest_amount_0, deposit_and_invest_amount_1], user, true);
        const expected_idle_funds = [
          [Number(total_managed_funds_after_invest[0].idle_amount)], 
          [Number(total_managed_funds_after_invest[1].idle_amount)]
        ];
        const expected_invested_funds = [
          [Number(total_managed_funds_after_invest[0].invested_amount) + deposit_and_invest_amount_0],
          [Number(total_managed_funds_after_invest[1].invested_amount) + deposit_and_invest_amount_1],
        ];

        console.log(expected_idle_funds);
        console.log(expected_invested_funds);
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds);
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        console.log(total_managed_funds);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);

        return { 
          instructions, 
          readBytes, 
          writeBytes,
          total_managed_funds,
          result
        };
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
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
    total_managed_funds: total_managed_funds_after_unwind,
    result: unwind_result,
    error: unwind_error,
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
              strategy: params[0].strategies[0].address,
              amount: BigInt(unwind_amount_0),
            },
            {
              type: "Unwind",
              strategy: params[1].strategies[0].address,
              amount: BigInt(unwind_amount_1),
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
              strategy: params[0].strategies[0].address,
              amount: BigInt(unwind_amount_0),
            },
            {
              type: "Unwind",
              strategy: params[1].strategies[0].address,
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
            strategy: params[0].strategies[0].address,
            amount: BigInt(unwind_amount_0),
          },
          {
            type: "Unwind",
            strategy: params[1].strategies[0].address,
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
        const expected_idle_funds = [
          [Number(total_managed_funds_after_deposit_and_invest[0].idle_amount) + unwind_amount_0], 
          [Number(total_managed_funds_after_deposit_and_invest[1].idle_amount) + unwind_amount_1]
        ];
        const expected_invested_funds = [
          [Number(total_managed_funds_after_deposit_and_invest[0].invested_amount) - unwind_amount_0], 
          [Number(total_managed_funds_after_deposit_and_invest[1].invested_amount) - unwind_amount_1]
        ];

        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds);
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);
        

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
    total_managed_funds: total_managed_funds_after_rebalance_swap_e_in,
    result: rebalance_swap_e_in_result,
    error: rebalance_swap_e_in_error,
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
              strategy: params[0].strategies[0].address,
              amount: BigInt(7_0_000),
            },
            {
              type: "Unwind",
              strategy: params[0].strategies[0].address,
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
        const unwind_amount = 1_0_000_000;
        const swapEIn_amount = 5_0_000_000;
        const invest_amount = 5_0_000_000;
        const rebalanceArgs: Instruction[] = [
        /*   {
            type: "Invest",
            strategy: params[0].strategies[0].address,
            amount: BigInt(invest_amount),
          },  */
          {
            type: "SwapExactIn",
            amount_in: BigInt(swapEIn_amount),
            amount_out_min: BigInt(0),
            token_in: params[0].address.toString(),
            token_out: params[1].address.toString(),
            deadline: BigInt(getCurrentTimePlusOneHour()),
          },
/*           {
            type: "Unwind",
            strategy: params[1].strategies[0].address,
            amount: BigInt(unwind_amount),
          },  */
        ];       
     
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );
        const expected_idle_funds = [
          [Number(total_managed_funds_after_unwind[0].idle_amount) - swapEIn_amount],
          [Number(total_managed_funds_after_unwind[1].idle_amount) + swapEIn_amount]
        ];
        const expected_invested_funds = [
          [Number(total_managed_funds_after_unwind[0].invested_amount) + swapEIn_amount],
          [Number(total_managed_funds_after_unwind[1].invested_amount) - swapEIn_amount]
        ];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds);
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);

        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result
        }
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e,
        };
      }
    }
  )();
  //rebalance with a unwind, swap exact out, invest 
  const { 
    instructions: rebalance_swap_e_out_instructions, 
    readBytes:rebalance_swap_e_out_read_bytes, 
    writeBytes:rebalance_swap_e_out_write_bytes,
    total_managed_funds: total_managed_funds_after_rebalance_swap_e_out,
    result: rebalance_swap_e_out_result,
    error: rebalance_swap_e_out_error,
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
              strategy: params[0].strategies[0].address,
              amount: BigInt(7_0_000),
            },
            {
              type: "Unwind",
              strategy: params[0].strategies[0].address,
              amount: BigInt(6_0_00),
            },
            {
              type: "SwapExactIn",
              amount_in: BigInt(1_0_000),
              amount_out_min: BigInt(0),
              token_in: params[0].address.toString(),
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
 /*          {
            type: "Unwind",
            strategy: params[0].strategies[0].address,
            amount: BigInt(5_000_000),
          },  */
          {
            type: "SwapExactOut",
            amount_out: BigInt(5_000_000),
            amount_in_max: BigInt(10_0_000_000),
            token_in: params[1].address.toString(),
            token_out: params[0].address.toString(),
            deadline: BigInt(getCurrentTimePlusOneHour()),
          },
 /*          {
            type: "Invest",
            strategy: params[1].strategies[0].address,
            amount: BigInt(1_0_000_000),
          }, */
        ];       
     
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );
        const expected_idle_funds = [
          [Number(total_managed_funds_after_rebalance_swap_e_in[0].idle_amount) + 5_0_000_000],
          [Number(total_managed_funds_after_rebalance_swap_e_in[1].idle_amount) - 5_0_000_000]
        ];
        const expected_invested_funds = [
          [Number(total_managed_funds_after_rebalance_swap_e_in[0].invested_amount) - 5_0_000_000],
          [Number(total_managed_funds_after_rebalance_swap_e_in[1].invested_amount) + 5_0_000_000]
        ];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds);
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);

        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);


        return {
          instructions,
          readBytes,
          writeBytes,
          total_managed_funds,
          result,
        }
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          error: e,
          total_managed_funds: error_total_managed_funds,
        };
      }
    }
  )();
  // withdraw from vault
  const { 
    instructions: withdraw_instructions, 
    readBytes: withdraw_read_bytes, 
    writeBytes: withdraw_write_bytes,
    total_managed_funds: total_managed_funds_after_withdraw,
    result: withdraw_result,
    
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

        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, [0,0], withdraw_amount, user);
        
        let expected_idle_funds = [
          [Number(total_managed_funds_after_rebalance_swap_e_out[0].idle_amount) + withdraw_amount], 
          [Number(total_managed_funds_after_rebalance_swap_e_out[1].idle_amount) + withdraw_amount]
        ];
        let expected_invested_funds = [
          [Number(total_managed_funds_after_rebalance_swap_e_out[0].invested_amount)],
          [Number(total_managed_funds_after_rebalance_swap_e_out[1].invested_amount)]
        ];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds);
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);
        
        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          total_managed_funds,
          result,
        };
      } catch (e) {
        console.error(red, e);
        return {
          withdraw_instructions: 0,
          withdraw_read_bytes: 0,
          withdraw_write_bytes: 0,
          total_managed_funds: error_total_managed_funds,
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
    total_managed_funds: total_managed_funds_after_rescue,
    result: rescue_result,
    error: rescue_error,
  } = await (
    async () => {
      try {
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
        const { instructions: asset_0_rescue_instructions, readBytes: asset_0_rescue_readBytes, writeBytes: asset_0_rescue_writeBytes} = await rescueFromStrategy(vault_address, params[0].strategies[0].address, manager);
        const { instructions: asset_1_rescue_instructions, readBytes: asset_1_rescue_readBytes, writeBytes: asset_1_rescue_writeBytes } = await rescueFromStrategy(vault_address, params[1].strategies[0].address, manager);
        const instructions = [asset_0_rescue_instructions, asset_1_rescue_instructions];
        const readBytes = [asset_0_rescue_readBytes, asset_1_rescue_readBytes];
        const writeBytes = [asset_0_rescue_writeBytes, asset_1_rescue_writeBytes];
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const expected_idle_funds = [
          [Number(0)],
          [Number(0)]
        ];
        const expected_invested_funds = [
          [Number(0)],
          [Number(0)]
        ];
        const expected_total_managed_funds = generateExpectedTotalAmounts(params, expected_idle_funds, expected_invested_funds);
        const result = compareTotalManagedFunds(expected_total_managed_funds, total_managed_funds, 1);
        
        return {
          instructions,
          readBytes,
          writeBytes,
          result,
          total_managed_funds
        }
      } catch (error:any) {
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: error,
      }
    }
  }   
  )();

  //Show data
  const tableData = {
    "Initial balance": {
      asset_0_idle: initial_total_managed_funds[0].idle_amount,
      asset_1_idle: initial_total_managed_funds[1].idle_amount,
      asset_0_invested: initial_total_managed_funds[0].invested_amount,
      asset_1_invested: initial_total_managed_funds[1].invested_amount,
    },
    "After deposit": {
      asset_0_idle: total_managed_funds_after_deposit[0].idle_amount,
      asset_1_idle: total_managed_funds_after_deposit[1].idle_amount,
      asset_0_invested: total_managed_funds_after_deposit[0].invested_amount,
      asset_1_invested: total_managed_funds_after_deposit[1].invested_amount,
    },
    "After invest": {
      asset_0_idle: total_managed_funds_after_invest[0].idle_amount,
      asset_1_idle: total_managed_funds_after_invest[1].idle_amount,
      asset_0_invested: total_managed_funds_after_invest[0].invested_amount,
      asset_1_invested: total_managed_funds_after_invest[1].invested_amount,
    },
    "After deposit and invest": {
      asset_0_idle: total_managed_funds_after_deposit_and_invest[0].idle_amount,
      asset_1_idle: total_managed_funds_after_deposit_and_invest[1].idle_amount,
      asset_0_invested: total_managed_funds_after_deposit_and_invest[0].invested_amount,
      asset_1_invested: total_managed_funds_after_deposit_and_invest[1].invested_amount,
    },
    "After unwind": {
      asset_0_idle: total_managed_funds_after_unwind[0].idle_amount,
      asset_1_idle: total_managed_funds_after_unwind[1].idle_amount,
      asset_0_invested: total_managed_funds_after_unwind[0].invested_amount,
      asset_1_invested: total_managed_funds_after_unwind[1].invested_amount,
    },
    "After rebalance swap exact in": {
      asset_0_idle: total_managed_funds_after_rebalance_swap_e_in[0].idle_amount,
      asset_1_idle: total_managed_funds_after_rebalance_swap_e_in[1].idle_amount,
      asset_0_invested: total_managed_funds_after_rebalance_swap_e_in[0].invested_amount,
      asset_1_invested: total_managed_funds_after_rebalance_swap_e_in[1].invested_amount,
    },
    "After rebalance swap exact out": {
      asset_0_idle: total_managed_funds_after_rebalance_swap_e_out[0].idle_amount,
      asset_1_idle: total_managed_funds_after_rebalance_swap_e_out[1].idle_amount,
      asset_0_invested: total_managed_funds_after_rebalance_swap_e_out[0].invested_amount,
      asset_1_invested: total_managed_funds_after_rebalance_swap_e_out[1].invested_amount,
    },
    "After withdraw": {
      asset_0_idle: total_managed_funds_after_withdraw[0].idle_amount,
      asset_1_idle: total_managed_funds_after_withdraw[1].idle_amount,
      asset_0_invested: total_managed_funds_after_withdraw[0].invested_amount,
      asset_1_invested: total_managed_funds_after_withdraw[1].invested_amount,
    },
    "After rescue": {
      asset_0_idle: total_managed_funds_after_rescue[0].idle_amount,
      asset_1_idle: total_managed_funds_after_rescue[1].idle_amount,
      asset_0_invested: total_managed_funds_after_rescue[0].invested_amount,
      asset_1_invested: total_managed_funds_after_rescue[1].invested_amount,
    }
  };
  const budgetData = {
    deploy: {
      status: deploy_instructions + deploy_read_bytes + deploy_write_bytes ? "success" : "failed",
      instructions: deploy_instructions,
      readBytes: deploy_read_bytes,
      writeBytes: deploy_write_bytes,
    },
    deposit: {
      status: deposit_instructions! + deposit_read_bytes! + deposit_write_bytes! ? "success" : "failed",
      instructions: deposit_instructions,
      readBytes: deposit_read_bytes,
      writeBytes: deposit_write_bytes,
    },
    invest: {
      status: invest_instructions + invest_read_bytes + invest_write_bytes ? "success" : "failed",
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    deposit_and_invest: {
      status: deposit_and_invest_instructions + deposit_and_invest_read_bytes + deposit_and_invest_write_bytes ? "success" : "failed",
      instructions: deposit_and_invest_instructions,
      readBytes: deposit_and_invest_read_bytes,
      writeBytes: deposit_and_invest_write_bytes,
    },
    unwind: {
      status: unwind_instructions + unwind_read_bytes + unwind_write_bytes ? "success" : "failed",
      instructions: unwind_instructions,
      readBytes: unwind_read_bytes,
      writeBytes: unwind_write_bytes,
    },
    rebalance_swap_e_in: {
      status: rebalance_swap_e_in_instructions + rebalance_swap_e_in_read_bytes + rebalance_swap_e_in_write_bytes ? "success" : "failed",
      instructions: rebalance_swap_e_in_instructions,
      readBytes: rebalance_swap_e_in_read_bytes,
      writeBytes: rebalance_swap_e_in_write_bytes,
    },
    rebalance_swap_e_out: {
      status: rebalance_swap_e_out_instructions + rebalance_swap_e_out_read_bytes + rebalance_swap_e_out_write_bytes ? "success" : "failed",
      instructions: rebalance_swap_e_out_instructions,
      readBytes: rebalance_swap_e_out_read_bytes,
      writeBytes: rebalance_swap_e_out_write_bytes,
    },
    withdraw: {
      status: withdraw_instructions! + withdraw_read_bytes! + withdraw_write_bytes! ? "success" : "failed",
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
    rescue: {
      status: Number(rescue_instructions) + Number(rescue_read_bytes) + Number(rescue_write_bytes) > 0 ? "success" : "failed",
      instructions: rescue_instructions,
      readBytes: rescue_read_bytes,
      writeBytes: rescue_write_bytes,
    },
  }
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}