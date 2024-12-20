use soroban_sdk::{vec as sorobanvec, String, Vec, Map};

use crate::test::defindex_vault::{AssetStrategySet, ContractError};
use crate::test::{
    create_defindex_vault, create_strategy_params_token_0, create_strategy_params_token_1, DeFindexVaultTest
};



// test get_asset_amounts_per_shares function after every deposit
// do a bunch of deposits with different ratios and check that shares are calculated correctly
#[test]
fn deposit_several_assets_get_asset_amounts_per_shares() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();
    let strategy_params_token_0 = create_strategy_params_token_0(&test);
    let strategy_params_token_1 = create_strategy_params_token_1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token_0.address.clone(),
            strategies: strategy_params_token_0.clone()
        },
        AssetStrategySet {
            address: test.token_1.address.clone(),
            strategies: strategy_params_token_1.clone()
        }
    ];

    let defindex_contract = create_defindex_vault(
        &test.env,
        assets,
        test.manager.clone(),
        test.emergency_manager.clone(),
        test.vault_fee_receiver.clone(),
        2000u32,
        test.defindex_protocol_receiver.clone(),
        2500u32,
        test.defindex_factory.clone(),
        test.soroswap_router.address.clone(),
        sorobanvec![&test.env, String::from_str(&test.env, "dfToken"), String::from_str(&test.env, "DFT")],
    );

    let amount0 = 123456789i128;
    let amount1 = 987654321i128;

    let users = DeFindexVaultTest::generate_random_users(&test.env, 2);

    // Balances before deposit
    test.token_0_admin_client.mint(&users[0], &amount0);
    test.token_1_admin_client.mint(&users[0], &amount1);
    let user_balance0 = test.token_0.balance(&users[0]);
    assert_eq!(user_balance0, amount0);
    let user_balance1 = test.token_1.balance(&users[0]);
    assert_eq!(user_balance1, amount1);

    let df_balance = defindex_contract.balance(&users[0]);
    assert_eq!(df_balance, 0i128);

    // deposit
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount0, amount1],
        &sorobanvec![&test.env, amount0, amount1],
        &users[0],
        &false,
    );

    // function is fn get_asset_amounts_per_shares(e: Env, vault_shares: i128) -> Map<Address, i128>
    // get several results of the function using different vault_shares
    let result1 = defindex_contract.get_asset_amounts_per_shares(&0i128);
    let result2 = defindex_contract.get_asset_amounts_per_shares(&1000i128);
    let result3 = defindex_contract.get_asset_amounts_per_shares(&2000i128);
    let result4 = defindex_contract.get_asset_amounts_per_shares(&3000i128);
    let result5 = defindex_contract.get_asset_amounts_per_shares(&4000i128);
    let result6 = defindex_contract.get_asset_amounts_per_shares(&5000i128);

    // calculate result1_should by hand (put aritmentic as a comment) and check that results are ok
    // result1_should = {token_0: 0, token_1: 0}
    let mut result1_should = Map::new(&test.env);
    result1_should.set(test.token_0.address.clone(), 0i128);
    result1_should.set(test.token_1.address.clone(), 0i128);
    assert_eq!(result1, result1_should);

    // next we will consider that total shares are amount0 + amount1 = 123456789 + 987654321 = 1111111110
    // and we will calculate the shares for each asset
    // amount should 1 for token_0: 
    // amount0 * shares 0 = 123456789 * 1000 = 123456789000
    // amount 0 * shares 0 / total supply = 123456789000 / 1111111110 = 111.111110211
    // because truncating, amount should be 111
    
    // amount should 1 for token_1:
    // amount1 * shares 0 = 987654321 * 1000 = 987654321000
    // amount 1 * shares 0 / total supply = 987654321000 / 1111111110 = 888.888889789
    // because truncating, amount should be 888
    // result2_should = {token_0: 111, token_1: 888}
    let mut result2_should = Map::new(&test.env);
    result2_should.set(test.token_0.address.clone(), 111i128);
    result2_should.set(test.token_1.address.clone(), 888i128);
    assert_eq!(result2, result2_should);

    // amount should 2 for token_0:
    // amount0 * shares 0 = 123456789 * 2000 = 246913578000
    // amount 0 * shares 0 / total supply = 246913578000 / 1111111110 = 222.222220422
    // because truncating, amount should be 222
    
    // amount should 2 for token_1:
    // amount1 * shares 0 = 987654321 * 2000 = 1975308642000
    // amount 1 * shares 0 / total supply = 1975308642000 / 1111111110 = 1777.777779578
    // because truncating, amount should be 1777
    // result3_should = {token_0: 222, token_1: 1777}
    let mut result3_should = Map::new(&test.env);
    result3_should.set(test.token_0.address.clone(), 222i128);
    result3_should.set(test.token_1.address.clone(), 1777i128);
    assert_eq!(result3, result3_should);

    // amount should 3 for token_0:
    // amount0 * shares 0 = 123456789 * 3000 = 370370367000
    // amount 0 * shares 0 / total supply = 370370367000 / 1111111110 = 333.333330633
    // because truncating, amount should be 333
    
    // amount should 3 for token_1:
    // amount1 * shares 0 = 987654321 * 3000 = 2962962963000
    // amount 1 * shares 0 / total supply = 2962962963000 / 1111111110 = 2666.666670633
    // because truncating, amount should be 2666
    // result4_should = {token_0: 333, token_1: 2666}
    let mut result4_should = Map::new(&test.env);
    result4_should.set(test.token_0.address.clone(), 333i128);
    result4_should.set(test.token_1.address.clone(), 2666i128);
    assert_eq!(result4, result4_should);

    // amount should 4 for token_0:
    // amount0 * shares 0 = 123456789 * 4000 = 493827156000
    // amount 0 * shares 0 / total supply = 493827156000 / 1111111110 = 444.444440844
    // because truncating, amount should be 444
    
    // amount should 4 for token_1:
    // amount1 * shares 0 = 987654321 * 4000 = 3950617284000
    // amount 1 * shares 0 / total supply = 3950617284000 / 1111111110 = 3555.555561844
    // because truncating, amount should be 3555
    // result5_should = {token_0: 444, token_1: 3555}
    let mut result5_should = Map::new(&test.env);
    result5_should.set(test.token_0.address.clone(), 444i128);
    result5_should.set(test.token_1.address.clone(), 3555i128);
    assert_eq!(result5, result5_should);

    // amount should 5 for token_0:
    // amount0 * shares 0 = 123456789 * 5000 = 617283945000
    // amount 0 * shares 0 / total supply = 617283945000 / 1111111110 = 555.555550055
    // because truncating, amount should be 555
    
    // amount should 5 for token_1:
    // amount1 * shares 0 = 987654321 * 5000 = 4938271605000
    // amount 1 * shares 0 / total supply = 4938271605000 / 1111111110 = 4444.444450055
    // because truncating, amount should be 4444
    // result6_should = {token_0: 555, token_1: 4444}
    let mut result6_should = Map::new(&test.env);
    result6_should.set(test.token_0.address.clone(), 555i128);
    result6_should.set(test.token_1.address.clone(), 4444i128);
    assert_eq!(result6, result6_should);

    // *************************************************
    // now we will consider an amount over total supply , we should get error AmountOverTotalSupply
    let result7 = defindex_contract.try_get_asset_amounts_per_shares(&1111111111i128);
    assert_eq!(result7, Err(Ok(ContractError::AmountOverTotalSupply)));

}

#[test]
fn deposit_and_invest_several_assets_get_asset_amounts_per_shares() {
    todo!();

}
