import {
  calculateProjections,
  calculateROI,
  calculateTVL,
  RevenueInputs,
  RevenueProjection,
  ROIMetrics,
} from '@/utils/revenueCalculations';
import { useCallback, useMemo, useState } from 'react';

export type ScenarioType = 'conservative' | 'moderate' | 'peak';

export interface UseRevenueCalculatorReturn {
  // Input values
  activeUsers: number;
  avgBalance: number;
  adoptionRate: number;
  walletFeePercent: number;
  integrationCost: number;
  selectedScenario: ScenarioType;

  // Setters
  setActiveUsers: (value: number) => void;
  setAvgBalance: (value: number) => void;
  setAdoptionRate: (value: number) => void;
  setWalletFeePercent: (value: number) => void;
  setIntegrationCost: (value: number) => void;
  setSelectedScenario: (scenario: ScenarioType) => void;

  // Computed values
  tvl: number;
  defindexFee: number;
  partnerFee: number;
  projections: RevenueProjection[];
  roiMetrics: ROIMetrics;
}

// Default values
const DEFAULT_ACTIVE_USERS = 34500;
const DEFAULT_AVG_BALANCE = 1000;
const DEFAULT_ADOPTION_RATE = 15;
const DEFAULT_WALLET_FEE_PERCENT = 10;
const DEFAULT_INTEGRATION_COST = 4000;

/**
 * Custom React hook for managing and calculating revenue projections, ROI, and related metrics
 * for a wallet integration scenario. Maintains state for user inputs and provides setters with validation.
 *
 * @returns {UseRevenueCalculatorReturn} An object containing input values, setters, and computed metrics.
 */
export function useRevenueCalculator(): UseRevenueCalculatorReturn {
  // State for inputs
  const [activeUsers, setActiveUsersState] = useState(DEFAULT_ACTIVE_USERS);
  const [avgBalance, setAvgBalanceState] = useState(DEFAULT_AVG_BALANCE);
  const [adoptionRate, setAdoptionRateState] = useState(DEFAULT_ADOPTION_RATE);
  const [walletFeePercent, setWalletFeePercentState] = useState(
    DEFAULT_WALLET_FEE_PERCENT
  );
  const [integrationCost, setIntegrationCostState] = useState(
    DEFAULT_INTEGRATION_COST
  );
  const [selectedScenario, setSelectedScenarioState] =
    useState<ScenarioType>('moderate');

  // Memoized setters with validation
  const setActiveUsers = useCallback((value: number) => {
    setActiveUsersState(Math.max(1, Math.min(10000000, value)));
  }, []);

  const setAvgBalance = useCallback((value: number) => {
    setAvgBalanceState(Math.max(1, value));
  }, []);

  const setAdoptionRate = useCallback((value: number) => {
    setAdoptionRateState(Math.max(5, Math.min(100, value)));
  }, []);

  const setWalletFeePercent = useCallback((value: number) => {
    setWalletFeePercentState(Math.max(0.5, Math.min(100, value)));
  }, []);

  const setIntegrationCost = useCallback((value: number) => {
    setIntegrationCostState(Math.max(0, value));
  }, []);

  const setSelectedScenario = useCallback((scenario: ScenarioType) => {
    setSelectedScenarioState(scenario);
  }, []);

  // Memoized input object
  const inputs: RevenueInputs = useMemo(
    () => ({
      activeUsers,
      avgBalance,
      adoptionRate,
      walletFeePercent,
      integrationCost,
    }),
    [activeUsers, avgBalance, adoptionRate, walletFeePercent, integrationCost]
  );

  // Memoized computed values
  const tvl = useMemo(() => calculateTVL(inputs), [inputs]);

  const defindexFee = useMemo(
    () => walletFeePercent / 2,
    [walletFeePercent]
  );

  const partnerFee = useMemo(
    () => walletFeePercent / 2,
    [walletFeePercent]
  );

  const projections = useMemo(() => calculateProjections(inputs), [inputs]);

  const roiMetrics = useMemo(() => {
    // Use selected scenario for ROI calculations
    const selectedProjection = projections.find(
      (p) => p.scenario === selectedScenario
    );
    const yearlyRevenue = selectedProjection?.yearlyRevenue || 0;
    return calculateROI(yearlyRevenue, integrationCost);
  }, [projections, integrationCost, selectedScenario]);

  return {
    // Input values
    activeUsers,
    avgBalance,
    adoptionRate,
    walletFeePercent,
    integrationCost,
    selectedScenario,

    // Setters
    setActiveUsers,
    setAvgBalance,
    setAdoptionRate,
    setWalletFeePercent,
    setIntegrationCost,
    setSelectedScenario,

    // Computed values
    tvl,
    defindexFee,
    partnerFee,
    projections,
    roiMetrics,
  };
}
