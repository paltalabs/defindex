use soroban_sdk::{vec, Address, Env, Vec};

use crate::storage::{get_blend_pool, get_underlying_asset};

soroban_sdk::contractimport!(
  file = "../external_wasms/blend/blend_pool.wasm"
);
pub type BlendPoolClient<'a> = Client<'a>;

// Define the RequestType enum with explicit u32 values
#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum RequestType {
    // Supply = 0,
    // Withdraw = 1,
    SupplyCollateral = 2,
    WithdrawCollateral = 3,
    // Borrow = 4,
    // Repay = 5,
    // FillUserLiquidationAuction = 6,
    // FillBadDebtAuction = 7,
    // FillInterestAuction = 8,
    // DeleteLiquidationAuction = 9,
}

// Implement a method to convert RequestType to u32
impl RequestType {
    fn to_u32(self) -> u32 {
        self as u32
    }
}

pub fn submit(e: &Env, from: &Address, amount: i128, request_type: RequestType) -> Positions {
    // Setting up Blend Pool client
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    let underlying_asset = get_underlying_asset(&e);

    let requests: Vec<Request> = vec![&e, Request {
        address: underlying_asset,
        amount: amount,
        request_type: request_type.to_u32(),
    }];

    blend_pool_client.submit(from, from, from, &requests)
}

pub fn claim(e: &Env, from: &Address) -> i128 {
    // Setting up Blend Pool client
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    blend_pool_client.claim(from, &vec![&e, 3u32], from)
}

pub fn get_positions(e: &Env, from: &Address) -> Positions {
    // Setting up Blend Pool client
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    blend_pool_client.get_positions(from)
}