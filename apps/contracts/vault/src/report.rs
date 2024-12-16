use soroban_sdk::{Address, Env};

use crate::storage::{get_gains_or_losses, get_prev_balance, set_gains_or_losses, set_prev_balance};

pub fn report(e: &Env, strategy: &Address, current_balance: &i128) -> (i128, i128) {
    let prev_balance = get_prev_balance(e, strategy);
    let previous_gains_or_losses = get_gains_or_losses(e, strategy);
    
    let gains_or_losses = current_balance - prev_balance;
    let current_gains_or_losses = previous_gains_or_losses + gains_or_losses;
    
    set_gains_or_losses(e, &strategy, &current_gains_or_losses);
    set_prev_balance(e, &strategy, &current_balance);

    (current_balance.clone(), current_gains_or_losses)
}

// pub fn report_all_strategies() {
//     for strategy in strategies {
//         report(strategy);
//     }
// }