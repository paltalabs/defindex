interface InvestmentInputsProps {
  initialDeposit: number;
  monthlyContribution: number;
  years: number;
  onInitialDepositChange: (value: number) => void;
  onMonthlyContributionChange: (value: number) => void;
  onYearsChange: (value: number) => void;
}

const yearOptions = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];

export default function InvestmentInputs({
  initialDeposit,
  monthlyContribution,
  years,
  onInitialDepositChange,
  onMonthlyContributionChange,
  onYearsChange,
}: InvestmentInputsProps) {
  const formatCurrency = (value: number): string => {
    return new Intl.NumberFormat('en-US', {
      minimumFractionDigits: 0,
      maximumFractionDigits: 2,
    }).format(value);
  };

  const handleInputChange = (
    value: string,
    onChange: (value: number) => void,
    min = 0.0000001
  ) => {
    console.log(value);
    const numValue = Number(value);
    console.log(numValue);
    if (isNaN(numValue)) {
      onChange(0);
    } else if (!isNaN(numValue) && numValue >= min) {
      onChange(numValue);
    } else if (value === '' || value === '0') {
      onChange(0);
    }
  };

  return (
    <div 
      className="border border-cyan-900/50 rounded-lg p-6"
      style={{ background: 'linear-gradient(115deg, rgba(4, 74, 84, 1) 0%, rgba(3, 48, 54, 1) 100%)' }}
    >
      <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-2 xl:grid-cols-2 gap-4 md:gap-6">
        {/* Initial Deposit */}
        <div className="col-span-1">
          <label className="block text-[10px] sm:text-[12px] font-manrope font-medium text-white/75 mb-2">
            Initial deposit
          </label>
          <div className="relative">
            <div className="flex items-center">
              <button
                onClick={() => onInitialDepositChange(Math.max(0.0000001, initialDeposit - 100))}
                className="flex items-center justify-center w-8 md:w-10 h-12 bg-cyan-950 rounded-l-lg border border-cyan-800 text-white hover:bg-cyan-900 transition-colors text-lg"
              >
                −
              </button>
              <div className="relative flex-1">
                <span className="absolute left-2 md:left-3 top-1/2 transform -translate-y-1/2 text-white text-sm md:text-lg">
                  $
                </span>
                <input
                  type="text"
                  value={formatCurrency(initialDeposit)}
                  onChange={(e) => handleInputChange(e.target.value.replace(/,/g, ''), onInitialDepositChange)}
                  className="w-full h-12 pl-6 md:pl-8 pr-2 md:pr-4 bg-cyan-950 border-t border-b border-cyan-800 text-white text-sm md:text-lg font-medium text-center focus:outline-none focus:ring-2 focus:ring-orange-500"
                />
              </div>
              <button
                onClick={() => onInitialDepositChange(initialDeposit + 100)}
                className="flex items-center justify-center w-8 md:w-10 h-12 bg-cyan-950 rounded-r-lg border border-cyan-800 text-white hover:bg-cyan-900 transition-colors text-lg"
              >
                +
              </button>
            </div>
          </div>
        </div>

        {/* Monthly Contribution */}
        <div className="col-span-1">
          <label className="block text-[10px] sm:text-[12px] font-manrope font-medium text-white/75 mb-2">
            Monthly contribution
          </label>
          <div className="relative">
            <div className="flex items-center">
              <button
                onClick={() => onMonthlyContributionChange(Math.max(0.0000001, monthlyContribution - 50))}
                className="flex items-center justify-center w-8 md:w-10 h-12 bg-cyan-950 rounded-l-lg border border-cyan-800 text-white hover:bg-cyan-900 transition-colors text-lg"
              >
                −
              </button>
              <div className="relative flex-1">
                <span className="absolute left-2 md:left-3 top-1/2 transform -translate-y-1/2 text-white text-sm md:text-lg">
                  $
                </span>
                <input
                  type="text"
                  value={formatCurrency(monthlyContribution)}
                  onChange={(e) => handleInputChange(e.target.value.replace(/,/g, ''), onMonthlyContributionChange)}
                  className="w-full h-12 pl-6 md:pl-8 pr-2 md:pr-4 bg-cyan-950 border-t border-b border-cyan-800 text-white text-sm md:text-lg font-medium text-center focus:outline-none focus:ring-2 focus:ring-orange-500"
                />
              </div>
              <button
                onClick={() => onMonthlyContributionChange(monthlyContribution + 50)}
                className="flex items-center justify-center w-8 md:w-10 h-12 bg-cyan-950 rounded-r-lg border border-cyan-800 text-white hover:bg-cyan-900 transition-colors text-lg"
              >
                +
              </button>
            </div>
          </div>
        </div>

        {/* Years Dropdown */}
        <div className="col-span-2 md:col-span-1 lg:col-span-2 xl:col-span-2 mx-auto md:mx-0 lg:mx-auto xl:mx-auto w-full md:max-w-none lg:max-w-48 xl:max-w-48">
          <label className="flex text-[10px] sm:text-[12px] font-manrope font-medium text-white/75 mb-2 text-center md:text-left lg:text-center xl:text-center">
            Time period
          </label>
          <select
            value={years}
            onChange={(e) => onYearsChange(parseInt(e.target.value))}
            className="w-full h-12 px-4 bg-cyan-950 border border-cyan-800 rounded-lg text-white text-sm md:text-lg font-medium focus:outline-none focus:ring-2 focus:ring-orange-500 appearance-none"
            style={{
              backgroundImage: `url("data:image/svg+xml;charset=utf-8,%3Csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3E%3Cpath stroke='%23fff' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='M6 8l4 4 4-4'/%3E%3C/svg%3E")`,
              backgroundPosition: 'right 0.5rem center',
              backgroundRepeat: 'no-repeat',
              backgroundSize: '1.5em 1.5em'
            }}
          >
            {yearOptions.map((yearOption) => (
              <option key={yearOption} value={yearOption} className="bg-cyan-950">
                {yearOption} years
              </option>
            ))}
          </select>
        </div>
      </div>
    </div>
  );
}