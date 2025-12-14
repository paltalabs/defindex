'use client';

import { formatNumber } from '@/utils/revenueCalculations';

interface CalculatorInputsProps {
  activeUsers: number;
  avgBalance: number;
  adoptionRate: number;
  walletFeePercent: number;
  integrationCost: number;
  defindexFee: number;
  onActiveUsersChange: (value: number) => void;
  onAvgBalanceChange: (value: number) => void;
  onAdoptionRateChange: (value: number) => void;
  onWalletFeePercentChange: (value: number) => void;
  onIntegrationCostChange: (value: number) => void;
}

const BALANCE_OPTIONS = [1, 10, 100, 1000, 10000];

// Helper to calculate slider fill percentage
const getSliderBackground = (value: number, min: number, max: number) => {
  const percentage = ((value - min) / (max - min)) * 100;
  return `linear-gradient(90deg, #D3FFB4 0%, #D3FFB4 ${percentage}%, rgba(6, 95, 105, 0.8) ${percentage}%, rgba(8, 75, 85, 0.6) 100%)`;
};

// Custom slider styles with glow effect
const sliderStyles = `
  .custom-slider {
    -webkit-appearance: none;
    appearance: none;
    width: 100%;
    height: 6px;
    border-radius: 9999px;
    outline: none;
    cursor: pointer;
  }

  .custom-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    appearance: none;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #D3FFB4;
    cursor: pointer;
    box-shadow: 0 0 10px rgba(211, 255, 180, 0.6), 0 0 20px rgba(211, 255, 180, 0.3);
    border: 2px solid rgba(255, 255, 255, 0.2);
    transition: box-shadow 0.2s ease;
  }

  .custom-slider::-webkit-slider-thumb:hover {
    box-shadow: 0 0 15px rgba(211, 255, 180, 0.8), 0 0 30px rgba(211, 255, 180, 0.4);
  }

  .custom-slider::-moz-range-thumb {
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: #D3FFB4;
    cursor: pointer;
    box-shadow: 0 0 10px rgba(211, 255, 180, 0.6), 0 0 20px rgba(211, 255, 180, 0.3);
    border: 2px solid rgba(255, 255, 255, 0.2);
    transition: box-shadow 0.2s ease;
  }

  .custom-slider::-moz-range-thumb:hover {
    box-shadow: 0 0 15px rgba(211, 255, 180, 0.8), 0 0 30px rgba(211, 255, 180, 0.4);
  }

  .custom-slider::-moz-range-track {
    height: 6px;
    border-radius: 9999px;
  }
`;

export default function CalculatorInputs({
  activeUsers,
  avgBalance,
  adoptionRate,
  walletFeePercent,
  integrationCost,
  defindexFee,
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
        className="border border-cyan-800/50 rounded-2xl p-6 md:p-8 h-full flex flex-col"
        style={{
          background:
            'linear-gradient(115deg, rgba(4, 74, 84, 1) 0%, rgba(3, 48, 54, 1) 100%)',
        }}
      >
        <div className="space-y-8 flex-1 flex flex-col justify-between">
          {/* Active Users Slider */}
          <div>
            <div className="flex justify-between items-center mb-3">
              <label className="text-lg font-manrope font-bold text-white">
                Active Users
              </label>
              <span className="text-lg font-semibold text-white">
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
            />
            <div className="flex justify-between mt-2 text-xs text-white/50">
              <span>1K</span>
              <span>5M</span>
            </div>
          </div>

          {/* Average Stablecoin Balance */}
          <div>
            <label className="block text-lg font-manrope font-bold text-white mb-6">
              Average Stablecoin Balance
            </label>
            <div className="flex flex-wrap gap-2">
              {BALANCE_OPTIONS.map((balance) => {
                const isSelected = avgBalance === balance;
                return (
                  <button
                    key={balance}
                    onClick={() => onAvgBalanceChange(balance)}
                    className={`px-4 py-2 rounded-full font-medium transition-all duration-200 ${
                      isSelected
                        ? 'text-cyan-900'
                        : 'bg-cyan-950 text-white border border-cyan-800 hover:bg-cyan-900 hover:border-cyan-600'
                    }`}
                    style={
                      isSelected
                        ? {
                            background: '#D3FFB4',
                            boxShadow:
                              '0 0 12px rgba(211, 255, 180, 0.5), 0 0 24px rgba(211, 255, 180, 0.25)',
                          }
                        : undefined
                    }
                  >
                    ${formatNumber(balance)}
                  </button>
                );
              })}
            </div>
          </div>

          {/* Adoption Percentage Slider */}
          <div>
            <div className="flex justify-between items-center mb-3">
              <label className="text-lg font-manrope font-bold text-white">
                Adoption Percentage
              </label>
              <span className="text-lg font-semibold text-white">
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
            />
            <div className="flex justify-between text-xs text-white/50 mb-0">
              <span>5%</span>
              <span>100%</span>
            </div>
            <p className="text-xs text-white/50 my-0">
              Industry average is 15-25% for opt-in yield features
            </p>
          </div>

          {/* Wallet Fee Slider */}
          <div>
            <div className="flex justify-between items-center mb-3">
              <label className="text-lg font-manrope font-bold text-white">
                Wallet Fee
              </label>
              <span className="text-lg font-semibold text-white">
                {walletFeePercent}%
              </span>
            </div>
            <input
              type="range"
              min={0.5}
              max={10}
              step={0.5}
              value={walletFeePercent}
              onChange={(e) => onWalletFeePercentChange(Number(e.target.value))}
              className="custom-slider"
              style={{ background: getSliderBackground(walletFeePercent, 0.5, 10) }}
            />
            <div className="flex justify-between mt-2 text-xs text-white/50">
              <span>0.5%</span>
              <span>10%</span>
            </div>
            <div className="mt-3 flex gap-4 text-sm">
              <div className="flex items-center gap-2">
                <span
                  className="w-3 h-3 rounded-full"
                  style={{
                    background: '#D3FFB4',
                    boxShadow: '0 0 6px rgba(211, 255, 180, 0.6)',
                  }}
                ></span>
                <span className="text-white/75">
                  Your share:{' '}
                  <span className="text-white font-medium">{defindexFee}%</span>
                </span>
              </div>
              <div className="flex items-center gap-2">
                <span
                  className="w-3 h-3 rounded-full"
                  style={{
                    background: '#C084FC',
                    boxShadow: '0 0 6px rgba(192, 132, 252, 0.6)',
                  }}
                ></span>
                <span className="text-white/75">
                  DeFindex:{' '}
                  <span className="text-white font-medium">{defindexFee}%</span>
                </span>
              </div>
            </div>
          </div>

          {/* Integration Cost Input */}
          <div>
            <label className="block text-lg font-manrope font-bold text-white mb-6">
              Integration Cost
            </label>
            <div
              className="flex items-center rounded-2xl overflow-hidden border-2 border-cyan-700/50 transition-all duration-200 hover:border-cyan-600/70"
              style={{
                background:
                  'linear-gradient(135deg, rgba(6, 78, 88, 0.6) 0%, rgba(4, 58, 68, 0.8) 100%)',
                boxShadow: '0 0 15px rgba(6, 182, 212, 0.1)',
              }}
            >
              <button
                onClick={() =>
                  onIntegrationCostChange(Math.max(0, integrationCost - 500))
                }
                className="flex items-center justify-center w-14 h-14 text-white/80 hover:text-white hover:bg-cyan-800/30 transition-all text-2xl font-light"
              >
                −
              </button>
              <div className="relative flex-1">
                <span className="absolute left-4 top-1/2 transform -translate-y-1/2 text-white/60 text-lg">
                  $
                </span>
                <input
                  type="text"
                  value={formatNumber(integrationCost)}
                  onChange={(e) =>
                    handleIntegrationCostInputChange(e.target.value)
                  }
                  className="w-full h-14 pl-10 pr-4 bg-transparent text-white text-xl font-semibold text-center focus:outline-none"
                />
              </div>
              <button
                onClick={() => onIntegrationCostChange(integrationCost + 500)}
                className="flex items-center justify-center w-14 h-14 text-white/80 hover:text-white hover:bg-cyan-800/30 transition-all text-2xl font-light"
              >
                +
              </button>
            </div>
          </div>
        </div>
      </div>
    </>
  );
}
