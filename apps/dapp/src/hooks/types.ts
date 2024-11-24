/* 
#[contracttype]
struct StrategyInvestment {
  amount: i128,
  strategy: address
}

#[contracttype]
struct AssetInvestmentAllocation {
  asset: address,
  strategy_investments: vec<option<StrategyInvestment>>
} 
*/


export interface StrategyInvestment {
  amount: number;
  strategy: string;
}

export interface AssetInvestmentAllocation {
  asset: string;
  strategy_investments: StrategyInvestment[];
}