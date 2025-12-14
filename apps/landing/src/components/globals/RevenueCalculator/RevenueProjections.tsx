'use client';

import { RevenueProjection, formatCurrency } from '@/utils/revenueCalculations';

interface RevenueProjectionsProps {
  projections: RevenueProjection[];
  partnerFeePercent: number;
  selectedScenario: 'conservative' | 'moderate' | 'peak';
  onScenarioChange: (scenario: 'conservative' | 'moderate' | 'peak') => void;
}

export default function RevenueProjections({
  projections,
  partnerFeePercent,
  selectedScenario,
  onScenarioChange,
}: RevenueProjectionsProps) {
  const getScenarioLabel = (scenario: string) => {
    switch (scenario) {
      case 'conservative':
        return 'Conservative';
      case 'moderate':
        return 'Moderate';
      case 'peak':
        return 'Peak';
      default:
        return scenario;
    }
  };

  return (
    <div className="space-y-4">
      <h3 className="text-lg font-familjen-grotesk font-semibold text-white">
        Revenue Projections
      </h3>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        {projections.map((projection) => {
          const isSelected = projection.scenario === selectedScenario;

          return (
            <button
              key={projection.scenario}
              onClick={() => onScenarioChange(projection.scenario)}
              className={`relative rounded-2xl p-4 md:p-5 border-2 transition-all text-left cursor-pointer hover:scale-[1.02] active:scale-[0.98] ${
                isSelected
                  ? 'border-lime-200/80'
                  : 'border-cyan-800/50 bg-cyan-950/50 hover:border-cyan-600'
              }`}
              style={
                isSelected
                  ? {
                      background:
                        'linear-gradient(145deg, rgba(79, 121, 102, 0.8) 0%, rgba(45, 85, 75, 0.6) 50%, rgba(30, 60, 55, 0.7) 100%)',
                      boxShadow:
                        '0 0 20px rgba(211, 255, 180, 0.3), 0 0 40px rgba(211, 255, 180, 0.15), inset 0 1px 1px rgba(255, 255, 255, 0.1)',
                    }
                  : undefined
              }
            >
              {/* Mobile: Horizontal layout */}
              <div className="md:hidden">
                {/* Header row: Scenario name + APY */}
                <div className="flex items-center justify-between mb-3 flex-warp ">
                  <h4
                    className={`text-sm font-medium px-3 py-1 rounded-full ${
                      isSelected
                        ? 'text-lime-200 bg-lime-400/20'
                        : 'text-white/75'
                    }`}
                  >
                    {getScenarioLabel(projection.scenario)}
                  </h4>
                  <span className="text-white/50 text-xs px-3 py-1">
                    {projection.apy}% APY
                  </span>
                </div>
                {/* Values row: Monthly + Yearly side by side */}
                <div className="grid grid-cols-2 gap-4 mb-2">
                  <div className="text-center bg-cyan-900/20 rounded-xl">
                    <p className="text-white/60 text-xs mb-1">Monthly</p>
                    <p
                      className={`text-lg font-semibold pb-8 ${
                        isSelected ? 'text-lime-200' : 'text-white'
                      }`}
                    >
                      {formatCurrency(projection.monthlyRevenue)}
                    </p>
                  </div>
                  <div className="text-center bg-cyan-900/20 rounded-xl">
                    <p className="text-white/60 text-xs mb-1">Yearly</p>
                    <p
                      className={`text-lg font-bold pb-8 ${
                        isSelected ? 'text-lime-200' : 'text-white'
                      }`}
                    >
                      {formatCurrency(projection.yearlyRevenue)}
                    </p>
                  </div>
                </div>
              </div>

              {/* Desktop: Vertical layout */}
              <div className="hidden md:block">
                <div className="mb-4">
                  <h4
                    className={`text-sm font-medium ${
                      isSelected ? 'text-lime-200' : 'text-white/75'
                    }`}
                  >
                    {getScenarioLabel(projection.scenario)}
                  </h4>
                  <p className="text-white/50 text-xs mt-1">
                    {projection.apy}% APY
                  </p>
                </div>

                <div className="space-y-3">
                  <div>
                    <p className="text-white/60 text-xs">Monthly</p>
                    <p
                      className={`text-lg font-semibold truncate ${
                        isSelected ? 'text-lime-200' : 'text-white'
                      }`}
                    >
                      {formatCurrency(projection.monthlyRevenue)}
                    </p>
                  </div>
                  <div>
                    <p className="text-white/60 text-xs">Yearly</p>
                    <p
                      className={`text-xl font-bold truncate ${
                        isSelected ? 'text-lime-200' : 'text-white'
                      }`}
                    >
                      {formatCurrency(projection.yearlyRevenue)}
                    </p>
                  </div>
                </div>
              </div>
            </button>
          );
        })}
      </div>

      <p className="text-xs text-white/50 text-center mt-4">
        Based on {partnerFeePercent}% partner revenue share
      </p>
    </div>
  );
}
