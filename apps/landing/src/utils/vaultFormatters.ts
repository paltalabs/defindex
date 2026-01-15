import type { ManagedFunds } from '@/types/vault.types';
import { getTokenDecimals, getTokenSymbol } from '@/lib/tokenIcons';

/**
 * Format a large number with K/M/B suffixes
 */
function formatCompactNumber(value: number): string {
  if (value >= 1_000_000_000) {
    return `${(value / 1_000_000_000).toFixed(2)}B`;
  }
  if (value >= 1_000_000) {
    return `${(value / 1_000_000).toFixed(2)}M`;
  }
  if (value >= 1_000) {
    return `${(value / 1_000).toFixed(2)}K`;
  }
  return value.toFixed(2);
}

/**
 * Format token amount from stroops to human-readable format with symbol
 * @param amount - Amount in stroops (smallest unit)
 * @param symbol - Token symbol (e.g., 'USDC')
 * @param decimals - Number of decimal places (default 7 for Stellar)
 */
export function formatTokenAmount(
  amount: string | number,
  symbol: string,
  decimals = 7
): string {
  const value = typeof amount === 'string' ? BigInt(amount) : BigInt(Math.floor(amount));
  const divisor = BigInt(10 ** decimals);
  const intPart = value / divisor;
  const fracPart = value % divisor;

  const numValue = Number(intPart) + Number(fracPart) / Number(divisor);
  const formatted = formatCompactNumber(numValue);

  return `${formatted} ${symbol}`;
}

/**
 * Format token amount using contract address for symbol and decimals lookup
 */
export function formatTokenAmountByAddress(
  amount: string | number,
  contractAddress: string
): string {
  const symbol = getTokenSymbol(contractAddress);
  const decimals = getTokenDecimals(contractAddress);
  return formatTokenAmount(amount, symbol, decimals);
}

/**
 * Format APY as percentage string
 */
export function formatAPY(apy: number | undefined): string {
  if (apy === undefined || apy === null) {
    return '0.00%';
  }
  return `${apy.toFixed(2)}%`;
}

/**
 * Calculate total TVL from managed funds array
 * Returns the sum of all total_amount values as a string
 */
export function calculateTotalTVL(managedFunds: ManagedFunds[]): string {
  const total = managedFunds.reduce((sum, fund) => {
    return sum + BigInt(fund.total_amount);
  }, BigInt(0));
  return total.toString();
}

/**
 * Calculate total TVL as a number (for sorting)
 */
export function calculateTotalTVLNumber(managedFunds: ManagedFunds[]): number {
  const total = managedFunds.reduce((sum, fund) => {
    return sum + BigInt(fund.total_amount);
  }, BigInt(0));
  return Number(total);
}

/**
 * Get the primary asset from managed funds (for display purposes)
 * Returns the first asset's address or empty string
 */
export function getPrimaryAssetAddress(managedFunds: ManagedFunds[]): string {
  return managedFunds[0]?.asset ?? '';
}

/**
 * Truncate address for display
 */
export function truncateAddress(address: string, chars = 4): string {
  if (address.length <= chars * 2 + 3) {
    return address;
  }
  return `${address.slice(0, chars)}...${address.slice(-chars)}`;
}
