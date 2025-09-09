"use client";

import { formatCurrency, formatNumber, InvestmentData } from '@/utils/investmentCalculations';
import { useState } from 'react';
import {
  Line,
  LineChart,
  ReferenceDot,
  ResponsiveContainer,
  Tooltip,
  XAxis,
  YAxis,
} from 'recharts';

interface InvestmentChartProps {
  data: InvestmentData[];
  years: number;
}

interface CustomTooltipProps {
  active?: boolean;
  payload?: Array<{
    value: number;
    dataKey: string;
    color: string;
  }>;
  label?: number;
}

function CustomTooltip({ active, payload, label }: CustomTooltipProps) {
  if (active && payload && payload.length) {
    const projectedTotal = payload.find(p => p.dataKey === 'projectedTotal')?.value || 0;
    const totalInvested = payload.find(p => p.dataKey === 'totalInvested')?.value || 0;

    return (
      <div className="bg-black/90 border border-white/20 rounded-lg p-3 backdrop-blur-sm">
        <p className="text-white font-bold mb-1">{label}</p>
        <div className="space-y-1">
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 bg-white rounded-full"></div>
            <span className="text-white text-sm">
              Projected total: {formatNumber(projectedTotal)}
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-3 h-3 border-2 border-white/60 bg-transparent rounded-full"></div>
            <span className="text-white text-sm">
              Total invested: {formatNumber(totalInvested)}
            </span>
          </div>
        </div>
      </div>
    );
  }
  return null;
}

export default function InvestmentChart({ data, years }: InvestmentChartProps) {
  const [hoveredData, setHoveredData] = useState<InvestmentData | null>(null);

  const finalData = data[data.length - 1];
  const displayData = hoveredData || finalData;

  const handleMouseMove = (state: any) => {
    if (state && state.activePayload && state.activePayload[0]) {
      const activeData = state.activePayload[0].payload;
      setHoveredData(activeData);
    }
  };

  const handleMouseLeave = () => {
    setHoveredData(null);
  };

  // Create custom gradient definition
  const gradientId = "investmentGradient";

  return (
    <div 
      style={{ background: 'linear-gradient(-115deg, rgba(4, 74, 84, 1) 0%, rgba(3, 48, 54, 1) 100%)' }}
      className="w-full rounded-3xl p-6 md:p-8 relative overflow-hidden">

      {/* Header Information */}
      <div className="mb-6 relative z-10">
        <h3 className="text-white text-sm md:text-base font-manrope mb-3">
          In {displayData?.year || years} years you will have
        </h3>
        <div className="text-white text-[48px] md:text-2xl lg:text-3xl font-bold mb-3 break-words">
          {formatCurrency(displayData?.projectedTotal || 0)}
        </div>
        <div className="text-white/90 space-y-1 text-sm md:text-base">
          <p>
            Invested: {formatCurrency(displayData?.totalInvested || 0)}
          </p>
          <p>
            +{formatCurrency(displayData?.earnings || 0)} earned
          </p>
        </div>
      </div>

      {/* Chart Container */}
      <div className="h-48 md:h-64 relative">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart
            data={data}
            onMouseMove={handleMouseMove}
            onMouseLeave={handleMouseLeave}
            margin={{ top: 20, right: 30, left: 20, bottom: 60 }}
          >
            <defs>
              <linearGradient id={gradientId} x1="0" y1="0" x2="0" y2="1">
                <stop offset="0%" stopColor="#014751" stopOpacity={1} />
                <stop offset="100%" stopColor="#033036" stopOpacity={1} />
              </linearGradient>
            </defs>
            
            <XAxis 
              dataKey="year"
              axisLine={false}
              tickLine={false}
              tick={{ fill: 'white', fontSize: 12 }}
              tickFormatter={(value) => `${value}Y`}
              interval="preserveStartEnd"
            />
            <YAxis hide />
            
            <Tooltip content={<CustomTooltip />} />
            
            {/* Projected Total Line (Solid) */}
            <Line
              type="monotone"
              dataKey="projectedTotal"
              stroke="white"
              strokeWidth={3}
              dot={false}
              activeDot={{
                r: 6,
                fill: 'white',
                stroke: 'rgba(255,255,255,0.5)',
                strokeWidth: 2
              }}
            />
            
            {/* Total Invested Line (Dashed) */}
            <Line
              type="monotone"
              dataKey="totalInvested"
              stroke="rgba(255,255,255,0.6)"
              strokeWidth={2}
              strokeDasharray="5 5"
              dot={false}
              activeDot={{
                r: 4,
                fill: 'rgba(255,255,255,0.6)',
                stroke: 'rgba(255,255,255,0.3)',
                strokeWidth: 2
              }}
            />

            {/* Reference dot for hovered point */}
            {hoveredData && (
              <ReferenceDot
                x={hoveredData.year}
                y={hoveredData.projectedTotal}
                r={6}
                fill="white"
                stroke="rgba(255,255,255,0.5)"
                strokeWidth={2}
              />
            )}
          </LineChart>
        </ResponsiveContainer>
      </div>
      
      {/* Disclaimer */}
      <div className="absolute bottom-5 right-2 text-[12px] text-gray-400 max-w-64 leading-tight text-pretty">
        *Calculation based on compound interest strategy with daily reinvestment (24h)
      </div>
      
      {/* Bottom gradient overlay for visual effect */}
      <div className="absolute inset-0 bg-gradient-to-t from-cyan-800/30 to-transparent pointer-events-none rounded-3xl"></div>
    </div>
  );
}