import { useState, useEffect, useMemo, useCallback } from 'react';
import { calculateInvestmentGrowth, InvestmentData } from '@/utils/investmentCalculations';

export interface UseInvestmentCalculatorReturn {
  initialDeposit: number;
  monthlyContribution: number;
  years: number;
  apy: number;
  investmentData: InvestmentData[];
  setInitialDeposit: (value: number) => void;
  setMonthlyContribution: (value: number) => void;
  setYears: (value: number) => void;
  isLoading: boolean;
}

export function useInvestmentCalculator(): UseInvestmentCalculatorReturn {
  const [initialDeposit, setInitialDeposit] = useState(1000);
  const [monthlyContribution, setMonthlyContribution] = useState(100);
  const [years, setYears] = useState(50);
  const [apy, setApy] = useState(10);
  const [isLoading, setIsLoading] = useState(true);

  // Fetch APY from API with fallback
  useEffect(() => {
    const fetchAPY = async () => {
      try {
        setIsLoading(true);
        const response = await fetch('/api/apy');
        const data = await response.json();
        
        if (data.apy && data.apy > 0) {
          setApy(data.apy);
        } else {
          setApy(10); // fallback
        }
      } catch (error) {
        console.error('Error fetching APY:', error);
        setApy(10); // fallback to 10%
      } finally {
        setIsLoading(false);
      }
    };

    fetchAPY();
  }, []);

  // Memoize investment calculations to avoid unnecessary recalculations
  const investmentData = useMemo(() => {
    if (isLoading) return [];
    return calculateInvestmentGrowth(initialDeposit, monthlyContribution, apy, years);
  }, [initialDeposit, monthlyContribution, apy, years, isLoading]);

  // Memoized setters to avoid unnecessary re-renders
  const memoizedSetInitialDeposit = useCallback((value: number) => {
    setInitialDeposit(value);
  }, []);

  const memoizedSetMonthlyContribution = useCallback((value: number) => {
    setMonthlyContribution(value);
  }, []);

  const memoizedSetYears = useCallback((value: number) => {
    setYears(value);
  }, []);

  return {
    initialDeposit,
    monthlyContribution,
    years,
    apy,
    investmentData,
    setInitialDeposit: memoizedSetInitialDeposit,
    setMonthlyContribution: memoizedSetMonthlyContribution,
    setYears: memoizedSetYears,
    isLoading,
  };
}