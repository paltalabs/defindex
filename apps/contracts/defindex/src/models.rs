use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Strategy {
    pub address: Address,
    pub name: String,
    pub paused: bool,
    pub ratio: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Asset {
    pub address: Address,
    pub strategies: Vec<Strategy>,
}
