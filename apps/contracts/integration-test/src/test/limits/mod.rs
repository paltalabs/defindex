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
    let mut table: Vec<(String, u64, u64)> = vec![];
    let header = vec!["Message".to_string(), "CPU Instructions".to_string(), "Memory".to_string()];
    for (message, cpu_used, mem_used) in data.clone() {
        table.push((message, cpu_used, mem_used));
    }

    println!("|{:-<27}+{:-<21}+{:-<13}|", "", "", "");
    println!("| {:<26}| {:<20}| {:<12}|", header[0], header[1], header[2]);
    println!("|{:-<27}+{:-<21}+{:-<13}|", "", "", "");
    for row in &table {
        let (message, cpu_used, mem_used) = (&row.0, &row.1, &row.2);
        if (cpu_used >= &CPU_LIMIT) || (mem_used >= &MEM_LIMIT) {
            println!("|🟥{:<25}| {:<20}| {:<12}|", row.0, row.1, row.2);
        } else {
            println!("|{:<27}| {:<20}| {:<12}|", row.0, row.1, row.2);
        }
    }
    println!("|{:-<27}-{:-<21}-{:-<13}|", "", "", "");

    for (message, cpu_used, mem_used) in data.clone() {
        assert!(cpu_used <= CPU_LIMIT, "🟥 {} CPU instructions exceeded limit", message);
        assert!(mem_used <= MEM_LIMIT, "🟥 {} Memory usage exceeded limit", message);
    }

}