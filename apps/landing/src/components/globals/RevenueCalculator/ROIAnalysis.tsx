'use client';

import { ROIMetrics, formatCurrency } from '@/utils/revenueCalculations';
import { gradients } from './styles';

interface ROIAnalysisProps {
  roiMetrics: ROIMetrics;
  integrationCost: number;
  selectedScenario: 'conservative' | 'moderate' | 'peak';
}

export default function ROIAnalysis({
  roiMetrics,
  integrationCost,
  selectedScenario,
}: ROIAnalysisProps) {
  const scenarioLabels = {
    conservative: 'conservative',
    moderate: 'moderate',
    peak: 'peak',
  };
  const { paybackWeeks, firstYearProfitMargin } = roiMetrics;

  return (
    <div
      className="rounded-lg p-4 border border-cyan-800/50"
      style={{ background: gradients.card }}
    >
      <div className="flex items-center justify-between mb-3">
        <h3 className="text-sm font-familjen-grotesk font-bold text-white">
          ROI Analysis
        </h3>
        <p className="text-[10px] text-white/50">
          Based on {scenarioLabels[selectedScenario]} scenario
        </p>
      </div>

      <div className="grid grid-cols-3 gap-3 text-center">
        {/* Integration Cost */}
        <div>
          <p className="text-white/50 text-[10px] mb-0.5">
            Integration Cost
          </p>
          <p className="text-base font-bold text-white pt-4">
            {formatCurrency(integrationCost)}
          </p>
        </div>

        {/* Payback Period */}
        <div>
          <p className="text-white/50 text-[10px] mb-0.5">
            Payback Period
          </p>
          <p className="text-base font-bold text-white pt-0.5">
            {paybackWeeks > 0 ? (
              <>
                <span className="text-lime-200">{paybackWeeks.toFixed(1)}</span>{' '}
                <span className="text-xs font-normal text-white/75">weeks</span>
              </>
            ) : (
              <span className="text-white/50">N/A</span>
            )}
          </p>
        </div>

        {/* First Year Profit Margin */}
        <div>
          <p className="text-white/50 text-[10px] mb-0.5">
            First-Year Profit Margin
          </p>
          <p className="text-base font-bold text-white pt-4">
            {firstYearProfitMargin !== 0 ? (
              <span className={firstYearProfitMargin >= 0 ? 'text-lime-200' : 'text-orange-400'}>
                {firstYearProfitMargin.toFixed(1)}%
              </span>
            ) : (
              <span className="text-white/50">0%</span>
            )}
          </p>
        </div>
      </div>
    </div>
  );
}
