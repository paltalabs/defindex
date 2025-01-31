import { Address, Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import {
  CreateVaultParams,
  depositToVault,
  Instruction,
  manager,
  rebalanceVault,
  rescueFromStrategy,
  withdrawFromVault
} from "../vault.js";
import { green, purple, red, usdcAddress, yellow } from "../common.js";
import { airdropAccount } from "../../utils/contract.js";
import { deployDefindexVault, fetchBalances } from "./utils.js";
import { getCurrentTimePlusOneHour } from "../../utils/tx.js";
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
            strategy: addressBook.getContractId("fixed_apr_strategy"),
            amount: BigInt(invest_amount_0),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("blend_strategy"),
            amount: BigInt(invest_amount_0),
          },
          {
            type: "Invest",
            strategy: addressBook.getContractId("hodl_usdc_strategy"),
            amount: BigInt(invest_amount_1),
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
          strategy: addressBook.getContractId("blend_strategy"),
          amount: BigInt(invest_amount_0),
        },
        {
          type: "Invest",
          strategy: addressBook.getContractId("fixed_apr_strategy"),
          amount: BigInt(invest_amount_0),
        },
        {
          type: "Invest",
          strategy: addressBook.getContractId("fixed_usdc_strategy"),
          amount: BigInt(invest_amount_1),
        },
        {
          type: "Invest",
          strategy: addressBook.getContractId("hodl_usdc_strategy"),
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

        const tolerance = BigInt(1_000);
        const expected_idle_funds = [BigInt(0), BigInt(0)];
        const expected_invested_funds = [BigInt(20_0_000_000), BigInt(10_0_000_000)];

        if(BigInt(idle_funds_after_invest[0].amount) !== expected_idle_funds[0]) {
          console.error(red, `idle funds: ${idle_funds_after_invest[0].amount} !== ${expected_idle_funds[0]}`);
          throw Error("Idle 0 funds after invest failed");
        }

        if(idle_funds_after_invest[1].amount !== expected_idle_funds[1]) {
          console.error(red, `idle funds: ${idle_funds_after_invest[1].amount} !== ${expected_idle_funds[1]}`);
          throw Error("Idle 1 funds after invest failed");
        }

        if(
          BigInt(invested_funds_after_invest[0].amount) > expected_invested_funds[0] + tolerance || 
          BigInt(invested_funds_after_invest[0].amount) < expected_invested_funds[0] - tolerance
        ) {
          console.error(red, `invested funds: ${invested_funds_after_invest[0].amount} !== approx ${expected_invested_funds[0]}`);
          throw Error("Invested 0 funds after invest failed");
        }

        if(
          BigInt(invested_funds_after_invest[1].amount) > expected_invested_funds[1] + tolerance || 
          BigInt(invested_funds_after_invest[1].amount) < expected_invested_funds[1] - tolerance
        ) {
          console.error(red, `invested funds: ${invested_funds_after_invest[1].amount} !== ${expected_invested_funds[1]}`);
          throw Error("Invested 1 funds after invest failed");
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
          idle_funds_after_invest:[{ amount: BigInt(0) }, { amount: BigInt(0) }], 
          invested_funds_after_invest:[{ amount: BigInt(0) }, { amount: BigInt(0) }], 
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
      const deposit_and_invest_amount_0: number = 1_0_000_000;
      const deposit_and_invest_amount_1: number = 2_0_000_000;
      
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
        const tolerance = BigInt(1_000);
        const expected_idle_funds = [BigInt(0), BigInt(0)];
        const expected_invested_funds = [BigInt(invested_funds_after_invest[0].amount) + BigInt(deposit_and_invest_amount_1), BigInt(invested_funds_after_invest[1].amount) + BigInt(deposit_and_invest_amount_0)];

        if(idle_funds_after_deposit_and_invest[0].amount !== expected_idle_funds[0]) {
          console.error(red, `idle funds: ${idle_funds_after_deposit_and_invest[0].amount} !== ${expected_idle_funds[0]}`);
          throw Error("Idle 0 funds after deposit and invest failed");
        }

        if(idle_funds_after_deposit_and_invest[1].amount !== expected_idle_funds[1]) {
          console.error(red, `idle funds: ${idle_funds_after_deposit_and_invest[1].amount} !== ${expected_idle_funds[1]}`);
          throw Error("Idle 1 funds after deposit and invest failed");
        }

        if (
          BigInt(invested_funds_after_deposit_and_invest[0].amount) > expected_invested_funds[0] + tolerance ||
          BigInt(invested_funds_after_deposit_and_invest[0].amount) < expected_invested_funds[0] - tolerance
        ) {
          console.error(red, `invested funds: ${invested_funds_after_deposit_and_invest[0].amount} !== approx ${expected_invested_funds[0]}`);
          throw Error("Invested 0 funds after deposit and invest failed");
        }

        if (
          BigInt(invested_funds_after_deposit_and_invest[1].amount) > expected_invested_funds[1] + tolerance ||
          BigInt(invested_funds_after_deposit_and_invest[1].amount) < expected_invested_funds[1] - tolerance
        ) {
          console.error(red, `invested funds: ${invested_funds_after_deposit_and_invest[1].amount} !== approx ${expected_invested_funds[1]}`);
          throw Error("Invested 1 funds after deposit and invest failed");
        }


        return { 
          instructions, 
          readBytes, 
          writeBytes,
          idle_funds_after_deposit_and_invest,
          invested_funds_after_deposit_and_invest,
        };
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          idle_funds_after_deposit_and_invest: [{ amount: BigInt(0) }],
          invested_funds_after_deposit_and_invest: [{ amount: BigInt(0) }],
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
    idle_funds: idle_funds_after_rebalance_swap_e_in,
    invested_funds: invested_funds_after_rebalance_swap_e_in,
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
              strategy: addressBook.getContractId("fixed_usdc_strategy"),
              amount: BigInt(5_000_000),
            }, 
            {
              type: "Invest",
              strategy: addressBook.getContractId("hodl_usdc_strategy"),
              amount: BigInt(2_500_000),
            },
            {
              type: "Unwind",
              strategy: addressBook.getContractId("fixed_apr_strategy"),
              amount: BigInt(5_0_000_000),
            }, 
            {
              type: "Invest",
              strategy: addressBook.getContractId("blend_strategy"),
              amount: BigInt(2_5_000_000),
            },
            {
              type: "SwapExactIn",
              amount_in: BigInt(2_500_000),
              amount_out_min: BigInt(0),
              token_in: usdcAddress.toString(),
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

          
        } catch (error) {
          console.error(red, error);
          console.log(yellow, "-------------------------------------------------");
          console.log(yellow, "| we should handle this error better |");
          console.log(yellow, "-------------------------------------------------");
        }
        console.log(purple, "---------------------------------------");
        console.log(purple, "Rebalance swap exact in"); 
        console.log(purple, "---------------------------------------");

        const rebalanceArgs: Instruction[] = [
          {
            type: "Unwind",
            strategy: addressBook.getContractId("fixed_usdc_strategy"),
            amount: BigInt(5_000_000),
          }, 
          {
            type: "SwapExactIn",
            amount_in: BigInt(2_500_000),
            amount_out_min: BigInt(0),
            token_in: usdcAddress.toString(),
            token_out: xlmAddress.toString(),
            deadline: BigInt(getCurrentTimePlusOneHour()),
          },       
          {
            type: "Invest",
            strategy: addressBook.getContractId("hodl_usdc_strategy"),
            amount: BigInt(2_500_000),
          },
/*           {
            type: "Invest",
            strategy: addressBook.getContractId("blend_strategy"),
            amount: BigInt(2_5_000_000),
          }, */
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

        const tolerance = BigInt(1_000);
  

        return {
          instructions,
          readBytes,
          writeBytes,
          idle_funds,
          invested_funds,
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
        } = await fetchBalances(addressBook, vault_address, params, user);
        
        return { 
          instructions, 
          readBytes, 
          writeBytes, 
          idle_funds, 
          invested_funds,
        };
      } catch (e) {
        console.error(red, e);
        return {
          withdraw_instructions: 0,
          withdraw_read_bytes: 0,
          withdraw_write_bytes: 0,
          idle_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          invested_funds: [{ amount: BigInt(0) }, { amount: BigInt(0) }],
          error: e,
        };
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
    },
    "After deposit and invest": {
      "Idle funds a_0": idle_funds_after_deposit_and_invest[0].amount,
      "Idle funds a_1": idle_funds_after_deposit_and_invest[1].amount,
      "Invested funds a_0": invested_funds_after_deposit_and_invest[0].amount,
      "Invested funds a_1": invested_funds_after_deposit_and_invest[1].amount,
    },
    "After rebalance swap exact in": {
      "Idle funds a_0": idle_funds_after_rebalance_swap_e_in[0].amount,
      "Idle funds a_1": idle_funds_after_rebalance_swap_e_in[1].amount,
      "Invested funds a_0": invested_funds_after_rebalance_swap_e_in[0].amount,
      "Invested funds a_1": invested_funds_after_rebalance_swap_e_in[1].amount,
    },
    "After withdraw": {
      "Idle funds a_0": idle_funds_after_withdraw[0].amount,
      "Idle funds a_1": idle_funds_after_withdraw[1].amount,
      "Invested funds a_0": invested_funds_after_withdraw[0].amount,
      "Invested funds a_1": invested_funds_after_withdraw[1].amount,
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
    rebalance_swap_e_in: {
      status: !!rebalance_swap_e_in_instructions && !!rebalance_swap_e_in_read_bytes && !!rebalance_swap_e_in_write_bytes ? "success" : "failed",
      instructions: rebalance_swap_e_in_instructions,
      readBytes: rebalance_swap_e_in_read_bytes,
      writeBytes: rebalance_swap_e_in_write_bytes,
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