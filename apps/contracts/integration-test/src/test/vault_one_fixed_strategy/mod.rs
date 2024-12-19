use super::ONE_YEAR_IN_SECONDS;

mod deposit;
mod withdraw;

mod fee_performance;
pub fn calculate_yield(user_balance: i128, apr: u32, time_elapsed: u64) -> i128 {
    // Calculate yield based on the APR, time elapsed, and user's balance
    let seconds_per_year = ONE_YEAR_IN_SECONDS;
    let apr_bps = apr as i128;
    let time_elapsed_i128 = time_elapsed as i128;

    (user_balance * apr_bps * time_elapsed_i128) / (seconds_per_year as i128 * 10000)
}