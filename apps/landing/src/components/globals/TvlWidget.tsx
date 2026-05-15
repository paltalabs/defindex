"use client";

import { useMemo } from "react";
import Link from "next/link";
import { useVaults } from "@/hooks/useVaultInfo";
import { useTokenPrices } from "@/hooks/useTokenPrices";
import { VAULT_ADDRESSES } from "@/components/vaults/VaultsTable";
import { stroopsToNum } from "@/utils/vaultFormatters";
import { formatVaultName, getPartnerInfo } from "@/lib/vaultLogos";
import type { ManagedFunds } from "@/types/vault.types";

function formatTvl(value: number): string {
  if (value >= 1_000_000) return `$${(value / 1_000_000).toFixed(2)}M`;
  if (value >= 1_000) return `$${(value / 1_000).toFixed(1)}K`;
  return `$${value.toFixed(0)}`;
}

function StatCard({
  label,
  value,
  highlight,
  loading,
}: {
  label: string;
  value: string;
  highlight?: boolean;
  loading: boolean;
}) {
  return (
    <div className="flex flex-col items-center justify-center py-6 px-4 sm:px-6 flex-1 min-w-0">
      {loading ? (
        <div className="h-8 sm:h-10 w-24 sm:w-32 rounded-lg bg-white/10 animate-pulse mb-2" />
      ) : (
        <span
          className={`font-familjen-grotesk font-bold text-2xl sm:text-3xl md:text-4xl leading-none mb-1 ${
            highlight ? "text-lime-300" : "text-white"
          }`}
        >
          {value}
        </span>
      )}
      <span className="font-inter text-xs sm:text-sm text-white/50 uppercase tracking-widest text-center">
        {label}
      </span>
    </div>
  );
}

export default function TvlWidget() {
  const { sortedVaults, isAnyLoading } = useVaults({ vaultIds: VAULT_ADDRESSES });
  const { prices, isLoading: pricesLoading } = useTokenPrices();

  const loading = isAnyLoading || pricesLoading;

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
      sortedVaults.map((v) => getPartnerInfo(formatVaultName(v.name)).name)
    );
    return names.size;
  }, [sortedVaults]);

  return (
    <section className="py-6 sm:py-10 px-4 sm:px-6">
      <div
        className="mx-auto w-full rounded-2xl sm:rounded-3xl overflow-hidden"
        style={{
          maxWidth: 900,
          background: "rgba(255,255,255,0.04)",
          border: "1px solid rgba(255,255,255,0.10)",
          backdropFilter: "blur(12px)",
        }}
      >
        {/* Stats row — stacks on mobile, side-by-side on sm+ */}
        <div className="flex flex-col sm:flex-row divide-y sm:divide-y-0 sm:divide-x divide-white/10">
          <StatCard
            label="Total Value Locked"
            value={formatTvl(totalTvl)}
            highlight
            loading={loading}
          />
          <StatCard
            label="Active Partners"
            value={String(partnerCount || "—")}
            loading={loading}
          />
          <StatCard
            label="Vaults"
            value={String(sortedVaults.length || "—")}
            loading={loading}
          />
        </div>

        {/* Footer link */}
        <div
          className="px-4 sm:px-8 py-3 sm:py-4 flex justify-center sm:justify-end"
          style={{ borderTop: "1px solid rgba(255,255,255,0.08)" }}
        >
          <Link
            href="/partners"
            className="font-inter-tight text-xs sm:text-sm text-white/40 hover:text-lime-300 transition-colors duration-200"
          >
            View all partners →
          </Link>
        </div>
      </div>
    </section>
  );
}
