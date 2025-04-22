#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::storage::{self};
use crate::test::blend::soroswap_setup::create_soroswap_pool;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils, ONE_DAY_IN_SECONDS};
use crate::{reserves, shares_to_underlying, BlendStrategyClient};
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, Env, Bytes};
use crate::test::std::println;

const SCALAR_7: i128 = 10000000;

/// Sets up a basic environment for testing rounding attacks against the Blend strategy
/// Returns a tuple with all the necessary components for testing
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

fn print_strategy_reserves(e: &BlendStrategyTestSetup) {
    e.env.as_contract(&e.strategy, || {
        let reserves = storage::get_strategy_reserves(&e.env);
        println!("Reserves: {:?}", reserves);
    });
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
    mint_and_deposit_to_strategy(&setup, &setup.admin, INFLATION_AMOUNT);
    
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.admin, "Initial state");

    // Victim deposits 100 USDC
    mint_and_deposit_to_strategy(&setup, &setup.victim, 100 * SCALAR_7);
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After victim deposit");
    
    borrow_from_blend_pool(&setup, &setup.admin, &setup.usdc_token, 50_000* SCALAR_7);
    // Make Strategy earn some money
    setup.env.jump_time(ONE_DAY_IN_SECONDS*365*1000);
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After inflation");
    
    // Harvest
    setup.strategy_client.harvest(&setup.keeper, &None::<Bytes>);
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After harvest");

    // Withdraw
    let strategy_balance = print_strategy_balance(&setup, &setup.victim, "Before withdraw");
    let user_usdc_balance_before = print_usdc_balance(&setup, &setup.victim, "Before withdraw");
    
    let withdraw_amount = 100;
    withdraw_from_strategy(&setup, &setup.victim, withdraw_amount);
    print_strategy_reserves(&setup);
    get_underlying_value(&setup, &setup.victim, "After withdraw");
    let strategy_balance_after = print_strategy_balance(&setup, &setup.victim, "After withdraw");
    let user_usdc_balance_after = print_usdc_balance(&setup, &setup.victim, "After withdraw");
    println!("Strategy balance difference: {:?}", strategy_balance_after - strategy_balance);
    println!("USDC balance difference: {:?}", user_usdc_balance_after - user_usdc_balance_before);

    // Strategy balance difference should be equal to the withdraw amount
    assert_eq!(user_usdc_balance_after - user_usdc_balance_before, withdraw_amount);
    assert_eq!( strategy_balance - strategy_balance_after, withdraw_amount); // its failing with 152
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

fn mint_and_deposit_to_strategy(e: &BlendStrategyTestSetup, user: &Address, amount: i128) {
    println!("Minting {:?} tokens to user", amount);
    e.usdc_client.mint(user, &amount);
    println!("Depositing {:?} tokens to vault", amount);
    e.strategy_client.deposit(&amount, user);
}

fn withdraw_from_strategy(e: &BlendStrategyTestSetup, user: &Address, amount: i128) {
    println!("Withdrawing {:?} tokens from strategy", amount);
    e.strategy_client.withdraw(&amount, user, user);
}
