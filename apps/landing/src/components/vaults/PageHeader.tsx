'use client';

import { fmtUsd, fmtApy } from '@/utils/vaultFormatters';

type PageHeaderProps = {
  strategyCount: number;
  totalTvl: number;
  avgApy: number | null;
} & (
  | { tab: 'partners'; partnerCount: number }
  | { tab: 'strategies'; partnerCount?: never }
);

const COPY = {
  partners: {
    pill: 'DeFindex Partners',
    titlePre: 'DeFindex',
    titleEm: 'Partners',
    sub: (partnerCount: number) => (
      <>
        Wallets and apps already{' '}
        <em style={{ fontStyle: 'italic', color: '#FC5B31' }}>integrating</em> DeFindex.{' '}
        <strong>{partnerCount}</strong> brands earning yield for their users.
      </>
    ),
  },
  strategies: {
    pill: 'DeFindex Strategies',
    titlePre: 'DeFindex',
    titleEm: 'Strategies',
    sub: (strategyCount: number) => (
      <>
        The yield{' '}
        <em style={{ fontStyle: 'italic', color: '#FC5B31' }}>engines</em> powering every partner vault.{' '}
        <strong>{strategyCount}</strong> on-chain strategies — auto-compounded, audited, composable.
      </>
    ),
  },
} as const;

function Stat({ label, value, highlight }: { label: string; value: string; highlight?: boolean }) {
  return (
    <div>
      <div
        style={{
          fontSize: 12,
          fontWeight: 500,
          color: 'rgba(255,255,255,.4)',
          letterSpacing: '0.04em',
          textTransform: 'uppercase',
        }}
      >
        {label}
      </div>
      <div
        style={{
          fontFamily: 'Familjen Grotesk, sans-serif',
          fontSize: 28,
          fontWeight: 700,
          color: highlight ? '#D9F99D' : '#fff',
          marginTop: 6,
          fontFeatureSettings: '"tnum"',
          letterSpacing: '-0.02em',
        }}
      >
        {value}
      </div>
    </div>
  );
}

function StatDivider() {
  return (
    <div
      className="hidden sm:block"
      style={{ width: 1, alignSelf: 'stretch', background: 'rgba(255,255,255,.08)' }}
    />
  );
}

export default function PageHeader({ tab, partnerCount, strategyCount, totalTvl, avgApy }: PageHeaderProps) {
  const copy = COPY[tab];
  const subtitle = tab === 'partners' ? copy.sub(partnerCount) : copy.sub(strategyCount);

  return (
    <div style={{ position: 'relative', padding: '40px 0 32px' }} className="sm:pt-16 sm:pb-10">

      {/* H1 */}
      <h1
        key={`h1-${tab}`}
        className="text-[36px] sm:text-[52px] lg:text-[64px]"
        style={{
          fontFamily: 'Familjen Grotesk, sans-serif',
          fontWeight: 700,
          color: '#fff',
          margin: 0,
          letterSpacing: '-0.025em',
          lineHeight: 1.02,
          animation: 'subSlide 280ms cubic-bezier(0.4,0,0.2,1) both',
        }}
      >
        {copy.titlePre}{' '}
        <em style={{ fontStyle: 'italic', color: '#FC5B31', fontWeight: 700 }}>
          {copy.titleEm}
        </em>
      </h1>

      {/* Subtitle */}
      <p
        key={`p-${tab}`}
        className="text-[15px] sm:text-[17px]"
        style={{
          color: 'rgba(255,255,255,.6)',
          maxWidth: 640,
          marginTop: 16,
          marginBottom: 0,
          lineHeight: 1.5,
          animation: 'subSlide 320ms cubic-bezier(0.4,0,0.2,1) both',
          fontFamily: 'Inter Tight, sans-serif',
        }}
      >
        {subtitle}
      </p>

      {/* Stat strip — 2×2 grid on mobile, single row on sm+ */}
      <div
        className="grid grid-cols-2 gap-x-6 gap-y-6 mt-8 sm:flex sm:gap-12 sm:mt-9 sm:items-center"
      >
        <Stat label="Strategies" value={String(strategyCount)} />
        <StatDivider />
        <Stat label="Total TVL" value={fmtUsd(totalTvl)} />
        {tab === 'partners' && partnerCount !== undefined && (
          <>
            <StatDivider />
            <Stat label="Active partners" value={String(partnerCount)} />
          </>
        )}
      </div>
    </div>
  );
}
