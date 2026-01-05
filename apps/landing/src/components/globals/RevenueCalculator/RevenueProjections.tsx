'use client';

import { RevenueProjection, formatCurrency } from '@/utils/revenueCalculations';
import { getSelectedCardStyles, gradients } from './styles';

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
    <div>
      <h3 className="text-sm font-familjen-grotesk font-semibold text-white mb-2">
        Revenue Projections
      </h3>

      <div className="grid grid-cols-3 gap-2">
        {projections.map((projection) => {
          const isSelected = projection.scenario === selectedScenario;

          return (
            <button
              key={projection.scenario}
              onClick={() => onScenarioChange(projection.scenario)}
              className={`relative rounded-lg p-3 border transition-all text-left cursor-pointer hover:scale-[1.02] active:scale-[0.98] flex flex-col justify-between min-h-[150px] ${
                isSelected
                  ? 'border-lime-200/80'
                  : 'border-cyan-800/50 hover:border-cyan-600'
              }`}
              style={
                isSelected
                  ? getSelectedCardStyles()
                  : { background: gradients.card }
              }
            >
              {/* Header: Scenario name + APY */}
              <div>
                <h4
                  className={`text-[16px] font-extrabold mb-0.5 ${
                    isSelected ? 'text-lime-200' : 'text-white/75'
                  }`}
                >
                  {getScenarioLabel(projection.scenario)} <span className="text-[10px] font-thin text-white/50 h-full absolute top-3"> *</span>
                </h4>
                <p className="text-base font-bold mb-1 text-white ">
                  {projection.apy}% APY
                </p>
              </div>

              {/* Values */}
              <div>
                <p className="text-[12px] text-white/50">Monthly</p>
                <p
                  className={`text-base font-bold mb-1 ${
                    isSelected ? 'text-lime-200' : 'text-white'
                  }`}
                >
                  {formatCurrency(projection.monthlyRevenue)}
                </p>
                <p className="text-[12px] text-white/50">Yearly</p>
                <p
                  className={`text-lg font-bold ${
                    isSelected ? 'text-lime-200' : 'text-white/90'
                  }`}
                >
                  {formatCurrency(projection.yearlyRevenue)} <span className="text-[10px] font-thin text-white/50 h-full absolute top-32"> **</span>
                </p>
              </div>
            </button>
          );
        })}
      </div>

      <p className="text-[10px] text-white/40 text-center mt-1">
        *APY values shown are for reference only. Actual rates vary based on market conditions.
      </p>
      <p className="text-[10px] text-white/50 text-center mt-1">
        **Based on {partnerFeePercent}% partner revenue share.
      </p>
    
    </div>
  );
}
