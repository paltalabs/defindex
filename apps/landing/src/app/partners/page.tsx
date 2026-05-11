'use client';

import { Suspense, useCallback, useEffect, useMemo, useState } from 'react';
import Navbar from '@/components/globals/navbar/Navbar';
import Footer from '@/components/globals/Footer';
import BackgroundLayers from '@/components/vaults/BackgroundLayers';
import PageHeader from '@/components/vaults/PageHeader';
import FilterBar from '@/components/vaults/FilterBar';
import VaultsTable, { VAULT_ADDRESSES } from '@/components/vaults/VaultsTable';
import { useVaults } from '@/hooks/useVaultInfo';
import { useStrategies } from '@/hooks/useStrategies';
import { stroopsToNum } from '@/utils/vaultFormatters';
import { formatVaultName, getPartnerInfo } from '@/lib/vaultLogos';
import { useTokenPrices } from '@/hooks/useTokenPrices';
import type { ManagedFunds, SortKey } from '@/types/vault.types';

function PartnersContent() {
  const [search, setSearch] = useState('');
  const [sort, setSort] = useState<SortKey>('TVL');
  const [updatedAt, setUpdatedAt] = useState<Date | null>(null);
  const { sortedVaults } = useVaults({ vaultIds: VAULT_ADDRESSES });
  const { strategies } = useStrategies();
  const { prices } = useTokenPrices();

  useEffect(() => {
    if (sortedVaults.length > 0 || strategies.length > 0) {
      setUpdatedAt(new Date());
    }
  }, [sortedVaults.length, strategies.length]);

  const totalTvl = useMemo(() => {
    return sortedVaults.reduce((sum, v) => {
      const funds = v.totalManagedFunds as ManagedFunds[];
      if (!funds?.length) return sum;
      const amount = stroopsToNum(funds[0].total_amount);
      const price = prices[funds[0].asset] ?? 1;
      return sum + amount * price;
    }, 0);
  }, [sortedVaults, prices]);

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

  const updatedLabel = useMemo(() => {
    if (!updatedAt) return null;
    const diffMs = Date.now() - updatedAt.getTime();
    const diffMin = Math.floor(diffMs / 60000);
    if (diffMin < 1) return 'Updated just now';
    if (diffMin === 1) return 'Updated 1 min ago';
    return `Updated ${diffMin} min ago`;
  }, [updatedAt]);

  const handleSearch = useCallback((q: string) => setSearch(q), []);
  const handleSort = useCallback((s: SortKey) => setSort(s), []);

  return (
    <div style={{ minHeight: '100vh', position: 'relative', color: '#fff' }}>
      <BackgroundLayers />

      <div style={{ position: 'relative', zIndex: 1 }}>
        <Navbar />

        <div style={{ maxWidth: 1240, margin: '0 auto' }} className="px-4 sm:px-8">
          <PageHeader
            tab="partners"
            partnerCount={partnerCount}
            strategyCount={strategies.length}
            totalTvl={totalTvl}
            avgApy={avgApy}
          />

          <div
            style={{
              marginTop: 8,
              marginBottom: 24,
              display: 'flex',
              justifyContent: 'flex-end',
            }}
          >
            {updatedLabel && (
              <div style={{ fontSize: 12, color: 'rgba(255,255,255,.4)' }}>
                {updatedLabel}
              </div>
            )}
          </div>

          <FilterBar onSearch={handleSearch} sort={sort} onSort={handleSort} />

          <VaultsTable search={search} sort={sort} />

          <div style={{ height: 80 }} />
        </div>
      </div>

      <Footer />
    </div>
  );
}

export default function PartnersPage() {
  return (
    <Suspense>
      <PartnersContent />
    </Suspense>
  );
}
