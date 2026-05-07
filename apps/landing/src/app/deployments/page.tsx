'use client';

import { Suspense, useCallback, useEffect, useMemo, useRef, useState } from 'react';
import { useRouter, useSearchParams } from 'next/navigation';
import Navbar from '@/components/globals/navbar/Navbar';
import Footer from '@/components/globals/Footer';
import BackgroundLayers from '@/components/vaults/BackgroundLayers';
import PageHeader from '@/components/vaults/PageHeader';
import FilterBar from '@/components/vaults/FilterBar';
import VaultsTable, { VAULT_ADDRESSES } from '@/components/vaults/VaultsTable';
import StrategiesTable from '@/components/vaults/StrategiesTable';
import { useVaults } from '@/hooks/useVaultInfo';
import { useStrategies } from '@/hooks/useStrategies';
import { stroopsToNum } from '@/utils/vaultFormatters';
import { formatVaultName, getPartnerInfo } from '@/lib/vaultLogos';
import type { ManagedFunds } from '@/types/vault.types';

type Tab = 'partners' | 'strategies';
type SortKey = 'TVL' | 'APY' | 'Name';

function DeploymentsContent() {
  const router = useRouter();
  const searchParams = useSearchParams();

  const rawTab = searchParams.get('tab');
  const tab: Tab = rawTab === 'strategies' ? 'strategies' : 'partners';

  const [search, setSearch] = useState('');
  const [sort, setSort] = useState<SortKey>('TVL');
  const [updatedAt, setUpdatedAt] = useState<Date | null>(null);
  const updatedAtRef = useRef(updatedAt);
  updatedAtRef.current = updatedAt;

  const { sortedVaults } = useVaults({ vaultIds: VAULT_ADDRESSES });
  const { strategies } = useStrategies();

  // Update timestamp when data arrives
  useEffect(() => {
    if (sortedVaults.length > 0 || strategies.length > 0) {
      setUpdatedAt(new Date());
    }
  }, [sortedVaults.length, strategies.length]);

  const setTab = useCallback(
    (t: Tab) => router.push(`/deployments?tab=${t}`),
    [router]
  );

  // Compute stat strip values
  const totalTvl = useMemo(() => {
    return sortedVaults.reduce((sum, v) => {
      const funds = v.totalManagedFunds as ManagedFunds[];
      if (!funds?.length) return sum;
      return sum + stroopsToNum(funds[0].total_amount);
    }, 0);
  }, [sortedVaults]);

  const partnerCount = useMemo(() => {
    const names = new Set(
      sortedVaults.map(v => getPartnerInfo(formatVaultName(v.name)).name)
    );
    return names.size;
  }, [sortedVaults]);

  const avgApy = useMemo(() => {
    if (strategies.length === 0) {
      if (sortedVaults.length === 0) return null;
      const total = sortedVaults.reduce((s, v) => s + (v.apy ?? 0), 0);
      return total / sortedVaults.length;
    }
    const live = strategies.filter(s => s.apy7d != null);
    if (live.length === 0) return null;
    return live.reduce((s, st) => s + (st.apy7d ?? 0), 0) / live.length;
  }, [strategies, sortedVaults]);

  // Format "updated X ago"
  const updatedLabel = useMemo(() => {
    if (!updatedAt) return null;
    const diffMs = Date.now() - updatedAt.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    if (diffMin < 1) return 'Updated just now';
    if (diffMin === 1) return 'Updated 1 min ago';
    return `Updated ${diffMin} min ago`;
  }, [updatedAt]);

  const handleTabKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowRight') setTab('strategies');
    if (e.key === 'ArrowLeft') setTab('partners');
  };

  const ACCENT = '#D9F99D';

  const tabStyle = {
    background: 'rgba(29,57,62,.5)',
    border: '1px solid rgba(193,200,201,.12)',
    borderRadius: 999,
    padding: 4,
    gap: 2,
    backdropFilter: 'blur(8px)',
    WebkitBackdropFilter: 'blur(8px)',
  } as const;

  const makeTabBtnStyle = (active: boolean) => ({
    all: 'unset' as const,
    cursor: 'pointer',
    padding: '8px 18px',
    borderRadius: 999,
    display: 'inline-flex',
    alignItems: 'center',
    gap: 8,
    fontFamily: 'Inter Tight, sans-serif',
    fontSize: 14,
    fontWeight: 600,
    color: active ? ACCENT : 'rgba(255,255,255,.6)',
    background: active ? `${ACCENT}1A` : 'transparent',
    transition: 'all 200ms cubic-bezier(0.4,0,0.2,1)',
  });

  const makeCountStyle = (active: boolean) => ({
    fontSize: 11,
    fontWeight: 700,
    padding: '1px 7px',
    borderRadius: 99,
    background: active ? `${ACCENT}33` : 'rgba(255,255,255,.06)',
    color: active ? ACCENT : 'rgba(255,255,255,.45)',
    fontFeatureSettings: '"tnum"' as const,
  });

  return (
    <div style={{ minHeight: '100vh', position: 'relative', color: '#fff' }}>
      <BackgroundLayers />

      <div style={{ position: 'relative', zIndex: 1 }}>
        <Navbar />

        <div style={{ maxWidth: 1240, margin: '0 auto', padding: '0 32px' }}>
          <PageHeader
            tab={tab}
            partnerCount={partnerCount}
            strategyCount={strategies.length}
            totalTvl={totalTvl}
            avgApy={avgApy}
          />

          {/* Tab bar + updated label */}
          <div
            style={{
              marginTop: 8,
              marginBottom: 24,
              display: 'flex',
              alignItems: 'flex-end',
              justifyContent: 'space-between',
              gap: 24,
            }}
          >
            <div
              role="tablist"
              aria-label="View"
              style={{ display: 'inline-flex', ...tabStyle }}
              onKeyDown={handleTabKeyDown}
            >
              {(['partners', 'strategies'] as Tab[]).map(t => {
                const active = tab === t;
                const count = t === 'partners' ? partnerCount : strategies.length;
                const label = t === 'partners' ? 'Partners' : 'Strategies';
                return (
                  <button
                    key={t}
                    role="tab"
                    aria-selected={active}
                    onClick={() => setTab(t)}
                    style={makeTabBtnStyle(active)}
                    onFocus={e => ((e.target as HTMLButtonElement).style.outline = '4px solid rgba(217,249,157,.40)')}
                    onBlur={e => ((e.target as HTMLButtonElement).style.outline = 'none')}
                  >
                    {label}
                    <span style={makeCountStyle(active)}>{count}</span>
                  </button>
                );
              })}
            </div>

            {updatedLabel && (
              <div style={{ fontSize: 12, color: 'rgba(255,255,255,.4)' }}>
                {updatedLabel}
              </div>
            )}
          </div>

          <FilterBar onSearch={setSearch} sort={sort} onSort={setSort} />

          {tab === 'partners' ? (
            <VaultsTable search={search} sort={sort} />
          ) : (
            <StrategiesTable search={search} sort={sort} />
          )}

          <div style={{ height: 80 }} />
        </div>
      </div>

      <Footer />
    </div>
  );
}

export default function DeploymentsPage() {
  return (
    <Suspense>
      <DeploymentsContent />
    </Suspense>
  );
}
