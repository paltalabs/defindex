export interface RevenueInputs {
  activeUsers: number;
  avgBalance: number;
  adoptionRate: number; // 0-100 (percentage)
  walletFeePercent: number; // Total wallet fee charged to users (partner receives 50% of this; split handled in calculation logic)
  integrationCost: number;
}

export interface RevenueProjection {
  scenario: 'conservative' | 'moderate' | 'peak';
  apy: number;
  monthlyRevenue: number;
  yearlyRevenue: number;
}

export interface ROIMetrics {
  paybackWeeks: number;
  firstYearProfitMargin: number; // percentage
}

// APY scenarios (hardcoded)
export const APY_SCENARIOS = {
  conservative: 8,
  moderate: 12,
  peak: 18,
} as const;

/**
 * Calculate Total Value Locked (TVL)
 * TVL = activeUsers * avgBalance * (adoptionRate / 100)
 */
export function calculateTVL(inputs: RevenueInputs): number {
  const { activeUsers, avgBalance, adoptionRate } = inputs;
  return activeUsers * avgBalance * (adoptionRate / 100);
}

/**
 * Calculate yearly revenue for a given TVL, APY, and wallet fee
 * Partner keeps 50% of the wallet fee (DeFindex takes the other 50%)
 * Revenue = TVL * (APY / 100) * (walletFee / 100) * 0.5
 */
export function calculateRevenue(
  tvl: number,
  apy: number,
  walletFeePercent: number
): number {
  const partnerFeeShare = walletFeePercent / 2; // Partner keeps 50% of the fee
  return tvl * (apy / 100) * (partnerFeeShare / 100);
}

/**
 * Calculate revenue projections for all three scenarios
 */
export function calculateProjections(
  inputs: RevenueInputs
): RevenueProjection[] {
  const tvl = calculateTVL(inputs);

  const scenarios: Array<{
    scenario: 'conservative' | 'moderate' | 'peak';
    apy: number;
  }> = [
    { scenario: 'conservative', apy: APY_SCENARIOS.conservative },
    { scenario: 'moderate', apy: APY_SCENARIOS.moderate },
    { scenario: 'peak', apy: APY_SCENARIOS.peak },
  ];

  return scenarios.map(({ scenario, apy }) => {
    const yearlyRevenue = calculateRevenue(tvl, apy, inputs.walletFeePercent);
    const monthlyRevenue = yearlyRevenue / 12;

    return {
      scenario,
      apy,
      monthlyRevenue,
      yearlyRevenue,
    };
  });
}

/**
 * Calculate ROI metrics based on moderate scenario
 */
export function calculateROI(
  yearlyRevenue: number,
  integrationCost: number
): ROIMetrics {
  // Payback period in weeks
  const weeklyRevenue = yearlyRevenue / 52;
  const paybackWeeks = weeklyRevenue > 0 ? integrationCost / weeklyRevenue : 0;

  // First year profit margin: (revenue - cost) / revenue * 100
  const firstYearProfit = yearlyRevenue - integrationCost;
  const firstYearProfitMargin =
    yearlyRevenue > 0 ? (firstYearProfit / yearlyRevenue) * 100 : 0;

  return {
    paybackWeeks: Math.round(paybackWeeks * 10) / 10, // Round to 1 decimal
    firstYearProfitMargin: Math.round(firstYearProfitMargin * 10) / 10,
  };
}

/**
 * Format a number as currency (USD)
 */
export function formatCurrency(value: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(value);
}

/**
 * Format a large number with commas
 */
export function formatNumber(value: number): string {
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(value);
}

/**
 * Format a number with K/M suffix for large values
 */
export function formatCompactNumber(value: number): string {
  if (value >= 1000000) {
    return `${(value / 1000000).toFixed(1)}M`;
  }
  if (value >= 1000) {
    return `${(value / 1000).toFixed(0)}K`;
  }
  return value.toString();
}
