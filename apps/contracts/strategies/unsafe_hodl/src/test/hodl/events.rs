use crate::event::{DepositEvent, HarvestEvent, WithdrawEvent};
use crate::test::{create_hodl_strategy, HodlStrategyTest};
use soroban_sdk::{symbol_short, testutils::Events, vec, IntoVal};

#[test]
fn deposit() {
    let test = HodlStrategyTest::setup();
    let strategy = create_hodl_strategy(&test.env, &test.token.address);

    let amount = 123456;
    strategy.deposit(&amount, &test.user);

    let deposit_event = test.env.events().all().last().unwrap();
    let expected_deposit_event = DepositEvent {
        amount,
        from: test.user,
    };

    assert_eq!(
        vec![&test.env, deposit_event.clone()],
        vec![
            &test.env,
            (
                strategy.address.clone(),
                ("HodlStrategy", symbol_short!("deposit")).into_val(&test.env),
                (expected_deposit_event).into_val(&test.env)
            )
        ]
    );
}

#[test]
fn withdraw() {
    let test = HodlStrategyTest::setup();
    let strategy = create_hodl_strategy(&test.env, &test.token.address);
    let amount_to_deposit = 987654321;
    strategy.deposit(&amount_to_deposit, &test.user);

    let amount_to_withdraw = 123456;
    strategy.withdraw(&amount_to_withdraw, &test.user, &test.user);
    let withdraw_event = test.env.events().all().last().unwrap();
    let expected_withdraw_event = WithdrawEvent {
        amount: amount_to_withdraw,
        from: test.user,
    };

    assert_eq!(
        vec![&test.env, withdraw_event.clone()],
        vec![
            &test.env,
            (
                strategy.address.clone(),
                ("HodlStrategy", symbol_short!("withdraw")).into_val(&test.env),
                (expected_withdraw_event).into_val(&test.env)
            )
        ]
    );
}

#[test]
fn harvest() {
    let test = HodlStrategyTest::setup();
    let strategy = create_hodl_strategy(&test.env, &test.token.address);

    let amount = 123456;
    strategy.deposit(&amount, &test.user);
    strategy.harvest(&test.user);

    let harvest_event = test.env.events().all().last().unwrap();
    let expected_harvest_event = HarvestEvent {
        amount: 0i128,
        from: test.user,
    };

    assert_eq!(
        vec![&test.env, harvest_event.clone()],
        vec![
            &test.env,
            (
                strategy.address.clone(),
                ("HodlStrategy", symbol_short!("harvest")).into_val(&test.env),
                (expected_harvest_event).into_val(&test.env)
            )
        ]
    );
}
