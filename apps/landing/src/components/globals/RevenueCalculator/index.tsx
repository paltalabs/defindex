'use client';

import { useRevenueCalculator } from '@/hooks/useRevenueCalculator';
import { formatCurrency, formatNumber } from '@/utils/revenueCalculations';
import CalculatorInputs from './CalculatorInputs';
import ContactForm from './ContactForm';
import RevenueProjections from './RevenueProjections';
import ROIAnalysis from './ROIAnalysis';

export default function RevenueCalculator() {
  const {
    activeUsers,
    avgBalance,
    adoptionRate,
    walletFeePercent,
    integrationCost,
    selectedScenario,
    tvl,
    defindexFee,
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
    <div className="space-y-8">
      {/* Desktop/Tablet: 3-column layout | Mobile: stacked */}
      <div className="grid grid-cols-1 lg:grid-cols-12 gap-6 lg:gap-8 items-stretch">
        {/* Left Column: Hero + Contact Form */}
        <div className="lg:col-span-3 flex flex-col h-full  justify-between text-center">
          {/* Hero Content */}
          <div className="text-center lg:text-left mb-6 lg:mb-8">
            <h1 className="text-xl md:text-2xl lg:text-4xl font-familjen-grotesk font-bold text-white mb-4 lg:mb-6">
              Calculate Your{' '}
              <span className="text-lime-200 md:text-[52px]">Integration Revenue</span>
            </h1>
            <p className="text-sm font-manrope font-extrathin md:pr-12 md:text-lg text-white/50 mb-3 lg:mb-4 text-pretty text-justify">
              See how much your wallet could earn by integrating DeFindex yield
              features for your users.
            </p>
          </div>
          {/* Contact Form */}
          <ContactForm />
        </div>

        {/* Center Column: Calculator Inputs */}
        <div className="lg:col-span-5 h-full">
          <CalculatorInputs
            activeUsers={activeUsers}
            avgBalance={avgBalance}
            adoptionRate={adoptionRate}
            walletFeePercent={walletFeePercent}
            integrationCost={integrationCost}
            defindexFee={defindexFee}
            onActiveUsersChange={setActiveUsers}
            onAvgBalanceChange={setAvgBalance}
            onAdoptionRateChange={setAdoptionRate}
            onWalletFeePercentChange={setWalletFeePercent}
            onIntegrationCostChange={setIntegrationCost}
          />
        </div>

        {/* Right Column: Results */}
        <div className="lg:col-span-4 h-full flex flex-col justify-between gap-6">
          {/* TVL Summary Card */}
          <div
            className="rounded-2xl p-5 border border-cyan-800/50 text-center"
            style={{
              background:
                'linear-gradient(135deg, rgba(3, 48, 54, 0.8) 0%, rgba(1, 71, 81, 0.5) 100%)',
            }}
          >
            <p className="text-white/60 text-xs mb-1">
              Estimated Total Value Locked
            </p>
            <p className="text-3xl md:text-4xl font-bold text-lime-200 font-familjen-grotesk">
              {formatCurrency(tvl)}
            </p>
            <p className="text-white/50 text-xs mt-2">
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
    </div>
  );
}
