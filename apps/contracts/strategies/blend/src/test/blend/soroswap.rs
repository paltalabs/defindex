#![cfg(test)]
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address};
use crate::soroswap::internal_swap_exact_tokens_for_tokens;
use crate::storage;

use super::utils::create_generic_strategy;

extern crate std;
#[test]
#[should_panic(expected = "#454")] // InternalSwapError
fn internal_swap_exact_tokens_for_tokens_random_address_path(){
    let strategy = create_generic_strategy();
    let e = strategy.e;

    let amount_in = 1000000_0_000_000i128;
    strategy.blnd_client.mint(&strategy.address, &amount_in);

    let random_address = Address::generate(&e);
    let random_address_1 = Address::generate(&e);
    let amount_out_min = 0i128;
    let path = vec![&e, random_address.clone(), random_address_1.clone()];
    let to = strategy.address.clone();
    let deadline = e.ledger().timestamp() + 1000;
    let config = e.as_contract(&strategy.address, ||storage::get_config(&e).expect("Failed to get config"));

    let _ = e.as_contract(&strategy.address, || internal_swap_exact_tokens_for_tokens(&e, &amount_in, &amount_out_min, path, &to, &deadline, &config));
}
#[test]
#[should_panic(expected = "#423")] // SoroswapPairError
fn internal_swap_exact_tokens_for_tokens_duplicated_address_path(){
    let strategy = create_generic_strategy();
    let e = strategy.e;

    let amount_in = 1000000_0_000_000i128;
    strategy.blnd_client.mint(&strategy.address, &amount_in);

    let amount_out_min = 0i128;
    let path = vec![&e, strategy.usdc.clone(), strategy.usdc.clone()];
    let to = strategy.address.clone();
    let deadline = e.ledger().timestamp() + 1000;
    let config = e.as_contract(&strategy.address, ||storage::get_config(&e).expect("Failed to get config"));

    let _ = e.as_contract(&strategy.address, || internal_swap_exact_tokens_for_tokens(&e, &amount_in, &amount_out_min, path, &to, &deadline, &config));
}
#[test]
fn internal_swap_exact_tokens_for_tokens_success(){
    let strategy = create_generic_strategy();
    let e = strategy.e;

    let amount_in = 1000000_0_000_000i128;
    strategy.blnd_client.mint(&strategy.address, &amount_in);

    let amount_out_min = 0i128;
    let path = vec![&e, strategy.blnd.clone(), strategy.usdc.clone()];
    let to = strategy.address.clone();
    let deadline = e.ledger().timestamp() + 1000;
    let config = e.as_contract(&strategy.address, ||storage::get_config(&e).expect("Failed to get config"));

    let internal_swap_result = e.as_contract(&strategy.address, || internal_swap_exact_tokens_for_tokens(&e, &amount_in, &amount_out_min, path, &to, &deadline, &config));
    std::println!("internal_swap_result: {:?}", internal_swap_result);
    let expected_amount_out = get_soroswap_output(amount_in, strategy.reserve_blnd, strategy.reserve_usdc);
    std::println!("expected_amount_out: {:?}", expected_amount_out);
    assert_eq!(internal_swap_result.is_ok(), true);
    assert_eq!(internal_swap_result.unwrap().get(1).unwrap(), expected_amount_out);
}

fn get_soroswap_output(amount_in: i128, reserve_in: i128, reserve_out: i128) -> i128 {
    let amount_in_with_fee = amount_in * 997;
    let numerator = amount_in_with_fee * reserve_out;
    let denominator = reserve_in * 1000 + amount_in_with_fee;
    numerator / denominator
}