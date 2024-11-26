use soroban_sdk::{auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation}, vec, Address, Env, IntoVal, Symbol, Vec};

use crate::storage::get_blend_pool;

soroban_sdk::contractimport!(
  file = "../external_wasms/blend/blend_pool.wasm"
);
pub type BlendPoolClient<'a> = Client<'a>;

// Define the RequestType enum with explicit u32 values
#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum RequestType {
    Supply = 0,
    Withdraw = 1,
    // SupplyCollateral = 2,
    // WithdrawCollateral = 3,
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

pub fn supply(e: &Env, from: &Address, underlying_asset: Address, amount: i128) -> Positions {
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    let requests: Vec<Request> = vec![&e, Request {
        address: underlying_asset.clone(),
        amount,
        request_type: RequestType::Supply.to_u32(),
    }];

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: underlying_asset.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: (
                    e.current_contract_address(),
                    blend_pool_address.clone(),
                    amount.clone()).into_val(e),
            },
            sub_invocations: vec![&e],
        }),
    ]);
    
    blend_pool_client.submit(
        &from,
        &e.current_contract_address(),
        &from,
        &requests
    )
}

pub fn withdraw(e: &Env, from: &Address, underlying_asset: Address, amount: i128) -> Positions {
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    let requests: Vec<Request> = vec![&e, Request {
        address: underlying_asset.clone(),
        amount,
        request_type: RequestType::Withdraw.to_u32(),
    }];

    let new_positions = blend_pool_client.submit(
        &from,
        &from,
        &from,
        &requests
    );

    new_positions
}

pub fn claim(e: &Env, from: &Address) -> i128 {
    // Setting up Blend Pool client
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    // TODO: Check reserve_token_ids and how to get the correct one
    blend_pool_client.claim(from, &vec![&e, 3u32], from)
}

pub fn get_positions(e: &Env, from: &Address) -> Positions {
    // Setting up Blend Pool client
    let blend_pool_address = get_blend_pool(e);
    let blend_pool_client = BlendPoolClient::new(e, &blend_pool_address);

    blend_pool_client.get_positions(from)
}