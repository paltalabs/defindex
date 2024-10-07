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
pub struct AssetAllocation {
    pub address: Address,
    pub ratio: i128,
    pub strategies: Vec<Strategy>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Investment {
    pub strategy: Address,
    pub amount: i128,
}