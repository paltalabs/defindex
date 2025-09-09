"use client";

import { useInvestmentCalculator } from '@/hooks/useInvestmentCalculator';
import { FiExternalLink } from 'react-icons/fi';

export default function InvestmentGrowth() {
  const {
    initialDeposit,
    monthlyContribution,
    years,
    apy,
    investmentData,
    setInitialDeposit,
    setMonthlyContribution,
    setYears,
    isLoading,
  } = useInvestmentCalculator();

  if (isLoading) {
    return (
      <section className="py-20 px-4 max-w-7xl mx-auto">
        <div className="text-center">
          <div className="animate-pulse">
            <div className="h-8 bg-gray-600 rounded w-3/4 mx-auto mb-4"></div>
            <div className="h-4 bg-gray-600 rounded w-1/2 mx-auto"></div>
          </div>
        </div>
      </section>
    );
  }

  return (
    <section className="py-12 md:py-20 px-4 max-w-7xl mx-auto">
      <div className="text-center lg:text-left mb-12 md:mb-16">
        <div className="grid lg:grid-cols-2 gap-8 lg:gap-12 items-center">
          {/* Left Column - Title and Text */}
          <div className="space-y-6">
            <h2 className="text-2xl md:text-3xl lg:text-2xl font-bold font-familjen-grotesk leading-tight text-center text-pretty">
              <span 
                className="bg-clip-text text-transparent"
                style={{
                  background: 'linear-gradient(121deg, #FFF 7.14%, #DEC9F4 82.55%)',
                  WebkitBackgroundClip: 'text',
                  WebkitTextFillColor: 'transparent'
                }}
              >
                SEE HOW YOUR
              </span>
              <br />
              <span 
                className="bg-clip-text text-transparent"
                style={{
                  background: 'linear-gradient(121deg, #FFF 7.14%, #DEC9F4 82.55%)',
                  WebkitBackgroundClip: 'text',
                  WebkitTextFillColor: 'transparent'
                }}
              >
                MONEY GROWS
              </span>
              <br />
              <span 
                className="font-bold font-familjen-grotesk leading-[1.11em] text-[48px] sm:text-[56px] md:text-[64px] lg:text-3xl tracking-[-0.03em] bg-clip-text text-transparent"
                style={{
                  background: 'linear-gradient(121deg, #FFF 7.14%, #DEC9F4 82.55%)',
                  WebkitBackgroundClip: 'text',
                  WebkitTextFillColor: 'transparent'
                }}
              >WITH DEFINDEX</span>
            </h2>
          </div>
          
          {/* Right Column - Spacer for large screens */}
          <div className="hidden lg:block">
              <p className="text-base md:text-lg text-white/80 leading-relaxed text-center text-pretty">
                Discover how compounding APY can boost your wealth, from today to{' '}
                {years} years in the future. Watch your money grow each day, with no
                restrictions, no caps, and no unexpected charges. Maintain control,
                withdraw whenever you wish, and let your earnings work for you.
              </p> 
              <p className="text-sm text-white/60 text-center">
                APY varies, capital at risk.
              </p>
          </div>
        </div>
      </div>

      <div className="max-w-7xl mx-auto">
        <div 
              className="border border-cyan-900/50 rounded-lg p-6"
              style={{ background: 'linear-gradient(115deg, rgba(4, 74, 84, 1) 0%, rgba(3, 48, 54, 1) 100%)' }}
              >
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-manrope font-semibold text-white">
                  Current APY
                </h3>
                <span className="text-2xl font-bold text-lime-200">
                  {apy.toFixed(1)}%
                </span>
              </div>
              <p className="text-white/70 text-sm mb-4">
                Check the current APY % in the app!
              </p>
                <button
                onClick={() => {
                  const ctaForm = document.getElementById('cta-form');
                  if (ctaForm) {
                    const offset = -150;
                    const ctaFormPosition = ctaForm.getBoundingClientRect().top + window.scrollY + offset;
                    window.scrollTo({ 
                      top: ctaFormPosition, 
                      behavior: 'smooth',
                    });
                  }
                }}
                style={{
                  width: '100%',
                  background: 'linear-gradient(to right, rgba(8, 120, 120, 1), rgba(2, 80, 80, 1))',
                  color: 'rgba(255, 255, 255, 1)',
                  fontFamily: 'Manrope, sans-serif',
                  fontWeight: 'bold',
                  padding: '0.75rem 1.5rem',
                  borderRadius: '0.5rem',
                  transition: 'all 0.2s',
                  transform: 'scale(1)',
                  cursor: 'pointer',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.background = 'linear-gradient(to right, rgba(6, 95, 95, 1), rgba(1, 60, 60, 1))';
                  e.currentTarget.style.transform = 'scale(1.025)';
                  e.currentTarget.style.transition = 'ease-in 0.1s';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.background = 'linear-gradient(to right, rgba(8, 120, 120, 1), rgba(2, 80, 80, 1))';
                  e.currentTarget.style.transform = 'scale(1)';
                }}
                >
                  <a href="https://v2.soroswap.finance/earn" target="_blank" rel="noopener noreferrer">
                    Start earning now <FiExternalLink className="inline-block ml-2 align-middle" />
                  </a>
                </button>
            </div>
        <div className="grid lg:grid-cols-2 gap-8 lg:gap-12 items-center">
          {/* Left Column - Content and Controls */}
          <div className="space-y-8">
            {/* Input Controls */}
            {/* APY Info */}
            {/* <div 
              className="border border-cyan-900/50 rounded-lg p-6"
              style={{ background: 'linear-gradient(115deg, rgba(4, 74, 84, 1) 0%, rgba(3, 48, 54, 1) 100%)' }}
              >
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-manrope font-semibold text-white">
                  Current APY
                </h3>
                <span className="text-2xl font-bold text-lime-200">
                  {apy.toFixed(1)}%
                </span>
              </div>
              <p className="text-white/70 text-sm mb-4">
                Check the current APY % in the app!
              </p>
                <button
                onClick={() => {
                  const ctaForm = document.getElementById('cta-form');
                  if (ctaForm) {
                    const offset = -150;
                    const ctaFormPosition = ctaForm.getBoundingClientRect().top + window.scrollY + offset;
                    window.scrollTo({ 
                      top: ctaFormPosition, 
                      behavior: 'smooth',
                    });
                  }
                }}
                style={{
                  width: '100%',
                  background: 'linear-gradient(to right, rgba(8, 120, 120, 1), rgba(2, 80, 80, 1))',
                  color: 'rgba(255, 255, 255, 1)',
                  fontFamily: 'Manrope, sans-serif',
                  fontWeight: 'bold',
                  padding: '0.75rem 1.5rem',
                  borderRadius: '0.5rem',
                  transition: 'all 0.2s',
                  transform: 'scale(1)',
                  cursor: 'pointer',
                }}
                onMouseEnter={(e) => {
                  e.currentTarget.style.background = 'linear-gradient(to right, rgba(6, 95, 95, 1), rgba(1, 60, 60, 1))';
                  e.currentTarget.style.transform = 'scale(1.025)';
                  e.currentTarget.style.transition = 'ease-in 0.1s';
                }}
                onMouseLeave={(e) => {
                  e.currentTarget.style.background = 'linear-gradient(to right, rgba(8, 120, 120, 1), rgba(2, 80, 80, 1))';
                  e.currentTarget.style.transform = 'scale(1)';
                }}
                >
                Start earning now
                </button>
            </div> */}
            {/* 
            <InvestmentInputs
              initialDeposit={initialDeposit}
              monthlyContribution={monthlyContribution}
              years={years}
              onInitialDepositChange={setInitialDeposit}
              onMonthlyContributionChange={setMonthlyContribution}
              onYearsChange={setYears}
            /> */}

          </div>

          {/* Right Column - Chart */}
          {/* <div className="w-full">
            <InvestmentChart data={investmentData} years={years} />
          </div> */}
        </div>
      </div>

      {/* Mobile Layout Adjustments */}
      <div className="lg:hidden mt-12">
        <div className="text-center space-y-4">
          <p className="text-white/60 text-sm">
            Drag on the chart above to see projections at different time periods
          </p>
        </div>
      </div>
    </section>
  );
}