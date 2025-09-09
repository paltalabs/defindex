export interface InvestmentData {
  year: number;
  totalInvested: number;
  projectedTotal: number;
  earnings: number;
}

export function calculateInvestmentGrowth(
  initialDeposit: number,
  monthlyContribution: number,
  annualInterestRate: number,
  years: number
): InvestmentData[] {
  const data: InvestmentData[] = [];
  const dailyRate = annualInterestRate / 100 / 365; // Daily compounding rate
  
  for (let year = 1; year <= years; year++) {
    const days = year * 365;
    const months = year * 12;
    
    // Calculate total invested (initial + monthly contributions)
    const totalInvested = initialDeposit + (monthlyContribution * months);
    
    // Calculate compound interest for initial deposit with daily compounding
    const initialGrowth = initialDeposit * Math.pow(1 + dailyRate, days);
    
    // Calculate compound interest for monthly contributions with daily compounding
    // We need to calculate each monthly contribution's growth from when it was added
    let monthlyContributionsGrowth = 0;
    for (let month = 1; month <= months; month++) {
      const daysInvested = (months - month) * 30.44 + (days % 365 > (month - 1) * 30.44 ? days % 365 - (month - 1) * 30.44 : 0);
      monthlyContributionsGrowth += monthlyContribution * Math.pow(1 + dailyRate, daysInvested);
    }
    
    const projectedTotal = initialGrowth + monthlyContributionsGrowth;
    const earnings = projectedTotal - totalInvested;
    
    data.push({
      year,
      totalInvested: Math.round(totalInvested),
      projectedTotal: Math.round(projectedTotal),
      earnings: Math.round(earnings)
    });
  }
  
  return data;
}

export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'USD',
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(amount);
}

export function formatNumber(amount: number): string {
  return new Intl.NumberFormat('en-US', {
    minimumFractionDigits: 0,
    maximumFractionDigits: 0,
  }).format(amount);
}