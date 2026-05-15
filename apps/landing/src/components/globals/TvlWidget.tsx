"use client";

import { useMemo } from "react";
import Link from "next/link";
import { useVaults } from "@/hooks/useVaultInfo";
import { useTokenPrices } from "@/hooks/useTokenPrices";
import { stroopsToNum, fmtUsd } from "@/utils/vaultFormatters";
import { formatVaultName, getPartnerInfo } from "@/lib/vaultLogos";
import { VAULT_ADDRESSES } from "@/components/vaults/VaultsTable";
import type { ManagedFunds } from "@/types/vault.types";

const GLASS_CARD = {
    background: "rgba(29,57,62,.40)",
    border: "1px solid rgba(193,200,201,.10)",
    borderRadius: 20,
    backdropFilter: "blur(14px)",
    WebkitBackdropFilter: "blur(14px)",
} as const;

function StatItem({
    label,
    value,
    highlight,
    loading,
}: {
    label: string;
    value: string;
    highlight?: boolean;
    loading?: boolean;
}) {
    return (
        <div style={{ textAlign: "center" }}>
            <div
                style={{
                    fontSize: 12,
                    fontWeight: 500,
                    color: "rgba(255,255,255,.4)",
                    letterSpacing: "0.08em",
                    textTransform: "uppercase",
                    marginBottom: 8,
                }}
            >
                {label}
            </div>
            {loading ? (
                <div
                    style={{
                        height: 36,
                        width: 120,
                        background: "rgba(255,255,255,.08)",
                        borderRadius: 8,
                        margin: "0 auto",
                        animation: "pulse 1.4s linear infinite",
                    }}
                />
            ) : (
                <div
                    style={{
                        fontFamily: "Familjen Grotesk, sans-serif",
                        fontSize: "clamp(24px, 3vw, 36px)",
                        fontWeight: 700,
                        color: highlight ? "#D9F99D" : "#fff",
                        letterSpacing: "-0.02em",
                        fontFeatureSettings: '"tnum"',
                        lineHeight: 1,
                    }}
                >
                    {value}
                </div>
            )}
        </div>
    );
}

function Divider() {
    return (
        <div
            className="hidden sm:block"
            style={{
                width: 1,
                height: 48,
                background: "rgba(255,255,255,.10)",
                alignSelf: "center",
            }}
        />
    );
}

export default function TvlWidget() {
    const { sortedVaults, isAnyLoading } = useVaults({ vaultIds: VAULT_ADDRESSES });
    const { prices } = useTokenPrices();

    const { totalTvl, partnerCount } = useMemo(() => {
        const tvl = sortedVaults.reduce((sum, v) => {
            const funds = v.totalManagedFunds as ManagedFunds[];
            if (!funds?.length) return sum;
            const amount = stroopsToNum(funds[0].total_amount);
            const price = prices[funds[0].asset] ?? 1;
            return sum + amount * price;
        }, 0);

        const names = new Set(
            sortedVaults.map((v) => getPartnerInfo(formatVaultName(v.name)).name)
        );

        return { totalTvl: tvl, partnerCount: names.size };
    }, [sortedVaults, prices]);

    const loading = isAnyLoading && sortedVaults.length === 0;

    return (
        <section className="py-6 md:py-10 px-4">
            <div className="max-w-[1180px] mx-auto">
                <div
                    style={GLASS_CARD}
                    className="px-8 py-7 sm:px-12 sm:py-8"
                >
                    <div className="flex flex-col sm:flex-row items-center justify-between gap-6 sm:gap-0">
                        {/* Left label */}
                        <div className="sm:flex-none text-center sm:text-left" style={{ minWidth: 160 }}>
                            <div
                                style={{
                                    fontSize: 11,
                                    fontWeight: 600,
                                    color: "#FC5B31",
                                    letterSpacing: "0.12em",
                                    textTransform: "uppercase",
                                    marginBottom: 4,
                                }}
                            >
                                Live on Stellar
                            </div>
                            <div
                                style={{
                                    fontFamily: "Familjen Grotesk, sans-serif",
                                    fontSize: 18,
                                    fontWeight: 700,
                                    color: "#fff",
                                    letterSpacing: "-0.01em",
                                }}
                            >
                                Protocol Metrics
                            </div>
                        </div>

                        {/* Stats */}
                        <div className="flex flex-row items-center gap-8 sm:gap-16 flex-1 justify-center">
                            <StatItem
                                label="Total TVL"
                                value={fmtUsd(totalTvl)}
                                highlight
                                loading={loading}
                            />
                            <Divider />
                            <StatItem
                                label="Active Partners"
                                value={loading ? "—" : String(partnerCount)}
                                loading={loading}
                            />
                            <Divider />
                            <StatItem
                                label="Vaults"
                                value={String(VAULT_ADDRESSES.length)}
                            />
                        </div>

                        {/* CTA */}
                        <div className="sm:flex-none text-center sm:text-right" style={{ minWidth: 160 }}>
                            <Link
                                href="/partners"
                                style={{
                                    display: "inline-flex",
                                    alignItems: "center",
                                    gap: 6,
                                    fontSize: 13,
                                    fontWeight: 600,
                                    color: "rgba(255,255,255,.7)",
                                    textDecoration: "none",
                                    letterSpacing: "0.01em",
                                    transition: "color 0.2s",
                                    fontFamily: "Inter Tight, sans-serif",
                                }}
                                onMouseEnter={(e) => {
                                    (e.currentTarget as HTMLAnchorElement).style.color = "#fff";
                                }}
                                onMouseLeave={(e) => {
                                    (e.currentTarget as HTMLAnchorElement).style.color =
                                        "rgba(255,255,255,.7)";
                                }}
                            >
                                View all partners
                                <svg
                                    width="14"
                                    height="14"
                                    viewBox="0 0 14 14"
                                    fill="none"
                                    xmlns="http://www.w3.org/2000/svg"
                                    style={{ flexShrink: 0 }}
                                >
                                    <path
                                        d="M2.917 7h8.166M7.583 4l3.5 3-3.5 3"
                                        stroke="currentColor"
                                        strokeWidth="1.4"
                                        strokeLinecap="round"
                                        strokeLinejoin="round"
                                    />
                                </svg>
                            </Link>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    );
}
