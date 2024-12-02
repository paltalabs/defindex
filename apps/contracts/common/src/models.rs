use soroban_sdk::{contracttype, Address, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Strategy {
    pub address: Address,
    pub name: String,
    pub paused: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetStrategySet {
    pub address: Address,
    pub strategies: Vec<Strategy>,
}