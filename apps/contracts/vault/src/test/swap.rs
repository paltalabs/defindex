// Testing that the test is correctly seted up XD
use soroban_sdk::{
    testutils::Address as _, 
    vec as sorobanvec, Address, String, Vec, 
    // Address, 
    // Env, 
    // String, 
    // Val, 
    // Vec, 
    // BytesN
};
use crate::test::{DeFindexVaultTest};
use crate::test::defindex_vault::{
    ActionType, AssetInvestmentAllocation, DexDistribution, Instruction, OptionalSwapDetailsExactIn,
    OptionalSwapDetailsExactOut, StrategyAllocation, SwapDetailsExactIn,
};
use crate::test::defindex_vault::{
    AssetStrategySet, ContractError, CurrentAssetInvestmentAllocation,
};

use crate::test::{
    create_defindex_vault, create_strategy_params_token0, create_strategy_params_token1
};
#[test]
fn test_swap() {
    let test = DeFindexVaultTest::setup();
    test.env.mock_all_auths();

    let strategy_params_token0 = create_strategy_params_token0(&test);
    let strategy_params_token1 = create_strategy_params_token1(&test);

    // initialize with 2 assets
    let assets: Vec<AssetStrategySet> = sorobanvec![
        &test.env,
        AssetStrategySet {
            address: test.token0.address.clone(),
            strategies: strategy_params_token0.clone()
        },
        AssetStrategySet {
            address: test.token1.address.clone(),
            strategies: strategy_params_token1.clone()
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
        test.defindex_factory.clone(),
        String::from_str(&test.env, "dfToken"),
        String::from_str(&test.env, "DFT"),
    );

    let user = Address::generate(&test.env);
    // Mint tokens to user
    let amount_0 = 1000_0_000_000;
    let amount_1 = 200_0_000_000;
    test.token0_admin_client.mint(&user, &amount_0);
    test.token1_admin_client.mint(&user, &amount_1);

    // Deposit to vault
    defindex_contract.deposit(
        &sorobanvec![&test.env, amount_0, amount_1],
        &sorobanvec![&test.env, amount_0, amount_1],
        &user,
        &true,
    );
    
    let mut distribution_vec = Vec::new(&test.env);
    // add one with part 1 and other with part 0
    let mut path: Vec<Address> = Vec::new(&test.env);
    path.push_back(test.token0.address.clone());
    path.push_back(test.token1.address.clone());

    let distribution_0 = DexDistribution {
        protocol_id: String::from_str(&test.env, "soroswap"),
        path,
        parts: 1,
    };
    distribution_vec.push_back(distribution_0);

    test.token0_admin_client.mint(&defindex_contract.address.clone(), &100000000);

    // Rebalance from here on
    let instructions = sorobanvec![
        &test.env,
        // Instruction {
        //     action: ActionType::Withdraw,
        //     strategy: Some(test.strategy_client_token0.address.clone()),
        //     amount: Some(1000),
        //     swap_details_exact_in: OptionalSwapDetailsExactIn::None,
        //     swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        // },
        Instruction {
            action: ActionType::SwapExactIn,
            strategy: None,
            amount: None,
            swap_details_exact_in: OptionalSwapDetailsExactIn::Some(SwapDetailsExactIn {
                token_in: test.token0.address.clone(),
                token_out: test.token1.address.clone(),
                amount_in: 100,
                amount_out_min: 0,
                distribution: distribution_vec,
                deadline: test.env.ledger().timestamp() + 3600u64,
                router: test.soroswap_router.address.clone(),
                pair: test.soroswap_pair.clone(),
            }),
            swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        },
        // Instruction {
        //     action: ActionType::Invest,
        //     strategy: Some(test.strategy_client_token1.address.clone()),
        //     amount: Some(expected_swap_out?),
        //     swap_details_exact_in: OptionalSwapDetailsExactIn::None,
        //     swap_details_exact_out: OptionalSwapDetailsExactOut::None,
        // }
    ];

    defindex_contract.rebalance(&instructions);
    
}