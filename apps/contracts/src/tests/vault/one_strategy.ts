import { Keypair } from "@stellar/stellar-sdk";
import { AddressBook } from "../../utils/address_book.js";
import { checkUserBalance } from "../strategy.js";
import {
  CreateVaultParams,
  deployVault,
  depositToVault,
  fetchCurrentInvestedFunds,
  fetchParsedCurrentIdleFunds,
  Instruction,
  manager,
  rebalanceVault,
  withdrawFromVault
} from "../vault.js";
import { green, purple, red, yellow } from "../common.js";

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

  - [ ] try withdraw from unauthorized
  - [ ] try withdraw more than total funds
  - [x] withdraw
  - [x] check balance

  - [ ] pause strategy
  - [ ] try rescue from unauthorized
  - [ ] try rescue from unauthorized

  - [ ] try upgrade from unauthorized
  - [ ] upgrade
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

  } = await (async () => {
    try {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Deploying vault with one strategy");
      console.log(purple, "---------------------------------------");
      const { 
        address: vault_address, 
        instructions:deploy_instructions, 
        readBytes:deploy_read_bytes, 
        writeBytes:deploy_write_bytes} = await deployVault(
        addressBook,
        params,
        "TestVault",
        "TSTV"
      );
      console.log(vault_address);
      return {address: vault_address, deploy_instructions, deploy_read_bytes, deploy_write_bytes};
    } catch (error) {
      console.error(red, error);
      return {
        address: null, 
        deploy_instructions:0, 
        deploy_read_bytes: 0, 
        deploy_write_bytes: 0,
        error
      };
    }
  })();
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
  //To-Do: Return error handling status
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
  //  To-do: Return error handling status
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
    writeBytes:rebalance_write_bytes 
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
     
        return await rebalanceVault(
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
     
        return await rebalanceVault(
          vault_address,
          rebalanceArgs,
          manager
        );
      } catch (e) {
        console.error(red, e);
        return {
          instructions: 0,
          readBytes: 0,
          writeBytes: 0,
          error: e,
        };
      }
    }
  )();
  const {
    idle_funds:idle_funds_after_rebalance, 
    invested_funds:invested_funds_after_rebalance, 
    hodl_balance:hodl_balance_after_rebalance
  } = await fetchBalances(addressBook, vault_address, user);

  // withdraw from vault
  const { 
    instructions:withdraw_instructions, 
    readBytes:withdraw_read_bytes, 
    writeBytes:withdraw_write_bytes 
  } = await (
    async () => {
      console.log(purple, "---------------------------------------");
      console.log(purple, "Withdraw 65_0_000 from one strategy");
      console.log(purple, "---------------------------------------");
      try {
        const {
          instructions,
          readBytes,
          writeBytes,
        } = await withdrawFromVault(vault_address, 65_0_000, user);
        return { instructions, readBytes, writeBytes };
      } catch (e) {
        console.error(red, e);
        return {
          withdraw_instructions: 0,
          withdraw_read_bytes: 0,
          withdraw_write_bytes: 0,
          error: e,
        };
      }
    }
  )();
  const { 
    idle_funds:idle_funds_after_withdraw, 
    invested_funds:invested_funds_after_withdraw, 
    hodl_balance:hodl_balance_after_withdraw 
  } = await fetchBalances(addressBook, vault_address, user);

  //Pause strategy

  //Rescue funds

  //Show data
  const tableData = {
    hodlStrategy: {
      "Balance before deposit": hodl_balance_before_deposit,
      "Balance after deposit": hodl_balance_after_deposit,
      "Balance after invest": hodl_balance_after_invest,
      "Balance after deposit and invest": hodl_balance_after_deposit_and_invest,
      "Balance after unwind": hodl_balance_after_unwind,
      "Balance after rebalance": hodl_balance_after_rebalance,
      "Balance after withdraw": hodl_balance_after_withdraw,
    },
    "Invested funds": {
      "Balance before deposit": invested_funds_before_deposit[0].amount,
      "Balance after deposit": invested_funds_after_deposit[0].amount,
      "Balance after invest": invested_funds_after_invest[0].amount,
      "Balance after deposit and invest": invested_funds_after_deposit_and_invest[0].amount,
      "Balance after unwind": invested_funds_after_unwind[0].amount,
      "Balance after rebalance": invested_funds_after_rebalance[0].amount,
      "Balance after withdraw": invested_funds_after_withdraw[0].amount,
    },
    "Idle funds": {
      "Balance before deposit": idle_funds_before_deposit[0].amount,
      "Balance after deposit": idle_funds_after_deposit[0].amount,
      "Balance after invest": idle_funds_after_invest[0].amount,
      "Balance after deposit and invest": idle_funds_after_deposit_and_invest[0].amount,
      "Balance after unwind": idle_funds_after_unwind[0].amount,
      "Balance after rebalance": idle_funds_after_rebalance[0].amount,
      "Balance after withdraw": idle_funds_after_withdraw[0].amount,
    },
  };
  console.table(tableData);

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
    deposit_and_invest: {
      instructions: deposit_and_invest_instructions,
      readBytes: deposit_and_invest_read_bytes,
      writeBytes: deposit_and_invest_write_bytes,
    },
    withdraw: {
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
    invest: {
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    rebalance: {
      instructions: rebalance_instructions,
      readBytes: rebalance_read_bytes,
      writeBytes: rebalance_write_bytes,
    },
  }
  console.table(budgetData);
  return {tableData, budgetData};
}

/* 
// Access control tests:
  - [ ] try setManager without previous queued manager
  - [ ] try queueManager from unauthorized
  - [ ] queueManager
  - [ ] try setManager before time is up
  - [ ] try setManager from unauthorized
  - [ ] setManager

  - [ ] try setRebalanceManager from unauthorized
  - [ ] setRebalanceManager

  - [ ] try setFeeReceiver from unauthorized
  - [ ] setFeeReceiver

  - [ ] try setEmergencyManager from unauthorized
  - [ ] setEmergencyManager

*/

/* 
// Upgrade tests:
  - [ ] try upgrade from unauthorized
  - [ ] upgrade
*/
export async function testVaultOneStrategy(addressBook: AddressBook, params: CreateVaultParams[], user: Keypair) {
  const {tableData, budgetData} = await successFlow(addressBook, params, user);
  return {tableData, budgetData};
  /* console.log(yellow, "---------------------------------------");
  console.log(yellow, "Running one strategy vault tests");
  console.log(yellow, "---------------------------------------");

  // deploy vault
  console.log(purple, "---------------------------------------");
  console.log(purple, "Deploying vault with one strategy");
  console.log(purple, "---------------------------------------");
  const { 
    address: vault_address, 
    instructions:deploy_instructions, 
    readBytes:deploy_read_bytes, 
    writeBytes:deploy_write_bytes} = await deployVault(
    addressBook,
    params,
    "TestVault",
    "TSTV"
  );
  console.log(vault_address)

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idle_funds_before_deposit = await fetchParsedCurrentIdleFunds(
    vault_address,
    user
  );
  const invested_funds_before_deposit = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance_before_deposit = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  // deposit to vault

  const {
    instructions: deposit_instructions,
    readBytes:deposit_read_bytes,
    writeBytes:deposit_write_bytes,
  } = await depositToVault(vault_address, [987654321], user);

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idle_funds_after_deposit = await fetchParsedCurrentIdleFunds(
    vault_address,
    user
  );
  const invested_funds_after_deposit = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance_after_deposit = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  // withdraw from vault
  const {
    instructions: withdraw_instructions,
    readBytes:withdraw_read_bytes,
    writeBytes:withdraw_write_bytes,
  } = await withdrawFromVault(vault_address, 65_0_000, user);

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");

  const idle_funds_after_withdraw = await fetchParsedCurrentIdleFunds(
    vault_address,
    user
  );
  const invested_funds_after_withdraw = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance_after_withdraw = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  // invest in vault
  console.log(purple, "---------------------------------------");
  console.log(purple, "Investing in vault");
  console.log(purple, "---------------------------------------");

  const investArgs: Instruction[] = [
    {
      type: "Invest",
      strategy: addressBook.getContractId("hodl_strategy"),
      amount: BigInt(43_0_0),
    },
  ];

  const { 
    result: investResult,
    instructions: invest_instructions,
    readBytes:invest_read_bytes,
    writeBytes:invest_write_bytes
  } = await rebalanceVault(
    vault_address,
    investArgs,
    manager
  );
  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");
  const idle_funds_after_invest = await fetchParsedCurrentIdleFunds(
    vault_address,
    user
  );
  const invested_funds_after_invest = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance_after_invest = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  // rebalance vault

  console.log(purple, "---------------------------------------");
  console.log(purple, "Rebalancing vault"); 
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

  const mappedParams = mapInstructionsToParams(rebalanceArgs);

  const { 
    result: rebalanceResult,
    instructions: rebalance_instructions,
    readBytes:rebalance_read_bytes,
    writeBytes:rebalance_write_bytes
  } = await rebalanceVault(
    vault_address,
    investArgs,
    manager
  );

  console.log(yellow, "---------------------------------------");
  console.log(yellow, "Fetching balances");
  console.log(yellow, "---------------------------------------");
  const idle_funds_after_rebalance = await fetchParsedCurrentIdleFunds(
    vault_address,
    user
  );
  const invested_funds_after_rebalance = await fetchCurrentInvestedFunds(
    vault_address,
    user
  );
  const hodl_balance_after_rebalance = await checkUserBalance(
    addressBook.getContractId("hodl_strategy"),
    vault_address,
    user
  );

  const tableData = {
    hodlStrategy: {
      "Balance before deposit": hodl_balance_before_deposit,
      "Balance after deposit": hodl_balance_after_deposit,
      "Balance after withdraw": hodl_balance_after_withdraw,
      "Balance after invest": hodl_balance_after_invest,
      "Balance after rebalance": hodl_balance_after_rebalance,
    },
    "Invested funds": {
      "Balance before deposit": invested_funds_before_deposit[0].amount,
      "Balance after deposit": invested_funds_after_deposit[0].amount,
      "Balance after withdraw": invested_funds_after_withdraw[0].amount,
      "Balance after invest": invested_funds_after_invest[0].amount,
      "Balance after rebalance": invested_funds_after_rebalance[0].amount,
    },
    "Idle funds": {
      "Balance before deposit": idle_funds_before_deposit[0].amount,
      "Balance after deposit": idle_funds_after_deposit[0].amount,
      "Balance after withdraw": idle_funds_after_withdraw[0].amount,
      "Balance after invest": idle_funds_after_invest[0].amount,
      "Balance after rebalance": idle_funds_after_rebalance[0].amount,
    },
  };
  console.table(tableData);

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
    withdraw: {
      instructions: withdraw_instructions,
      readBytes: withdraw_read_bytes,
      writeBytes: withdraw_write_bytes,
    },
    invest: {
      instructions: invest_instructions,
      readBytes: invest_read_bytes,
      writeBytes: invest_write_bytes,
    },
    rebalance: {
      instructions: rebalance_instructions,
      readBytes: rebalance_read_bytes,
      writeBytes: rebalance_write_bytes,
    },
  }
  console.table(budgetData);
  return {tableData, budgetData}; */
}