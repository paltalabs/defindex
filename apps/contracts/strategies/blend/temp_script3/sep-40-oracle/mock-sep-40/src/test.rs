#![cfg(test)]

use sep_40_oracle::Asset;
use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    vec, Address, Env, Symbol, Vec,
};

use super::*;

fn setup_price_feed_oracle<'a>(
    env: &Env,
    admin: &Address,
    base: &Asset,
    assets: &Vec<Asset>,
    decimals: u32,
    resolution: u32,
) -> (Address, MockOracleClient<'a>) {
    let oracle_id = Address::generate(env);
    env.register_contract(&oracle_id, MockOracle {});
    let oracle_client = MockOracleClient::new(env, &oracle_id);
    oracle_client.set_data(admin, base, assets, &decimals, &resolution);
    (oracle_id, oracle_client)
}

#[test]
fn test_stable_price_feed() {
    let env = Env::default();
    env.mock_all_auths();
    let start_time = 1441065600;
    let start_block = 123;
    env.ledger().set(LedgerInfo {
        timestamp: start_time,
        protocol_version: 20,
        sequence_number: start_block,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 6312000,
    });

    let bomadil = Address::generate(&env);
    let base = Asset::Other(Symbol::new(&env, "USD"));
    let asset_1 = Asset::Stellar(Address::generate(&env));
    let asset_2 = Asset::Other(Symbol::new(&env, "EURO"));

    let (_, oracle_client) = setup_price_feed_oracle(
        &env,
        &bomadil,
        &base,
        &vec![&env, asset_1.clone(), asset_2.clone()],
        7,
        300,
    );

    let prices: Vec<i128> = vec![&env, 94_234_1234567, 1_1021304];
    oracle_client.set_price_stable(&prices);

    // verify price data can be fetched
    let price_1 = oracle_client.lastprice(&asset_1).unwrap();
    assert_eq!(price_1.price, prices.get_unchecked(0));
    assert_eq!(price_1.timestamp, start_time);

    let price_2 = oracle_client.lastprice(&asset_2).unwrap();
    assert_eq!(price_2.price, prices.get_unchecked(1));
    assert_eq!(price_2.timestamp, start_time);

    // pass time
    env.ledger().set(LedgerInfo {
        timestamp: start_time + 6 * 24 * 60 * 60,
        protocol_version: 20,
        sequence_number: start_block + (6 * 24 * 60 * 60) / 5,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 6312000,
    });

    // verify price data can still be fetched and timestamp adapts
    let price_1 = oracle_client.lastprice(&asset_1).unwrap();
    assert_eq!(price_1.price, prices.get_unchecked(0));
    assert_eq!(price_1.timestamp, start_time + 6 * 24 * 60 * 60);

    let price_2 = oracle_client.lastprice(&asset_2).unwrap();
    assert_eq!(price_2.price, prices.get_unchecked(1));
    assert_eq!(price_2.timestamp, start_time + 6 * 24 * 60 * 60);
}

#[test]
fn test_price_feed() {
    let env = Env::default();
    env.mock_all_auths();
    let start_time = 1441065600;
    let start_block = 123;
    env.ledger().set(LedgerInfo {
        timestamp: start_time,
        protocol_version: 20,
        sequence_number: start_block,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 6312000,
    });

    let bomadil = Address::generate(&env);
    let base = Asset::Other(Symbol::new(&env, "USD"));
    let asset_1 = Asset::Stellar(Address::generate(&env));
    let asset_2 = Asset::Other(Symbol::new(&env, "EURO"));

    let (_, oracle_client) = setup_price_feed_oracle(
        &env,
        &bomadil,
        &base,
        &vec![&env, asset_1.clone(), asset_2.clone()],
        7,
        300,
    );

    let prices_1: Vec<i128> = vec![&env, 94_234_1234567, 1_1021304];
    oracle_client.set_price(&prices_1, &start_time);

    // verify price data can be fetched
    let result_1 = oracle_client.lastprice(&asset_1).unwrap();
    assert_eq!(result_1.price, prices_1.get_unchecked(0));
    assert_eq!(result_1.timestamp, start_time);

    let result_2 = oracle_client.lastprice(&asset_2).unwrap();
    assert_eq!(result_2.price, prices_1.get_unchecked(1));
    assert_eq!(result_2.timestamp, start_time);

    // pass time
    env.ledger().set(LedgerInfo {
        timestamp: start_time + 325,
        protocol_version: 20,
        sequence_number: start_block + 325 / 5,
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 6312000,
    });

    // verify price data can still be fetched and timestamp does not
    let result_1 = oracle_client.lastprice(&asset_1).unwrap();
    assert_eq!(result_1.price, prices_1.get_unchecked(0));
    assert_eq!(result_1.timestamp, start_time);

    let result_2 = oracle_client.lastprice(&asset_2).unwrap();
    assert_eq!(result_2.price, prices_1.get_unchecked(1));
    assert_eq!(result_2.timestamp, start_time);

    // set another round of prices
    let prices_2: Vec<i128> = vec![&env, 95_214_7654321, 1_1040921];
    oracle_client.set_price(&prices_2, &(start_time + 300));

    // verify most recent prices are fetched
    let result_1 = oracle_client.lastprice(&asset_1).unwrap();
    assert_eq!(result_1.price, prices_2.get_unchecked(0));
    assert_eq!(result_1.timestamp, start_time + 300);

    let result_2 = oracle_client.lastprice(&asset_2).unwrap();
    assert_eq!(result_2.price, prices_2.get_unchecked(1));
    assert_eq!(result_2.timestamp, start_time + 300);

    // verify old prices can be fetched
    let result_1 = oracle_client.price(&asset_1, &start_time).unwrap();
    assert_eq!(result_1.price, prices_1.get_unchecked(0));
    assert_eq!(result_1.timestamp, start_time);

    let result_2 = oracle_client.price(&asset_2, &start_time).unwrap();
    assert_eq!(result_2.price, prices_1.get_unchecked(1));
    assert_eq!(result_2.timestamp, start_time);

    // verify get prices can fetch both
    let result_1_vec = oracle_client.prices(&asset_1, &2).unwrap();
    assert_eq!(result_1_vec.len(), 2);
    let result_1_0 = result_1_vec.get_unchecked(0);
    assert_eq!(result_1_0.price, prices_2.get_unchecked(0));
    assert_eq!(result_1_0.timestamp, start_time + 300);
    let result_1_1 = result_1_vec.get_unchecked(1);
    assert_eq!(result_1_1.price, prices_1.get_unchecked(0));
    assert_eq!(result_1_1.timestamp, start_time);

    let result_2_vec = oracle_client.prices(&asset_2, &2).unwrap();
    assert_eq!(result_2_vec.len(), 2);
    let result_2_0 = result_2_vec.get_unchecked(0);
    assert_eq!(result_2_0.price, prices_2.get_unchecked(1));
    assert_eq!(result_2_0.timestamp, start_time + 300);
    let result_2_1 = result_2_vec.get_unchecked(1);
    assert_eq!(result_2_1.price, prices_1.get_unchecked(1));
    assert_eq!(result_2_1.timestamp, start_time);
}
