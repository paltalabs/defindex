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
    let e = create_vault_one_blend_strategy();
    // make first deposit to vault
    // mint 1000 tokens to the user
    const INFLATION_AMOUNT: i128 = 1001;
    e.usdc_client.mint(&e.admin, &INFLATION_AMOUNT);
    e.vault_contract.deposit(
        &vec![&e.setup.env, INFLATION_AMOUNT],
        &vec![&e.setup.env, INFLATION_AMOUNT],
        &e.admin,
        &false
    );

    println!("Inflation deposit: ");
    e.usdc_client.mint(&e.admin, &INFLATION_AMOUNT);
    e.strategy_contract.deposit(&INFLATION_AMOUNT,&e.admin );

    let total_managed_funds = e.vault_contract.fetch_total_managed_funds();
    print_total_managed_funds(total_managed_funds);
   
   // Get and print positions from the Blend pool for the strategy contract
   println!("\x1b[32mInitial Strategy Positions:\x1b[0m");
   let initial_positions = e.blend_pool_client.get_positions(&e.strategy_contract.address);
   println!("Strategy positions: {:?}", initial_positions);

   
   // invest x tokens through the vault
   println!("\x1b[31mStarting Loop\x1b[0m");
    for i in 0..10 {
        let x = 2 * LUMENS+i;
        println!(" \x1b[36mInvesting {:?} tokens to strategy\x1b[0m", x);
        let user = Address::generate(&e.setup.env);
        // make second deposit to the vault
        println!("Minting {:?} tokens to user", x);
        e.usdc_client.mint(&user, &x);
        println!("Depositing {:?} tokens to vault", x);
        e.vault_contract.deposit(
            &vec![&e.setup.env, x],
            &vec![&e.setup.env, x],
            &user,
            &false
        );
    
        // println!("Total managed funds after deposit: ");
        // let total_managed_funds = e.vault_contract.fetch_total_managed_funds();
        // print_total_managed_funds(total_managed_funds);

        //report the vault
        let report = e.vault_contract.report();
        println!("Report after investing: {:?}", report);

        // make investment
        println!("Investing {:?} tokens to strategy", x);
        let invest_instructions = vec![
            &e.setup.env,
            Instruction::Invest(
                e.strategy_contract.address.clone(),
                x,
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
        
        println!("Total managed funds after rebalance: ");
        let total_managed_funds = e.vault_contract.fetch_total_managed_funds();
        print_total_managed_funds(total_managed_funds);
    
        // Verify the investment was successful
        let strategy_positions = e.blend_pool_client.get_positions(&e.strategy_contract.address);
        println!("Strategy positions: {:?}", strategy_positions);

        // report the vault
        let report = e.vault_contract.report();
        println!("Report after investing: {:?}", report);

        // Withdraw all user shares
        let user_shares = e.vault_contract.balance(&user);
        println!("withdrawing {:?} shares", user_shares);
        let withdraw_amounts = e.vault_contract.withdraw(
            &user_shares,
            &vec![&e.setup.env, 0i128],
            &user,
        );
        println!("Withdraw amount - x: {:?}", withdraw_amounts.get(0).unwrap() - x);
        
        
        assert_eq!(strategy_positions.supply.get(0).unwrap(), x+i+INFLATION_AMOUNT);
        assert_eq!(withdraw_amounts.len(), 1);
        assert_eq!(withdraw_amounts.get(0).unwrap(), x);
    }
    

    // Check the balances after investment
    println!("Vault USDC Balance: {}", e.usdc.balance(&e.vault_contract.address));
    println!("Vault Balance on Strategy: {}", e.strategy_contract.balance(&e.vault_contract.address));
    
    // Get a report from the vault
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