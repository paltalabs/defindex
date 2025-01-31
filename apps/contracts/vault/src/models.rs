use soroban_sdk::{contracttype, Address, Vec};

// Investment Allocation in Strategies
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StrategyAllocation {
    pub strategy_address: Address,
    pub amount: i128,
    pub paused: bool,
}

// Current Asset Investment Allocation
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentAssetInvestmentAllocation {
    pub asset: Address,
    pub total_amount: i128,
    pub idle_amount: i128,
    pub invested_amount: i128,
    pub strategy_allocations: Vec<StrategyAllocation>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetInvestmentAllocation {
    pub asset: Address,
    pub strategy_allocations: Vec<Option<StrategyAllocation>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    /// Withdraw funds from a strategy.
    Unwind(Address, i128), // (strategy, amount)

    /// Invest funds into a strategy.
    Invest(Address, i128), // (strategy, amount)

    /// Perform a swap with an exact input amount.
    SwapExactIn(
        Address, // token_in
        Address, // token_out
        i128,    // amount_in
        i128,    // amount_out_min
        u64,     // deadline
    ),

    /// Perform a swap with an exact output amount.
    SwapExactOut(
        Address, // token_in
        Address, // token_out
        i128,    // amount_out
        i128,    // amount_in_max
        u64,     // deadline
    ),
    // /// Placeholder for zap operations (commented for future use).
    // Zapper(Vec<ZapperInstruction>), // instructions
}
