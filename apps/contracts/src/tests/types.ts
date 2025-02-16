import { Address } from "@stellar/stellar-sdk";

export interface  StrategyAllocations {
  amount: bigint,
  paused: boolean,
  strategy_address: string
}
export interface TotalManagedFunds {
  asset: string;
  idle_amount: bigint;
  invested_amount: bigint;
  total_amount: bigint;
  strategy_allocations: StrategyAllocations[];
}
export interface AssetInvestmentAllocation {
  asset: Address;
  strategy_investments: { amount: bigint; strategy: Address }[];
}

export interface AssetInvestmentAllocation {
  asset: Address;
  strategy_investments: { amount: bigint; strategy: Address }[];
}

export interface CreateVaultParams {
  address: Address;
  strategies: Array<{
    name: string;
    address: string;
    paused: boolean;
  }>;
}