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
    invest(&e, INFLATION_AMOUNT, &e.strategy_contract.address);
    
    print_strategy_balance(&e, "After Investing");
    print_vault_report(&e, "after investing inflation");
    print_strategy_positions(&e, "Strategy Positions After Inflation Investment");
    print_user_emissions(&e, e.strategy_contract.address.clone(), "User Emissions After Inflation Investment");
    
    // Make time pass and let the strategy earn some money
    let ledgers = 107;
    e.setup.env.jump(ledgers);
    println!("\x1b[35m---- HARVESTING !-----\x1b[0m");
    e.strategy_contract.harvest(&e.keeper, &None::<Bytes>);

    print_strategy_balance(&e, "After Harvest");
    print_strategy_positions(&e, "Strategy Positions After Harvest");
    print_user_emissions(&e, e.strategy_contract.address.clone(), "User Emissions After Harvest");

    // invest x tokens through the vault
    println!("\x1b[31mStarting Loop\x1b[0m");
    for i in 0..1 {
        let x = 2 * LUMENS + i;
        println!(" \x1b[36mInvesting {:?} tokens to strategy\x1b[0m", x);
        let user = Address::generate(&e.setup.env);
        
        // Deposit to vault
        mint_and_deposit_to_vault(&e, &user, x);
        
        // Report and check balances
        print_vault_report(&e, "after depositing");
        print_strategy_balance(&e, "after depositing");

        // Make investment
        invest(&e, x, &e.strategy_contract.address);
        print_vault_state(&e, "after rebalance");
        print_strategy_positions(&e, "after investment");
        print_strategy_balance(&e, "after investment");

        // Withdraw all user shares
        withdraw_all_shares(&e, &user, x);
        
        // Final checks
        print_vault_report(&e, "after withdrawing");
        print_strategy_balance(&e, "after withdrawal");
                
        // Assertions
        let strategy_positions = e.blend_pool_client.get_positions(&e.strategy_contract.address);
        assert_eq!(strategy_positions.supply.get(0).unwrap(), 2*INFLATION_AMOUNT);
        
        // Check user USDC balance after withdrawal
        let user_usdc_balance = e.usdc.balance(&user);
        println!("User USDC balance after withdrawal: {}", user_usdc_balance);
    }
    
    // Final state check
    println!("Vault USDC Balance: {}", e.usdc.balance(&e.vault_contract.address));
    println!("Vault Balance on Strategy: {}", e.strategy_contract.balance(&e.vault_contract.address));
    print_vault_report(&e, "after investing");
}

// New helper functions

fn mint_and_deposit_to_vault(e: &crate::setup::VaultOneBlendStrategy<'_>, user: &Address, amount: i128) {
    println!("Minting {:?} tokens to user", amount);
    e.usdc_client.mint(user, &amount);
    println!("Depositing {:?} tokens to vault", amount);
    e.vault_contract.deposit(
        &vec![&e.setup.env, amount],
        &vec![&e.setup.env, amount],
        user,
        &false
    );
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

fn print_strategy_balance(e: &crate::setup::VaultOneBlendStrategy<'_>, context: &str) {
    let balance = e.strategy_contract.balance(&e.vault_contract.address);
    println!("Strategy balance {}: {:?}", context, balance);
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
    assert_eq!(withdraw_amounts.get(0).unwrap(), expected_amount);
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