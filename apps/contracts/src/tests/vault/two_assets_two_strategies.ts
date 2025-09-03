import { Address, Keypair } from "@stellar/stellar-sdk";
import { BLEND_USDC_ADDRESS, USDC_ADDRESS } from "../../constants.js";
import { AddressBook } from "../../utils/address_book.js";
import { airdropAccount } from "../../utils/contract.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";
import {
  depositToVault,
  fetchTotalManagedFunds,
  Instruction,
  manager,
  rebalanceVault,
  withdrawFromVault
} from "../../utils/vault.js";
import { purple, red, yellow } from "../common.js";
import { CreateVaultParams } from "../types.js";
import { compareTotalManagedFunds, deployDefindexVault, generateExpectedTotalAmounts, generateTotalAmountsError } from "./utils.js";
/* 
### Two assets two strategies tests:
- [x] deposit
- [x] invest
- [x] deposit and invest
- [x] Make a list of instructions so big that we reach Soroban limits
- [ ] Downsize the list to the maximum size Soroban allows (search for the maximum size)
- [x] rebalance with unwind, swap exact in, invest, invest
- [x] withdraw more than idle
*/
export async function testVaultTwoAssetsTwoStrategies(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair, xlmAddress:Address,) {
  const error_total_managed_funds = generateTotalAmountsError(params);
  
  console.log(yellow, "-------------------------------------------");
  console.log(yellow, "Testing two assets two strategies vault    ");
  console.log(yellow, "-------------------------------------------");

  //Deploy vault
  const { 
    address:vault_address, 
    deploy_instructions, 
    deploy_read_bytes,
    deploy_write_bytes 

  } = await deployDefindexVault(addressBook, params);
  if (!vault_address) throw new Error("Vault was not deployed");

  // Deposit to vault
  const deposit_amount_0 = 10_0_000_000;
  const deposit_amount_1 = 20_0_000_000;
  const {
    instructions:deposit_instructions, 
    readBytes:deposit_read_bytes, 
    writeBytes:deposit_write_bytes,
    result: deposit_result,
    total_managed_funds: total_managed_funds_after_deposit,
    error: deposit_error,   
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
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const expected_idle_funds = [[Number(deposit_amount_0), Number(0)], [Number(deposit_amount_1), Number(0)]];
        const expected_invested_funds = [[Number(0), Number(0)], [Number(0), Number(0)]];
        const expected_total_managed_funds = generateExpectedTotalAmounts(
          params,
          expected_idle_funds,
          expected_invested_funds
        )
        const result = compareTotalManagedFunds(
          expected_total_managed_funds,
          total_managed_funds,
        );
        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          result,
          total_managed_funds 
        };
      } catch (e) {
        console.error(red, e);
        return {
          deposit_instructions: 0,
          deposit_read_bytes: 0,
          deposit_write_bytes: 0,
          total_managed_funds: error_total_managed_funds,
          error: e,
        };
      }
    }
  )();


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
      await (async () => {
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
              strategy: params[0].strategies[1].address,
              amount: BigInt(invest_amount_0),
            },
            {
              type: "Invest",
              strategy: params[0].strategies[0].address,
              amount: BigInt(invest_amount_0),
            },
            {
              type: "Invest",
              strategy: params[1].strategies[1].address,
              amount: BigInt(invest_amount_1),
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
          console.error(red, error);
          console.log(red, "-----------------------------------------------------");
          console.log(red, "| Investing more than idle funds failed as expected |");
          console.log(red, "-----------------------------------------------------");
        }
      })();
      
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
          strategy: params[0].strategies[1].address,
          amount: BigInt(invest_amount_0),
        },
        {
          type: "Invest",
          strategy: params[1].strategies[0].address,
          amount: BigInt(invest_amount_1),
        },
        {
          type: "Invest",
          strategy: params[1].strategies[1].address,
          amount: BigInt(invest_amount_1),
        },
      ];

      try {
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          investArgs,
          manager
        );
        const expected_idle_funds = [[Number(0), Number(0)],[Number(0), Number(0)]]; 
        const expected_invested_funds =  [[Number(invest_amount_0), Number(invest_amount_0)],[Number(invest_amount_1), Number(invest_amount_1)]];
        const expected_total_managed_funds = generateExpectedTotalAmounts(
          params,
          expected_idle_funds,
          expected_invested_funds
        )
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(
          expected_total_managed_funds,
          total_managed_funds,
          1
        );

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
          result: null,
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          total_managed_funds: error_total_managed_funds,
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
    total_managed_funds: total_managed_funds_after_deposit_and_invest,
    result: deposit_and_invest_result,
    error: deposit_and_invest_error,
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Deposit and invest in vault");
      console.log(purple, "---------------------------------------");
      const deposit_and_invest_amount_0: number = 1_0_000_000;
      const deposit_and_invest_amount_1: number = 2_0_000_000;
      
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await depositToVault(vault_address, [deposit_and_invest_amount_0, deposit_and_invest_amount_1], user, true);
        
        const expected_idle_funds = [[Number(0), Number(0)],[Number(0), Number(0)]];
        const expected_invested_funds = [[Number(total_managed_funds_after_invest[0].invested_amount) + deposit_and_invest_amount_0, Number(total_managed_funds_after_invest[0].invested_amount)],[Number(total_managed_funds_after_invest[1].invested_amount)  + deposit_and_invest_amount_1, Number(total_managed_funds_after_invest[1].invested_amount)]];
        const expected_total_managed_funds = generateExpectedTotalAmounts(
          params,
          expected_idle_funds,
          expected_invested_funds
        )
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(
          expected_total_managed_funds,
          total_managed_funds,
          1
        );
        return { 
          instructions, 
          readBytes, 
          writeBytes,
          total_managed_funds,
          result,
        };
      } catch (e) {
        console.error(red, e);
        console.error(red, "----------------------- Deposit and invest failed -----------------------");
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
  //rebalance with a unwind, swap exact in, invest, invest
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
        try {
          console.log(purple, "---------------------------------------");
          console.log(purple, "Rebalance exceed intructions limit"); 
          console.log(purple, "---------------------------------------");
  
          const rebalanceArgs: Instruction[] = [
            {
              type: "Unwind",
              strategy: params[1].strategies[0].address,
              amount: BigInt(1_000_000),
            }, 
            {
              type: "Invest",
              strategy: params[1].strategies[1].address,
              amount: BigInt(1_000_000),
            },
            {
              type: "Unwind",
              strategy: params[0].strategies[1].address,
              amount: BigInt(1_000_000),
            }, 
            {
              type: "Invest",
              strategy: params[0].strategies[0].address,
              amount: BigInt(1_000_000),
            },
            {
              type: "SwapExactIn",
              amount_in: BigInt(500_000),
              amount_out_min: BigInt(0),
              token_in: USDC_ADDRESS.toString(),
              token_out: xlmAddress.toString(),
              deadline: BigInt(getCurrentTimePlusOneHour()),
            },       
          ];       
       
          const {instructions, readBytes, writeBytes} = await rebalanceVault(
            vault_address,
            rebalanceArgs,
            manager
          );

          console.log(instructions, readBytes, writeBytes);

          
        } catch (error: any) {
          console.error(red, error);
          console.log(red, "---------------------------------------------------------");
          console.log(red, "| Rebalance exceed instructions limit failed as expected |");
          console.log(red, "---------------------------------------------------------");
        }
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rebalance swap exact in"); 
        console.log(purple, "---------------------------------------");

        const rebalanceArgs: Instruction[] = [
/*           {
            type: "Unwind",
            strategy: params[1].strategies[0].address,
            amount: BigInt(5_000_000),
          },  */
          {
            type: "SwapExactIn",
            amount_in: BigInt(1_000),
            amount_out_min: BigInt(0),
            token_in: BLEND_USDC_ADDRESS.toString(),
            token_out: xlmAddress.toString(),
            deadline: BigInt(getCurrentTimePlusOneHour()),
          },       
/*           {
            type: "Invest",
            strategy: params[1].strategies[1].address,
            amount: BigInt(2_500_000),
          }, */
        ];       
     
        const {instructions, readBytes, writeBytes} = await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );

        const expected_idle_funds = [[Number(0), Number(0)],[Number(0), Number(0)]];
        const expected_invested_funds = [[Number(0), Number(0)],[Number(0), Number(0)]];
        const expected_total_managed_funds = generateExpectedTotalAmounts(
          params,
          expected_idle_funds,
          expected_invested_funds
        )
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(
          expected_total_managed_funds,
          total_managed_funds
        );

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
          total_managed_funds: error_total_managed_funds,
          error: e,
        };
      }
    }
  )();

  // withdraw from vault
  const { 
    total_managed_funds: withdraw_total_managed_funds,
    instructions: withdraw_instructions, 
    readBytes: withdraw_read_bytes, 
    writeBytes: withdraw_write_bytes,
  } = await (
    async () => {
      let withdraw_amount = 1_0_000_000;
      console.log(purple, "----------------------------------------------");
      console.log(purple,`Withdraw ${withdraw_amount} from two strategies`);
      console.log(purple, "----------------------------------------------");
      try {
        //Try withdraw from unauthorized
        await (async () => {
          try {
            console.log(purple, "---------------------------------------");
            console.log(purple, "Try withdraw from unauthorized");
            console.log(purple, "---------------------------------------");
            const withdraw_amount = 65_0_000;
            const random_user = Keypair.random();
            await airdropAccount(random_user);

            await withdrawFromVault(vault_address, [0,0], withdraw_amount, random_user);
            
          } catch (error:any) {
            console.error(red, error);
            console.log(red, "-------------------------------------------------");
            console.log(red, "| Withdraw from unauthorized failed as expected |");
            console.log(red, "-------------------------------------------------");
          }
        })();
        //Try withdraw more than total funds
        await (async () => {
          try {
            console.log(purple, "-----------------------------------------------------");
            console.log(purple, "Try withdraw more than total funds");
            console.log(purple, "-----------------------------------------------------");

            await withdrawFromVault(vault_address, [0,0], 100_0_000_000, user);

          } catch (error:any) {
            console.error(red, error);
            console.log(red, "-----------------------------------------------------");
            console.log(red, "| Withdraw more than total funds failed as expected |");
            console.log(red, "-----------------------------------------------------");
          }
        })();
        //Withdraw

        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, [0,0], withdraw_amount, user);
        
        const expected_idle_funds = [[Number(withdraw_amount), Number(0)],[Number(withdraw_amount), Number(0)]];
        const expected_invested_funds = [[Number(0), Number(0)],[Number(0), Number(0)]];
        const expected_total_managed_funds = generateExpectedTotalAmounts(
          params,
          expected_idle_funds,
          expected_invested_funds
        )
        const total_managed_funds = await fetchTotalManagedFunds(vault_address, user);
        const result = compareTotalManagedFunds(
          expected_total_managed_funds,
          total_managed_funds
        );
      
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
          total_managed_funds: error_total_managed_funds,
          error: e,
        };
      }
    }
  )();


  //Show data
  const tableData = {
    "Initial balance": {
      "Idle funds a_0": 0n,
      "Idle funds a_1": 0n,
      "Invested funds a_0": 0n,
      "Invested funds a_1": 0n,
    },
    "After deposit": {
      "Idle funds a_0": total_managed_funds_after_deposit[0].idle_amount,
      "Idle funds a_1": total_managed_funds_after_deposit[1].idle_amount,
      "Invested funds a_0": total_managed_funds_after_deposit[0].invested_amount,
      "Invested funds a_1": total_managed_funds_after_deposit[1].invested_amount,
    },
    "After invest": {
      "Idle funds a_0": total_managed_funds_after_invest[0].idle_amount,
      "Idle funds a_1": total_managed_funds_after_invest[1].idle_amount,
      "Invested funds a_0": total_managed_funds_after_invest[0].invested_amount,
      "Invested funds a_1": total_managed_funds_after_invest[1].invested_amount,
    },
    "After deposit and invest": {
      "Idle funds a_0": total_managed_funds_after_deposit_and_invest[0].idle_amount,
      "Idle funds a_1": total_managed_funds_after_deposit_and_invest[1].idle_amount,
      "Invested funds a_0": total_managed_funds_after_deposit_and_invest[0].invested_amount,
      "Invested funds a_1": total_managed_funds_after_deposit_and_invest[1].invested_amount,
    },
    "After rebalance swap exact in": {
      "Idle funds a_0": total_managed_funds_after_rebalance_swap_e_in[0].idle_amount,
      "Idle funds a_1": total_managed_funds_after_rebalance_swap_e_in[1].idle_amount,
      "Invested funds a_0": total_managed_funds_after_rebalance_swap_e_in[0].invested_amount,
      "Invested funds a_1": total_managed_funds_after_rebalance_swap_e_in[1].invested_amount,
    },
    "After withdraw": {
      "Idle funds a_0": withdraw_total_managed_funds[0].idle_amount,
      "Idle funds a_1": withdraw_total_managed_funds[1].idle_amount,
      "Invested funds a_0": withdraw_total_managed_funds[0].invested_amount,
      "Invested funds a_1": withdraw_total_managed_funds[1].invested_amount,
    },
  };
  const budgetData = {
    deploy: {
      status: deploy_instructions + deploy_read_bytes + deploy_write_bytes > 0 ? "success" : "failed",
      instructions: deploy_instructions,
      readBytes: deploy_read_bytes,
      writeBytes: deploy_write_bytes,
    },
    deposit: {
      status: deposit_instructions! + deposit_read_bytes! + deposit_write_bytes! > 0 ? "success" : "failed",
      instructions: deposit_instructions,
      readBytes: deposit_read_bytes,
      writeBytes: deposit_write_bytes,
    },
    invest: {
      status: invest_instructions + invest_read_bytes + invest_write_bytes > 0 ? "success" : "failed",
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    deposit_and_invest: {
      status: deposit_and_invest_instructions + deposit_and_invest_read_bytes + deposit_and_invest_write_bytes > 0 ? "success" : "failed",
      instructions: deposit_and_invest_instructions,
      readBytes: deposit_and_invest_read_bytes,
      writeBytes: deposit_and_invest_write_bytes,
    },
    rebalance_swap_e_in: {
      status: rebalance_swap_e_in_instructions + rebalance_swap_e_in_read_bytes + rebalance_swap_e_in_write_bytes > 0 ? "success" : "failed",
      instructions: rebalance_swap_e_in_instructions,
      readBytes: rebalance_swap_e_in_read_bytes,
      writeBytes: rebalance_swap_e_in_write_bytes,
    },
    withdraw: {
      status: withdraw_instructions! + withdraw_read_bytes! + withdraw_write_bytes! > 0 ? "success" : "failed",
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
  }
  console.table(tableData);
  console.table(budgetData);
  return {tableData, budgetData};
}