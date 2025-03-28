use soroban_sdk::Env;

mod asset_n_strategies;
// mod rebalance;
// mod n_asset_one_strategy;

pub const CPU_LIMIT: u64 = 100000000;
pub const MEM_LIMIT: u64 = 41943040;

pub fn check_limits(e: &Env, message: &str) {
    let cpu_used = e.cost_estimate().budget().cpu_instruction_cost();
    let mem_used = e.cost_estimate().budget().memory_bytes_cost();
    println!("{} CPU Instructions: {:?}", message, cpu_used);
    println!("{} MEMORY: {:?}", message, mem_used);
    println!("===========================================");
    assert!(cpu_used <= CPU_LIMIT, "CPU instructions exceeded limit");
    assert!(mem_used <= MEM_LIMIT, "Memory usage exceeded limit");
}
pub fn check_limits_return_info(e: &Env, message: &str) -> (String, u64, u64) {
    let cpu_used = e.cost_estimate().budget().cpu_instruction_cost();
    let mem_used = e.cost_estimate().budget().memory_bytes_cost();
    println!("{} CPU Instructions: {:?}", message, cpu_used);
    println!("{} MEMORY: {:?}", message, mem_used);
    println!("===========================================");
    (message.to_string(), cpu_used, mem_used)
}

pub fn create_results_table(e: &Env, data: Vec<(String, u64, u64)>) {
    let mut table = vec![
        vec!["Message".to_string(), "CPU Instructions".to_string(), "Memory".to_string()],
    ];
    for (message, cpu_used, mem_used) in data {
        assert!(cpu_used <= CPU_LIMIT, "🟥 {} CPU instructions exceeded limit", message);
        assert!(mem_used <= MEM_LIMIT, "🟥 {} Memory usage exceeded limit", message);
        table.push(vec![message, cpu_used.to_string(), mem_used.to_string()]);
    }

    println!("===========================================");
    println!("Results Table:");
    for row in &table {
        println!("✅{:?}", row);
    }
    println!("===========================================");
    assert!(table.len() > 1, "No data to display in the table");
    assert!(table[1].len() == 3, "Table format is incorrect");
    assert!(table[1][0] != "", "Message column is empty");
    assert!(table[1][1].parse::<u64>().is_ok(), "CPU Instructions column is not a number");
    assert!(table[1][2].parse::<u64>().is_ok(), "Memory column is not a number");
    assert!(table[1][1].parse::<u64>().unwrap() <= CPU_LIMIT, "🟥 {} CPU instructions exceeded limit", table[1][0]);
    assert!(table[1][2].parse::<u64>().unwrap() <= MEM_LIMIT, "🟥 {} Memory usage exceeded limit", table[1][0]);
    println!("===========================================");
    println!("Results Table created successfully");
    println!("===========================================");

}