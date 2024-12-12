use soroban_sdk::{Address, Env};

use crate::funds::fetch_strategy_invested_funds;

pub fn report(e: &Env, strategy: Address) -> (i128, i128) {
    let current_balance = fetch_strategy_invested_funds(e, &strategy);
    let prev_balance = get_prev_balance(strategy);
    let previous_gains_or_losses = get_gains_or_losses(strategy);
    
    let gains_or_losses = current_balance - prev_balance;
    let current_gains_or_losses = previous_gains_or_losses + gains_or_losses;
    
    store_gains_or_losses(strategy, current_gains_or_losses);
    store_prev_balance(strategy, current_balance);

    (0,0)
}

pub fn report_all_strategies() {
    for strategy in strategies {
        report(strategy);
    }
}