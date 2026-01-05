'use client';

import { formatNumber } from '@/utils/revenueCalculations';
import {
  getSelectedStyles,
  getSliderBackground,
  gradients,
  shadows,
  sliderStyles
} from './styles';

interface CalculatorInputsProps {
  activeUsers: number;
  avgBalance: number;
  adoptionRate: number;
  walletFeePercent: number;
  integrationCost: number;
  partnerFee: number;
  onActiveUsersChange: (value: number) => void;
  onAvgBalanceChange: (value: number) => void;
  onAdoptionRateChange: (value: number) => void;
  onWalletFeePercentChange: (value: number) => void;
  onIntegrationCostChange: (value: number) => void;
}

const BALANCE_OPTIONS = [1, 10, 100, 1000, 10000];

export default function CalculatorInputs({
  activeUsers,
  avgBalance,
  adoptionRate,
  walletFeePercent,
  integrationCost,
  partnerFee,
  onActiveUsersChange,
  onAvgBalanceChange,
  onAdoptionRateChange,
  onWalletFeePercentChange,
  onIntegrationCostChange,
}: CalculatorInputsProps) {
  const handleIntegrationCostInputChange = (value: string) => {
    const numValue = Number(value.replace(/,/g, ''));
    if (!isNaN(numValue) && numValue >= 0) {
      onIntegrationCostChange(numValue);
    }
  };

  return (
    <>
      <style>{sliderStyles}</style>
      <div
        className="border border-cyan-800/50 rounded-xl px-6 py-4 md:py-8 md:px-10 h-full"
        style={{ background: gradients.inputsPanel }}
      >
        <div className="space-y-4 h-full flex flex-col justify-between">
          {/* Active Users Slider */}
          <div>
            <div className="flex justify-between items-center mb-2">
              <label className="text-sm font-manrope font-medium text-white">
                Active Users
              </label>
              <span className="text-base font-semibold text-white">
                {formatNumber(activeUsers)}
              </span>
            </div>
            <input
              type="range"
              min={1000}
              max={5000000}
              step={1105}
              value={activeUsers}
              onChange={(e) => onActiveUsersChange(Number(e.target.value))}
              className="custom-slider"
              style={{ background: getSliderBackground(activeUsers, 1000, 5000000) }}
              aria-label="Active Users"
            />
            <div className="flex justify-between mt-1 text-[10px] text-white/50">
              <span>1K</span>
              <span>5M</span>
            </div>
          </div>

          {/* Adoption Percentage Slider */}
          <div>
            <div className="flex justify-between items-center mb-2">
              <label className="text-sm font-manrope font-medium text-white">
                Adoption Percentage
              </label>
              <span className="text-base font-semibold text-white">
                {adoptionRate}%
              </span>
            </div>
            <input
              type="range"
              min={5}
              max={100}
              step={5}
              value={adoptionRate}
              onChange={(e) => onAdoptionRateChange(Number(e.target.value))}
              className="custom-slider"
              style={{ background: getSliderBackground(adoptionRate, 5, 100) }}
              aria-label="Adoption Percentage"
            />
            <div className="flex justify-between text-[10px] text-white/50">
              <span>5%</span>
              <span>100%</span>
            </div>
            <p className="text-[10px] text-white/50 mt-0.5">
              Industry average is 15-25% for opt-in yield features
            </p>
          </div>

          {/* Average Stablecoin Balance */}
          <div>
            <label className="block text-sm font-manrope font-medium text-white mb-2">
              Average Stablecoin Balance
            </label>
            <div className="flex flex-wrap w-full justify-evenly gap-1.5">
              {BALANCE_OPTIONS.map((balance) => {
                const isSelected = avgBalance === balance;
                return (
                  <button
                    key={balance}
                    onClick={() => onAvgBalanceChange(balance)}
                    className={`px-3 py-1.5 text-sm rounded-full font-medium transition-all duration-200 ${
                      isSelected
                        ? 'text-cyan-900'
                        : 'bg-cyan-950 text-white border border-cyan-800 hover:bg-cyan-900 hover:border-cyan-600'
                    }`}
                    style={isSelected ? getSelectedStyles() : undefined}
                  >
                    ${formatNumber(balance)}
                  </button>
                );
              })}
            </div>
          </div>

          {/* Wallet Fee Slider */}
          <div>
            <div className="flex justify-between items-center mb-2">
              <label className="text-sm font-manrope font-medium text-white">
                Wallet Fee
              </label>
              <span className="text-base font-semibold text-white">
                {walletFeePercent}%
              </span>
            </div>
            <input
              type="range"
              min={0.5}
              max={100}
              step={0.5}
              value={walletFeePercent}
              onChange={(e) => onWalletFeePercentChange(Number(e.target.value))}
              className="custom-slider"
              style={{ background: getSliderBackground(walletFeePercent, 0.5, 100) }}
              aria-label="Wallet Fee Percentage"
            />
            <div className="flex justify-between mt-1 text-[10px] text-white/50">
              <span>0.5%</span>
              <span>100%</span>
            </div>
          </div>

          {/* Integration Cost Input */}
          <div>
            <label className="block text-sm font-manrope font-medium text-white mb-2">
              Integration Cost
            </label>
            <div
              className="flex items-center rounded-xl overflow-hidden border border-cyan-700/50 transition-all duration-200 hover:border-cyan-600/70"
              style={{
                background: gradients.costInput,
                boxShadow: shadows.cyanGlow,
              }}
            >
              <button
                onClick={() =>
                  onIntegrationCostChange(Math.max(0, integrationCost - 500))
                }
                className="flex items-center justify-center w-10 h-10 text-white/80 hover:text-white hover:bg-cyan-800/30 transition-all text-xl font-light"
              >
                −
              </button>
              <div className="relative flex-1">
                <span className="absolute left-3 top-1/2 transform -translate-y-1/2 text-white/60 text-base">
                  $
                </span>
                <input
                  type="text"
                  value={formatNumber(integrationCost)}
                  onChange={(e) =>
                    handleIntegrationCostInputChange(e.target.value)
                  }
                  className="w-full h-10 pl-8 pr-3 bg-transparent text-white text-lg font-semibold text-center focus:outline-none"
                />
              </div>
              <button
                onClick={() => onIntegrationCostChange(integrationCost + 500)}
                className="flex items-center justify-center w-10 h-10 text-white/80 hover:text-white hover:bg-cyan-800/30 transition-all text-xl font-light"
              >
                +
              </button>
            </div>
            <p className="text-[10px] text-white/50 mt-2 text-center">
              Your estimated development cost to integrate DeFindex. A one time setup that drives ongoing revenue.
            </p>
          </div>
        </div>
      </div>
    </>
  );
}
