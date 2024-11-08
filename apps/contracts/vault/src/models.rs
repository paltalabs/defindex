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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Investment {
    pub strategy: Address,
    pub amount: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction {
    pub action: ActionType,
    pub strategy: Option<Address>,
    pub amount: Option<i128>,
    pub swap_details_exact_in: OptionalSwapDetailsExactIn,
    pub swap_details_exact_out: OptionalSwapDetailsExactOut,
    // pub zapper_instructions: Option<Vec<ZapperInstruction>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum ActionType {
    Withdraw = 0,
    Invest = 1,
    SwapExactIn = 2,
    SwapExactOut = 3,
    Zapper = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapDetailsExactIn {
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: i128,
    pub amount_out_min: i128,
    pub distribution: Vec<DexDistribution>,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapDetailsExactOut {
    pub token_in: Address,
    pub token_out: Address,
    pub amount_out: i128,
    pub amount_in_max: i128,
    pub distribution: Vec<DexDistribution>,
    pub deadline: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DexDistribution {
    pub protocol_id: String,
    pub path: Vec<Address>,
    pub parts: u32,
}

// Workaround for Option<SwapDetails> as it is not supported by the contracttype macro
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptionalSwapDetailsExactIn {
    Some(SwapDetailsExactIn),
    None,
}

// Workaround for Option<SwapDetails> as it is not supported by the contracttype macro
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OptionalSwapDetailsExactOut {
    Some(SwapDetailsExactOut),
    None,
}
