use soroban_sdk::Env;

mod asset_n_strategies;
// mod rebalance;
// mod n_asset_one_strategy;

// limits got from https://developers.stellar.org/docs/networks/resource-limits-fees
pub const CPU_LIMIT: u64 = 100000000;
pub const MEM_LIMIT: u64 = 41943040;
pub const READ_ENTRIES_LIMIT: u32 = 40;
pub const WRITE_ENTRIES_LIMIT: u32 = 25;
pub const READ_BYTES_LIMIT: u32 =  204800 ;
pub const WRITE_BYTES_LIMIT: u32 = 132096;

pub fn check_limits(e: &Env, message: &str) {
    let cpu_used = e.cost_estimate().budget().cpu_instruction_cost();
    let mem_used = e.cost_estimate().budget().memory_bytes_cost();
    println!("{} CPU Instructions: {:?}", message, cpu_used);
    println!("{} MEMORY: {:?}", message, mem_used);
    println!("===========================================");
    assert!(cpu_used <= CPU_LIMIT, "CPU instructions exceeded limit");
    assert!(mem_used <= MEM_LIMIT, "Memory usage exceeded limit");
}
pub fn check_limits_return_info(e: &Env, message: &str) -> (String, u64, u64, u32, u32, u32, u32) {
    let cost_estimate = e.cost_estimate();
    let cpu_used = cost_estimate.budget().cpu_instruction_cost();
    let mem_used = cost_estimate.budget().memory_bytes_cost();
    let resources = cost_estimate.resources();
    println!("{} CPU Instructions: {:?}", message, cpu_used);
    println!("{} MEMORY: {:?}", message, mem_used);
    println!("read_entries: {}", resources.read_entries);
    println!("write_entries: {}", resources.write_entries);
    println!("read_bytes: {}", resources.read_bytes);
    println!("write_bytes: {}", resources.write_bytes);
    println!("===========================================");
    (
        message.to_string(), 
        cpu_used, 
        mem_used, 
        resources.read_entries, 
        resources.write_entries, 
        resources.read_bytes, 
        resources.write_bytes
    )
}

pub fn print_resources(e: &Env, message: &str) {
    let resources = e.cost_estimate().resources();
    println!("{}", message);
    println!("{:?}", resources);
    println!("===========================================");
}

pub fn create_results_table(e: &Env, data: Vec<(String, u64, u64, u32, u32, u32, u32)>) {
    let mut table: Vec<(String, u64, u64, u32, u32, u32, u32)> = vec![];
    let header = vec![
        "Message".to_string(), 
        "CPU Instructions".to_string(), 
        "Memory".to_string(),
        "Read Entries".to_string(),
        "Write Entries".to_string(),
        "Read Bytes".to_string(),
        "Write Bytes".to_string()
    ];
    
    for (message, cpu_used, mem_used, read_entries, write_entries, read_bytes, write_bytes) in data.clone() {
        table.push((message, cpu_used, mem_used, read_entries, write_entries, read_bytes, write_bytes));
    }

    println!("|{:-<27}+{:-<21}+{:-<13}+{:-<15}+{:-<16}+{:-<14}+{:-<15}|", "", "", "", "", "", "", "");
    println!("| {:<26}| {:<20}| {:<12}| {:<14}| {:<15}| {:<13}| {:<14}|", 
        header[0], header[1], header[2], header[3], header[4], header[5], header[6]);
    println!("|{:-<27}+{:-<21}+{:-<13}+{:-<15}+{:-<16}+{:-<14}+{:-<15}|", "", "", "", "", "", "", "");
    
    // Print the limits header
    println!("| {:<26}| {:<20}| {:<12}| {:<14}| {:<15}| {:<13}| {:<14}|", 
        "Limits", CPU_LIMIT, MEM_LIMIT, READ_ENTRIES_LIMIT, WRITE_ENTRIES_LIMIT, READ_BYTES_LIMIT, WRITE_BYTES_LIMIT);
    println!("|{:-<27}+{:-<21}+{:-<13}+{:-<15}+{:-<16}+{:-<14}+{:-<15}|", "", "", "", "", "", "", "");
    for row in &table {
        let (message, cpu_used, mem_used, _, _, _, _) = (&row.0, &row.1, &row.2, &row.3, &row.4, &row.5, &row.6);
        if (cpu_used >= &CPU_LIMIT) || (mem_used >= &MEM_LIMIT) {
            println!("|🟥{:<25}| {:<20}| {:<12}| {:<14}| {:<15}| {:<13}| {:<14}|", 
                row.0, row.1, row.2, row.3, row.4, row.5, row.6);
        } else {
            println!("|{:<27}| {:<20}| {:<12}| {:<14}| {:<15}| {:<13}| {:<14}|", 
                row.0, row.1, row.2, row.3, row.4, row.5, row.6);
        }
    }
    println!("|{:-<27}-{:-<21}-{:-<13}-{:-<15}-{:-<16}-{:-<14}-{:-<15}|", "", "", "", "", "", "", "");

    for (message, cpu_used, mem_used, read_entries, write_entries, read_bytes, write_bytes) in data.clone() {
        assert!(cpu_used <= CPU_LIMIT, "🟥 {} CPU instructions exceeded limit", message);
        assert!(mem_used <= MEM_LIMIT, "🟥 {} Memory usage exceeded limit", message);
        assert!(read_entries <= READ_ENTRIES_LIMIT, "🟥 {} Read entries exceeded limit", message);
        assert!(write_entries <= WRITE_ENTRIES_LIMIT, "🟥 {} Write entries exceeded limit", message);
        assert!(read_bytes <= READ_BYTES_LIMIT, "🟥 {} Read bytes exceeded limit", message);
        assert!(write_bytes <= WRITE_BYTES_LIMIT, "🟥 {} Write bytes exceeded limit", message);
    }
}