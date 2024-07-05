use soroban_sdk::{contracttype, Vec, Address, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterParams {
    pub index: u32,
    pub share: u32,
    pub address: Address,
}