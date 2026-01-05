'use client';

import { useRevenueCalculator } from '@/hooks/useRevenueCalculator';
import { formatCurrency, formatNumber } from '@/utils/revenueCalculations';
import CalculatorInputs from './CalculatorInputs';
import ContactForm from './ContactForm';
import RevenueProjections from './RevenueProjections';
import ROIAnalysis from './ROIAnalysis';
import { gradients } from './styles';

export default function RevenueCalculator() {
  const {
    activeUsers,
    avgBalance,
    adoptionRate,
    walletFeePercent,
    integrationCost,
    selectedScenario,
    tvl,
    partnerFee,
    projections,
    roiMetrics,
    setActiveUsers,
    setAvgBalance,
    setAdoptionRate,
    setWalletFeePercent,
    setIntegrationCost,
    setSelectedScenario,
  } = useRevenueCalculator();

  return (
    <div className="space-y-12 relative py-24">
      {/* Header Section - Centered */}
      <div className="text-center mb-4">
        <h1 className="text-2xl md:text-3xl lg:text-4xl font-familjen-grotesk font-bold text-white mb-2">
          Calculate Your{' '}
          <span className="text-lime-200">Integration Revenue</span>
        </h1>
        <p className="text-white/60 text-sm md:text-base max-w-3xl mx-auto">
          See how much your wallet could earn by integrating DeFindex yield
          features for your users.
        </p>
      </div>

      {/* Main Calculator Grid - 2 columns */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-4 lg:gap-6 mx-4 md:mx-12 lg:mx-6 xl:mx-16">
        {/* Left Column: Calculator Inputs (7/12) */}
        <div className="lg:col-span-7">
          <CalculatorInputs
            activeUsers={activeUsers}
            avgBalance={avgBalance}
            adoptionRate={adoptionRate}
            walletFeePercent={walletFeePercent}
            integrationCost={integrationCost}
            partnerFee={partnerFee}
            onActiveUsersChange={setActiveUsers}
            onAvgBalanceChange={setAvgBalance}
            onAdoptionRateChange={setAdoptionRate}
            onWalletFeePercentChange={setWalletFeePercent}
            onIntegrationCostChange={setIntegrationCost}
          />
        </div>

        {/* Right Column: Results (5/12) */}
        <div className="lg:col-span-5 flex flex-col gap-4">
          {/* TVL Summary Card */}
          <div
            className="rounded-xl p-4 border border-cyan-800/50 text-center backdrop-blur-md"
            style={{ background: gradients.cardDark }}
          >
            <p className="text-white/60 text-xs mb-1">
              Estimated Total Value Locked
            </p>
            <p className="text-3xl md:text-4xl font-bold text-lime-200 font-familjen-grotesk mb-1">
              {formatCurrency(tvl)}
            </p>
            <p className="text-white/50 text-[10px]">
              {formatNumber(activeUsers)} users × ${formatNumber(avgBalance)}{' '}
              avg × {adoptionRate}% adoption
            </p>
          </div>

          {/* Revenue Projections */}
          <RevenueProjections
            projections={projections}
            partnerFeePercent={partnerFee}
            selectedScenario={selectedScenario}
            onScenarioChange={setSelectedScenario}
          />

          {/* ROI Analysis */}
          <ROIAnalysis
            roiMetrics={roiMetrics}
            integrationCost={integrationCost}
            selectedScenario={selectedScenario}
          />
        </div>
      </div>

      {/* Contact Form Section - Full Width Below */}
      <div className="mt-4  mx-4 md:mx-12 lg:mx-6 xl:mx-16">
        <ContactForm />
      </div>
    </div>
  );
}
