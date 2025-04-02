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
export interface RebalanceInstruction {
  action: ActionType;
  amount: number;
  strategy: string;
  swapDetailsExactIn?: any//SwapDetails;
  swapDetailsExactOut?: any//SwapDetails;
}
export enum ActionType {
  Unwind = 0,
  Invest = 1,
  SwapExactIn = 2,
  SwapExactOut = 3,
  Zapper = 4,
}