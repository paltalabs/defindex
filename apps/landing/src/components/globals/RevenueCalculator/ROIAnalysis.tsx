'use client';

import { ROIMetrics, formatCurrency } from '@/utils/revenueCalculations';

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
      className="rounded-2xl p-4 md:p-5 border border-cyan-800/50"
      style={{
        background:
          'linear-gradient(135deg, rgba(3, 48, 54, 0.8) 0%, rgba(1, 71, 81, 0.5) 100%)',
      }}
    >
      <div className="flex items-center justify-between mb-3 lg:mr-6 align-bottom">
        <h3 className="text-lg font-familjen-grotesk font-semibold text-white">
          ROI Analysis
        </h3>
        <p className="text-xs text-white/50">
          Based on {scenarioLabels[selectedScenario]} scenario
        </p>
      </div>

      <div className="grid grid-cols-3 gap-3 md:gap-4">
        {/* Integration Cost */}
        <div>
          <p className="text-white/60 text-xs mb-1 leading-tight md:leading-normal">
            Integration Cost
          </p>
          <p className="text-base md:text-lg font-bold text-white">
            {formatCurrency(integrationCost)}
          </p>
        </div>

        {/* Payback Period */}
        <div>
          <p className="text-white/60 text-xs mb-1 leading-tight md:leading-normal">
            Payback Period
          </p>
          <p className="text-base md:text-lg font-bold text-lime-200">
            {paybackWeeks > 0 ? (
              <>
                {paybackWeeks.toFixed(1)}{' '}
                <span className="text-sm font-normal text-white/75">weeks</span>
              </>
            ) : (
              <span className="text-white/50">N/A</span>
            )}
          </p>
        </div>

        {/* First Year Profit Margin */}
        <div>
          <p className="text-white/60 text-xs mb-1 leading-tight md:leading-normal">
            First-Year Profit Margin
          </p>
          <p
            className={`text-base md:text-lg font-bold ${
              firstYearProfitMargin >= 0 ? 'text-lime-200' : 'text-orange-400'
            }`}
          >
            {firstYearProfitMargin !== 0 ? (
              <>
                {firstYearProfitMargin.toFixed(1)}
                <span className="text-sm">%</span>
              </>
            ) : (
              <span className="text-white/50">0%</span>
            )}
          </p>
        </div>
      </div>
    </div>
  );
}
