//#![no_std]

mod allowance;
mod balance;
mod contract;
mod metadata;
mod storage_types;
mod total_supply;

pub use contract::VaultTokenClient; 
pub use contract::VaultToken;
pub use contract::{internal_mint, internal_burn};
pub use metadata::write_metadata;
