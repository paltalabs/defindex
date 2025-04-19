use crate::{
    setup::create_vault_one_blend_strategy,
    test::{
        
    },
    vault::defindex_vault_contract::{
        CurrentAssetInvestmentAllocation, Instruction, StrategyAllocation
    }
};
use soroban_sdk::{
    vec,
    Vec,
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

    // Invest the inflation amount to the strategy
    println!("\x1b[32mInvesting inflation amount to strategy\x1b[0m");
    invest(&e, INFLATION_AMOUNT, &e.strategy_contract.address);
    print_vault_report(&e, "after investing inflation");
    print_strategy_positions(&e, "Strategy Positions After Inflation Investment");
   
    // invest x tokens through the vault
    println!("\x1b[31mStarting Loop\x1b[0m");
    for i in 0..7 {
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