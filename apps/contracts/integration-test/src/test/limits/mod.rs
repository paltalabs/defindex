use soroban_sdk::Env;

mod asset_n_strategies;

pub const CPU_LIMIT: u64 = 100000000;
pub const MEM_LIMIT: u64 = 41943040;

pub fn check_limits(e: &Env, name: &str) {
    let cpu_used = e.budget().cpu_instruction_cost();
    let mem_used = e.budget().memory_bytes_cost();
    println!("{} CPU Instructions: {:?}", name, cpu_used);
    println!("{} MEMORY: {:?}", name, mem_used);
    println!("===========================================");
    assert!(cpu_used <= CPU_LIMIT, "CPU instructions exceeded limit");
    assert!(mem_used <= MEM_LIMIT, "Memory usage exceeded limit");
}