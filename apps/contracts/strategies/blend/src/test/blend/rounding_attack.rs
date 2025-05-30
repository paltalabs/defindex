#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::constants::SCALAR_12;
use crate::reserves::StrategyReserves;
use crate::storage::{self};
use crate::utils;
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils, ONE_DAY_IN_SECONDS};
use crate::{reserves, shares_to_underlying, BlendStrategyClient};
use defindex_strategy_core::event::DepositEvent;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _, Events};
use soroban_sdk::{vec, Address, Bytes, Env, IntoVal, Vec};
use crate::test::std::println;

const SCALAR_7: i128 = 10000000;

/// Sets up a basic environment for testing rounding attacks against the Blend strategy
/// Returns a tuple with all the necessary components for testing
#[allow(dead_code)]
struct BlendStrategyTestSetup<'a> {
    env: Env,
    admin: Address,
    attacker: Address,
    victim: Address,
    keeper: Address,
    blnd_token: Address,
    usdc_token: Address,
    xlm_token: Address,
    blnd_client: MockTokenClient<'a>,
    usdc_client: MockTokenClient<'a>,
    xlm_client: MockTokenClient<'a>,
    blend_pool: Address,
    strategy: Address,
    strategy_client: BlendStrategyClient<'a>,
    blend_pool_client: BlendPoolClient<'a>,
}

fn setup_blend_strategy<'a>() -> BlendStrategyTestSetup<'a> {
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    let admin = Address::generate(&e);
    let attacker = Address::generate(&e);
    let victim = Address::generate(&e);
    let keeper = Address::generate(&e);

    let blnd = e.register_stellar_asset_contract_v2(admin.clone());
    let usdc = e.register_stellar_asset_contract_v2(admin.clone());
    let xlm = e.register_stellar_asset_contract_v2(admin.clone());
    let blnd_client = MockTokenClient::new(&e, &blnd.address());
    let usdc_client = MockTokenClient::new(&e, &usdc.address());
    let xlm_client = MockTokenClient::new(&e, &xlm.address());

    // Setting up soroswap pool
    let pool_admin = Address::generate(&e);
    // Assume 1 BLND == 1 USDC for simplicity
    let amount_a = 100000000000_0_000_000;
    let amount_b = 100000000000_0_000_000;
    blnd_client.mint(&pool_admin, &amount_a);
    usdc_client.mint(&pool_admin, &amount_b);
    let soroswap_router = create_soroswap_pool(
        &e,
        &pool_admin,
        &blnd.address(),
        &usdc.address(),
        &amount_a,
        &amount_b,
    );

    // Setup the Blend pool
    let blend_fixture = BlendFixture::deploy(&e, &admin, &blnd.address(), &usdc.address());

    let pool = create_blend_pool(
        &e,
        &blend_fixture,
        &admin,
        &usdc_client,
        &xlm_client,
        &blnd_client,
    );
    let pool_client = BlendPoolClient::new(&e, &pool);

    let requests = vec![
        &e,
        Request {
            address: usdc.address().clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: usdc.address().clone(),
            amount: 100_000_0000000,
            request_type: 4,
        },
        Request {
            address: xlm.address().clone(),
            amount: 200_000_0000000,
            request_type: 2,
        },
        Request {
            address: xlm.address().clone(),
            amount: 100_000_0000000,
            request_type: 4,
        },
    ];
    pool_client.submit(&admin, &admin, &admin, &requests);

    // Create the Blend strategy
    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
        &keeper,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    BlendStrategyTestSetup {
        env: e,
        admin,
        attacker,
        victim,
        keeper,
        blnd_token: blnd.address(),
        usdc_token: usdc.address(),
        xlm_token: xlm.address(),
        blnd_client,
        usdc_client,
        xlm_client,
        blend_pool: pool,
        blend_pool_client: pool_client,
        strategy,
        strategy_client,
    }
}


/// Borrows tokens from the Blend pool
///
/// # Arguments
///
/// * `e` - The test setup environment
/// * `user` - The address of the user who is borrowing
/// * `asset` - The address of the asset to borrow
/// * `amount` - The amount to borrow
fn borrow_from_blend_pool(e: &BlendStrategyTestSetup, user: &Address, asset: &Address, amount: i128) {
    println!("Borrowing {:?} tokens from Blend pool", amount);
    
    // Create a borrow request
    let requests = vec![
        &e.env,
        Request {
            request_type: 4,
            address: asset.clone(),
            amount,
        },
    ];
    
    // Execute the borrow request
    e.blend_pool_client.submit(&user, &e.admin, &e.admin, &requests);
    
    println!("Successfully borrowed {:?} tokens", amount);
}



#[test]
fn rounding_attack() {
    // Use the setup function to initialize the environment
    let setup = setup_blend_strategy();

    // Initial setup with inflation deposit
    const INFLATION_AMOUNT: i128 = 1001;
    mint_and_deposit_to_strategy(&setup, &setup.admin, 2*INFLATION_AMOUNT);
    
    let reserves_before_first_deposit = print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.admin, "Initial state");
    
    // Victim deposits 100 USDC
    print_usdc_balance(&setup, &setup.victim, "Before victim deposit");

    let first_deposit_amount = 100 * SCALAR_7;
    let expected_deposit_amount = utils::calculate_optimal_deposit_amount(first_deposit_amount, &reserves_before_first_deposit).unwrap().0;
    let first_deposit_event = mint_and_deposit_to_strategy(&setup, &setup.victim, first_deposit_amount);

    assert_eq!(first_deposit_event.amount, expected_deposit_amount);

    print_usdc_balance(&setup, &setup.victim, "After victim deposit");
    
    print_strategy_reserves(&setup);
    print_b_rate(&setup);
    
    get_underlying_value(&setup, &setup.victim, "After victim deposit");
    
    borrow_from_blend_pool(&setup, &setup.admin, &setup.usdc_token, 50_000* SCALAR_7);
    // Make Strategy earn some money
    setup.env.jump_time(ONE_DAY_IN_SECONDS*365*1000);
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After inflation");
    
    print_b_rate(&setup);

    
    // Harvest
    println!("\x1b[32mHarvesting...\x1b[0m");
    setup.strategy_client.harvest(&setup.keeper, &None::<Bytes>);
    let reserves_after_harvest = print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After harvest");
    
    // Victim deposits 100 USDC
    print_usdc_balance(&setup, &setup.victim, "Before victim deposit");

    let deposit_amount = 100 * SCALAR_7;
    let expected_deposit_amount = utils::calculate_optimal_deposit_amount(deposit_amount, &reserves_after_harvest).unwrap().0;

    let deposit_event = mint_and_deposit_to_strategy(&setup, &setup.victim, deposit_amount);
    assert_eq!(deposit_event.amount, expected_deposit_amount);
    
    print_usdc_balance(&setup, &setup.victim, "After victim deposit");
    print_strategy_reserves(&setup);
    let b_rate = print_b_rate(&setup);
    let expected_b_tokens_minted = deposit_amount * b_rate / SCALAR_7;
    println!("Expected b tokens minted: {:?}", expected_b_tokens_minted);
    let expected_shares_minted = expected_b_tokens_minted * SCALAR_7 / b_rate;
    println!("Expected shares minted: {:?}", expected_shares_minted);
    
    // Withdraw
    let strategy_balance = print_strategy_balance(&setup, &setup.victim, "Before withdraw");
    let user_usdc_balance_before = print_usdc_balance(&setup, &setup.victim, "Before withdraw");
    
    let withdraw_amount = 1000000;
    withdraw_from_strategy(&setup, &setup.victim, withdraw_amount);
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After withdraw");
    let strategy_balance_after = print_strategy_balance(&setup, &setup.victim, "After withdraw");
    let user_usdc_balance_after = print_usdc_balance(&setup, &setup.victim, "After withdraw");
    println!("Strategy balance difference: {:?}", strategy_balance_after - strategy_balance);
    println!("USDC balance difference: {:?}", user_usdc_balance_after - user_usdc_balance_before);
    
    // Strategy balance difference should be equal to the withdraw amount
    assert!(user_usdc_balance_after - user_usdc_balance_before >= withdraw_amount);
    assert!(strategy_balance - strategy_balance_after >= withdraw_amount); // its failing with 152
}


fn check_withdraw(optimal_w_amount:i128, expected_w_amount:i128) -> bool {
    let res = optimal_w_amount >= expected_w_amount;
    res
}

fn check_deposit(optimal_d_amount:i128, expected_d_amount:i128) -> bool {    
    let res = optimal_d_amount <= expected_d_amount;
    res
}

fn test_optimal_deposit_amounts(setup: &BlendStrategyTestSetup, reserves_list: &Vec<StrategyReserves>, test_amount: i128){
    for reserves in reserves_list {
         // Calculate expected values
         let b_tokens_minted = test_amount * SCALAR_12 / reserves.b_rate;
         let shares_minted = reserves.b_tokens_to_shares_down(b_tokens_minted).unwrap();
         let optimal_b_tokens = (shares_minted * reserves.total_b_tokens - 1) / reserves.total_shares + 1;
         let expected_optimal_deposit = (optimal_b_tokens * reserves.b_rate - 1) / SCALAR_12 + 1;
         
         // Get actual optimal deposit
         let (optimal_deposit, optimal_b_tokens_actual) = utils::calculate_optimal_deposit_amount(test_amount, &reserves).unwrap();
         
         println!("Deposit amount: {}", test_amount);
         println!("B tokens that would be minted: {}", b_tokens_minted);
         println!("Shares that would be minted: {}", shares_minted);
         println!("Optimal b tokens expected: {}", optimal_b_tokens);
         println!("Optimal b tokens actual: {}", optimal_b_tokens_actual);
         println!("Expected optimal deposit: {}", expected_optimal_deposit);
         println!("Actual optimal deposit: {}", optimal_deposit);
         println!("Difference: {}", optimal_deposit - test_amount);
         println!("---");
         
         // The optimal deposit should match our manual calculation
         assert_eq!(optimal_deposit, expected_optimal_deposit);
         assert_eq!(optimal_b_tokens_actual, optimal_b_tokens);
         
        let (optimal_deposit_amount, _) = setup.env.as_contract(&setup.strategy, || {
            utils::calculate_optimal_deposit_amount(test_amount, &reserves)
        }).expect("failed to calculate amount");
        println!("Test amount: {:?}", test_amount);
        println!("Optimal deposit amount: {:?}", optimal_deposit_amount);
        assert!(check_deposit(optimal_deposit_amount, test_amount));
    }
    
}

fn test_optimal_withdraw_amounts(setup: &BlendStrategyTestSetup, reserves_list: &Vec<StrategyReserves>, test_amount: i128) {
    for reserves in reserves_list {
        print_reserves(&reserves);
        // Calculate expected values
        let b_tokens_burnt = ((test_amount * SCALAR_12 - 1) / reserves.b_rate) + 1;
        let shares_burnt = reserves.b_tokens_to_shares_up(b_tokens_burnt).unwrap();
        let optimal_b_tokens = (shares_burnt * reserves.total_b_tokens) / reserves.total_shares;
        let expected_optimal_withdraw = (optimal_b_tokens * reserves.b_rate) / SCALAR_12;
        
        // Get actual optimal Withdraw
        let (optimal_withdraw, optimal_b_tokens_actual) = utils::calculate_optimal_withdraw_amount(test_amount, &reserves).unwrap();

        println!("Withdraw amount: {}", test_amount);
        println!("B tokens that would be burnt: {}", b_tokens_burnt);
        println!("Shares that would be burnt: {}", shares_burnt);
        println!("Optimal b tokens expected: {}", optimal_b_tokens);
        println!("Optimal b tokens actual: {}", optimal_b_tokens_actual);
        println!("Expected optimal withdraw: {}", expected_optimal_withdraw);
        println!("Actual optimal withdraw: {}", optimal_withdraw);
        println!("Difference: {}", optimal_withdraw - test_amount);
        println!("---");
        
        // The optimal withdraw should match our manual calculation
        assert_eq!(optimal_withdraw, expected_optimal_withdraw);
        assert_eq!(optimal_b_tokens_actual, optimal_b_tokens);

        let (optimal_withdraw_amount, _) = setup.env.as_contract(&setup.strategy, || {
            utils::calculate_optimal_withdraw_amount(test_amount, &reserves)
        }).expect("failed to calculate amount");
        println!("Test amount: {:?}", test_amount);
        println!("Optimal withdraw amount: {:?}", optimal_withdraw_amount);

        assert!(check_withdraw(optimal_withdraw_amount, test_amount));
    }
}
#[test]
fn calculate_optimal_deposit_amount() {
    let setup = setup_blend_strategy();

    let reserves_list = vec![
        &setup.env,
        StrategyReserves { total_shares: 2000, total_b_tokens: 2000, b_rate: 1000000000000 },
        StrategyReserves { total_shares: 1000002000, total_b_tokens: 1000002000, b_rate: 1000000000000 },
        StrategyReserves { total_shares: 1000002000, total_b_tokens: 1002977244, b_rate: 75962518665704 },
        StrategyReserves { total_shares: 1013126950, total_b_tokens: 1016141244, b_rate: 75962518665704 },
        StrategyReserves { total_shares: 1013113987, total_b_tokens: 1016128243, b_rate: 75962518665704 },
        StrategyReserves { total_shares: 1013113988, total_b_tokens: 1016128244, b_rate: 75962518665704 },
    ];
    let test_amounts = vec![
        &setup.env,
        1*SCALAR_7,              // 1 token
        10 * SCALAR_7,         // 10 tokens
        100 * SCALAR_7,        // 100 tokens
        1000 * SCALAR_7,       // 1000 tokens
        10000 * SCALAR_7,      // 10000 tokens
        ];
    for amount in test_amounts {
        test_optimal_deposit_amounts(&setup, &reserves_list, amount);
    }
}
#[test]
fn calculate_optimal_withdraw_amount(){
    let setup = setup_blend_strategy();
    let reserves_list = vec![
        &setup.env,
        StrategyReserves { total_shares: 2000, total_b_tokens: 2000, b_rate: 1000000000000 },
        StrategyReserves { total_shares: 1000002000, total_b_tokens: 1000002000, b_rate: 1000000000000 },
        StrategyReserves { total_shares: 1000002000, total_b_tokens: 1002977244, b_rate: 75962518665704 },
        StrategyReserves { total_shares: 1013126950, total_b_tokens: 1016141244, b_rate: 75962518665704 },
        StrategyReserves { total_shares: 1013113987, total_b_tokens: 1016128243, b_rate: 75962518665704 },
    ];
    let test_amounts = vec![
        &setup.env,
            1*SCALAR_7,              // 1 token
            10 * SCALAR_7,         // 10 tokens
            100 * SCALAR_7,        // 100 tokens
            1000 * SCALAR_7,       // 1000 tokens
            10000 * SCALAR_7,      // 10000 tokens
    ];
    for amount in test_amounts {
        test_optimal_withdraw_amounts(&setup, &reserves_list, amount);
    }
}

// Helper functions
fn print_reserves(reserves: &StrategyReserves) {
    println!("Reserves: {:?}", reserves);
}

fn print_strategy_reserves(e: &BlendStrategyTestSetup) -> StrategyReserves {
    let reserves = e.env.as_contract(&e.strategy, 
        || storage::get_strategy_reserves(&e.env)
    );
    print_reserves(&reserves);
    reserves
}

fn get_underlying_value(e: &BlendStrategyTestSetup, user: &Address, label: &str) {
    e.env.as_contract(&e.strategy, || {
        let config = storage::get_config(&e.env).expect("Failed to get config");
        let reserves = reserves::get_strategy_reserve_updated(&e.env, &config);
        let user_shares = storage::get_vault_shares(&e.env, user);
        
        if reserves.total_shares > 0 {
            let one_share_value = shares_to_underlying(1, reserves.clone())
                .expect("Failed to convert one share to underlying");
            
            if user_shares > 0 {
                let underlying = shares_to_underlying(user_shares, reserves.clone())
                    .expect("Failed to convert shares to underlying");
                println!("{} - One share value: {}, User shares: {}, Underlying value: {}", 
                         label, one_share_value, user_shares, underlying);
            } else {
                println!("{} - One share value: {}, User has no shares", 
                         label, one_share_value);
            }
        } else if user_shares > 0 {
            println!("{} - User shares: {} (no share value available)", 
                     label, user_shares);
        } else {
            println!("{} - User has no shares", label);
        }
    });
}

fn print_b_rate(e: &BlendStrategyTestSetup) -> i128 {
    let b_rate = e.blend_pool_client.get_reserve(&e.usdc_token).data.b_rate;
    println!("B rate: {:?}", b_rate);
    return b_rate;
}

fn print_strategy_balance(e: &BlendStrategyTestSetup, user: &Address, label: &str) -> i128 {
    let balance = e.strategy_client.balance(user);
    println!("{}: Strategy balance: {:?}", label, balance);
    return balance;
}

fn print_usdc_balance(e: &BlendStrategyTestSetup, user: &Address, label: &str) -> i128 {
    let balance = e.usdc_client.balance(user);
    println!("{}: Asset balance: {:?}", label, balance);
    return balance;
}

fn mint_and_deposit_to_strategy(e: &BlendStrategyTestSetup, user: &Address, amount: i128) -> DepositEvent {
    println!("Minting {:?} tokens to user", amount);
    e.usdc_client.mint(user, &amount);
    println!("Depositing {:?} tokens to vault", amount);
    e.strategy_client.deposit(&amount, user);
    let event: DepositEvent = e.env.events().all().last().expect("No events found").2.into_val(&e.env);
    event
}

fn withdraw_from_strategy(e: &BlendStrategyTestSetup, user: &Address, amount: i128) {
    println!("Withdrawing {:?} tokens from strategy", amount);
    e.strategy_client.withdraw(&amount, user, user);
}
