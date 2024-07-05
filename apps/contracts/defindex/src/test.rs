#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Allocator);
    let client = AllocatorClient::new(&env, &contract_id);

    let client_initialize = client.initialize(
        &String::from_slice(
            &env,
            "CB74KXQXEGKGPU5C5FI22X64AGQ63NANVLRZBS22SSCMLJDXNHED72MO",
        ),
        &vec![1, 2, 3],
        &vec![Address::default(), Address::default()],
    );

    // let client_default_title = client.read_title();
    // assert_eq!(
    //     client_default_title,
    //     String::from_slice(&env, "Default Title")
    // );

    // client.set_title(&String::from_slice(&env, "Trying my new title"));
    // let client_new_title = client.read_title();

    // assert_eq!(
    //     client_new_title,
    //     String::from_slice(&env, "You will never be able to change this title anymmore")
    // );
}
