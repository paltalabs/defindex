use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterParams {
    pub index: u32,
    pub share: u32,
    pub address: Address,
}