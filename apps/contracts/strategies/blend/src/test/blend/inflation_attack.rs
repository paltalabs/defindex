#![cfg(test)]
use crate::blend_pool::{BlendPoolClient, Request};
use crate::storage::{self};
use crate::test::blend::soroswap_setup::create_soroswap_pool;
// use crate::test::std::println;
use crate::test::{create_blend_pool, create_blend_strategy, BlendFixture, EnvTestUtils};
use crate::BlendStrategyClient;
// use defindex_strategy_core::StrategyError;
use sep_41_token::testutils::MockTokenClient;
use soroban_sdk::testutils::{Address as _};
use soroban_sdk::{vec, Address, Env};

#[test]
fn inflation_attack() {
    let e = Env::default();
    e.cost_estimate().budget().reset_unlimited();
    e.mock_all_auths();
    e.set_default_info();

    let admin = Address::generate(&e);
    let attacker = Address::generate(&e);
    let victim = Address::generate(&e);

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
    // End of setting up soroswap pool

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
    // End of setting up the Blend pool

    let strategy = create_blend_strategy(
        &e,
        &usdc.address(),
        &pool,
        &blnd.address(),
        &soroswap_router.address,
    );
    let strategy_client = BlendStrategyClient::new(&e, &strategy);

    // Scenario:
    // 1. Initially the pool is empty, and a victim is attempting to deposit 10_000 USDC
    // 2. Attacker owns a big quantity of BLND tokens(or any other token which he can swap for BLND).
    //    His goal is to frontrun the victim and cause them to lose funds.
    //
    // Exploit process:
    // 1. Attacker deposits some USDC in the Strategy and immediately withdraws everything, but 1 unit.
    // 2. Then, he donates all his BLND in the strategy and harvests.
    //    Reinvesting takes place, all the BLND are swapped for USDC and deposited in the Blend pool.
    //    At this point, the share price is inflated really high, since total_supply == 1, while the b_token supply is much higher
    // 3. Victim's transaction lands, and due to the round down, he loses approx. 25% of his funds

    // Setup initial balances:
    let scalar_7 = 1_0_000_000;
    let victim_usdc_balance = 10_000 * scalar_7;

    let attacker_usdc_balance = 1 * scalar_7; // Attacker doesn't need much USDC
    let attacker_blnd_balance = 1_000_000 * scalar_7; // He does need a big quantity of BLND

    usdc_client.mint(&victim, &victim_usdc_balance);
    usdc_client.mint(&attacker, &attacker_usdc_balance);
    blnd_client.mint(&attacker, &attacker_blnd_balance);

    // Since we've assumed 1 USDC == 1 BLND, the attacker's total balance is attacker_usdc_balance + attacker_blnd_balance.
    // Also adding his strategy balance for completeness, even though it's 0
    let total_attacker_init_balance = usdc_client.balance(&attacker)
        + blnd_client.balance(&attacker)
        + strategy_client.balance(&attacker);

    // Step 1: Attacker deposits and withdraws everything but 1 unit
    strategy_client.deposit(&attacker_usdc_balance, &attacker);

    // total shares should be equal to attacker_usdc_balance
    // and because b_rate = 1, total b tokens are also attacker_usdc_balance
    e.as_contract(&strategy, || {
        let reserve = storage::get_strategy_reserves(&e);
        assert_eq!(reserve.total_shares, attacker_usdc_balance);
        assert_eq!(reserve.total_b_tokens, attacker_usdc_balance);

    });

    // He cannot withdraw all but one because there are 1,000 stroop units that where blocked for the first depositor
    let try_result = strategy_client.try_withdraw(&(attacker_usdc_balance - 1), &attacker, &attacker);
    assert!(try_result.is_err());

    

    // But he can try to withdraw everything but 1 of this minted units (attacker_usdc_balance - 1000)
    strategy_client.withdraw(&(attacker_usdc_balance - 1000 - 1), &attacker, &attacker);

    // At this point, the vault cannot have total_shares == 1
    // But it will have total_shares = 1001
    e.as_contract(&strategy, || {
        let reserve = storage::get_strategy_reserves(&e);
        assert_ne!(reserve.total_shares, 1);
        assert_eq!(reserve.total_shares, 1001); // the attacker share + the 1,000 locked
        assert_eq!(reserve.total_b_tokens, 1001); // the attacker share + the 1,000 locked divided by the brate (1)
        assert_eq!(reserve.shares_to_b_tokens_down(1), Ok(1)); // one share = 1 btoken = 1usdc
 
    });

    // Step 2: Atacker donates some his BLND and harvests
    // Note: Tried to find an amount that maximizes the profit, as explained below
    blnd_client.transfer(&attacker, &strategy, &5015_0_453_863);
    // this amount will be swapped for 5000 * scale7 usdc, in fact
    let expeced_usdc_output_amount = soroswap_router.router_get_amounts_out(
        &5015_0_453_863, 
        &vec![&e, blnd.address().clone(), usdc.address().clone()])
    .get(1).unwrap();
    assert_eq!(expeced_usdc_output_amount, 5000* scalar_7);
    // we choose 5000 because is equal to (victim_usdc_balance / 2)
    assert_eq!((victim_usdc_balance / 2), 5000* scalar_7);

    strategy_client.harvest(&attacker);

    e.as_contract(&strategy, || {
        let reserve = storage::get_strategy_reserves(&e);
        
        assert_eq!(reserve.total_shares, 1001);

        // The ideal scenario is `total_b_tokens == victim_deposit_amount / 2 + 1`.
        // We aim for something strictly greater than that(otherwise the deposit would fail), but it's fine if it's not exactly equal,
        // since it may be complex to achieve `==` due to the share calcs and the precision loss.
        // In this case, we found an amount that achieves it through trial and error, plus a small binary search

        assert_ne!(reserve.total_b_tokens, (victim_usdc_balance / 2) + 1); // however because our 1000 locked this is not possible
        assert_eq!(reserve.total_b_tokens, (victim_usdc_balance / 2) + 1 + 1000); // this is the best the attacker can gets
        assert_eq!(reserve.total_b_tokens, 50000001001); // this is the best the attacker can gets


        // The attacked would lik that the price at this point is `5000_0_000_001`.
        // But in fact is `50000001001÷1001 = 49950050` (much lower that he planed it to be)

        assert_eq!(reserve.shares_to_b_tokens_down(1), Ok(49950050));
    });


    // Step 3: Victim deposits
    strategy_client.deposit(&victim_usdc_balance, &victim);

    /*
        Because 1 share = 49950050 b_tokens
        If the victim wants to deposit 10_000 * scalar_7; tokens, he will receive 
        // amount.fixed_mul_floor(self.total_shares, self.total_b_tokens)
        (100000000000 * 1001 ) / 50000001001 = 2001.99995992 = 2001
        [HERE THE VICTIM ALREADO LOST SOME MONEY]
    */

    e.as_contract(&strategy, || {
        let reserve = storage::get_strategy_reserves(&e);
        
        assert_eq!(reserve.total_shares, 1001 + 2001); // = 1001 + 2001 = 3002
        assert_eq!(reserve.total_b_tokens, (victim_usdc_balance / 2) + 1 + 1000 + victim_usdc_balance); // 150000001001
        assert_eq!(reserve.total_b_tokens, 150000001001);


        // 150000001001/3002 = 49966689.207528314
        assert_eq!(reserve.shares_to_b_tokens_down(1), Ok(49966689));

        let victim_shares = storage::get_vault_shares(&e, &victim);
        assert_eq!(victim_shares, 2001);

    });


    // The attacker's final balance is `usdc_balance` + `blnd_balance` + `strategy balance`
    let attacker_final_balance = usdc_client.balance(&attacker)
        + blnd_client.balance(&attacker)
        + strategy_client.balance(&attacker);

    // The attacker expects to have a greater balance than before, however
    // he is loosing money trying to do this attack
    assert!(total_attacker_init_balance > attacker_final_balance);

    // We know that now the victim won't have 100% of its value due to rounding errors 
    // But will accept anything lower than 0.02% loss
    let victim_final_balance = strategy_client.balance(&victim);
    // Calculate the minimum acceptable balance (99.98% of initial balance)
    let min_acceptable_balance = victim_usdc_balance * 9998 / 10000; // 99.98%
    assert!(
        victim_final_balance >= min_acceptable_balance,
        "Victim final balance ({}) is too low; expected at least {} (0.02% loss tolerance)",
        victim_final_balance,
        min_acceptable_balance
    );
}