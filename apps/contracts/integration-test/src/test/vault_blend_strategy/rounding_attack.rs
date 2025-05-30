use crate::{
    blend_strategy::{self, BlendStrategyClient}, setup::create_vault_one_blend_strategy, test::{EnvTestUtils, ONE_YEAR_IN_SECONDS}, vault::defindex_vault_contract::{
        CurrentAssetInvestmentAllocation, Instruction, StrategyAllocation
    }
};

use soroban_sdk::{
    vec,
    Vec,
    Bytes,
    testutils::{Address as _, MockAuth, MockAuthInvoke}, token::{StellarAssetClient as SorobanTokenAdminClient, TokenClient as SorobanTokenClient}, vec as sorobanvec, Address, BytesN, Env, IntoVal, Map, String, 
};

const LUMENS: i128 = 1_0_000_000;

#[test]
fn rounding_attack() {
    let e: crate::setup::VaultOneBlendStrategy<'_> = create_vault_one_blend_strategy();
    
    // Initial setup with inflation deposit
    const INFLATION_AMOUNT: i128 = 1001;
    mint_and_deposit_to_vault(&e, &e.admin, INFLATION_AMOUNT);
    mint_and_deposit_to_vault(&e, &e.admin, 2*LUMENS);
    
    println!("Inflation deposit: ");
    e.usdc_client.mint(&e.admin, &INFLATION_AMOUNT);
    e.strategy_contract.deposit(&INFLATION_AMOUNT, &e.admin);

    print_vault_state(&e, "Initial state");
   
    // Get and print positions from the Blend pool for the strategy contract
    print_strategy_positions(&e, "Initial Strategy Positions");
    print_strategy_balance(&e
        , "Initial Strategy Balance");
    // Invest the inflation amount to the strategy
    println!("\x1b[32mInvesting inflation amount to strategy\x1b[0m");
    print_report(&e, "before investing inflation");
    invest(&e, INFLATION_AMOUNT + 2*LUMENS, &e.strategy_contract.address);
    
    print_strategy_balance(&e, "After Investing");
    print_vault_report(&e, "after investing inflation");
    print_strategy_positions(&e, "Strategy Positions After Inflation Investment");
    print_user_emissions(&e, e.strategy_contract.address.clone(), "User Emissions After Inflation Investment");
    print_shares_value(&e, "after inflation investment");
    
    // Make time pass and let the strategy earn some money
    let ledgers = 107;
    e.setup.env.jump(ledgers);
    println!("\x1b[35m---- HARVESTING !-----\x1b[0m");
    e.strategy_contract.harvest(&e.keeper, &None::<Bytes>);
    print_shares_value(&e, "after harvest");

    print_strategy_balance(&e, "After Harvest");
    print_strategy_positions(&e, "Strategy Positions After Harvest");
    print_user_emissions(&e, e.strategy_contract.address.clone(), "User Emissions After Harvest");

    // invest x tokens through the vault
    println!("\x1b[31mStarting Loop\x1b[0m");
    for i in 0..5 {
        println!("\x1b[36mIteration: {:?}\x1b[0m", i);
        let x = 2 * LUMENS + i;
        println!(" \x1b[36mInvesting {:?} tokens to strategy\x1b[0m", x);
        let user = Address::generate(&e.setup.env);

        print_shares_value(&e, "before deposit");

        e.vault_contract.lock_fees(&None::<u32>);
        print_shares_value(&e, "after lock_fees");

        // Deposit to vault
        mint_and_deposit_to_vault(&e, &user, x);
        print_usdc_user_balance(&e, &user, "User balance after deposit");
        print_user_balance(&e, &user, "User balance after deposit");
        print_shares_value(&e, "after deposit");


        // Report and check balances
        print_vault_report(&e, "after depositing");
        print_strategy_balance(&e, "after depositing");
        print_user_balance(&e, &user, "User balance after depositing");

        // Make investment
        print_vault_state(&e, "before investment");
        print_report(&e, "before investing");

        invest(&e, x, &e.strategy_contract.address);
        print_vault_state(&e, "after investment");
        print_strategy_positions(&e, "after investment");
        print_strategy_balance(&e, "after investment");

        print_user_balance(&e, &user, "User balance before withdraw");
        // Withdraw all user shares
        print_shares_value(&e, "before withdraw");
        withdraw_all_shares(&e, &user, x);
        print_shares_value(&e, "after withdraw");
        
        // Final checks
        print_vault_report(&e, "after withdrawing");
        print_strategy_balance(&e, "after withdrawal");
        print_vault_state(&e, "after withdrawing");
        let usdc_balance = print_usdc_user_balance(&e, &user, "User balance after withdrawing");

        // We may have differences due to rounding, so we check if the balance is close to the expected amount
        assert!((usdc_balance- x).abs() <= 10, "User balance after withdrawing is too far from the expected amount");
    }
    
    // Final state check
    println!("Vault USDC Balance: {}", e.usdc.balance(&e.vault_contract.address));
    println!("Vault Balance on Strategy: {}", e.strategy_contract.balance(&e.vault_contract.address));
    print_vault_report(&e, "Final state");
}

// New helper functions

fn print_usdc_user_balance(e: &crate::setup::VaultOneBlendStrategy<'_>, user: &Address, context: &str) -> i128 {
    let balance = e.usdc.balance(user);
    println!("\x1b[32mUSDC User balance: {}\x1b[0m", balance);
    balance
}


fn print_report(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) {
    let report = e.vault_contract.report();
    println!("Report {}: {:?}", context, report);
}

fn mint_and_deposit_to_vault(e: &crate::setup::VaultOneBlendStrategy<'_>, user: &Address, amount: i128) {
    println!("Minting {:?} tokens to user", amount);
    e.usdc_client.mint(user, &amount);
    println!("Depositing {:?} tokens to vault", amount);
    let res = e.vault_contract.deposit(
        &vec![&e.setup.env, amount],
        &vec![&e.setup.env, amount],
        user,
        &false
    );
    print!("-----------------------------------\n");
    print!("Deposit result: {:?}", res);
}

fn print_shares_value(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) {
    let shares = e.vault_contract.get_asset_amounts_per_shares(&1i128);
    println!("Value of shares: {:?} {}", shares, context);
}

fn print_user_balance(e: &crate::setup::VaultOneBlendStrategy<'_>, user: &Address, context: &str) { 
    let user_balance = e.vault_contract.balance(user);
    let user_balance_underlying = e.vault_contract.get_asset_amounts_per_shares(&user_balance);
    println!("\x1b[32mUser balance in shares | in underlying => {:?} | {:?}. {}\x1b[0m", user_balance, user_balance_underlying.get(0).unwrap(), context);
}
fn print_vault_state(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) {
    println!("Total managed funds {}: ", context);
    let total_managed_funds = e.vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(total_managed_funds);
}

fn print_vault_report(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) {
    let report = e.vault_contract.report();
    println!("Report {}: {:?}", context, report);
}

fn print_strategy_positions(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) {
    println!("\x1b[32m{}:\x1b[0m", context);
    let positions = e.blend_pool_client.get_positions(&e.strategy_contract.address);
    println!("Strategy positions: {:?}", positions);
}

fn print_strategy_balance(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) -> i128 {
    let balance = e.strategy_contract.balance(&e.vault_contract.address);
    println!("Strategy balance {}: {:?}", context, balance);
    balance
}

fn withdraw_all_shares(e: &crate::setup::VaultOneBlendStrategy<'_>, user: &Address, expected_amount: i128) {
    let user_shares = e.vault_contract.balance(user);
    println!("withdrawing {:?} shares", user_shares);
    let withdraw_amounts = e.vault_contract.withdraw(
        &user_shares,
        &vec![&e.setup.env, 0i128],
        user,
    );
    println!("Withdraw amount - expected: {:?}", withdraw_amounts.get(0).unwrap() - expected_amount);
    assert_eq!(withdraw_amounts.len(), 1);
    
    // Calculate tolerance (0.02% of expected amount)
    let tolerance = (expected_amount as f64 * 0.0002).round() as i128;
    let difference = (withdraw_amounts.get(0).unwrap() - expected_amount).abs();
    
    println!("Tolerance: {:?}, Actual difference: {:?}", tolerance, difference);
    assert!(difference <= tolerance, "Difference {} exceeds tolerance {}", difference, tolerance);
}

fn invest(e: &crate::setup::VaultOneBlendStrategy<'_>, amount: i128, strategy_address: &Address) {
    println!("Investing {:?} tokens to strategy", amount);
    let invest_instructions = vec![
        &e.setup.env,
        Instruction::Invest(
            strategy_address.clone(),
            amount,
        ),
    ];

    e.vault_contract
        .mock_auths(&[MockAuth {
            address: &e.manager.clone(),   
            invoke: &MockAuthInvoke {
                contract: &e.vault_contract.address.clone(),
                fn_name: "rebalance",
                args: (e.manager.clone(), invest_instructions.clone()).into_val(&e.setup.env),
                sub_invokes: &[],
            },
        }])
        .rebalance(&e.manager, &invest_instructions);
    
    // Report after investing
    let report = e.vault_contract.report();
    println!("Report after investing: {:?}", report);
}

fn print_total_managed_funds(total_managed_funds: Vec<CurrentAssetInvestmentAllocation>) {
    for asset in total_managed_funds {
        println!("  \x1b[33mAsset: {:?}\x1b[0m", asset.asset);
        println!("  \x1b[33mTotal amount: {:?}\x1b[0m", asset.total_amount);
        println!("  \x1b[33mIdle amount: {:?}\x1b[0m", asset.idle_amount);
        println!("  \x1b[33mInvested amount: {:?}\x1b[0m", asset.invested_amount);
        print_strategy_allocations(asset.strategy_allocations);
    }
}

fn print_strategy_allocations(strategy_allocations: Vec<StrategyAllocation>) {
    println!("  Strategy allocations: {:?}", strategy_allocations.len());
    for allocation in strategy_allocations {
        print_strategy_allocation(allocation);
    }
}
fn print_strategy_allocation(strategy_allocation: StrategyAllocation) {
    println!("  - Strategy allocations: ");
    println!("    - {:?}", strategy_allocation.strategy_address);
    println!("    - Amount: {:?}", strategy_allocation.amount);
    println!("    - Paused: {:?}", strategy_allocation.paused);
}

fn print_user_emissions(e: &crate::setup::VaultOneBlendStrategy<'_>, user: Address, context: &str) {
    println!("\x1b[32m{}:\x1b[0m", context);
    
    // Instead of trying to get the reserve list directly, we'll check emissions for specific reserves
    // We know the USDC reserve is at index 0 in this test setup
    
    // Loop through reserve indices (0 = USDC)
    for i in 0..2 {
        // For supply positions (bTokens): reserve_token_id = reserve_index * 2 + 1
        let supply_token_id = i * 2 + 1;
        // For borrow positions (dTokens): reserve_token_id = reserve_index * 2
        let borrow_token_id = i * 2;
        
        let token_name = if i == 0 { "USDC" } else { "Other" };
        
        // Get and print supply emissions
        if let Some(supply_emissions) = e.blend_pool_client.get_user_emissions(&user, &supply_token_id) {
            println!("  Supply emissions for {}: {:?}", token_name, supply_emissions);
        } else {
            println!("  No supply emissions for {}", token_name);
        }
        
        // Get and print borrow emissions
        if let Some(borrow_emissions) = e.blend_pool_client.get_user_emissions(&user, &borrow_token_id) {
            println!("  Borrow emissions for {}: {:?}", token_name, borrow_emissions);
        } else {
            println!("  No borrow emissions for {}", token_name);
        }
    }
}